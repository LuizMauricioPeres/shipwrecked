CRATE: swed_nm v0.2.0
TYPE: library (lib)
ROLE: Semantic Normalizer — mutable AST rewrite pass between Semantic Analyzer and Codegen
STATUS: P1 passes functional (BuiltinNameResolver, IndexAssignNorm); P2 stub (IncrDecrNorm)

DEPS:
  swed_co — ast::Program and all AST types (SrcSpan, Expr, Stmt, CallExpr, …)

DESIGN:
  - Entry point: pub fn normalize(program: &mut Program)
  - Chain of NormPass trait objects executed in priority order
  - Each pass is idempotent: running twice yields the same result
  - Passes mutate the AST in-place; no new allocation of Program nodes unless replacing
  - walker.rs provides shared traversal utilities (walk_program, walk_stmts, walk_expr)

WHY_SEPARATE_CRATE:
  - swed (binary) → swed_nm → swed_co : no circular dependency
  - AST types live in swed_co so both swed and swed_nm can share them
  - swed/src/ast.rs is now a thin re-export: `pub use swed_co::ast::*`

SOURCE_FILES:
  lib.rs                         — pub fn normalize(&mut Program); chains all passes
  pass.rs                        — NormPass trait { run(&mut Program); name() }
  walker.rs                      — walk_program / walk_stmts / walk_expr (closure-based)
  passes/mod.rs                  — pub use all passes
  passes/builtin_name_resolver.rs — P1 FUNCTIONAL
  passes/index_assign_norm.rs    — P1 FUNCTIONAL
  passes/incr_decr_norm.rs       — P2 STUB

PASS_CHAIN (execution order):
  1. BuiltinNameResolver
  2. IndexAssignNorm
  3. IncrDecrNorm

PASS: BuiltinNameResolver (P1 — FUNCTIONAL)
  Problem: codegen generic branch lowercases callee → chr() not hb_chr()
  Solution: rename known Harbour built-in callees to HB_XXX before codegen
  Scope: ExprKind::Call and StmtKind::Call across entire AST
  Skips: AADD LEN ASIZE QOUT ALLTRIM LTRIM RTRIM STR VAL SUBSTR AT (have dedicated codegen arms)
  Built-ins renamed (55 entries):
    String:  CHR ASC SPACE REPLICATE UPPER LOWER LEFT RIGHT PADC STRTRAN STRZERO HARDCR RAT TRIM
    Numeric: ABS INT MAX MIN ROUND MOD SQRT EXP LOG
    Date:    DATE YEAR MONTH DAY DTOS STOD DTOC CTOD TIME SECONDS
    Type:    VALTYPE TYPE EMPTY ISNIL ISALPHA ISDIGIT ISLOWER ISUPPER
    Array:   ARRAY ATAIL ASORT AEVAL AFILL ADEL AINS ACLONE
    Screen:  SETCOLOR SETMODE ROW COL MAXROW MAXCOL INKEY LASTKEY NEXTKEY TONE
    Misc:    OS VERSION
  Codegen generic branch: to_snake_case(callee.to_lowercase()) → HB_CHR → hb_chr ✓

PASS: IndexAssignNorm (P1 — FUNCTIONAL)
  Problem: a[i] := v → atabuleiro.hb_get_val(i) = v → E0070 in Rust
  Solution: rewrite Assign{target:Index, value} → StmtKind::Call(HB_SET_VAL/HB_SET_NESTED)
  Codegen arms added:
    HB_SET_VAL(base, idx, val)      → base.hb_set_val(idx, val)
    HB_SET_NESTED(base, i, j, val)  → base.hb_set_nested(i, j, val)
  Cases handled:
    a[i] := v         → StmtKind::Call("HB_SET_VAL", [a, i, v])
    a[i][j] := v      → StmtKind::Call("HB_SET_NESTED", [a, i, j, v])
  NOTE: swed_rt::HbArray needs hb_set_val / hb_set_nested methods (pending)

PASS: IncrDecrNorm (P2 — STUB)
  Problem: x++ / x-- in rvalue position → codegen emits __EXPR_STMT__ only at stmt level
  Planned: rewrite ExprKind::UnOp(PostIncrement|PostDecrement, e) inside expressions
           to { let __old = e.clone(); e += 1; __old }
  Status: empty run() body; parser __EXPR_STMT__ handles stmt-level cases for now

WALKER (walker.rs):
  walk_program(program, f: &mut FnMut(&mut Stmt))
    → walks all TopLevel bodies recursively, calls f on every Stmt
  walk_stmts(stmts, f)
    → recurses into nested bodies (If/DoWhile/For), then calls f on each stmt
  walk_exprs_in_stmt(stmt, f: &mut FnMut(&mut Expr))
    → visits all direct Expr nodes in a stmt (does NOT recurse into child stmts)
  walk_expr(expr, f: &mut FnMut(&mut Expr))
    → bottom-up walk of all sub-expressions; calls f on each

PIPELINE_INTEGRATION (swed/src/main.rs):
  let mut program = parser::parse(tokens)?;
  swed_nm::normalize(&mut program);          // ← added
  analyzer.analyze(&program);
  codegen::generate_dual(&program, stem)

PENDING (this crate):
  P2 — IncrDecrNorm: full rvalue x++/x-- desugaring inside complex expressions
  P2 — CompoundAssignNorm: x += y / x -= y in all expression contexts
  P2 — TuiCallDetector: detect screen function calls → mark PRG as requires:swed_ui
  P3 — BuiltinImportInjector: detect used HB_XXX → generate `use swed_bf::{…}` in _module.rs
  P3 — AliasResolver: TRIM→HB_RTRIM already done; extend for other aliases

INTEGRATES_WITH:
  swed (binary): calls normalize() in main pipeline
  swed_co:       imports all AST types; swed_co::ast is the source of truth
  swed_rt:       hb_set_val / hb_set_nested must be added to HbArray/HbValue
