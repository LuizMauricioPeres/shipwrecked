CRATE: swed_rt v0.2.0
TYPE: library (lib)
ROLE: Harbour runtime — HbValue type system, operators, builtins, PUBLIC/MEMVAR store
STATUS: functional; zero workspace deps; builtins partial (migration to swed_bf ongoing)

DEPS: none (zero workspace deps; only std)

DESIGN_RULE: Generated Rust code links ONLY against swed_rt. No other workspace crate.

PUBLIC_API (lib.rs re-exports):

TYPES:
  HbValue — Harbour "any" type
    Nil | Logical(bool) | Integer(i64) | Float(f64)
    String(String) | Array(HbArray) | Date(i32)   [days since 1970-01-01]
  impl: Debug Clone PartialEq PartialOrd Display
  impl: Add Sub Mul Div Rem Neg (all NIL-safe: Nil op X → Nil)
  impl: AddAssign SubAssign
  impl: Not (logical not; non-Logical → Nil)
  impl: Index<&HbValue> → &HbValue  (zero-clone 1-based subscript)
  From: i64 i32 f64 bool String &str HbArray

  HbValue methods (value.rs):
    hb_set_val(&mut self, idx:HbValue, val:HbValue)
    hb_set_nested(&mut self, row:HbValue, col:HbValue, val:HbValue)
    hb_get_val(idx:HbValue) -> HbValue
    hb_len() -> HbValue::Integer
    hb_len_as_i64() -> i64
    hb_trunc(dec:i32) -> HbValue
    hb_round(dec:i32) -> HbValue
    hb_str_format(w:i32, d:i32) -> String
    format_date() -> String       — "dd/mm/yyyy"
    hb_instr_contains(&HbValue) -> HbValue::Logical
    val_type() -> &'static str    — "N"/"C"/"L"/"D"/"A"/"U"
    is_truthy() -> bool
    to_hb_str() -> String
    len() -> usize
    pow_hb(exp) -> HbValue        — ^ operator

  HbArray (array.rs):
    new() / with_capacity(n) / filled(n)  — constructors
    hb_aadd(&mut self, val) -> &mut Self
    hb_asize(&mut self, n:HbValue)
    hb_adel(&mut self, idx:HbValue)
    hb_ains(&mut self, idx:HbValue)
    hb_afill(&mut self, val:&HbValue, start:usize, count:usize)
    hb_ascan(&self, val:&HbValue) -> HbValue::Integer
    hb_asort(&mut self)
    hb_get(i:usize) -> HbValue      — 1-based; OOB→Nil
    hb_get_val(idx:HbValue) -> HbValue
    hb_set(i:usize, val)            — 1-based; OOB→panic
    hb_set_val(idx:HbValue, val)
    len() / is_empty() / hb_len() -> HbValue::Integer
    iter() / iter_mut()
  macro: hb_array![e1,e2,...] -> HbArray

TRAITS (unwrap.rs):
  IntoHbValue::into_hb(self) -> HbValue
  TryAsRust: try_as_i64/f64/bool/str
  ScopeStack: new() / inject(name,val) / get(name) -> HbValue
  UnwrapError

PUBLIC_STORE (publics_var.rs):
  pub_declare(name) / pub_get(name) / pub_set(name, val)
  memvar_assign(name, val) / memvar_get(name)
  public_store() -> Arc<RwLock<PublicVars>>

BUILTINS (builtins.rs — migration target: swed_bf):
  hb_array(n:HbValue) -> HbValue          — Array(n); NIL-filled
  hb_range(from,to,step) -> HbRangeIter   — FOR loop
  hb_qout(v) / hb_qqout(v)               — ? / ??
  hb_space(n) / hb_replicate(s,n)        — Space / Replicate
  hb_chr(n) / hb_asc(s)                  — Chr / Asc
  hb_str(n,w,d) / hb_val(s)              — Str / Val
  hb_substr(s,n,l)                        — SubStr (1-based)
  hb_alltrim/ltrim/rtrim/upper/lower(s)
  hb_at(n,h) / hb_rat(n,h)              — At / RAt (1-based; 0=not found)
  hb_padl/padr(s,n,pad)
  hb_len(v) -> Integer                    — Len (String bytes or Array elements)
  hb_int(n) / hb_abs(n)                  — Int / Abs
  hb_round(n,d) / hb_max(a,b) / hb_min(a,b)
  hb_valtype(v) / hb_empty(v) / hb_isnil(v)
  hb_aadd(arr:&mut HbArray, val) -> HbValue
  hb_asize(arr,n) / hb_ascan(arr,val)
  hb_setcolor(col) -> HbValue            — stub (returns "")
  hb_inkey(timeout) -> HbValue           — stub (returns 0)
  hb_lastkey() -> HbValue               — stub (returns 0)
  hb_macro(val) -> HbValue              — &varName stub (returns val)
  hb_instr(needle, haystack) -> HbValue — wrapper for hb_instr_contains

INVARIANTS:
  NIL propagation: arithmetic/comparison with Nil → Nil (no panic)
  1-indexed arrays: external 1-based; internal Vec 0-based (adjusted once at boundary)
  Thread-safety: PUBLIC store is Arc<RwLock<>>
  No unsafe code (workspace forbid)
  No external dependencies (zero deps)

PENDING (this crate):
  hb_eq fuzzy string match (SET EXACT OFF semantics)
  MulAssign / DivAssign operators
  hb_get_val non-generic (accept HbValue directly, fixes E0282)
  Migrate builtins.rs to swed_bf (premissa de mínima dependência)
