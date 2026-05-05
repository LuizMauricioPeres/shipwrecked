# swed_rt — Runtime

The Harbour runtime library linked by every SWed-transpiled `.rs` file. Provides the value type, array operations, arithmetic operators, PUBLIC variable store, and built-in function stubs.

## Core types

| Type | File | Description |
|---|---|---|
| `HbValue` | `value.rs` | Harbour type union: `Nil / Logical / Integer / Float / String / Date / Array` |
| `HbArray` | `array.rs` | 1-indexed dynamic array; `hb_array![]` macro for literals |
| `PublicVars` | `publics_var.rs` | Thread-safe singleton for PUBLIC/MEMVAR variables |

## HbValue operators

All arithmetic and comparison operators are overloaded and NIL-safe (`Nil op X → Nil`):

| Trait | Operators |
|-------|-----------|
| `Add / Sub / Mul / Div / Rem` | `+ - * / %` |
| `AddAssign / SubAssign` | `+= -=` |
| `Neg / Not` | unary `-` and `!` |
| `PartialOrd / PartialEq` | `< <= > >= ==` |
| `Index<&HbValue>` | `arr[&idx]` — zero-clone subscript |

## HbValue methods

```rust
// Array mutations (for generated code)
val.hb_set_val(index: HbValue, val: HbValue)
val.hb_set_nested(row: HbValue, col: HbValue, val: HbValue)

// Queries
val.hb_len() -> HbValue          // Len() for String/Array
val.hb_len_as_i64() -> i64       // for codegen loop bounds
val.hb_get_val(index: HbValue)   // array element by value
val.val_type() -> &'static str   // "N"/"C"/"L"/"D"/"A"/"U"
val.is_truthy() -> bool          // for if/while conditions
```

## PUBLIC variable access

```rust
use swed_rt::{pub_declare, pub_get, pub_set, memvar_assign, memvar_get};

pub_declare("N_EMPRESA");
pub_set("N_EMPRESA", HbValue::Integer(1));
let v = pub_get("N_EMPRESA");
```

## Built-in functions (builtins.rs)

These live in `swed_rt` for use in generated code. **Migration target: `swed_bf`** as that crate matures.

| Function | Harbour equivalent |
|----------|--------------------|
| `hb_array(n)` | `Array(n)` — NIL-filled array |
| `hb_range(from, to, step)` | `FOR` loop iterator |
| `hb_qout(v)` / `hb_qqout(v)` | `?` / `??` |
| `hb_space(n)` / `hb_replicate(s,n)` | `Space()` / `Replicate()` |
| `hb_chr(n)` / `hb_asc(s)` | `Chr()` / `Asc()` |
| `hb_str(n,w,d)` / `hb_val(s)` | `Str()` / `Val()` |
| `hb_substr(s,n,l)` | `SubStr()` |
| `hb_alltrim/ltrim/rtrim/upper/lower` | trim / case |
| `hb_at(n,h)` / `hb_rat(n,h)` | `At()` / `RAt()` |
| `hb_padl/padr(s,n)` | `PadL()` / `PadR()` |
| `hb_len(v)` | `Len()` |
| `hb_int(n)` / `hb_abs(n)` | `Int()` / `Abs()` |
| `hb_round(n,d)` / `hb_max/min(a,b)` | `Round()` / `Max()` / `Min()` |
| `hb_valtype(v)` / `hb_empty(v)` | `ValType()` / `Empty()` |
| `hb_aadd(arr,v)` / `hb_asize/ascan` | array ops |
| `hb_setcolor(v)` | `SetColor()` — stub |
| `hb_inkey(t)` / `hb_lastkey()` | keyboard — stubs |
| `hb_macro(v)` | `&varName` — stub |

## HbArray methods

```rust
HbArray::new()          HbArray::filled(n)
arr.hb_aadd(val)        arr.hb_asize(n)
arr.hb_get(i)           arr.hb_set(i, val)
arr.hb_adel(idx)        arr.hb_ains(idx)
arr.hb_afill(v, s, c)   arr.hb_ascan(val)
arr.hb_asort()          arr.len()
```

## Dependency note

`swed_rt` has **zero workspace dependencies** — only `std`. Generated code links only against this crate. `swed_bf` builds on top of `swed_rt` and will gradually absorb the functions in `builtins.rs`.
