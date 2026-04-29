//! Harbour string padding functions: PadL, PadC, PadR.

use swed_co::NativeFunction;
use swed_rt::HbValue;

// ---------------------------------------------------------------------------
// Internal helper
// ---------------------------------------------------------------------------

fn pad_char(pad: &HbValue) -> char {
    match pad {
        HbValue::String(p) => p.chars().next().unwrap_or(' '),
        _ => ' ',
    }
}

fn extract_args(args: Vec<HbValue>) -> (HbValue, HbValue, HbValue) {
    let mut it = args.into_iter();
    let s   = it.next().unwrap_or(HbValue::Nil);
    let len = it.next().unwrap_or(HbValue::Nil);
    let pad = it.next().unwrap_or(HbValue::Nil);
    (s, len, pad)
}

// ---------------------------------------------------------------------------
// PadL( cStr, nLen [, cPad] ) — pad string on the left to nLen chars.
// ---------------------------------------------------------------------------

/// `PADL(cStr, nLen [, cPad])` — left-pad to length with `cPad` (default space).
///
/// If `cStr` is longer than `nLen`, the rightmost `nLen` chars are returned.
pub fn hb_padl(s: HbValue, len: HbValue, pad: HbValue) -> HbValue {
    let text = match s   { HbValue::String(t) => t, _ => return HbValue::Nil };
    let n    = match len { HbValue::Integer(n) if n > 0 => n as usize, _ => return HbValue::Nil };
    let ch   = pad_char(&pad);
    if text.len() >= n {
        HbValue::String(text[text.len() - n..].to_owned())
    } else {
        HbValue::String(ch.to_string().repeat(n - text.len()) + &text)
    }
}

/// `PADR(cStr, nLen [, cPad])` — right-pad to length with `cPad` (default space).
///
/// If `cStr` is longer than `nLen`, the leftmost `nLen` chars are returned.
pub fn hb_padr(s: HbValue, len: HbValue, pad: HbValue) -> HbValue {
    let text = match s   { HbValue::String(t) => t, _ => return HbValue::Nil };
    let n    = match len { HbValue::Integer(n) if n > 0 => n as usize, _ => return HbValue::Nil };
    let ch   = pad_char(&pad);
    if text.len() >= n {
        HbValue::String(text[..n].to_owned())
    } else {
        let extra = n - text.len();
        HbValue::String(text + &ch.to_string().repeat(extra))
    }
}

/// `PADC(cStr, nLen [, cPad])` — center-pad to length with `cPad` (default space).
///
/// Extra space is distributed left-heavy (left = ceil, right = floor).
/// If `cStr` is longer than `nLen`, leftmost `nLen` chars are returned.
pub fn hb_padc(s: HbValue, len: HbValue, pad: HbValue) -> HbValue {
    let text = match s   { HbValue::String(t) => t, _ => return HbValue::Nil };
    let n    = match len { HbValue::Integer(n) if n > 0 => n as usize, _ => return HbValue::Nil };
    let ch   = pad_char(&pad);
    if text.len() >= n {
        return HbValue::String(text[..n].to_owned());
    }
    let total = n - text.len();
    let right = total / 2;
    let left  = total - right;
    HbValue::String(ch.to_string().repeat(left) + &text + &ch.to_string().repeat(right))
}

// ---------------------------------------------------------------------------
// NativeFunction implementations
// ---------------------------------------------------------------------------

/// Registry struct for `PADL`.
pub struct PadL;
impl NativeFunction<HbValue> for PadL {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (s, len, pad) = extract_args(args);
        hb_padl(s, len, pad)
    }
    fn name(&self) -> &'static str { "PADL" }
    fn arity(&self) -> (usize, usize) { (2, 3) }
}

/// Registry struct for `PADR`.
pub struct PadR;
impl NativeFunction<HbValue> for PadR {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (s, len, pad) = extract_args(args);
        hb_padr(s, len, pad)
    }
    fn name(&self) -> &'static str { "PADR" }
    fn arity(&self) -> (usize, usize) { (2, 3) }
}

/// Registry struct for `PADC`.
pub struct PadC;
impl NativeFunction<HbValue> for PadC {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (s, len, pad) = extract_args(args);
        hb_padc(s, len, pad)
    }
    fn name(&self) -> &'static str { "PADC" }
    fn arity(&self) -> (usize, usize) { (2, 3) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> HbValue { HbValue::String(v.into()) }
    fn n(v: i64)  -> HbValue { HbValue::Integer(v) }
    fn sp()       -> HbValue { HbValue::Nil }

    #[test]
    fn padl_shorter() {
        assert_eq!(hb_padl(s("Hi"), n(5), sp()), s("   Hi"));
    }

    #[test]
    fn padl_exact() {
        assert_eq!(hb_padl(s("Hello"), n(5), sp()), s("Hello"));
    }

    #[test]
    fn padl_longer_truncates_right() {
        assert_eq!(hb_padl(s("Hello!"), n(5), sp()), s("ello!"));
    }

    #[test]
    fn padl_custom_char() {
        assert_eq!(hb_padl(s("42"), n(5), s("0")), s("00042"));
    }

    #[test]
    fn padr_shorter() {
        assert_eq!(hb_padr(s("Hi"), n(5), sp()), s("Hi   "));
    }

    #[test]
    fn padr_longer_truncates_left() {
        assert_eq!(hb_padr(s("Hello!"), n(5), sp()), s("Hello"));
    }

    #[test]
    fn padr_custom_char() {
        assert_eq!(hb_padr(s("Hi"), n(5), s("-")), s("Hi---"));
    }

    #[test]
    fn padc_odd_extra_left_heavy() {
        // "Hi" (2) → n=7: extra=5, left=ceil(5/2)=3, right=2 → "   Hi  "
        assert_eq!(hb_padc(s("Hi"), n(7), sp()), s("   Hi  "));
    }

    #[test]
    fn padc_even_extra_symmetric() {
        // "Hi" (2) → n=6: extra=4, left=2, right=2 → "  Hi  "
        assert_eq!(hb_padc(s("Hi"), n(6), sp()), s("  Hi  "));
    }

    #[test]
    fn padc_longer_truncates() {
        assert_eq!(hb_padc(s("Hello!"), n(5), sp()), s("Hello"));
    }

    #[test]
    fn native_fn_trait_padl() {
        let result = PadL.call(vec![s("X"), n(4), sp()]);
        assert_eq!(result, s("   X"));
        assert_eq!(PadL.name(), "PADL");
        assert_eq!(PadL.arity(), (2, 3));
    }

    #[test]
    fn native_fn_trait_padc() {
        let result = PadC.call(vec![s("AB"), n(6), sp()]);
        assert_eq!(result, s("  AB  "));
        assert_eq!(PadC.name(), "PADC");
    }
}
