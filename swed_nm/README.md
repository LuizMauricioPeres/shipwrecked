# swed_nm — Semantic Normalizer

AST rewrite pass that runs between the Semantic Analyzer and the Codegen inside the `swed` transpiler pipeline. Mutates `ast::Program` in place to fix patterns that generate invalid Rust.

## Why this crate exists

The parser produces a Harbour-semantic AST. Some of those patterns don't map 1-to-1 to valid Rust:

| Harbour pattern | Parser AST | Rust error |
|---|---|---|
| `Chr(219)` | `Call { callee: "CHR" }` | `E0425` — `chr` not in scope |
| `Array(7)` | `Call { callee: "ARRAY" }` | `E0425` — `array` not in scope |
| `a[i] := v` | `Assign { target: Index(a, i) }` | `E0070` — invalid LHS |
| `a[i][j] := v` | `Assign { target: Index(Index(a,i), j) }` | `E0070` — invalid LHS |

`swed_nm` normalizes these before the codegen sees them.

## Pipeline position

```
Parser → ast::Program
              │
              ▼
       swed_nm::normalize()     ← this crate
              │
              ▼
         Codegen → .rs file
```

## Entry point

```rust
// swed/src/main.rs
let mut program = parser::parse(tokens)?;
swed_nm::normalize(&mut program);          // ← single call, runs all passes
codegen::generate_dual(&program, stem)
```

## Pass chain

| # | Pass | Status | What it fixes |
|---|------|--------|---------------|
| 1 | `BuiltinNameResolver` | **functional** | `CHR` → `HB_CHR`, `MAX` → `HB_MAX`, … (55 built-ins) |
| 2 | `IndexAssignNorm` | **functional** | `a[i]:=v` → `HB_SET_VAL(a,i,v)` / `a[i][j]:=v` → `HB_SET_NESTED` |
| 3 | `IncrDecrNorm` | stub (P2) | `x++` / `x--` in rvalue position |

## BuiltinNameResolver

Renames Harbour built-in call sites to their `HB_XXX` canonical form. The codegen's generic branch lowercases the callee, so `HB_CHR` → `hb_chr(...)` transparently.

**Excluded** (already have dedicated codegen arms): `AADD`, `LEN`, `ASIZE`, `QOUT`, `ALLTRIM`/`LTRIM`/`RTRIM`, `STR`, `VAL`, `SUBSTR`, `AT`.

## IndexAssignNorm

Rewrites array-element assignments into method-call form:

```
a[i]    := v   →   StmtKind::Call("HB_SET_VAL",    [a, i, v])
a[i][j] := v   →   StmtKind::Call("HB_SET_NESTED", [a, i, j, v])
```

The codegen renders these as:

```rust
a.hb_set_val(i, v);
a.hb_set_nested(i, j, v);
```

> **Dependency**: `swed_rt::HbArray` / `HbValue` must implement `hb_set_val` and `hb_set_nested` (pending in `swed_rt`).

## Source layout

```
swed_nm/src/
├── lib.rs                         ← pub fn normalize(&mut Program)
├── pass.rs                        ← NormPass trait
├── walker.rs                      ← shared AST traversal utilities
└── passes/
    ├── mod.rs
    ├── builtin_name_resolver.rs   ← P1 — 55 built-in renames
    ├── index_assign_norm.rs       ← P1 — a[i]:=v rewrite
    └── incr_decr_norm.rs          ← P2 — stub
```

## AST dependency

`swed_nm` imports `ast::Program` from `swed_co` — the same types used by the `swed` binary. `swed/src/ast.rs` is now a thin re-export:

```rust
// swed/src/ast.rs
pub use swed_co::ast::*;
```

This eliminates any circular dependency (`swed` → `swed_nm` → `swed_co`).

## Adding a new pass

1. Create `src/passes/my_pass.rs` implementing `NormPass`.
2. `pub use` it in `src/passes/mod.rs`.
3. Add it to the chain in `src/lib.rs::normalize()`.
