// swed/src/semantic.rs
// Semantic Analysis Pass
//
// Walks the AST and:
//   1. Populates ScopeEnv with variable declarations.
//   2. Validates function calls against SymbolTable.
//   3. Annotates identifiers with their resolved scope.
//   4. Emits Diagnostics (warnings/errors) for:
//      - Undeclared variables (auto-PRIVATE in Harbour runtime).
//      - Wrong arity on built-in calls.
//      - Unreachable code after RETURN (warning only).

use crate::ast::*;
use crate::scope::{HbType, ScopeEnv};
use crate::symbol_table::SymbolTable;

// ---------------------------------------------------------------------------
// Diagnostic
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagLevel,
    pub message: String,
    pub span: SrcSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagLevel {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for DiagLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagLevel::Error   => write!(f, "error"),
            DiagLevel::Warning => write!(f, "warning"),
            DiagLevel::Info    => write!(f, "info"),
        }
    }
}

// ---------------------------------------------------------------------------
// Analyzer
// ---------------------------------------------------------------------------

pub struct Analyzer<'a> {
    sym: &'a SymbolTable,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a> Analyzer<'a> {
    pub fn new(sym: &'a SymbolTable) -> Self {
        Analyzer {
            sym,
            diagnostics: Vec::new(),
        }
    }

    fn warn(&mut self, msg: impl Into<String>, span: SrcSpan) {
        self.diagnostics.push(Diagnostic {
            level: DiagLevel::Warning,
            message: msg.into(),
            span,
        });
    }

    fn error(&mut self, msg: impl Into<String>, span: SrcSpan) {
        self.diagnostics.push(Diagnostic {
            level: DiagLevel::Error,
            message: msg.into(),
            span,
        });
    }

    // -----------------------------------------------------------------------
    // Entry point
    // -----------------------------------------------------------------------

    pub fn analyze(&mut self, program: &Program) {
        for unit in &program.units {
            match unit {
                TopLevel::Procedure(p) => self.analyze_proc(p),
                TopLevel::Function(f)  => self.analyze_func(f),
                TopLevel::Class(c)     => self.analyze_class(c),
            }
        }
    }

    // -----------------------------------------------------------------------
    // Procedures / Functions
    // -----------------------------------------------------------------------

    fn analyze_proc(&mut self, proc: &ProcDef) {
        let mut env = ScopeEnv::new(&proc.name);
        for param in &proc.params {
            env.declare_local(&param.name, HbType::Any);
        }
        self.analyze_body(&proc.body, &mut env);
    }

    fn analyze_func(&mut self, func: &FuncDef) {
        let mut env = ScopeEnv::new(&func.name);
        for param in &func.params {
            env.declare_local(&param.name, HbType::Any);
        }
        self.analyze_body(&func.body, &mut env);
    }

    fn analyze_class(&mut self, class: &ClassDef) {
        for method in &class.methods {
            let mut env = ScopeEnv::new(format!("{}:{}", class.name, method.name));
            for param in &method.params {
                env.declare_local(&param.name, HbType::Any);
            }
            self.analyze_body(&method.body, &mut env);
        }
    }

    // -----------------------------------------------------------------------
    // Body / Statements
    // -----------------------------------------------------------------------

    fn analyze_body(&mut self, stmts: &[Stmt], env: &mut ScopeEnv) {
        let mut seen_return = false;
        for stmt in stmts {
            if seen_return {
                self.warn("Unreachable code after RETURN", stmt.span.clone());
            }
            self.analyze_stmt(stmt, env);
            if matches!(stmt.kind, StmtKind::Return(_)) {
                seen_return = true;
            }
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt, env: &mut ScopeEnv) {
        match &stmt.kind {
            StmtKind::VarDecl(d) => {
                for v in &d.vars {
                    if let Some(init) = &v.init {
                        self.analyze_expr(init, env);
                    }
                    env.declare_local(&v.name, HbType::Any);
                }
            }
            StmtKind::StaticDecl(d) => {
                for v in &d.vars {
                    if let Some(init) = &v.init {
                        self.analyze_expr(init, env);
                    }
                    env.declare_static(&v.name, HbType::Any);
                }
            }
            StmtKind::FieldDecl { names, .. } => {
                for n in names {
                    env.declare_field(n);
                }
            }
            StmtKind::MemvarDecl(names) => {
                for n in names {
                    env.declare_private(n, HbType::Any);
                }
            }
            StmtKind::PublicDecl(d) => {
                for v in &d.vars {
                    env.declare_public(&v.name, HbType::Any);
                    if let Some(init) = &v.init {
                        self.analyze_expr(init, env);
                    }
                }
            }
            StmtKind::Assign(a) => {
                self.analyze_expr(&a.value, env);
                // LHS: if it's a bare ident that's undeclared, auto-declare as PRIVATE
                if let ExprKind::Ident(name) = &a.target.kind {
                    if env.resolve(name).is_none() {
                        self.warn(
                            format!(
                                "Variable `{name}` assigned without declaration; \
                                 auto-creating as PRIVATE (Harbour runtime behaviour)"
                            ),
                            a.target.span.clone(),
                        );
                        env.declare_private(name, HbType::Any);
                    }
                } else {
                    self.analyze_expr(&a.target, env);
                }
            }
            StmtKind::Call(c) => self.analyze_call(c, env),
            StmtKind::Print(e) => self.analyze_expr(e, env),
            StmtKind::If(i) => {
                self.analyze_expr(&i.condition, env);
                self.analyze_body(&i.then_body, env);
                for (cond, body) in &i.elseif_branches {
                    self.analyze_expr(cond, env);
                    self.analyze_body(body, env);
                }
                if let Some(b) = &i.else_body {
                    self.analyze_body(b, env);
                }
            }
            StmtKind::DoWhile(w) => {
                self.analyze_expr(&w.condition, env);
                self.analyze_body(&w.body, env);
            }
            StmtKind::For(f) => {
                self.analyze_expr(&f.start, env);
                self.analyze_expr(&f.end, env);
                if let Some(s) = &f.step {
                    self.analyze_expr(s, env);
                }
                // FOR variable is implicitly LOCAL to the loop
                env.declare_local(&f.var, HbType::Any);
                self.analyze_body(&f.body, env);
            }
            StmtKind::Return(e) => {
                if let Some(expr) = e {
                    self.analyze_expr(expr, env);
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Expressions
    // -----------------------------------------------------------------------

    fn analyze_expr(&mut self, expr: &Expr, env: &mut ScopeEnv) {
        match &expr.kind {
            ExprKind::Ident(name) => {
                if env.resolve(name).is_none() {
                    self.warn(
                        format!("Use of undeclared variable `{name}`"),
                        expr.span.clone(),
                    );
                }
            }
            ExprKind::Call(c) => self.analyze_call(c, env),
            ExprKind::BinOp(_, l, r) => {
                self.analyze_expr(l, env);
                self.analyze_expr(r, env);
            }
            ExprKind::UnOp(_, e) => self.analyze_expr(e, env),
            ExprKind::Index(arr, idx) => {
                self.analyze_expr(arr, env);
                self.analyze_expr(idx, env);
            }
            ExprKind::Field(obj, _) => self.analyze_expr(obj, env),
            ExprKind::ArrayLit(elems) => {
                for e in elems {
                    self.analyze_expr(e, env);
                }
            }
            ExprKind::Iif(cond, then, else_) => {
                self.analyze_expr(cond, env);
                self.analyze_expr(then, env);
                self.analyze_expr(else_, env);
            }
            // Literals and Nil are always valid
            ExprKind::Nil
            | ExprKind::Bool(_)
            | ExprKind::Integer(_)
            | ExprKind::Float(_)
            | ExprKind::String(_)
            | ExprKind::Macro(_) => {}
        }
    }

    fn analyze_call(&mut self, call: &CallExpr, env: &mut ScopeEnv) {
        // Validate each argument first
        for arg in &call.args {
            self.analyze_expr(arg, env);
        }
        // Validate against symbol table (built-ins only for now)
        match self.sym.validate_call(&call.callee, call.args.len()) {
            Ok(_) => {}
            Err(e) => {
                // Not in builtins → could be user-defined; downgrade to warning
                self.warn(format!("{e}"), call.span.clone());
            }
        }
    }

    // -----------------------------------------------------------------------
    // Summary helpers
    // -----------------------------------------------------------------------

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.level == DiagLevel::Error)
    }

    pub fn print_diagnostics(&self, src_name: &str) {
        for d in &self.diagnostics {
            eprintln!(
                "[{}] {}:{}-{}: {}",
                d.level, src_name, d.span.start, d.span.end, d.message
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{normalize, tokenize};
    use crate::parser::parse;
    use crate::symbol_table::SymbolTable;

    fn analyze_src(src: &str) -> Vec<Diagnostic> {
        let norm = normalize(src);
        let toks: Vec<_> = tokenize(&norm).into_iter().filter_map(|(t, _)| t.ok()).collect();
        let prog = parse(toks).expect("parse failed");
        let sym = SymbolTable::with_builtins();
        let mut analyzer = Analyzer::new(&sym);
        analyzer.analyze(&prog);
        analyzer.diagnostics
    }

    #[test]
    fn test_no_diags_for_clean_code() {
        let src = r#"
PROCEDURE Main()
   LOCAL x := 42
   LOCAL y := x + 1
RETURN
"#;
        let diags = analyze_src(src);
        assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    }

    #[test]
    fn test_undeclared_var_warning() {
        let src = "PROCEDURE Main()\n ghost := 1\nRETURN";
        let diags = analyze_src(src);
        assert!(diags.iter().any(|d| d.message.contains("ghost") || d.message.contains("GHOST")));
    }

    #[test]
    fn test_aadd_arity_warning() {
        let src = "PROCEDURE Main()\nLOCAL a := {}\nAAdd(a, 1, 2)\nRETURN";
        let diags = analyze_src(src);
        assert!(diags.iter().any(|d| d.message.contains("AADD") || d.message.contains("AAdd")));
    }

    #[test]
    fn test_unreachable_after_return() {
        let src = "PROCEDURE Main()\nRETURN\nLOCAL x := 1\nRETURN\nRETURN";
        let diags = analyze_src(src);
        assert!(diags.iter().any(|d| d.message.contains("Unreachable")));
    }
}
