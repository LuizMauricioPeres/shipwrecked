CRATE: swed_co v0.2.0
TYPE: library (lib)
ROLE: Workspace DNA — shared types and traits only; zero runtime logic; zero external deps
STATUS: functional; stable API surface

DEPS: none (only std)

DESIGN_RULE: No impl blocks beyond derive. No runtime behavior. Must compile with zero warnings.
             If you are adding logic → wrong crate. Use swed_rt, swed_bf, or domain crate.

TYPES (ast.rs — moved from swed/src/ast.rs 2026-05-04):
  SrcSpan = std::ops::Range<usize>   — byte-offset span; replaces logos::Span alias
  Program { units: Vec<TopLevel> }
  TopLevel: Procedure(ProcDef) | Function(FuncDef) | Class(ClassDef)
  ProcDef / FuncDef: { name: String, params: Vec<Param>, body: Vec<Stmt>, span: SrcSpan }
  ClassDef: { name, superclass: Option<String>, data: Vec<DataDecl>, methods: Vec<MethodDef>, span }
  Stmt { kind: StmtKind, span: SrcSpan }
  StmtKind:
    VarDecl|StaticDecl|PublicDecl(VarDeclStmt)
    MemvarDecl(Vec<String>)
    FieldDecl { names, alias }
    Assign(AssignStmt)  Store(StoreStmt)
    Exit | Loop | Cls | Read
    Call(CallExpr)  Print(Expr)
    If(IfStmt)  DoWhile(DoWhileStmt)  For(ForStmt)
    Return(Option<Expr>)  AtSay(AtSayStmt)  AtGet(AtGetStmt)
  AssignStmt { target: Expr, value: Expr }
  Expr { kind: ExprKind, span: SrcSpan }
  ExprKind: Nil|Bool|Integer|Float|String|ArrayLit|Ident|Index|Field|
            BinOp|UnOp|Call(CallExpr)|Iif|Macro
  CallExpr { callee: String (UPPERCASE canonical), args: Vec<Expr>, span }
  BinOp: Add|Sub|Mul|Div|Mod|Pow|Eq|StrictEq|NotEq|Lt|Lte|Gt|Gte|And|Or|Concat|InStr
  UnOp: Neg|Not|PreIncrement|PreDecrement|PostIncrement|PostDecrement
  impl Expr: nil / ident / int / string (span constructors)
  impl Stmt: new(kind, span)

  REASON FOR MOVE: swed_nm needs AST types; swed is a binary crate — importing from it
                   would create a circular dep (swed→swed_nm→swed).
                   Now: swed→swed_nm→swed_co (no cycles).
                   swed/src/ast.rs = `pub use swed_co::ast::*` (thin re-export, no parser changes).

TYPES (error.rs):
  SeverityLevel — diagnostic severity
    Notice     — informational (encoding fallback, style)
    Warning    — undeclared var, deprecated syntax; transpilation continues
    Critical   — parse error, arity mismatch; transpilation aborted
    Panic      — internal compiler error; should never reach user
  impl: Debug Clone PartialEq PartialOrd

  SwedError — typed error carrier
    severity: SeverityLevel
    message: String
    source: Option<String>    — file path
    line: Option<usize>
    col: Option<usize>
  impl: Debug Clone Display Error (thiserror)
  constructor: SwedError::new(severity, message) -> SwedError
               SwedError::at(severity, message, source, line, col) -> SwedError

TYPES (hb_type.rs):
  HbType — Harbour type system (Hungarian notation)
    Numeric    'N'
    Character  'C'
    Logical    'L'
    Date       'D'
    Array      'A'
    Object     'O'
    Block      'B'
    Unknown    'U'   — NIL or unresolved
  impl: Debug Clone PartialEq
  fn HbType::from_char(c:char) -> Option<HbType>
  fn HbType::to_char(&self) -> char
  fn HbType::from_hb_value(v:&HbValue)  — NOTE: takes HbValue ref; avoid circular dep;
                                            swed_co must NOT import swed_rt
                                            → this method lives in swed_rt::value via Into<HbType>

TRAITS (traits/):
  NativeFunction (native_fn.rs):
    fn call(&self, args: Vec<HbValue>) -> HbValue
    fn name(&self) -> &'static str
    fn arity(&self) -> (usize, usize)   — (min_args, max_args)
    Implemented by: every function struct in swed_bf

  FunctionResolver (resolver.rs):
    fn resolve(&self, name: &str) -> Option<&dyn NativeFunction>
    fn register(&mut self, func: Box<dyn NativeFunction>)
    Implemented by: swed_bf registry (lib.rs BfRegistry struct)
    Used by: swed codegen to validate calls; swed_rt builtins dispatch

  ModuleComponent (module.rs):
    fn on_init(&mut self)
    fn on_shutdown(&mut self)
    fn name(&self) -> &'static str
    Implemented by: swed_db WorkArea, swed_ui AppState
    Used by: swed binary to lifecycle-manage pluggable modules

  ErrorInterceptor (interceptor.rs):
    fn on_critical(&self, err: &SwedError, val: &HbValue)
    Implemented by: swed_kn interceptor
    Used by: swed_rt arithmetic ops to fire before propagating Critical errors

SOURCE_LAYOUT:
  lib.rs
  ast.rs           — Harbour AST (Program, Stmt, Expr, CallExpr, BinOp, UnOp, SrcSpan)
  error.rs         — SwedError + SeverityLevel
  hb_type.rs       — HbType enum + char mapping
  traits/
    mod.rs
    native_fn.rs   — NativeFunction
    resolver.rs    — FunctionResolver
    module.rs      — ModuleComponent
    interceptor.rs — ErrorInterceptor

INVARIANTS:
  - zero external crate deps (only std + thiserror for derive)
  - all items pub; all pub items have doc comments (missing_docs = warn)
  - no circular deps: swed_co cannot import swed_rt or any workspace sibling

PENDING:
  doc comments on all ast.rs items (missing_docs warnings currently suppressed in swed_co)
