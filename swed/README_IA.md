CRATE: swed v0.2.0
TYPE: binary (bin/swed)
ROLE: Transpiler entry point — orchestrates Lexer→Parser→Semantic→Codegen
STATUS: ~95% functional; single-file output; dual-file de-prioritized

DEPS:
  logos 0.14.4       — lexer derive macro
  serde + serde_json — hbdocs.json loading
  thiserror          — ParseError
  encoding_rs        — CP1252 fallback (direct, before swed_io)
  swed_co            — SwedError, SeverityLevel, HbType, ast::Program (all AST types)
  swed_nm            — normalize(&mut Program) — AST rewrite pass (P1 functional)
  swed_rt            — HbValue (referenced in codegen output, not at compile-time)
  swed_mkh           — symbol manifest integration

FEATURES: oop=off  dbf=off  source-maps=off  (all unimplemented)

SOURCE_FILES:
  main.rs         — CLI: reads args[1]=.prg args[2]=hbdocs.json; writes output.rs
                    Pipeline: lexer → parser → swed_nm::normalize() → semantic → codegen
  lexer.rs        — Token enum (logos derive); Span = (usize,usize); case-normalize to UPPER
  ast.rs          — thin re-export: `pub use swed_co::ast::*`
                    AST types now live in swed_co/src/ast.rs (moved 2026-05-04)
  parser.rs       — Parser{tokens,pos}; recursive descent
                    pub fn parse(tokens) -> Result<Program,ParseError>
                    Handles: [context-sensitive], ; separator/continuation,
                             = assignment vs comparison, ? as QOUT, ++ -- prefix/postfix
  scope.rs        — ScopeChain: LOCAL>STATIC>PRIVATE>PUBLIC resolution
                    fn resolve(name:&str) -> Option<ScopeEntry>
  semantic.rs     — Analyzer{scope,diagnostics,fn_table}
                    fn analyse(program:&Program, fn_table:&FnTable) -> Vec<Diagnostic>
                    Undeclared vars → Warning + auto-declare as PRIVATE
  symbol_table.rs — FnTable loaded from hbdocs.json
                    fn load(path:&str) -> Result<FnTable,SwedError>
                    fn validate_call(name,arity) -> Option<Diagnostic>
  codegen.rs      — fn emit(program:&Program) -> String
                    Emits use swed_rt::*; at top
                    ProcDef → fn name(params) { body }
                    FuncDef → fn name(params) -> HbValue { body }
  hb_array.rs     — Codegen helpers for array literals and indexing

PARSE_QUIRKS:
  [ after =/(/[/{     → StringLiteral (Harbour bracket strings)
  [ after ident       → ArrayIndex
  = at stmt-start     → Assignment (legacy xBase)
  = inside expr       → Comparison
  ++/-- on CallExpr   → unsupported; swed_nm will normalize pre-codegen

CODEGEN_OUTPUT_HEADER (every .rs):
  #![allow(non_snake_case, unused_mut, unused_variables)]
  use swed_rt::*;
  fn main() { ... }    // for PROCEDURE Main

DIAGNOSTICS:
  SeverityLevel::Notice   — style / encoding fallback
  SeverityLevel::Warning  — undeclared var, deprecated syntax
  SeverityLevel::Critical — parse error, arity mismatch
  SeverityLevel::Panic    — internal compiler error

CODEGEN_ADDITIONS (2026-05-04 — for swed_nm output):
  HB_SET_VAL(base, idx, val)      → base.hb_set_val(idx, val)
  HB_SET_NESTED(base, i, j, val)  → base.hb_set_nested(i, j, val)
  (swed_rt must implement hb_set_val / hb_set_nested on HbArray/HbValue)

PENDING (this crate):
  AddAssign/SubAssign emit (+= -= *= /=)                 (priority: H)
  hb_eq / hb_exact_eq call sites in codegen             (priority: M)
  clap CLI: --output --verbose --check --json-errors     (priority: M)
  miette diagnostics (rustc-style spans)                 (priority: M)
  dual-file: <name>.rs + <name>_module.rs               (priority: L)
  source-maps: SrcSpan annotations in output            (priority: L)
  oop feature: CLASS/METHOD codegen                     (priority: L)

INTEGRATES_WITH:
  swed_mkh: call analyse() to enrich symbol table before semantic pass
  swed_rt:  generated code links against it at user's build time (not ours)
  swed_co:  SwedError carried through all stages
