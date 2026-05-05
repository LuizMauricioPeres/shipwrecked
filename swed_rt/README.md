# swed_rt — Runtime

The Harbour runtime library linked by every SWed-transpiled `.rs` file. Provides the value type, array operations, built-in functions, PUBLIC variable store, and database access.

## Core types

| Type | File | Description |
|---|---|---|
| `HbValue` | `value.rs` | Harbour type union: `Nil` / `Logical` / `Integer` / `Float` / `String` / `Date` / `Array` / `Block` |
| `HbArray` | `array.rs` | 1-indexed dynamic array; `hb_array!` macro for literals |
| `publics_var` | `publics_var.rs` | Thread-safe singleton for PUBLIC variables (`RwLock<HashMap>`) |
| `WorkArea` | `work_area.rs` | DBF cursor: open / close / navigate / field access |
| `DbfHandler` | `dbf_handler.rs` | Low-level DBF file I/O (dbase crate) |
| `Row` | `row.rs` | Single DBF record buffer |

## HbValue semantics

- All arithmetic and comparison operators overloaded on `HbValue`
- NIL propagation: `NIL op X → NIL` for all arithmetic — no panic
- Type coercions follow Harbour runtime rules

## PUBLIC variable access

```rust
use swed_rt::publics_var::public_store;

public_store().write().unwrap().set("N_EMPRESA", HbValue::Integer(1));
let v = public_store().read().unwrap().get("N_EMPRESA");
```

## Built-in functions

`builtins.rs` implements the most-used Harbour functions: `Date`, `Left`, `Right`, `AllTrim`, `Len`, `Str`, `Val`, `Type`, `Empty`, `AAdd`, `ASize`, `AScan`, and more. These will migrate to `swed_bf` as that crate matures.

### Pending additions (blocking generated code)

| Function | Signature | Fixes |
|----------|-----------|-------|
| `hb_array(n)` | `(n: HbValue) -> HbValue` | E0425 — `Array(n)` in generated code |
| `hb_set_val(idx, val)` | `(&mut HbValue, HbValue, HbValue)` | E0070 — `a[i] := v` pattern |
| `hb_set_nested(i, j, val)` | `(&mut HbValue, HbValue, HbValue, HbValue)` | E0070 — `a[i][j] := v` |
| non-generic `hb_get_val` | accept `HbValue` directly | E0282 — type inference failure |

## Dependency note

`swed_rt` is intentionally self-contained: it does not depend on `swed_co` or any other workspace crate. Generated code only needs to link against `swed_rt`.
