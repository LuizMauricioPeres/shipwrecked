# swed_bf — Basic Functions

Implementations of standard Harbour built-in functions, backed by `HbValue` from `swed_rt`. Depends on `swed_co` (for the `NativeFunction` trait) and `swed_rt` (for `HbValue`).

## Harbour → swed_bf mapping

| Harbour | swed_bf function | Return |
|---|---|---|
| `Date()` | `hb_date()` | `HbValue::Date` |
| `Left(cStr, n)` | `hb_left(s, n)` | `HbValue::String` |
| `Right(cStr, n)` | `hb_right(s, n)` | `HbValue::String` |
| `SubStr(cStr, n, len)` | `hb_substr(s, n, len)` | `HbValue::String` |
| `AllTrim(cStr)` | `hb_alltrim(s)` | `HbValue::String` |
| `Upper(c)` | `hb_upper(c)` | `HbValue::String` |
| `Lower(c)` | `hb_lower(c)` | `HbValue::String` |
| `Len(x)` | `hb_len(x)` | `HbValue::Integer` |
| `Str(n)` | `hb_str(n)` | `HbValue::String` |
| `Val(c)` | `hb_val(c)` | `HbValue::Float` |
| `Type(x)` | `hb_type(x)` | `HbValue::String` (type char) |
| `Empty(x)` | `hb_empty(x)` | `HbValue::Logical` |
| `At(sub, str)` | `hb_at(sub, s)` | `HbValue::Integer` |
| `PadL(c, n)` | `hb_padl(s, n)` | `HbValue::String` |
| `PadR(c, n)` | `hb_padr(s, n)` | `HbValue::String` |

All functions return `HbValue` and propagate `HbValue::Nil` on type mismatch, matching Harbour's NIL-safe semantics. No panics.

## NativeFunction trait

Every function implements the `NativeFunction` trait from `swed_co`:

```rust
pub trait NativeFunction {
    fn call(&self, args: Vec<HbValue>) -> HbValue;
    fn name(&self) -> &'static str;
    fn arity(&self) -> (usize, usize); // (min_args, max_args)
}
```

The `FunctionResolver` looks up functions by name against this registry. This replaces the hardcoded match arms currently in `swed_rt/src/builtins.rs`.

## Source layout

```
swed_bf/src/
├── lib.rs          ← registry: name → Box<dyn NativeFunction>
├── string.rs       ← Left, Right, SubStr, AllTrim, Upper, Lower, At, PadL, PadR
├── numeric.rs      ← Str, Val
├── date.rs         ← Date, DToS, SToD, Year, Month, Day
├── array.rs        ← AAdd, ALen, AScan, AEval, ASort
├── misc.rs         ← Type, Empty, Len
└── traits/
    └── mod.rs      ← re-export NativeFunction from swed_co
```
