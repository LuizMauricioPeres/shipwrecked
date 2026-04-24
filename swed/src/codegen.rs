// swed/src/codegen.rs
// Code Generator: ast::Program → Rust source (String)
//
// Mapping strategy (Harbour → Rust):
//   PROCEDURE p()       →  fn p()  (returns ())
//   FUNCTION  f()       →  fn f() -> HbValue
//   LOCAL x := v        →  let mut x = v;
//   STATIC x := v       →  static X: Mutex<HbValue> = ...  (via OnceCell helper)
//   DO WHILE cond       →  while cond { ... }
//   FOR i := s TO e     →  for i in (s..=e)   (step-1 shortcut)
//   FOR i := s TO e STEP n →  { let mut i=s; while i<=e { ...; i+=n; } }
//   IF/ELSEIF/ELSE      →  if / else if / else
//   AAdd(a, v)          →  a.hb_aadd(v)
//   LEN(x)              →  x.len()
//   ?  expr             →  println!("{}", expr)
//   NIL                 →  HbValue::Nil
//   .T. / .F.           →  HbValue::Logical(true/false)
//   [string]            →  HbValue::String("string".into())
//   { e1, e2 }          →  hb_array![ e1, e2 ]   (macro defined in preamble)

use std::collections::HashSet;
use crate::ast::*;
use crate::scope::to_snake_case;

// ---------------------------------------------------------------------------
// Codegen context
// ---------------------------------------------------------------------------

pub struct Codegen {
    indent: usize,
    out: String,
    /// Nomes PUBLIC (módulo inteiro). Emite `public_store()`.
    public_names: HashSet<String>,
    /// Nomes FIELD da unidade atual. Emite `field_get()`/`field_set()`.
    /// Prioridade: FIELD > MEMVAR/PUBLIC (mas abaixo de LOCAL/STATIC).
    field_names: HashSet<String>,
}

impl Codegen {
    pub fn new() -> Self {
        Codegen {
            indent: 0,
            out: String::new(),
            public_names: HashSet::new(),
            field_names: HashSet::new(),
        }
    }

    // ── Output helpers ───────────────────────────────────────────────────

    fn line(&mut self, s: &str) {
        let pad = "    ".repeat(self.indent);
        self.out.push_str(&pad);
        self.out.push_str(s);
        self.out.push('\n');
    }

    fn blank(&mut self) {
        self.out.push('\n');
    }

    fn push(&mut self, s: &str) {
        self.out.push_str(s);
    }

    fn indent(&mut self)   { self.indent += 1; }
    fn dedent(&mut self)   { if self.indent > 0 { self.indent -= 1; } }

    // -----------------------------------------------------------------------
    // Entry point
    // -----------------------------------------------------------------------

    pub fn generate(&mut self, program: &Program) -> String {
        // Pré-scan: coleta todos os nomes PUBLIC antes de emitir código,
        // para que referências em qualquer função sejam roteadas corretamente.
        self.scan_publics(program);
        self.emit_preamble();

        for unit in &program.units {
            self.blank();
            match unit {
                TopLevel::Procedure(p) => self.emit_proc(p),
                TopLevel::Function(f)  => self.emit_func(f),
                TopLevel::Class(c)     => self.emit_class(c),
            }
        }
        self.out.clone()
    }

    // -----------------------------------------------------------------------
    // Pré-scan: coleta nomes PUBLIC de todo o programa
    // -----------------------------------------------------------------------

    fn scan_publics(&mut self, program: &Program) {
        for unit in &program.units {
            let body: &[Stmt] = match unit {
                TopLevel::Procedure(p) => &p.body,
                TopLevel::Function(f)  => &f.body,
                TopLevel::Class(_)     => continue,
            };
            self.scan_stmts_publics(body);
        }
    }

