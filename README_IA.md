WORKSPACE: shipwrecked / SWed v0.2.0
PURPOSE: Source-to-source transpiler Harbour/xBase .prg → idiomatic Rust 2021
STATUS: ~95% pipeline functional; 36 tests passing; 0 clippy warnings

CRATE_GRAPH (dependency order):
  swed_co  →  swed_rt  →  swed_bf
                       →  swed_db
                       →  swed_kn
                       →  swed_ui
  swed_co  →  swed_io
  swed_co  →  swed_mkh (+ md5, walkdir)
  swed_co  →  swed_nm  [PLANNED — Semantic Normalizer; rewrites AST before codegen]
  swed_co + swed_rt + swed_mkh  →  swed (binary)

WORKSPACE_ROOT:
  Cargo.toml       — workspace; members: swed swed_rt swed_mkh swed_co swed_bf swed_db swed_io swed_kn swed_ui swed_nm(planned)
  hbdocs.json      — Harbour built-in function signatures (arity validation); 343 entries
  ANDAMENTO.md     — human progress report (2026-04-30)

PIPELINE (swed binary):
  .prg → Lexer (logos) → Vec<Token>
       → Parser (recursive descent) → ast::Program
       → Semantic/SymbolTable + hbdocs.json → Vec<Diagnostic>
       → swed_nm [PLANNED] → AST rewrite (BuiltinNameResolver, IndexAssignNorm, IncrDecrNorm, ...)
       → Codegen → output.rs + swed_rt linked

HARBOUR_SCOPE_CHAIN: LOCAL > STATIC > PRIVATE(MEMVAR) > PUBLIC
ENCODING: CP1252 auto-detected; falls back from UTF-8; swed_io handles

MAPPING (key constructs):
  PROCEDURE Main()     → fn main()
  FUNCTION f(x)        → fn f(x: HbValue) -> HbValue
  LOCAL x := v         → let mut x = v;
  STATIC x             → thread_local!{ static X: RefCell<HbValue> }
  PUBLIC nVar          → pub_declare/pub_set via publics_var singleton
  FOR i:=1 TO n        → for i in hb_range(1, n, 1)
  DO WHILE cond        → while cond { }
  AAdd(a, v)           → a.hb_aadd(v)
  LEN(x)               → x.hb_len()
  a[i]                 → a.hb_get_val(HbValue::Integer(i)) [read] / a.hb_set_val(i, v) [write]
  a[i][j] := v         → a.hb_set_nested(i, j, v)  [via swed_nm IndexAssignNorm]
  chr(n)               → hb_chr(n)   [via swed_nm BuiltinNameResolver]
  ? expr               → hb_qout(expr)
  NIL                  → HbValue::Nil
  .T./.F.              → HbValue::Logical(true/false)
  [str]                → HbValue::String("str".into())
  {e1,e2}              → hb_array![e1,e2]
  IIF(c,t,f)           → if c { t } else { f }
  FIELD->NAME          → field_get("NAME") / field_set("NAME",val)
  @r,c SAY/GET/READ    → AppState::new(widgets).run() scoped block

PENDING_HIGH:
  swed_nm crate        — criar; implementar P1: BuiltinNameResolver, IndexAssignNorm, ChainedIndexAssignNorm, IndexTypeAnnotation
  swed_rt::hb_set_val / hb_set_nested / hb_array — desbloqueia E0070/E0425 em resta1.rs
  AddAssign/SubAssign/MulAssign/DivAssign on HbValue
  hb_eq / hb_exact_eq  — SET EXACT OFF semantics

PENDING_MEDIUM:
  swed_nm P2           — IncrDecrNorm, CompoundAssignNorm, TuiCallDetector
  swed_nm P3           — BuiltinImportInjector, AliasResolver
  swed_bf P2-P3        — Left/Right/StrTran, ATail/ADel/AIns/AFill/AClone/AEval/ASort
  swed_kn              — full ErrorInterceptor impl, hex dump, patch suggestions
  clap CLI             — --output --verbose --check flags
  miette diagnostics   — rustc-style errors

PENDING_LOW:
  swed_bf P4-P7        — Mod/Sqrt/Log/Exp, DToC/CToD/Time/Seconds, IsAlpha..IsNegative, OS/Version
  VS Code LSP extension via .mkh
  swed_mkm (Master Maker — build orchestrator)
  OOP: CLASS/METHOD/inheritance via traits
  Macro expansion: #define #include &varName
  Source maps (.prg line numbers in generated Rust)
  WASM target

FEATURES (swed binary, all disabled by default):
  oop          — CLASS/METHOD codegen
  dbf          — swed_db integration in pipeline
  source-maps  — Span annotation in output

CONVENTIONS:
  Hungarian notation preserved in HbType (N C L D A O B U)
  Traits in swed_co::traits::{native_fn, resolver, module, interceptor}
  Thread-safety: thread_local! for STATIC; Arc<RwLock<>> for PUBLIC store
  No unsafe (workspace deny); no panics in HbValue arithmetic (NIL propagation)
