// swed/src/ast.rs
// Abstract Syntax Tree for Harbour source.
//
// Design goals:
//   - Nodes are cheap to clone (Strings, Vecs — no Rc/Arc yet).
//   - Every node carries a Span for error reporting.
//   - Representation is Harbour-semantic, NOT Rust-semantic.
//     The codegen layer handles the mapping.

use crate::lexer::Span;

// ---------------------------------------------------------------------------
// Span newtype (re-exported for convenience)
// ---------------------------------------------------------------------------
pub type SrcSpan = Span;

// ---------------------------------------------------------------------------
// Top-level program
// ---------------------------------------------------------------------------

/// A compiled .prg file may contain multiple procedures/functions/classes.
#[derive(Debug, Clone)]
pub struct Program {
    pub units: Vec<TopLevel>,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Procedure(ProcDef),
    Function(FuncDef),
    Class(ClassDef),
}

// ---------------------------------------------------------------------------
// Procedure / Function
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ProcDef {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub span: SrcSpan,
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub span: SrcSpan,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub span: SrcSpan,
}

// ---------------------------------------------------------------------------
// Class (skeleton — full OOP in a later pass)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub name: String,
    pub superclass: Option<String>,
    pub data: Vec<DataDecl>,
    pub methods: Vec<MethodDef>,
    pub span: SrcSpan,
}

#[derive(Debug, Clone)]
pub struct DataDecl {
    pub name: String,
    pub span: SrcSpan,
}

#[derive(Debug, Clone)]
pub struct MethodDef {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub span: SrcSpan,
}

// ---------------------------------------------------------------------------
// Statements
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: SrcSpan,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    /// LOCAL x, y, z := expr
    VarDecl(VarDeclStmt),
    /// STATIC x := expr
    StaticDecl(VarDeclStmt),
    /// MEMVAR / PRIVATE x
    MemvarDecl(Vec<String>),
    /// PUBLIC x [, y := expr, ...]
    PublicDecl(VarDeclStmt),
    /// FIELD name1, name2 [IN alias]
    FieldDecl { names: Vec<String>, alias: Option<String> },

    /// x := expr  or  x = expr  (legacy)
    Assign(AssignStmt),

    /// AAdd(arr, val), QOut(...), or any call used as statement
    Call(CallExpr),

    /// ? expr  (QOUT shorthand)
    Print(Expr),

    /// IF / ELSEIF / ELSE / ENDIF
    If(IfStmt),

    /// DO WHILE cond … ENDDO
    DoWhile(DoWhileStmt),

    /// FOR i := start TO end [STEP n] … NEXT
    For(ForStmt),

    /// RETURN [expr]
    Return(Option<Expr>),
}

// ── Variable declaration ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct VarDeclStmt {
    pub vars: Vec<VarInit>,
}

#[derive(Debug, Clone)]
pub struct VarInit {
    pub name: String,
    pub init: Option<Expr>,
    pub span: SrcSpan,
}

// ── Assignment ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AssignStmt {
    /// Left-hand side: simple ident, array element (a[i]), or OOP (obj:field)
    pub target: Expr,
    pub value: Expr,
}

// ── Control flow ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_body: Vec<Stmt>,
    pub elseif_branches: Vec<(Expr, Vec<Stmt>)>,
    pub else_body: Option<Vec<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct DoWhileStmt {
    pub condition: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ForStmt {
    pub var: String,
    pub start: Expr,
    pub end: Expr,
    pub step: Option<Expr>,
    pub body: Vec<Stmt>,
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: SrcSpan,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    // ── Literals ─────────────────────────────────────────────────────────
    Nil,
    Bool(bool),
    Integer(i64),
    Float(f64),
    /// Both `"string"`, `'string'`, and `[string]` collapse here.
    String(String),
    /// `{ e1, e2, … }` — array literal
    ArrayLit(Vec<Expr>),

    // ── Variable / identifier ─────────────────────────────────────────────
    Ident(String),

    // ── Access ────────────────────────────────────────────────────────────
    /// `arr[index]`
    Index(Box<Expr>, Box<Expr>),
    /// `obj:member`
    Field(Box<Expr>, String),

    // ── Operators ─────────────────────────────────────────────────────────
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),

    // ── Function call ─────────────────────────────────────────────────────
    Call(CallExpr),

    // ── Inline IF (IIF) ───────────────────────────────────────────────────
    Iif(Box<Expr>, Box<Expr>, Box<Expr>),

    // ── Macro substitution  &varName ──────────────────────────────────────
    Macro(String),
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: String, // UPPERCASE canonical name
    pub args: Vec<Expr>,
    pub span: SrcSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, StrictEq, NotEq, Lt, Lte, Gt, Gte,
    And, Or,
    Concat, // Harbour `+` on strings (context-sensitive — resolved in semantic)
    InStr,  // `$` substring test
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Neg,
    Not,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

impl Expr {
    pub fn nil(span: SrcSpan) -> Self {
        Expr { kind: ExprKind::Nil, span }
    }

    pub fn ident(name: impl Into<String>, span: SrcSpan) -> Self {
        Expr { kind: ExprKind::Ident(name.into()), span }
    }

    pub fn int(n: i64, span: SrcSpan) -> Self {
        Expr { kind: ExprKind::Integer(n), span }
    }

    pub fn string(s: impl Into<String>, span: SrcSpan) -> Self {
        Expr { kind: ExprKind::String(s.into()), span }
    }
}

impl Stmt {
    pub fn new(kind: StmtKind, span: SrcSpan) -> Self {
        Stmt { kind, span }
    }
}
