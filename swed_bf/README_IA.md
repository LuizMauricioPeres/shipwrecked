CRATE: swed_bf v0.2.0
TYPE: library (lib)
ROLE: Harbour built-in function implementations backed by HbValue; NativeFunction registry
STATUS: string/numeric/date/misc implemented; array functions pending; registry wired

DEPS:
  swed_co — NativeFunction, FunctionResolver traits
  swed_rt — HbValue

DESIGN: Each Harbour function = standalone fn + struct implementing NativeFunction.
        lib.rs BfRegistry implements FunctionResolver: name → &dyn NativeFunction.
        All functions return HbValue; propagate Nil on type mismatch (no panics).

SOURCE_FILES:
  lib.rs        — BfRegistry struct; impl FunctionResolver; register_all() constructor
  string.rs     — string operations
  numeric.rs    — numeric formatting and conversion
  date.rs       — date creation and decomposition
  misc.rs       — type inspection and array length
  traits/mod.rs — re-export NativeFunction from swed_co

FUNCTIONS:

string.rs:
  hb_left(s:HbValue, n:HbValue) -> HbValue::String       — Left(cStr, n)
  hb_right(s, n)                -> HbValue::String       — Right(cStr, n)
  hb_substr(s, start, len)      -> HbValue::String       — SubStr(cStr, n, len); len optional
  hb_alltrim(s)                 -> HbValue::String       — AllTrim(cStr)
  hb_upper(s)                   -> HbValue::String       — Upper(cStr)
  hb_lower(s)                   -> HbValue::String       — Lower(cStr)
  hb_at(needle, haystack)       -> HbValue::Integer      — At(sub,str); 1-based; 0=not found
  hb_padl(s, n)                 -> HbValue::String       — PadL(c,n)
  hb_padr(s, n)                 -> HbValue::String       — PadR(c,n)

numeric.rs:
  hb_str(n, width, dec)         -> HbValue::String       — Str(n[,width[,dec]])
  hb_val(s)                     -> HbValue::Float        — Val(cStr)
  hb_strzero(n, len, dec)       -> HbValue::String       — StrZero(n,len[,dec])
  hb_ntos(n)                    -> HbValue::String       — NToS(n) — no trailing zeros

date.rs:
  hb_date()                     -> HbValue::Date         — Date() — today
  hb_dtos(d)                    -> HbValue::String       — DToS(dDate) → "YYYYMMDD"
  hb_stod(s)                    -> HbValue::Date         — SToD(cStr) ← "YYYYMMDD"
  hb_year(d)                    -> HbValue::Integer      — Year(dDate)
  hb_month(d)                   -> HbValue::Integer      — Month(dDate)
  hb_day(d)                     -> HbValue::Integer      — Day(dDate)

misc.rs:
  hb_type(v)                    -> HbValue::String       — Type(x) → "N"/"C"/"L"/"D"/"A"/"U"
  hb_empty(v)                   -> HbValue::Logical      — Empty(x)
  hb_len(v)                     -> HbValue::Integer      — Len(x) for String or Array

NATIVE_FUNCTION_IMPL_PATTERN:
  struct HbLeft;
  impl NativeFunction for HbLeft {
      fn name(&self) -> &'static str { "LEFT" }
      fn arity(&self) -> (usize,usize) { (2,2) }
      fn call(&self, args: Vec<HbValue>) -> HbValue { hb_left(args[0].clone(), args[1].clone()) }
  }

REGISTRY (lib.rs):
  BfRegistry::new() — registers all functions
  impl FunctionResolver for BfRegistry
  Used by: swed binary (arity validation), swed_rt (dispatch)

NIL_SEMANTICS:
  All functions receiving wrong type → return HbValue::Nil (no panic, no Result)
  Matches Harbour runtime behaviour on type errors

PENDING (this crate — ordered by priority):

  P1 — Bloqueadores resta1.rs:
    hb_array(n:HbValue) -> HbValue            — Array(n); cria HbArray de n Nil (pode ir em swed_rt)
    hb_setcolor(spec:HbValue) -> HbValue      — SetColor(); stub ANSI ou delega a swed_ui
    hb_set_val / hb_set_nested                — corrigem E0070; vivem em swed_rt::HbArray

  P2 — String:
    hb_left(s, n)                             — Left(cStr, n)
    hb_right(s, n)                            — Right(cStr, n)
    hb_strtran(s, from, to)                   — StrTran(s, old, new)
    hb_hardcr(s)                              — HardCR(s) — soft \n → hard CR
    hb_valtostr(v)                            — hb_ValToStr(v) — equivale a Display
    hb_transform(val, pic)                    — Transform(val, picture) — complexo; stub OK

  P3 — Array (array.rs):
    hb_atail(a)                               — ATail(a) — último elemento
    hb_adel(a, n)                             — ADel(a, n) — remove e compacta
    hb_ains(a, n)                             — AIns(a, n) — insere Nil na posição
    hb_afill(a, val, start, count)            — AFill(a, v, s, c)
    hb_aclone(a)                              — AClone(a) — deep clone
    hb_acopy(src, dst, start, count, dstart)  — ACopy(...)
    hb_aeval(a, block)                        — AEval(a, {|x| ...})
    hb_asort(a, block)                        — ASort(a, block) — closure comparator

  P4 — Numérico:
    hb_mod(a, b)    hb_sqrt(n)    hb_exp(n)    hb_log(n)
    hb_word(n)      hb_i2bin / hb_bin2i / hb_l2bin / hb_bin2l

  P5 — Data/Hora:
    hb_dtoc(d)      hb_ctod(s)    hb_cmonth(d)  hb_cdow(d)   hb_dow(d)
    hb_time()       hb_seconds()  hb_secs(t)    hb_elaptime(t1,t2)  hb_days(n)

  P6 — Tipo/Validação:
    hb_isalpha / hb_isdigit / hb_islower / hb_isupper
    hb_isaffirm / hb_isnegative
    hb_eval(block, args)          — Eval(bBlock, arg1, ...)
    hb_pcount()                   — PCount() — sempre 0 em contexto Rust

  P7 — Sistema (stub OK):
    hb_os()    hb_version()    hb_curdir()

  MIGRATION:
    hb_chr / hb_asc / hb_space / hb_replicate — já em swed_rt; migrar para cá gradualmente
    impl Into<HbValue> on fn params            — callers passam literais sem .into()

INTEGRATES_WITH:
  swed_rt: imports HbValue; functions will eventually replace swed_rt::builtins
  swed_co: implements NativeFunction trait
  swed (binary): BfRegistry passed to semantic/codegen for arity validation
