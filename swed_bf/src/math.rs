//! Harbour math functions: Abs, Int, Round, Max, Min, Mod, Sqrt, Exp, Log.

use swed_co::NativeFunction;
use swed_rt::HbValue;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn to_f64(v: &HbValue) -> Option<f64> {
    match v {
        HbValue::Integer(n) => Some(*n as f64),
        HbValue::Float(f)   => Some(*f),
        _                   => None,
    }
}

fn from_f64_or_int(f: f64, keep_float: bool) -> HbValue {
    if keep_float {
        HbValue::Float(f)
    } else {
        HbValue::Integer(f as i64)
    }
}

// ---------------------------------------------------------------------------
// Abs( nNumber ) — absolute value
// ---------------------------------------------------------------------------

/// `ABS(nNumber)` — absolute value. Preserves Integer/Float variant.
pub fn hb_abs(n: HbValue) -> HbValue {
    match n {
        HbValue::Integer(v) => HbValue::Integer(v.abs()),
        HbValue::Float(v)   => HbValue::Float(v.abs()),
        _                   => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// Int( nNumber ) — truncate toward zero
// ---------------------------------------------------------------------------

/// `INT(nNumber)` — discards the decimal part (truncates toward zero).
///
/// `INT(3.9)` → `3`, `INT(-3.9)` → `-3`.
pub fn hb_int(n: HbValue) -> HbValue {
    match n {
        HbValue::Integer(v) => HbValue::Integer(v),
        HbValue::Float(v)   => HbValue::Integer(v.trunc() as i64),
        _                   => HbValue::Integer(0),
    }
}

// ---------------------------------------------------------------------------
// Round( nNumber, nPlace ) — round to n decimal places
// ---------------------------------------------------------------------------

/// `ROUND(nNumber, nPlace)` — rounds to `nPlace` decimal places.
///
/// `nPlace` ≤ 0 returns `Integer`; `nPlace` > 0 returns `Float`.
/// Uses "round half away from zero" (standard banker-friendly rounding).
pub fn hb_round(n: HbValue, dec: HbValue) -> HbValue {
    let f = match to_f64(&n) {
        Some(v) => v,
        None    => return HbValue::Nil,
    };
    let d = match dec {
        HbValue::Integer(d) => d,
        _                   => 0,
    };
    let factor = 10f64.powi(d as i32);
    let rounded = (f * factor).round() / factor;
    if d <= 0 {
        HbValue::Integer(rounded as i64)
    } else {
        HbValue::Float(rounded)
    }
}

// ---------------------------------------------------------------------------
// Max / Min
// ---------------------------------------------------------------------------

/// `MAX(xValue, xValue1)` — maximum of two numeric or date values.
pub fn hb_max(a: HbValue, b: HbValue) -> HbValue {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Less) => b,
        _                              => a,
    }
}

/// `MIN(xValue, xValue1)` — minimum of two numeric or date values.
pub fn hb_min(a: HbValue, b: HbValue) -> HbValue {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Greater) => b,
        _                                 => a,
    }
}

// ---------------------------------------------------------------------------
// Mod( nNumber, nNumber1 ) — modulo / remainder
// ---------------------------------------------------------------------------

