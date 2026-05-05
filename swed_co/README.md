# swed_co — Core / DNA

The foundational crate of the SWed workspace. Contains **only type definitions and traits** — no runtime logic, no external dependencies beyond `std`. All other crates depend on it.

## Types

| Type | Description |
|---|---|
| `Program` | Root AST node: `Vec<TopLevel>` (Procedure / Function / Class) |
| `SrcSpan` | `std::ops::Range<usize>` — byte-offset span in a `.prg` file |
| `Stmt` / `StmtKind` | All statement forms: Assign, Call, If, DoWhile, For, … |
| `Expr` / `ExprKind` | All expression forms: Index, BinOp, Call, Ident, … |
| `CallExpr` | `{ callee: String (UPPERCASE), args: Vec<Expr>, span }` |
| `AssignStmt` | `{ target: Expr, value: Expr }` — target may be `ExprKind::Index` |
| `HbType` | Harbour type system with Hungarian-notation mapping (`N`→Numeric, `C`→Character, `L`→Logical, `D`→Date, `A`→Array, `O`→Object, `B`→Block, `U`→Unknown/NIL) |
| `SeverityLevel` | Diagnostic levels: `Notice`, `Warning`, `Critical`, `Panic` |
| `SwedError` | Typed error carrying severity, message, and source location |

## Traits

| Trait | File | Purpose |
|---|---|---|
| `NativeFunction` | `traits/native_fn.rs` | Contract for Harbour built-in implementations in `swed_bf` |
| `FunctionResolver` | `traits/resolver.rs` | Maps function names to runtime implementations — dependency injection for transpiled code |
| `ModuleComponent` | `traits/module.rs` | Lifecycle hooks `on_init` / `on_shutdown`; modules plug into the engine without modifying `swed_rt` |
| `ErrorInterceptor` | `traits/interceptor.rs` | Observer for `Critical`-severity errors; receives `(&SwedError, &HbValue)` before propagation |

## Design rule

**No `impl` blocks beyond `derive`.** If you are adding runtime behavior, it belongs in `swed_rt`, `swed_bf`, or a domain crate. This crate must compile with zero warnings and zero external dependencies.

## AST (`ast.rs`)

The Harbour AST lives here (moved from `swed/src/ast.rs`) so that `swed_nm` can share it without a circular dependency. `swed/src/ast.rs` is now a thin re-export:

```rust
pub use swed_co::ast::*;
```

Dependency chain: `swed` → `swed_nm` → `swed_co` — no cycles.

## Source layout

```
swed_co/src/
├── lib.rs
├── ast.rs          ← Harbour AST (Program, Stmt, Expr, CallExpr, BinOp, UnOp, SrcSpan)
├── error.rs        ← SwedError + SeverityLevel
├── hb_type.rs      ← HbType enum + Hungarian char mapping
└── traits/
    ├── mod.rs
    ├── native_fn.rs
    ├── resolver.rs
    ├── module.rs
    └── interceptor.rs
```