    fn scan_stmts_publics(&mut self, stmts: &[Stmt]) {
        for s in stmts {
            match &s.kind {
                StmtKind::PublicDecl(d) => {
                    for v in &d.vars {
                        self.public_names.insert(v.name.to_ascii_uppercase());
                    }
                }
                StmtKind::If(i) => {
                    self.scan_stmts_publics(&i.then_body);
                    for (_, b) in &i.elseif_branches { self.scan_stmts_publics(b); }
                    if let Some(b) = &i.else_body { self.scan_stmts_publics(b); }
                }
                StmtKind::DoWhile(w) => self.scan_stmts_publics(&w.body),
                StmtKind::For(f)     => self.scan_stmts_publics(&f.body),
                _ => {}
            }
        }
    }

    /// Coleta nomes FIELD de um corpo de função — redefine `field_names`
    /// a cada unidade (FIELD é local ao escopo em que foi declarado).
    fn collect_field_names(&mut self, body: &[Stmt]) {
        self.field_names.clear();
        self.collect_fields_recursive(body);
    }

    fn collect_fields_recursive(&mut self, stmts: &[Stmt]) {
        for s in stmts {
            match &s.kind {
                StmtKind::FieldDecl { names, .. } => {
                    for n in names {
                        self.field_names.insert(n.to_ascii_uppercase());
                    }
                }
                StmtKind::If(i) => {
                    self.collect_fields_recursive(&i.then_body);
                    for (_, b) in &i.elseif_branches { self.collect_fields_recursive(b); }
                    if let Some(b) = &i.else_body { self.collect_fields_recursive(b); }
                }
                StmtKind::DoWhile(w) => self.collect_fields_recursive(&w.body),
                StmtKind::For(f)     => self.collect_fields_recursive(&f.body),
                _ => {}
            }
        }
    }

    // -----------------------------------------------------------------------
    // Preamble: use statements + HbValue/HbArray re-exports + macros
    // -----------------------------------------------------------------------

    fn emit_preamble(&mut self) {
        self.line("// Generated by SWed (Shipwrecked) — do not edit manually.");
        self.line("#![allow(non_snake_case, unused_mut, unused_variables, unused_imports)]");
        self.blank();
        self.line("use swed_rt::{HbValue, HbArray, field_get, field_set};");
        self.line("use swed_rt::publics_var::public_store;");
        self.blank();
        // Convenience macro for array literals  { 1, 2, 3 } → hb_array![...]
        self.line("macro_rules! hb_array {");
        self.indent();
        self.line("( $($e:expr),* $(,)? ) => {{");
        self.indent();
        self.line("let mut __a = HbArray::new();");
        self.line("$( __a.hb_aadd($e); )*");
        self.line("__a");
        self.dedent();
        self.line("}};");
        self.dedent();
        self.line("}");
    }

    // -----------------------------------------------------------------------
    // Procedure → fn name() { ... }
    // -----------------------------------------------------------------------

    fn emit_proc(&mut self, p: &ProcDef) {
        self.collect_field_names(&p.body);
        let fname = to_snake_case(&p.name);
        if p.name.to_ascii_uppercase() == "MAIN" {
            // Rust entry point: fn main() never takes parameters.
            // Harbour Main params come from the OS command line — bind them here.
            self.line("fn main() {");
            self.indent();
            if !p.params.is_empty() {
                self.line("let mut __args: Vec<HbValue> = std::env::args().skip(1)");
                self.line("    .map(|s| HbValue::String(s)).collect();");
                for (i, param) in p.params.iter().enumerate() {
                    let pname = to_snake_case(&param.name);
                    self.line(&format!(
                        "let mut {pname} = __args.get({i}).cloned().unwrap_or(HbValue::Nil);"
                    ));
                }
            }
        } else {
            let params = self.render_params(&p.params);
            self.line(&format!("fn {fname}({params}) {{"));
            self.indent();
        }
        self.emit_stmts(&p.body);
        self.dedent();
        self.line("}");
    }

    // -----------------------------------------------------------------------
    // Function → fn name(...) -> HbValue { ... }
    // -----------------------------------------------------------------------

    fn emit_func(&mut self, f: &FuncDef) {
        self.collect_field_names(&f.body);
        let params = self.render_params(&f.params);
        let fname = to_snake_case(&f.name);
        self.line(&format!("fn {fname}({params}) -> HbValue {{"));
        self.indent();
        self.emit_stmts(&f.body);
        // Ensure there is always a fallback return
        self.line("HbValue::Nil");
        self.dedent();
        self.line("}");
    }

