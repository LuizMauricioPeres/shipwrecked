CRATE: swed_rt v0.2.0
TYPE: library (lib)
ROLE: Harbour runtime — HbValue type system, builtins, PUBLIC/MEMVAR store, array macro
STATUS: functional; self-contained; builtins partially duplicated with swed_bf (migration ongoing)

DEPS: none (intentionally zero workspace deps; only std)

DESIGN_RULE: Generated Rust code links ONLY against swed_rt. No other workspace crate.

PUBLIC_API (re-exported in lib.rs):

TYPES:
  HbValue — Harbour "any" type
    Nil
    Logical(bool)
    Integer(i64)
    Float(f64)
    String(String)
    Array(HbArray)
    Date(i32)          — days since 1970-01-01
  impl: Debug Clone PartialEq Display
  impl: Add Sub Mul Div Rem Neg (all NIL-safe; Nil op X → Nil)
  impl: PartialOrd (Harbour numeric coercion rules)
  impl: Index<&HbValue>  → ref to inner HbArray element (zero-clone subscript)
  methods:
    hb_aadd(val)        — push to Array variant; panics if not Array
    hb_len()            — returns Integer(len) for String/Array; Nil otherwise
    hb_len_as_i64()     — unwrapped len for codegen use
    hb_instr_contains() — substring membership test
    into_hb()           — via IntoHbValue trait

  HbArray — 1-indexed dynamic array
    new() → HbArray
    hb_aadd(val)        — push
    len() → usize
    get(i:usize) → &HbValue   — 0-based internal; callers use 1-based via HbValue::Index
    get_mut(i) → &mut HbValue
  impl: Debug Clone PartialEq Index<usize>
  macro: hb_array![e1,e2,...] → HbArray

TRAITS (unwrap.rs):
  IntoHbValue
    fn into_hb(self) -> HbValue
    impl for: i64 f64 bool String &str i32 u32
  TryAsRust
    fn try_as_i64(&self) -> Option<i64>
    fn try_as_f64(&self) -> Option<f64>
    fn try_as_bool(&self) -> Option<bool>
    fn try_as_str(&self) -> Option<&str>
  ScopeStack
    fn new() → ScopeStack
    fn inject(name:&str, val:HbValue)
    fn get(name:&str) -> HbValue    — returns Nil if not found
  UnwrapError — error type for failed TryAsRust conversions

PUBLIC_STORE (publics_var.rs):
  pub_declare(name:&str)
  pub_get(name:&str) -> HbValue
  pub_set(name:&str, val:HbValue)
  memvar_assign(name:&str, val:HbValue)   — MEMVAR / PRIVATE
  memvar_get(name:&str) -> HbValue
  public_store() -> Arc<RwLock<PublicVars>>  — raw access for codegen

BUILTINS (builtins.rs — partial; migration target: swed_bf):
  hb_range(from,to,step:HbValue) -> impl Iterator<Item=HbValue>
  hb_qout(v:HbValue)             — println!("{v}")
  hb_qqout(v:HbValue)            — print!("{v}")
  hb_str(n, width, dec)          — Harbour Str() with optional width/decimals
  hb_substr(s, start, len)
  hb_alltrim / hb_ltrim / hb_rtrim / hb_upper / hb_lower
  hb_at(needle, haystack) -> HbValue::Integer (1-based position; 0=not found)
  hb_rat(needle, haystack)       — reverse At
  hb_padr / hb_padl(s, n)
  hb_replicate(s, n)
  hb_space(n)
  hb_chr(n) / hb_asc(c)
  hb_val(s) -> HbValue::Float
  hb_valtype(v) -> HbValue::String  — "N"/"C"/"L"/"D"/"A"/"U"
  hb_isnil(v) -> HbValue::Logical
  hb_empty(v) -> HbValue::Logical
  hb_len(v) -> HbValue::Integer
  hb_abs / hb_int / hb_max / hb_min / hb_round
  hb_aadd(arr, val) -> HbValue   — returns arr
  hb_ascan(arr, val) -> HbValue::Integer
  hb_asize(arr, n)               — resize array
  hb_macro(name:&str) -> HbValue — runtime &varName expansion via PUBLIC store
  hb_instr(needle, haystack)     — wrapper for hb_instr_contains

INVARIANTS:
  - NIL propagation: all arithmetic ops return Nil if either operand is Nil (no panic)
  - 1-indexed arrays: external API uses 1-based; internal Vec is 0-based
  - Thread-safety: PUBLIC store is Arc<RwLock<>>; STATIC uses thread_local!
  - No unsafe code (forbid attribute)
  - No external dependencies

PENDING (this crate):
  hb_set_val(idx:HbValue, val:HbValue)                   (priority: H) — corrige E0070 simples
    → LHS de `x.hb_get_val(i) = v` reescrito para `x.hb_set_val(i, v)` pelo swed_nm
  hb_set_nested(i:HbValue, j:HbValue, val:HbValue)       (priority: H) — corrige E0070 chained
    → `x.hb_get_val(i).hb_get_val(j) = v` → `x.hb_set_nested(i, j, v)`
  hb_array(n:HbValue) -> HbValue                         (priority: H) — cria HbArray de n Nil
    → Harbour Array(n); usado em resta1.rs linha 81/83/115
  impl AddAssign for HbValue  (+= on let mut x)          (priority: H)
  impl SubAssign / MulAssign / DivAssign                  (priority: H)
  hb_eq(other) — SET EXACT OFF fuzzy string match         (priority: H)
  hb_exact_eq(other) — SET EXACT ON strict match          (priority: H)
  hb_get_val tornar não-genérico (aceitar HbValue direto) (priority: M) — corrige E0282
  Migrate all builtins.rs functions to swed_bf            (priority: L)

INTEGRATES_WITH:
  swed (binary): generated code links against this crate
  swed_bf: will import HbValue; builtins.rs functions migrate there incrementally
  swed_db: WorkArea methods take/return HbValue
  swed_ui: GetElement::value() returns HbValue
