//! Harbour core/misc functions: Type, ValType, Empty, PCount.

use swed_co::NativeFunction;
use swed_rt::HbValue;

// ---------------------------------------------------------------------------
// Type / ValType — type inspection
// ---------------------------------------------------------------------------

/// `TYPE(x)` / `VALTYPE(x)` — returns the Harbour type character.
///
/// | Return | Harbour type      |
/// |--------|-------------------|
/// | `"C"`  | Character string  |
/// | `"N"`  | Numeric           |
/// | `"L"`  | Logical           |
/// | `"D"`  | Date              |
/// | `"A"`  | Array             |
/// | `"U"`  | NIL / undefined   |
pub fn hb_type(val: HbValue) -> HbValue {
    HbValue::String(val.val_type().to_owned())
}

// ---------------------------------------------------------------------------
// Empty — empty/zero/nil check
// ---------------------------------------------------------------------------

/// `EMPTY(x)` — returns `.T.` if the value is considered "empty" in Harbour.
///
/// | Type    | Empty when                     |
/// |---------|--------------------------------|
/// | NIL     | always                         |
/// | Logical | `.F.`                          |
/// | Integer | `0`                            |
/// | Float   | `0.0`                          |
/// | String  | `""` or all-spaces             |
/// | Array   | zero elements                  |
/// | Date    | days-since-epoch == 0          |
pub fn hb_empty(val: HbValue) -> HbValue {
    let empty = match &val {
        HbValue::Nil           => true,
        HbValue::Logical(b)    => !b,
        HbValue::Integer(n)    => *n == 0,
        HbValue::Float(f)      => *f == 0.0,
        HbValue::String(s)     => s.trim().is_empty(),
        HbValue::Array(a)      => a.is_empty(),
        HbValue::Date(d)       => *d == 0,
    };
    HbValue::Logical(empty)
}

// ---------------------------------------------------------------------------
// PCount — argument count
// ---------------------------------------------------------------------------

/// `PCOUNT()` — returns the number of arguments passed to the current function.
///
/// In the NativeFunction registry this is a stub returning `0`.
/// Proper per-call count requires runtime context injection (future work).
pub fn hb_pcount() -> HbValue {
    HbValue::Integer(0)
}

// ---------------------------------------------------------------------------
// NativeFunction implementations
// ---------------------------------------------------------------------------

/// Registry struct for `TYPE`.
pub struct Type;
impl NativeFunction<HbValue> for Type {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_type(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "TYPE" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `VALTYPE`.
pub struct ValType;
impl NativeFunction<HbValue> for ValType {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_type(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "VALTYPE" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `EMPTY`.
pub struct Empty;
impl NativeFunction<HbValue> for Empty {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_empty(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "EMPTY" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `PCOUNT`.
pub struct PCount;
impl NativeFunction<HbValue> for PCount {
    fn call(&self, _args: Vec<HbValue>) -> HbValue {
        hb_pcount()
    }
    fn name(&self) -> &'static str { "PCOUNT" }
    fn arity(&self) -> (usize, usize) { (0, 0) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> HbValue { HbValue::String(v.into()) }
    fn n(v: i64)  -> HbValue { HbValue::Integer(v) }
    fn l(v: bool) -> HbValue { HbValue::Logical(v) }

    // ── Type / ValType ───────────────────────────────────────────────────────

    #[test]
    fn type_character() {
        assert_eq!(hb_type(s("hi")), s("C"));
    }

    #[test]
    fn type_numeric_integer() {
        assert_eq!(hb_type(n(42)), s("N"));
    }

    #[test]
    fn type_numeric_float() {
        assert_eq!(hb_type(HbValue::Float(3.14)), s("N"));
    }

    #[test]
    fn type_logical() {
        assert_eq!(hb_type(l(true)), s("L"));
    }

    #[test]
    fn type_date() {
        assert_eq!(hb_type(HbValue::Date(0)), s("D"));
    }

    #[test]
    fn type_nil() {
        assert_eq!(hb_type(HbValue::Nil), s("U"));
    }

    #[test]
    fn native_fn_type() {
        assert_eq!(Type.call(vec![n(1)]), s("N"));
        assert_eq!(Type.name(), "TYPE");
        assert_eq!(Type.arity(), (1, 1));
    }

    #[test]
    fn native_fn_valtype_same_logic() {
        assert_eq!(ValType.call(vec![s("x")]), s("C"));
        assert_eq!(ValType.call(vec![HbValue::Nil]), s("U"));
        assert_eq!(ValType.name(), "VALTYPE");
        assert_eq!(ValType.arity(), (1, 1));
    }

    // ── Empty ────────────────────────────────────────────────────────────────

    #[test]
    fn empty_nil() {
        assert_eq!(hb_empty(HbValue::Nil), l(true));
    }

    #[test]
    fn empty_logical_false() {
        assert_eq!(hb_empty(l(false)), l(true));
        assert_eq!(hb_empty(l(true)),  l(false));
    }

    #[test]
    fn empty_zero_integer() {
        assert_eq!(hb_empty(n(0)), l(true));
        assert_eq!(hb_empty(n(1)), l(false));
    }

    #[test]
    fn empty_zero_float() {
        assert_eq!(hb_empty(HbValue::Float(0.0)), l(true));
        assert_eq!(hb_empty(HbValue::Float(0.1)), l(false));
    }

    #[test]
    fn empty_string_blank() {
        assert_eq!(hb_empty(s("")),     l(true));
        assert_eq!(hb_empty(s("   ")), l(true));
        assert_eq!(hb_empty(s("x")),   l(false));
    }

    #[test]
    fn empty_array() {
        use swed_rt::HbArray;
        let mut a = HbArray::new();
        assert_eq!(hb_empty(HbValue::Array(a.clone())), l(true));
        a.hb_aadd(HbValue::Nil);
        assert_eq!(hb_empty(HbValue::Array(a)), l(false));
    }

    #[test]
    fn empty_date_zero() {
        assert_eq!(hb_empty(HbValue::Date(0)),   l(true));
        assert_eq!(hb_empty(HbValue::Date(100)), l(false));
    }

    #[test]
    fn native_fn_empty() {
        assert_eq!(Empty.call(vec![HbValue::Nil]), l(true));
        assert_eq!(Empty.call(vec![n(42)]),        l(false));
        assert_eq!(Empty.name(), "EMPTY");
        assert_eq!(Empty.arity(), (1, 1));
    }

    // ── PCount ───────────────────────────────────────────────────────────────

    #[test]
    fn pcount_stub_returns_zero() {
        assert_eq!(hb_pcount(), n(0));
    }

    #[test]
    fn native_fn_pcount() {
        assert_eq!(PCount.call(vec![]), n(0));
        assert_eq!(PCount.name(), "PCOUNT");
        assert_eq!(PCount.arity(), (0, 0));
    }
}
