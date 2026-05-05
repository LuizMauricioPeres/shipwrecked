# swed_bf — Basic Functions

Harbour built-in functions backed by `HbValue`. Depends on `swed_co` (for the `NativeFunction` trait) and `swed_rt` (for `HbValue`/`HbArray`).

Every function is available as both a free Rust function (for direct use in generated code) and a `NativeFunction` implementor (for the interpreter registry).

## Source layout

```
swed_bf/src/
├── lib.rs      — all_functions() registry: name → Box<dyn NativeFunction>
├── array.rs    — Array, AAdd, ASize, AFill, AScan, ATail, ADel, AIns, AClone
├── date.rs     — Date, Year, Month, Day, DToS, SToD
├── math.rs     — Abs, Int, Round, Max, Min, Mod, Sqrt, Exp, Log
├── misc.rs     — Type, ValType, Empty, PCount
├── numeric.rs  — Str, StrZero, NToS, Chr
└── string.rs   — AllTrim, LTrim, RTrim, Trim, Upper, Lower,
                  Left, Right, SubStr, At, Asc, Len, Val,
                  PadL, PadR, PadC, Space, Replicate
```

## Implemented functions (56 in registry)

### Array (`array.rs`)

| Harbour | Free fn | Registry | Return |
|---------|---------|---------|--------|
| `Array(n)` | `hb_array_new` | `Array` | `HbValue::Array` (NIL-filled) |
| `AAdd(a, v)` | `hb_aadd` | `AAdd` | `v` (element added) |
| `ASize(a, n)` | `hb_asize` | `ASize` | `HbValue::Array` (resized) |
| `AFill(a, v [,s [,c]])` | `hb_afill` | `AFill` | `HbValue::Array` |
| `AScan(a, v [,s [,c]])` | `hb_ascan` | `AScan` | `HbValue::Integer` (1-based or 0) |
| `ATail(a)` | `hb_atail` | `ATail` | last element |
| `ADel(a, n)` | `hb_adel` | `ADel` | `HbValue::Array` (shifted) |
| `AIns(a, n)` | `hb_ains` | `AIns` | `HbValue::Array` |
| `AClone(a)` | `hb_aclone` | `AClone` | independent deep copy |

> **Nota sobre mutação no registry:** `AAdd`, `ADel`, `AIns`, `AFill` operam sobre cópia — a mutação não propaga ao caller. Código gerado usa métodos diretos `HbValue::hb_aadd` / `hb_set_val`.

### DateTime (`date.rs`)

| Harbour | Free fn | Return |
|---------|---------|--------|
| `Date()` | `hb_date` | `HbValue::Date` (today) |
| `Year(d)` | `hb_year` | `HbValue::Integer` |
| `Month(d)` | `hb_month` | `HbValue::Integer` |
| `Day(d)` | `hb_day` | `HbValue::Integer` |
| `DToS(d)` | `hb_dtos` | `HbValue::String` (`"YYYYMMDD"`) |
| `SToD(s)` | `hb_stod` | `HbValue::Date` |

### Math (`math.rs`)

| Harbour | Free fn | Notes |
|---------|---------|-------|
| `Abs(n)` | `hb_abs` | preserves Integer/Float variant |
| `Int(n)` | `hb_int` | truncates toward zero |
| `Round(n, dec)` | `hb_round` | dec≤0 → Integer; dec>0 → Float |
| `Max(a, b)` | `hb_max` | works on dates too |
| `Min(a, b)` | `hb_min` | works on dates too |
| `Mod(a, b)` | `hb_mod` | sign of `a`; div-by-zero → Nil |
| `Sqrt(n)` | `hb_sqrt` | negative → Nil |
| `Exp(n)` | `hb_exp` | e^n |
| `Log(n)` | `hb_log` | natural log; n≤0 → Nil |

### Core / Misc (`misc.rs`)

| Harbour | Free fn | Notes |
|---------|---------|-------|
| `Type(x)` | `hb_type` | returns `"C"/"N"/"L"/"D"/"A"/"U"` |
| `ValType(x)` | `hb_type` | alias for `Type`; same logic |
| `Empty(x)` | `hb_empty` | NIL/0/`""`/`[]`/`.F.`/Date(0) → `.T.` |
| `PCount()` | `hb_pcount` | stub — returns 0; real count needs runtime context |

### Numeric (`numeric.rs`)

| Harbour | Free fn | Notes |
|---------|---------|-------|
| `Str(n [,w [,d]])` | `hb_str` | right-justified; overflow → `"***"` |
| `StrZero(n, w [,d])` | `hb_strzero` | zero-padded; negative sign + zeros |
| `hb_NToS(n)` | `hb_ntos` | compact; no padding |
| `Chr(n)` | `hb_chr` | ASCII 0–255 → single-char string |

### String (`string.rs`)

| Harbour | Free fn | Notes |
|---------|---------|-------|
| `AllTrim(s)` | `hb_alltrim` | both sides |
| `LTrim(s)` | `hb_ltrim` | leading spaces |
| `RTrim(s)` | `hb_rtrim` | trailing spaces |
| `Trim(s)` | `hb_rtrim` | alias for RTrim |
| `Upper(s)` | `hb_upper` | |
| `Lower(s)` | `hb_lower` | |
| `Left(s, n)` | `hb_left` | first n chars; safe if n > Len |
| `Right(s, n)` | `hb_right` | last n chars |
| `SubStr(s, start [,n])` | `hb_substr` | 1-based; nil len → to end |
| `At(needle, str)` | `hb_at` | 1-based pos or 0 |
| `Asc(s)` | `hb_asc` | ASCII of first byte; 0 for empty |
| `Len(x)` | `hb_len` | bytes for String; elements for Array |
| `Val(s)` | `hb_val` | integer first, then float; 0 on failure |
| `PadL(s, n [,pad])` | `hb_padl` | left-pad; truncates right if oversized |
| `PadR(s, n [,pad])` | `hb_padr` | right-pad; truncates left if oversized |
| `PadC(s, n [,pad])` | `hb_padc` | center-pad; left-heavy on odd delta |
| `Space(n)` | `hb_space` | n blank spaces |
| `Replicate(s, n)` | `hb_replicate` | repeat string n times |

## NativeFunction trait

```rust
pub trait NativeFunction<V> {
    fn call(&self, args: Vec<V>) -> V;
    fn name(&self) -> &'static str;
    fn arity(&self) -> (usize, usize); // (min, max)
}
```

All functions return `HbValue`. Wrong-type arguments yield `HbValue::Nil` — no panics.

## Registry

```rust
use swed_bf::all_functions;

for (name, f) in all_functions() {
    resolver.register(name, f);
}
```

## Pending

| Priority | Function | Notes |
|----------|----------|-------|
| next | `StrTran(s, old, new)` | string replace |
| next | `hb_ValToStr(v)` | any value → string |
| next | `DToC(d)` / `CToD(s)` | date ↔ "dd/mm/yy" |
| next | `Time()` / `Seconds()` | clock |
| future | `IsAlpha/Digit/Lower/Upper` | char-type checks |
| future | `Eval(block, ...)` | codeblock evaluation |
| future | `ACopy / AEval / ASort` | advanced array ops |

**Premissa:** novas funções Harbour vão em `swed_bf`, não em `swed_rt`. Funções em `swed_rt::builtins` serão migradas gradualmente.
