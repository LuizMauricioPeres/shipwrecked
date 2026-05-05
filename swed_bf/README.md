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
├── string.rs       ← Left, Right, SubStr, AllTrim, Upper, Lower, At, PadL, PadR, StrTran, HardCR
├── numeric.rs      ← Str, Val, Mod, Sqrt, Exp, Log, Word
├── date.rs         ← Date, DToS, SToD, Year, Month, Day, DToC, CToD, CMonth, CDoW, DoW, Time, Seconds
├── array.rs        ← AAdd, ASize, AScan, ATail, ADel, AIns, AFill, AClone, ACopy, AEval, ASort
├── misc.rs         ← Type, Empty, Len, IsAlpha, IsDigit, IsLower, IsUpper, Eval, OS, Version
└── traits/
    └── mod.rs      ← re-export NativeFunction from swed_co
```

## Pending functions (by priority)

Functions listed in `hbdocs.json` not yet implemented (343 entries surveyed, ~30 relevant to non-DB use):

### P1 — Blocking resta1.rs compilation

| Harbour | swed_bf fn | Notes |
|---------|-----------|-------|
| `Array(n)` | `hb_array` | creates HbArray of n Nil elements |
| `SetColor([cSpec])` | `hb_setcolor` | ANSI stub or delegates to swed_ui |

### P2 — String

| Harbour | swed_bf fn |
|---------|-----------|
| `Left(s, n)` | `hb_left` |
| `Right(s, n)` | `hb_right` |
| `StrTran(s, old, new)` | `hb_strtran` |
| `HardCR(s)` | `hb_hardcr` |
| `hb_ValToStr(v)` | `hb_valtostr` |
| `Transform(val, pic)` | `hb_transform` |

### P3 — Array

| Harbour | swed_bf fn |
|---------|-----------|
| `ATail(a)` | `hb_atail` |
| `ADel(a, n)` | `hb_adel` |
| `AIns(a, n)` | `hb_ains` |
| `AFill(a, v, s, c)` | `hb_afill` |
| `AClone(a)` | `hb_aclone` |
| `ACopy(s,d,s,c,ds)` | `hb_acopy` |
| `AEval(a, block)` | `hb_aeval` |
| `ASort(a, block)` | `hb_asort` |

### P4 — Numeric / P5 — Date / P6 — Type checking / P7 — System

See `README_IA.md` for complete list.
