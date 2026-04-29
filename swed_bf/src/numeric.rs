//! Harbour numeric-to-string and ASCII functions: Str, StrZero, HB_NToS, Chr.

use swed_co::NativeFunction;
use swed_rt::HbValue;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Extract `f64` from an `HbValue`. Returns `None` for non-numeric values.
fn to_f64(v: &HbValue) -> Option<f64> {
    match v {
        HbValue::Integer(n) => Some(*n as f64),
        HbValue::Float(f)   => Some(*f),
        _ => None,
    }
}

/// Format a non-negative `f64` with `decimals` decimal places.
fn format_abs(n: f64, decimals: usize) -> String {
    if decimals > 0 {
        format!("{n:.decimals$}")
    } else {
        format!("{:.0}", n)
    }
}

/// Extract positional args `(num, len, dec)` from a `Vec<HbValue>`.
fn args3(args: Vec<HbValue>) -> (HbValue, HbValue, HbValue) {
    let mut it = args.into_iter();
    let num = it.next().unwrap_or(HbValue::Nil);
    let len = it.next().unwrap_or(HbValue::Nil);
    let dec = it.next().unwrap_or(HbValue::Nil);
    (num, len, dec)
}

// ---------------------------------------------------------------------------
// Str( nNum [, nLen [, nDec]] )
// ---------------------------------------------------------------------------

/// `STR(nNum [, nLen [, nDec]])` — right-justified numeric string.
///
/// - `nLen` controls total width (right-justified with spaces).
/// - `nDec` controls decimal places (default 0).
/// - If `nLen` is omitted or zero, uses the minimum required width.
/// - If the formatted number exceeds `nLen`, returns `"****…"` (overflow).
///
/// ```text
/// STR(42)          → "42"
/// STR(3.14, 8, 2)  → "    3.14"
/// STR(-42, 5)      → "  -42"
/// STR(99999, 4)    → "****"
/// ```
pub fn hb_str(num: HbValue, len: HbValue, dec: HbValue) -> HbValue {
    let n = match to_f64(&num) {
        Some(v) => v,
        None    => return HbValue::String(" ".repeat(10)),
    };
    let decimals = match dec {
        HbValue::Integer(d) if d >= 0 => d as usize,
        _ => 0,
    };
    let formatted = if n < 0.0 {
        let body = format_abs(n.abs(), decimals);
        format!("-{body}")
    } else {
        format_abs(n, decimals)
    };

    match len {
        HbValue::Integer(l) if l > 0 => {
            let l = l as usize;
            if formatted.len() > l {
                HbValue::String("*".repeat(l))
            } else {
                HbValue::String(format!("{formatted:>l$}"))
            }
        }
        _ => HbValue::String(formatted),
    }
}

// ---------------------------------------------------------------------------
// StrZero( nNum, nLen [, nDec] )
// ---------------------------------------------------------------------------

/// `STRZERO(nNum, nLen [, nDec])` — zero-padded numeric string.
///
/// Like `STR()` but pads with `'0'` instead of spaces.
/// For negative numbers the sign comes first, then zeros.
/// Overflows return `"****…"`.
///
/// ```text
/// STRZERO(42, 6)       → "000042"
/// STRZERO(3.14, 8, 2)  → "00003.14"
/// STRZERO(-42, 6)      → "-00042"
/// STRZERO(99999, 4)    → "****"
/// ```
pub fn hb_strzero(num: HbValue, len: HbValue, dec: HbValue) -> HbValue {
    let n = match to_f64(&num) {
        Some(v) => v,
        None    => return HbValue::Nil,
    };
    let total = match len {
        HbValue::Integer(l) if l > 0 => l as usize,
        _ => return HbValue::Nil,
    };
    let decimals = match dec {
        HbValue::Integer(d) if d >= 0 => d as usize,
        _ => 0,
    };

    let negative     = n < 0.0;
    let body         = format_abs(n.abs(), decimals);
    // Width available for digits (sign takes 1 char when negative)
    let digit_width  = if negative { total.saturating_sub(1) } else { total };

    if body.len() > digit_width {
        return HbValue::String("*".repeat(total));
    }

    let padded = format!("{body:0>digit_width$}");
    let result = if negative {
        format!("-{padded}")
    } else {
        padded
    };

    HbValue::String(result)
}

