CRATE: swed_bf v0.2.0
TYPE: library (lib)
ROLE: Harbour built-in functions backed by HbValue; NativeFunction registry (56 entries)
STATUS: P0-P4 complete; array/math/string/date/misc implemented; 177 tests passing

DEPS:
  swed_co — NativeFunction trait
  swed_rt — HbValue, HbArray

DESIGN:
  Each Harbour fn = free fn + struct implementing NativeFunction<HbValue>.
  all_functions() -> Vec<(&'static str, Box<dyn NativeFunction<HbValue>>)>
  All return HbValue; Nil on type mismatch (no panics, no Result).
  Premissa: novas funções Harbour vão aqui, não em swed_rt.

SOURCE_FILES:
  lib.rs      — all_functions() registry
  array.rs    — Array + AAdd ASize AFill AScan ATail ADel AIns AClone
  date.rs     — Date Year Month Day DToS SToD
  math.rs     — Abs Int Round Max Min Mod Sqrt Exp Log
  misc.rs     — Type ValType Empty PCount
  numeric.rs  — Str StrZero NToS Chr
  string.rs   — AllTrim LTrim RTrim Trim Upper Lower Left Right SubStr At Asc Len Val PadL PadR PadC Space Replicate

FUNCTIONS:

array.rs:
  hb_array_new(n:HbValue) -> HbValue::Array        — Array(n); NIL-filled; negative→Nil
  hb_aadd(arr, val)       -> val                   — AAdd; returns element (registry: copy semantics)
  hb_asize(arr, n)        -> HbValue::Array        — ASize; grow→Nil fill; shrink→truncate
  hb_afill(arr,v,s?,c?)   -> HbValue::Array        — AFill; s,c 1-based optional
  hb_ascan(arr,v,s?,c?)   -> HbValue::Integer      — AScan; 1-based index or 0; s,c optional
  hb_atail(arr)           -> HbValue              — ATail; last element; empty→Nil
  hb_adel(arr, n)         -> HbValue::Array        — ADel; shift left; last→Nil; size preserved
  hb_ains(arr, n)         -> HbValue::Array        — AIns; insert Nil at n; drop last
  hb_aclone(arr)          -> HbValue::Array        — AClone; deep copy via Clone

date.rs:
  hb_date()               -> HbValue::Date         — today (days since 1970-01-01)
  hb_year(d)              -> HbValue::Integer      — Year(dDate)
  hb_month(d)             -> HbValue::Integer      — Month(dDate)
  hb_day(d)               -> HbValue::Integer      — Day(dDate)
  hb_dtos(d)              -> HbValue::String       — DToS → "YYYYMMDD"
  hb_stod(s)              -> HbValue::Date         — SToD ← "YYYYMMDD"; bad→Nil

math.rs:
  hb_abs(n)               -> HbValue              — Abs; preserves Integer/Float
  hb_int(n)               -> HbValue::Integer      — Int; trunc toward zero
  hb_round(n, dec)        -> HbValue              — Round; dec≤0→Integer, dec>0→Float
  hb_max(a, b)            -> HbValue              — Max; PartialOrd; works on Date
  hb_min(a, b)            -> HbValue              — Min
  hb_mod(a, b)            -> HbValue              — Mod; sign of a; b=0→Nil
  hb_sqrt(n)              -> HbValue::Float        — Sqrt; negative→Nil
  hb_exp(n)               -> HbValue::Float        — Exp; e^n
  hb_log(n)               -> HbValue::Float        — Log; natural log; n≤0→Nil

misc.rs:
  hb_type(v)              -> HbValue::String       — Type/ValType → "C"/"N"/"L"/"D"/"A"/"U"
  hb_empty(v)             -> HbValue::Logical      — Empty; Nil/0/""/ []/F./Date(0)→T.
  hb_pcount()             -> HbValue::Integer(0)   — PCount stub; real count needs context

numeric.rs:
  hb_str(n, w?, d?)       -> HbValue::String       — Str; right-justified; overflow→"***"
  hb_strzero(n, w, d?)    -> HbValue::String       — StrZero; zero-padded
  hb_ntos(n)              -> HbValue::String       — hb_NToS; compact; no pad
  hb_chr(n)               -> HbValue::String       — Chr; ASCII 0-255; else ""

string.rs:
  hb_alltrim(s)           -> HbValue::String       — AllTrim; both sides
  hb_ltrim(s)             -> HbValue::String       — LTrim; leading
  hb_rtrim(s)             -> HbValue::String       — RTrim/Trim; trailing
  hb_upper(s)             -> HbValue::String       — Upper
  hb_lower(s)             -> HbValue::String       — Lower
  hb_left(s, n)           -> HbValue::String       — Left; n>Len→full string
  hb_right(s, n)          -> HbValue::String       — Right
  hb_substr(s, start, n?) -> HbValue::String       — SubStr; 1-based; n=Nil→to end
  hb_at(needle, hay)      -> HbValue::Integer      — At; 1-based pos or 0
  hb_asc(s)               -> HbValue::Integer      — Asc; first byte ASCII; ""→0
  hb_len(v)               -> HbValue::Integer      — Len; bytes(str) or elements(arr)
  hb_val(s)               -> HbValue              — Val; int→Integer float→Float else 0
  hb_padl(s, n, pad?)     -> HbValue::String       — PadL; truncates right if oversized
  hb_padr(s, n, pad?)     -> HbValue::String       — PadR; truncates left if oversized
  hb_padc(s, n, pad?)     -> HbValue::String       — PadC; left-heavy on odd delta
  hb_space(n)             -> HbValue::String       — Space; n spaces
  hb_replicate(s, n)      -> HbValue::String       — Replicate; repeat n times

REGISTRY_STRUCTS (one per fn, same name PascalCase):
  Array AAdd ASize AFill AScan ATail ADel AIns AClone
  Date Year Month Day DToS SToD
  Abs Int Round Max Min Mod Sqrt Exp Log
  Type ValType Empty PCount
  Str StrZero NToS Chr
  AllTrim LTrim RTrim Trim Upper Lower Left Right SubStr At Asc Len Val
  PadL PadR PadC Space Replicate

NIL_SEMANTICS:
  All functions: wrong type → HbValue::Nil (no panic, no Result)
  Exception: hb_int(Nil) → Integer(0); hb_val(Nil) → Integer(0) (Harbour compat)

MUTATION_NOTE:
  AAdd/ADel/AIns/AFill in registry operate on copies (Vec<HbValue> by value).
  Generated code uses HbValue::hb_aadd / hb_set_val / hb_set_nested directly.

PENDING:
  StrTran(s,old,new)     hb_ValToStr(v)    DToC/CToD     Time/Seconds
  IsAlpha/Digit/Lower/Upper/Affirmation    Eval(block)
  ACopy/AEval/ASort      PCount with context injection
  Migrate swed_rt::builtins to here (premissa de mínima dependência)