/// `MOD(a, b)` — remainder after dividing `a` by `b`.
///
/// Returns `Nil` when `b` is zero.
/// Follows Harbour semantics: result has the sign of `a`.
pub fn hb_mod(a: HbValue, b: HbValue) -> HbValue {
    match (to_f64(&a), to_f64(&b)) {
        (Some(_),   Some(bv)) if bv == 0.0 => HbValue::Nil,
        (Some(av),  Some(bv)) => {
            let result = av % bv;
            match (&a, &b) {
                (HbValue::Integer(_), HbValue::Integer(_)) => {
                    HbValue::Integer(result as i64)
                }
                _ => HbValue::Float(result),
            }
        }
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// Sqrt( nNumber ) — square root
// ---------------------------------------------------------------------------

/// `SQRT(nNumber)` — square root. Returns `Nil` for negative input.
pub fn hb_sqrt(n: HbValue) -> HbValue {
    match to_f64(&n) {
        Some(v) if v >= 0.0 => HbValue::Float(v.sqrt()),
        Some(_)             => HbValue::Nil,
        None                => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// Exp( nNumber ) — e raised to nNumber
// ---------------------------------------------------------------------------

/// `EXP(nNumber)` — returns e^nNumber (`std::f64::consts::E.powf(n)`).
pub fn hb_exp(n: HbValue) -> HbValue {
    match to_f64(&n) {
        Some(v) => HbValue::Float(v.exp()),
        None    => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// Log( nNumber ) — natural logarithm
// ---------------------------------------------------------------------------

/// `LOG(nNumber)` — natural logarithm (ln). Returns `Nil` for non-positive input.
pub fn hb_log(n: HbValue) -> HbValue {
    match to_f64(&n) {
        Some(v) if v > 0.0 => HbValue::Float(v.ln()),
        _                  => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// NativeFunction implementations
// ---------------------------------------------------------------------------

macro_rules! unary_math {
    ($struct:ident, $fn:ident, $name:literal) => {
        /// Registry struct.
        pub struct $struct;
        impl NativeFunction<HbValue> for $struct {
            fn call(&self, args: Vec<HbValue>) -> HbValue {
                $fn(args.into_iter().next().unwrap_or(HbValue::Nil))
            }
            fn name(&self) -> &'static str { $name }
            fn arity(&self) -> (usize, usize) { (1, 1) }
        }
    };
}

macro_rules! binary_math {
    ($struct:ident, $fn:ident, $name:literal, $min:literal, $max:literal) => {
        /// Registry struct.
        pub struct $struct;
        impl NativeFunction<HbValue> for $struct {
            fn call(&self, args: Vec<HbValue>) -> HbValue {
                let mut it = args.into_iter();
                let a = it.next().unwrap_or(HbValue::Nil);
                let b = it.next().unwrap_or(HbValue::Nil);
                $fn(a, b)
            }
            fn name(&self) -> &'static str { $name }
            fn arity(&self) -> (usize, usize) { ($min, $max) }
        }
    };
}

unary_math!(Abs,  hb_abs,  "ABS");
unary_math!(Int,  hb_int,  "INT");
unary_math!(Sqrt, hb_sqrt, "SQRT");
unary_math!(Exp,  hb_exp,  "EXP");
unary_math!(Log,  hb_log,  "LOG");

binary_math!(Round, hb_round, "ROUND", 2, 2);
binary_math!(Max,   hb_max,   "MAX",   2, 2);
binary_math!(Min,   hb_min,   "MIN",   2, 2);
binary_math!(Mod,   hb_mod,   "MOD",   2, 2);

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn i(n: i64)  -> HbValue { HbValue::Integer(n) }
    fn f(v: f64)  -> HbValue { HbValue::Float(v) }

    // ── Abs ──────────────────────────────────────────────────────────────────

    #[test]
    fn abs_positive_integer() { assert_eq!(hb_abs(i(5)),    i(5)); }
    #[test]
    fn abs_negative_integer() { assert_eq!(hb_abs(i(-5)),   i(5)); }
    #[test]
    fn abs_negative_float()   { assert_eq!(hb_abs(f(-3.5)), f(3.5)); }
    #[test]
    fn abs_nil()              { assert_eq!(hb_abs(HbValue::Nil), HbValue::Nil); }
    #[test]
    fn native_fn_abs()        {
        assert_eq!(Abs.call(vec![i(-7)]), i(7));
        assert_eq!(Abs.name(), "ABS");
        assert_eq!(Abs.arity(), (1, 1));
    }

    // ── Int ──────────────────────────────────────────────────────────────────

    #[test]
    fn int_truncates_positive() { assert_eq!(hb_int(f(3.9)),  i(3)); }
    #[test]
    fn int_truncates_negative() { assert_eq!(hb_int(f(-3.9)), i(-3)); }
    #[test]
    fn int_integer_identity()   { assert_eq!(hb_int(i(7)),    i(7)); }
    #[test]
    fn int_nil_returns_zero()   { assert_eq!(hb_int(HbValue::Nil), i(0)); }
    #[test]
    fn native_fn_int()          {
        assert_eq!(Int.call(vec![f(2.7)]), i(2));
        assert_eq!(Int.name(), "INT");
    }

    // ── Round ────────────────────────────────────────────────────────────────

    #[test]
    fn round_two_places()    { assert_eq!(hb_round(f(3.456), i(2)), f(3.46)); }
    #[test]
    fn round_zero_places()   { assert_eq!(hb_round(f(3.5),   i(0)), i(4)); }
    #[test]
    fn round_negative_dec()  { assert_eq!(hb_round(i(1567),  i(-2)), i(1600)); }
    #[test]
    fn round_nil()           { assert_eq!(hb_round(HbValue::Nil, i(2)), HbValue::Nil); }
    #[test]
    fn native_fn_round()     {
        // 3.456 * 100 = 345.6 → rounds to 346 → 3.46 (exact in f64)
        assert_eq!(Round.call(vec![f(3.456), i(2)]), f(3.46));
        assert_eq!(Round.name(), "ROUND");
        assert_eq!(Round.arity(), (2, 2));
    }

    // ── Max / Min ─────────────────────────────────────────────────────────────

    #[test]
    fn max_integers()     { assert_eq!(hb_max(i(3), i(7)),   i(7)); }
    #[test]
    fn max_mixed()        { assert_eq!(hb_max(i(3), f(3.1)), f(3.1)); }
    #[test]
    fn min_integers()     { assert_eq!(hb_min(i(3), i(7)),   i(3)); }
    #[test]
    fn min_equal()        { assert_eq!(hb_min(i(5), i(5)),   i(5)); }
    #[test]
    fn native_fn_max()    {
        assert_eq!(Max.call(vec![i(10), i(20)]), i(20));
        assert_eq!(Max.name(), "MAX");
    }
    #[test]
    fn native_fn_min()    {
        assert_eq!(Min.call(vec![i(10), i(20)]), i(10));
        assert_eq!(Min.name(), "MIN");
    }

    // ── Mod ───────────────────────────────────────────────────────────────────

    #[test]
    fn mod_integers()        { assert_eq!(hb_mod(i(10), i(3)),  i(1)); }
    #[test]
    fn mod_negative_num()    { assert_eq!(hb_mod(i(-10), i(3)), i(-1)); }
    #[test]
    fn mod_float()           {
        match hb_mod(f(7.5), f(2.5)) {
            HbValue::Float(v) => assert!((v - 0.0f64).abs() < 1e-10),
            other => panic!("expected Float, got {other:?}"),
        }
    }
    #[test]
    fn mod_div_by_zero()     { assert_eq!(hb_mod(i(5), i(0)), HbValue::Nil); }
    #[test]
    fn native_fn_mod()       {
        assert_eq!(Mod.call(vec![i(10), i(4)]), i(2));
        assert_eq!(Mod.name(), "MOD");
        assert_eq!(Mod.arity(), (2, 2));
    }

    // ── Sqrt ──────────────────────────────────────────────────────────────────

    #[test]
    fn sqrt_perfect_square() {
        assert_eq!(hb_sqrt(i(9)), f(3.0));
    }
    #[test]
    fn sqrt_float()          {
        match hb_sqrt(f(2.0)) {
            HbValue::Float(v) => assert!((v - 1.4142135f64).abs() < 1e-6),
            other => panic!("expected Float, got {other:?}"),
        }
    }
    #[test]
    fn sqrt_negative_returns_nil() { assert_eq!(hb_sqrt(i(-1)), HbValue::Nil); }
    #[test]
    fn sqrt_zero()           { assert_eq!(hb_sqrt(i(0)), f(0.0)); }
    #[test]
    fn native_fn_sqrt()      {
        assert_eq!(Sqrt.call(vec![i(4)]), f(2.0));
        assert_eq!(Sqrt.name(), "SQRT");
    }

    // ── Exp ───────────────────────────────────────────────────────────────────

    #[test]
    fn exp_zero()   { assert_eq!(hb_exp(i(0)), f(1.0)); }
    #[test]
    fn exp_one()    {
        match hb_exp(i(1)) {
            HbValue::Float(v) => assert!((v - std::f64::consts::E).abs() < 1e-10),
            other => panic!("expected Float, got {other:?}"),
        }
    }
    #[test]
    fn exp_nil()    { assert_eq!(hb_exp(HbValue::Nil), HbValue::Nil); }
    #[test]
    fn native_fn_exp() {
        assert_eq!(Exp.call(vec![i(0)]), f(1.0));
        assert_eq!(Exp.name(), "EXP");
    }

    // ── Log ───────────────────────────────────────────────────────────────────

    #[test]
    fn log_one()            { assert_eq!(hb_log(i(1)), f(0.0)); }
    #[test]
    fn log_e()              {
        match hb_log(f(std::f64::consts::E)) {
            HbValue::Float(v) => assert!((v - 1.0).abs() < 1e-10),
            other => panic!("expected Float, got {other:?}"),
        }
    }
    #[test]
    fn log_zero_returns_nil()     { assert_eq!(hb_log(i(0)),  HbValue::Nil); }
    #[test]
    fn log_negative_returns_nil() { assert_eq!(hb_log(i(-1)), HbValue::Nil); }
    #[test]
    fn log_nil()            { assert_eq!(hb_log(HbValue::Nil), HbValue::Nil); }
    #[test]
    fn native_fn_log()      {
        assert_eq!(Log.call(vec![i(1)]), f(0.0));
        assert_eq!(Log.name(), "LOG");
    }
}