// ---------------------------------------------------------------------------
// HB_NToS( nNum )
// ---------------------------------------------------------------------------

/// `HB_NTOS(nNum)` — compact numeric-to-string, no padding, no trailing spaces.
///
/// Integers are formatted without decimal point.
/// Floats use the minimal representation needed (`format!("{}")` in Rust).
///
/// ```text
/// HB_NTOS(42)    → "42"
/// HB_NTOS(-7)    → "-7"
/// HB_NTOS(3.14)  → "3.14"
/// HB_NTOS(0)     → "0"
/// ```
pub fn hb_ntos(val: HbValue) -> HbValue {
    match val {
        HbValue::Integer(n) => HbValue::String(n.to_string()),
        HbValue::Float(f)   => HbValue::String(format!("{f}")),
        _ => HbValue::String(String::new()),
    }
}

// ---------------------------------------------------------------------------
// Chr( nAscii )
// ---------------------------------------------------------------------------

/// `CHR(nAscii)` — converts an ASCII code (0–255) to a single-character string.
///
/// Values outside 0–255 return an empty string.
///
/// ```text
/// CHR(65)  → "A"
/// CHR(32)  → " "
/// CHR(0)   → "\0"
/// CHR(256) → ""
/// ```
pub fn hb_chr(val: HbValue) -> HbValue {
    match val {
        HbValue::Integer(n) if (0..=255).contains(&n) => {
            HbValue::String((n as u8 as char).to_string())
        }
        _ => HbValue::String(String::new()),
    }
}

// ---------------------------------------------------------------------------
// NativeFunction implementations
// ---------------------------------------------------------------------------

/// Registry struct for `STR`.
pub struct Str;
impl NativeFunction<HbValue> for Str {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (num, len, dec) = args3(args);
        hb_str(num, len, dec)
    }
    fn name(&self) -> &'static str { "STR" }
    fn arity(&self) -> (usize, usize) { (1, 3) }
}

/// Registry struct for `STRZERO`.
pub struct StrZero;
impl NativeFunction<HbValue> for StrZero {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (num, len, dec) = args3(args);
        hb_strzero(num, len, dec)
    }
    fn name(&self) -> &'static str { "STRZERO" }
    fn arity(&self) -> (usize, usize) { (2, 3) }
}

