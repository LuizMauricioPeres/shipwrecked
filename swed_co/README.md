# swed_co — Core / DNA

The foundational crate of the SWed workspace. Contains **only type definitions and traits** — no runtime logic, no external dependencies beyond `std`. All other crates depend on it.

## Types

| Type | Description |
|---|---|
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

## Source layout

```
swed_co/src/
├── lib.rs
├── error.rs        ← SwedError + SeverityLevel
├── hb_type.rs      ← HbType enum + Hungarian char mapping
└── traits/
    ├── mod.rs
    ├── native_fn.rs
    ├── resolver.rs
    ├── module.rs
    └── interceptor.rs
```