    fn render_params(&self, params: &[Param]) -> String {
        params
            .iter()
            .map(|p| format!("{}: HbValue", to_snake_case(&p.name)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    // -----------------------------------------------------------------------
    // Class → struct + impl block (skeleton)
    // -----------------------------------------------------------------------

    fn emit_class(&mut self, c: &ClassDef) {
        let sname = &c.name;
        self.line(&format!("// CLASS {sname}"));
        self.line(&format!("struct {sname} {{"));
        self.indent();
        for d in &c.data {
            let field = to_snake_case(&d.name);
            self.line(&format!("{field}: HbValue,"));
        }
        self.dedent();
        self.line("}");
        self.blank();
        self.line(&format!("impl {sname} {{"));
        self.indent();
        // Default constructor
        self.line("fn new() -> Self {");
        self.indent();
        self.line(&format!("{sname} {{"));
        self.indent();
        for d in &c.data {
            let field = to_snake_case(&d.name);
            self.line(&format!("{field}: HbValue::Nil,"));
        }
        self.dedent();
        self.line("}");
        self.dedent();
        self.line("}");
        // Methods
        for m in &c.methods {
            self.blank();
            let params = self.render_params(&m.params);
            let sep = if params.is_empty() { "".to_string() } else { format!(", {params}") };
            let mname = to_snake_case(&m.name);
            self.line(&format!("fn {mname}(&mut self{sep}) -> HbValue {{"));
            self.indent();
            self.emit_stmts(&m.body);
            self.line("HbValue::Nil");
            self.dedent();
            self.line("}");
        }
        self.dedent();
        self.line("}");
    }

    // -----------------------------------------------------------------------
    // Statements
    // -----------------------------------------------------------------------

    fn emit_stmts(&mut self, stmts: &[Stmt]) {
        for s in stmts {
            self.emit_stmt(s);
        }
    }

    fn emit_stmt(&mut self, stmt: &Stmt) {
        let pad = "    ".repeat(self.indent);
        match &stmt.kind {

            StmtKind::VarDecl(d) => {
                for v in &d.vars {
                    let name = to_snake_case(&v.name);
                    let init = v.init.as_ref()
                        .map(|e| self.render_expr(e))
                        .unwrap_or_else(|| "HbValue::Nil".into());
                    self.out.push_str(&format!("{pad}let mut {name} = {init};\n"));
                }
            }

            StmtKind::StaticDecl(d) => {
                // STATIC → thread_local! with RefCell
                for v in &d.vars {
                    let name = to_snake_case(&v.name).to_ascii_uppercase();
                    let init = v.init.as_ref()
                        .map(|e| self.render_expr(e))
                        .unwrap_or_else(|| "HbValue::Nil".into());
                    self.out.push_str(&format!(
                        "{pad}thread_local! {{ static {name}: std::cell::RefCell<HbValue> \
                         = std::cell::RefCell::new({init}); }}\n"
                    ));
                }
            }

            StmtKind::FieldDecl { .. } => {
                // FIELD é um mapeamento para a WorkArea; sem alocação de variável.
            }

            StmtKind::MemvarDecl(names) => {
                for n in names {
                    let rn = to_snake_case(n);
                    self.out.push_str(&format!("{pad}let mut {rn} = HbValue::Nil; // MEMVAR\n"));
                }
            }

            StmtKind::PublicDecl(d) => {
                // PUBLIC → armazém global; inicializador opcional
                for v in &d.vars {
                    let upper = v.name.to_ascii_uppercase();
                    let init = v.init.as_ref()
                        .map(|e| self.render_expr(e))
                        .unwrap_or_else(|| "HbValue::Nil".into());
                    self.out.push_str(&format!(
                        "{pad}public_store().write().unwrap().set(\"{upper}\", {init});\n"
                    ));
                }
            }

            StmtKind::Assign(a) => {
                if let ExprKind::Ident(name) = &a.target.kind {
                    let upper = name.to_ascii_uppercase();
                    // FIELD → escreve na WorkArea (prioridade sobre PUBLIC/MEMVAR)
                    if self.field_names.contains(&upper) {
                        let value = self.render_expr(&a.value);
                        self.out.push_str(&format!(
                            "{pad}field_set(\"{upper}\", {value});\n"
                        ));
                        return;
                    }
                    // PUBLIC → armazém global
                    if self.public_names.contains(&upper) {
                        let value = self.render_expr(&a.value);
                        self.out.push_str(&format!(
                            "{pad}public_store().write().unwrap().set(\"{upper}\", {value});\n"
                        ));
                        return;
                    }
                }
                let target = self.render_expr(&a.target);
                let value  = self.render_expr(&a.value);
                self.out.push_str(&format!("{pad}{target} = {value};\n"));
            }

            StmtKind::Call(c) => {
                let call = self.render_call(c);
                self.out.push_str(&format!("{pad}{call};\n"));
            }

            StmtKind::Print(e) => {
                let val = self.render_expr(e);
                self.out.push_str(&format!("{pad}println!(\"{{}}\", {val});\n"));
            }

            StmtKind::If(i) => {
                let cond = self.render_expr(&i.condition);
                self.out.push_str(&format!("{pad}if {cond} {{\n"));
                self.indent();
                self.emit_stmts(&i.then_body);
                self.dedent();
                for (ec, eb) in &i.elseif_branches {
                    let ec_r = self.render_expr(ec);
                    self.out.push_str(&format!("{pad}}} else if {ec_r} {{\n"));
                    self.indent();
                    self.emit_stmts(eb);
                    self.dedent();
                }
                if let Some(eb) = &i.else_body {
                    self.out.push_str(&format!("{pad}}} else {{\n"));
                    self.indent();
                    self.emit_stmts(eb);
                    self.dedent();
                }
                self.out.push_str(&format!("{pad}}}\n"));
            }

            StmtKind::DoWhile(w) => {
                let cond = self.render_expr(&w.condition);
                self.out.push_str(&format!("{pad}while {cond} {{\n"));
                self.indent();
                self.emit_stmts(&w.body);
                self.dedent();
                self.out.push_str(&format!("{pad}}}\n"));
            }

            StmtKind::For(f) => {
                let start = self.render_expr(&f.start);
                let end   = self.render_expr(&f.end);
                let var   = to_snake_case(&f.var);

                if f.step.is_none() {
                    // Simple STEP 1 case → idiomatic Rust range
                    self.out.push_str(&format!(
                        "{pad}for {var} in hb_range({start}, {end}, 1) {{\n"
                    ));
                } else {
                    let step = self.render_expr(f.step.as_ref().unwrap());
                    self.out.push_str(&format!(
                        "{pad}for {var} in hb_range({start}, {end}, {step}) {{\n"
                    ));
                }
                self.indent();
                self.emit_stmts(&f.body);
                self.dedent();
                self.out.push_str(&format!("{pad}}}\n"));
            }

            StmtKind::Return(e) => {
                match e {
                    Some(expr) => {
                        let val = self.render_expr(expr);
                        self.out.push_str(&format!("{pad}return {val};\n"));
                    }
                    None => self.out.push_str(&format!("{pad}return;\n")),
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Expressions → &str
    // -----------------------------------------------------------------------

    fn render_expr(&self, expr: &Expr) -> String {
        match &expr.kind {
            ExprKind::Nil               => "HbValue::Nil".into(),
            ExprKind::Bool(b)           => format!("HbValue::Logical({b})"),
            ExprKind::Integer(n)        => format!("HbValue::Integer({n})"),
            ExprKind::Float(f)          => format!("HbValue::Float({f})"),
            ExprKind::String(s)         => format!("HbValue::String(\"{}\".into())", s.replace('"', "\\\"")),
            ExprKind::Ident(name) => {
                let upper = name.to_ascii_uppercase();
                if self.field_names.contains(&upper) {
                    // FIELD → leitura dinâmica da WorkArea atual
                    format!("field_get(\"{upper}\")")
                } else if self.public_names.contains(&upper) {
                    format!("public_store().read().unwrap().get(\"{upper}\")")
                } else {
                    to_snake_case(name)
                }
            }
            ExprKind::Macro(name)       => format!("/*&*/hb_macro({})", to_snake_case(name)),

            ExprKind::ArrayLit(elems) => {
                let inner: Vec<String> = elems.iter().map(|e| self.render_expr(e)).collect();
                format!("hb_array![{}]", inner.join(", "))
            }

            ExprKind::Index(arr, idx) => {
                let a = self.render_expr(arr);
                let i = self.render_expr(idx);
                format!("{a}.hb_get_val({i})")
            }

            ExprKind::Field(obj, field) => {
                let o = self.render_expr(obj);
                format!("{o}.{}", to_snake_case(field))
            }

            ExprKind::BinOp(op, l, r) => {
                let lv = self.render_expr(l);
                let rv = self.render_expr(r);
                let op_str = match op {
                    BinOp::Add      => "+",
                    BinOp::Sub      => "-",
                    BinOp::Mul      => "*",
                    BinOp::Div      => "/",
                    BinOp::Mod      => "%",
                    BinOp::Pow      => ".pow_hb",
                    BinOp::Eq | BinOp::StrictEq => "==",
                    BinOp::NotEq    => "!=",
                    BinOp::Lt       => "<",
                    BinOp::Lte      => "<=",
                    BinOp::Gt       => ">",
                    BinOp::Gte      => ">=",
                    BinOp::And      => "&&",
                    BinOp::Or       => "||",
                    BinOp::Concat   => "+", // string concat handled by HbValue Display
                    BinOp::InStr    => "/* $ */",
                };
                if op == &BinOp::Pow {
                    format!("{lv}.pow_hb({rv})")
                } else if op == &BinOp::InStr {
                    format!("hb_instr({lv}, {rv})")
                } else {
                    format!("({lv} {op_str} {rv})")
                }
            }

            ExprKind::UnOp(op, e) => {
                let ev = self.render_expr(e);
                match op {
                    UnOp::Neg => format!("(-{ev})"),
                    UnOp::Not => format!("(!{ev})"),
                }
            }

            ExprKind::Call(c) => self.render_call(c),

            ExprKind::Iif(cond, then, else_) => {
                let c = self.render_expr(cond);
                let t = self.render_expr(then);
                let e = self.render_expr(else_);
                format!("(if {c} {{ {t} }} else {{ {e} }})")
            }
        }
    }

    // -----------------------------------------------------------------------
    // Function/method call rendering
    // -----------------------------------------------------------------------

    fn render_call(&self, call: &CallExpr) -> String {
        let args: Vec<String> = call.args.iter().map(|a| self.render_expr(a)).collect();

        // Well-known Harbour built-ins → idiomatic Rust mappings
        match call.callee.as_str() {
            "AADD" => {
                // AAdd(arr, val) → arr.hb_aadd(val)
                if args.len() == 2 {
                    return format!("{}.hb_aadd({})", args[0], args[1]);
                }
            }
            "LEN" => {
                if args.len() == 1 {
                    return format!("{}.hb_len()", args[0]);
                }
            }
            "ASIZE" => {
                if args.len() == 2 {
                    return format!("{}.hb_asize({})", args[0], args[1]);
                }
            }
            "QOUT" | "?" => {
                let val = args.first().cloned().unwrap_or_else(|| "\"\"".into());
                return format!("println!(\"{{}}\", {val})");
            }
            "ALLTRIM" | "LTRIM" | "RTRIM" => {
                if args.len() == 1 {
                    let method = match call.callee.as_str() {
                        "ALLTRIM" => "trim()",
                        "LTRIM"   => "trim_start()",
                        _         => "trim_end()",
                    };
                    return format!("{}.hb_str_op(\"|{method}|\")", args[0]);
                }
            }
            "STR" => {
                // STR(n) → format as string
                if !args.is_empty() {
                    return format!("hb_str({})", args.join(", "));
                }
            }
            "VAL" => {
                if args.len() == 1 {
                    return format!("hb_val({})", args[0]);
                }
            }
            "SUBSTR" => {
                return format!("hb_substr({})", args.join(", "));
            }
            "AT" => {
                if args.len() == 2 {
                    return format!("hb_at({}, {})", args[0], args[1]);
                }
            }
            _ => {}
        }

        // Generic call
        let fname = to_snake_case(&call.callee.to_ascii_lowercase());
        format!("{fname}({})", args.join(", "))
    }
}

// ---------------------------------------------------------------------------
// Public convenience
// ---------------------------------------------------------------------------

pub fn generate(program: &Program) -> String {
    Codegen::new().generate(program)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{normalize, tokenize};
    use crate::parser::parse;

    fn gen(src: &str) -> String {
        let norm = normalize(src);
        let toks: Vec<_> = tokenize(&norm).into_iter().filter_map(|(t, _)| t.ok()).collect();
        let prog = parse(toks).expect("parse failed");
        generate(&prog)
    }

    #[test]
    fn test_aadd_codegen() {
        // aTeste → to_snake_case → "ateste" (all-caps after normalize, no boundary)
        let src = "PROCEDURE Main()\nLOCAL aTeste := {}\nAAdd( aTeste, 1 )\nRETURN";
        let out = gen(src);
        assert!(out.contains("hb_aadd(HbValue::Integer(1))"), "got:\n{out}");
    }

    #[test]
    fn test_for_loop_codegen() {
        let src = "PROCEDURE Main()\nFOR i := 1 TO 10\nNEXT\nRETURN";
        let out = gen(src);
        assert!(out.contains("hb_range("), "got:\n{out}");
    }

    #[test]
    fn test_if_codegen() {
        let src = "PROCEDURE Main()\nLOCAL x := 1\nIF x == 1\nRETURN\nENDIF\nRETURN";
        let out = gen(src);
        assert!(out.contains("if (x == HbValue::Integer(1))"), "got:\n{out}");
    }

    #[test]
    fn test_func_returns_hbvalue() {
        let src = "FUNCTION Add( a, b )\nRETURN a + b";
        let out = gen(src);
        assert!(out.contains("-> HbValue"), "got:\n{out}");
        assert!(out.contains("return (a + b)"), "got:\n{out}");
    }

    #[test]
    fn test_len_mapping() {
        let src = "PROCEDURE Main()\nLOCAL a := {}\nLOCAL n := LEN(a)\nRETURN";
        let out = gen(src);
        assert!(out.contains("a.hb_len()"), "got:\n{out}");
    }

    #[test]
    fn test_public_decl_uses_store() {
        let src = "PROCEDURE Main()\nPUBLIC nEmpresa\nRETURN";
        let out = gen(src);
        assert!(out.contains("public_store().write().unwrap().set(\"NEMPRESA\", HbValue::Nil)"), "got:\n{out}");
    }

    #[test]
    fn test_public_decl_with_init() {
        let src = "PROCEDURE Main()\nPUBLIC lRecados := .T.\nRETURN";
        let out = gen(src);
        assert!(out.contains("public_store().write().unwrap().set(\"LRECADOS\", HbValue::Logical(true))"), "got:\n{out}");
    }

    #[test]
    fn test_public_assign_routes_to_store() {
        let src = "PROCEDURE Main()\nPUBLIC nEmpresa\nnEmpresa := 42\nRETURN";
        let out = gen(src);
        assert!(out.contains("public_store().write().unwrap().set(\"NEMPRESA\", HbValue::Integer(42))"), "got:\n{out}");
    }

    #[test]
    fn test_public_read_uses_store() {
        let src = "PROCEDURE Main()\nPUBLIC nEmpresa\n? nEmpresa\nRETURN";
        let out = gen(src);
        assert!(out.contains("public_store().read().unwrap().get(\"NEMPRESA\")"), "got:\n{out}");
    }

    #[test]
    fn test_top_level_memvar_skipped() {
        // MEMVAR no topo do arquivo não deve causar parse error
        let src = "memvar cor01, cor02\nPROCEDURE Main()\nRETURN";
        let out = gen(src);
        assert!(out.contains("fn main()"), "got:\n{out}");
    }
}