/// Registry struct for `HB_NTOS`.
pub struct NToS;
impl NativeFunction<HbValue> for NToS {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_ntos(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "HB_NTOS" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `CHR`.
pub struct Chr;
impl NativeFunction<HbValue> for Chr {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_chr(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "CHR" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn i(n: i64)  -> HbValue { HbValue::Integer(n) }
    fn f(v: f64)  -> HbValue { HbValue::Float(v) }
    fn s(v: &str) -> HbValue { HbValue::String(v.into()) }
    fn nil()      -> HbValue { HbValue::Nil }

    // ── Str ─────────────────────────────────────────────────────────────────

    #[test]
    fn str_integer_no_width() {
        assert_eq!(hb_str(i(42), nil(), nil()), s("42"));
    }

    #[test]
    fn str_integer_with_width() {
        assert_eq!(hb_str(i(42), i(6), nil()), s("    42"));
    }

    #[test]
    fn str_float_with_decimals() {
        assert_eq!(hb_str(f(3.14), i(8), i(2)), s("    3.14"));
    }

    #[test]
    fn str_negative() {
        assert_eq!(hb_str(i(-42), i(5), nil()), s("  -42"));
    }

    #[test]
    fn str_overflow_returns_asterisks() {
        assert_eq!(hb_str(i(99_999), i(4), nil()), s("****"));
    }

    #[test]
    fn str_exact_fit() {
        assert_eq!(hb_str(i(1234), i(4), nil()), s("1234"));
    }

    #[test]
    fn str_zero_padded_decimals() {
        // STR(3.1, 6, 3) → "3.100" right-justified in 6 → " 3.100"
        assert_eq!(hb_str(f(3.1), i(6), i(3)), s(" 3.100"));
    }

    // ── StrZero ─────────────────────────────────────────────────────────────

    #[test]
    fn strzero_positive() {
        assert_eq!(hb_strzero(i(42), i(6), nil()), s("000042"));
    }

    #[test]
    fn strzero_with_decimals() {
        assert_eq!(hb_strzero(f(3.14), i(8), i(2)), s("00003.14"));
    }

    #[test]
    fn strzero_negative() {
        assert_eq!(hb_strzero(i(-42), i(6), nil()), s("-00042"));
    }

    #[test]
    fn strzero_overflow() {
        assert_eq!(hb_strzero(i(99_999), i(4), nil()), s("****"));
    }

    #[test]
    fn strzero_exact_fit() {
        assert_eq!(hb_strzero(i(1234), i(4), nil()), s("1234"));
    }

    #[test]
    fn strzero_negative_overflow() {
        // -999 needs 4 chars, only 3 available → "***"
        assert_eq!(hb_strzero(i(-999), i(3), nil()), s("***"));
    }

    // ── HB_NToS ─────────────────────────────────────────────────────────────

    #[test]
    fn ntos_integer() {
        assert_eq!(hb_ntos(i(42)),  s("42"));
        assert_eq!(hb_ntos(i(-7)),  s("-7"));
        assert_eq!(hb_ntos(i(0)),   s("0"));
    }

    #[test]
    fn ntos_float() {
        assert_eq!(hb_ntos(f(3.14)), s("3.14"));
        assert_eq!(hb_ntos(f(-1.5)), s("-1.5"));
    }

    #[test]
    fn ntos_nil_returns_empty() {
        assert_eq!(hb_ntos(nil()), s(""));
    }

    // ── Chr ─────────────────────────────────────────────────────────────────

    #[test]
    fn chr_uppercase_a() {
        assert_eq!(hb_chr(i(65)), s("A"));
    }

    #[test]
    fn chr_space() {
        assert_eq!(hb_chr(i(32)), s(" "));
    }

    #[test]
    fn chr_null() {
        assert_eq!(hb_chr(i(0)), HbValue::String("\0".into()));
    }

    #[test]
    fn chr_max_ascii() {
        assert_eq!(hb_chr(i(255)), HbValue::String("\u{ff}".into()));
    }

    #[test]
    fn chr_out_of_range() {
        assert_eq!(hb_chr(i(256)),  s(""));
        assert_eq!(hb_chr(i(-1)),   s(""));
        assert_eq!(hb_chr(nil()),   s(""));
    }

    // ── NativeFunction trait ─────────────────────────────────────────────────

    #[test]
    fn native_str() {
        assert_eq!(Str.call(vec![i(42), i(6), nil()]), s("    42"));
        assert_eq!(Str.name(), "STR");
        assert_eq!(Str.arity(), (1, 3));
    }

    #[test]
    fn native_strzero() {
        assert_eq!(StrZero.call(vec![i(42), i(6), nil()]), s("000042"));
        assert_eq!(StrZero.name(), "STRZERO");
        assert_eq!(StrZero.arity(), (2, 3));
    }

    #[test]
    fn native_ntos() {
        assert_eq!(NToS.call(vec![i(100)]), s("100"));
        assert_eq!(NToS.name(), "HB_NTOS");
        assert_eq!(NToS.arity(), (1, 1));
    }

    #[test]
    fn native_chr() {
        assert_eq!(Chr.call(vec![i(65)]), s("A"));
        assert_eq!(Chr.name(), "CHR");
        assert_eq!(Chr.arity(), (1, 1));
    }
}
