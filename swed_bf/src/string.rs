//! Harbour string functions: padding, trim, case, slice, search, type conversion.

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
// AllTrim / LTrim / RTrim / Trim
// ---------------------------------------------------------------------------

/// `ALLTRIM(cStr)` — removes leading and trailing spaces.
pub fn hb_alltrim(s: HbValue) -> HbValue {
    match s {
        HbValue::String(t) => HbValue::String(t.trim().to_owned()),
        other => other,
    }
}

/// `LTRIM(cStr)` — removes leading spaces.
pub fn hb_ltrim(s: HbValue) -> HbValue {
    match s {
        HbValue::String(t) => HbValue::String(t.trim_start().to_owned()),
        other => other,
    }
}

/// `RTRIM(cStr)` / `TRIM(cStr)` — removes trailing spaces.
pub fn hb_rtrim(s: HbValue) -> HbValue {
    match s {
        HbValue::String(t) => HbValue::String(t.trim_end().to_owned()),
        other => other,
    }
}

// ---------------------------------------------------------------------------
// Upper / Lower
// ---------------------------------------------------------------------------

/// `UPPER(cStr)` — converts to uppercase.
pub fn hb_upper(s: HbValue) -> HbValue {
    match s {
        HbValue::String(t) => HbValue::String(t.to_uppercase()),
        other => other,
    }
}

/// `LOWER(cStr)` — converts to lowercase.
pub fn hb_lower(s: HbValue) -> HbValue {
    match s {
        HbValue::String(t) => HbValue::String(t.to_lowercase()),
        other => other,
    }
}

// ---------------------------------------------------------------------------
// Left / Right
// ---------------------------------------------------------------------------

/// `LEFT(cStr, nLen)` — first `nLen` characters of `cStr`.
///
/// Returns empty string if `nLen` ≤ 0 or args are invalid.
/// Does not panic if `nLen` > `Len(cStr)`.
pub fn hb_left(s: HbValue, n: HbValue) -> HbValue {
    match (s, n) {
        (HbValue::String(t), HbValue::Integer(n)) if n >= 0 => {
            HbValue::String(t.chars().take(n as usize).collect())
        }
        _ => HbValue::String(String::new()),
    }
}

/// `RIGHT(cStr, nLen)` — last `nLen` characters of `cStr`.
///
/// Returns empty string if `nLen` ≤ 0 or args are invalid.
pub fn hb_right(s: HbValue, n: HbValue) -> HbValue {
    match (s, n) {
        (HbValue::String(t), HbValue::Integer(n)) if n >= 0 => {
            let chars: Vec<char> = t.chars().collect();
            let skip = chars.len().saturating_sub(n as usize);
            HbValue::String(chars[skip..].iter().collect())
        }
        _ => HbValue::String(String::new()),
    }
}

// ---------------------------------------------------------------------------
// SubStr
// ---------------------------------------------------------------------------

/// `SUBSTR(cStr, nStart [, nLen])` — substring extraction (1-based).
///
/// - `nStart` is 1-based; positions beyond the string return `""`.
/// - `nLen` omitted (or Nil) means "to end of string".
pub fn hb_substr(s: HbValue, start: HbValue, len: HbValue) -> HbValue {
    let text = match s {
        HbValue::String(t) => t,
        _ => return HbValue::Nil,
    };
    let start = match start {
        HbValue::Integer(n) if n >= 1 => (n - 1) as usize,
        _ => return HbValue::Nil,
    };
    let chars: Vec<char> = text.chars().collect();
    if start >= chars.len() {
        return HbValue::String(String::new());
    }
    let slice = &chars[start..];
    let result: String = match len {
        HbValue::Integer(n) if n > 0 => slice.iter().take(n as usize).collect(),
        _ => slice.iter().collect(),
    };
    HbValue::String(result)
}

// ---------------------------------------------------------------------------
// At / Asc
// ---------------------------------------------------------------------------

/// `AT(cSearch, cString)` — 1-based position of first occurrence, or 0.
pub fn hb_at(needle: HbValue, haystack: HbValue) -> HbValue {
    match (needle, haystack) {
        (HbValue::String(n), HbValue::String(h)) => match h.find(n.as_str()) {
            Some(pos) => HbValue::Integer((pos + 1) as i64),
            None      => HbValue::Integer(0),
        },
        _ => HbValue::Integer(0),
    }
}

/// `ASC(cChar)` — ASCII code of the first character, or 0.
pub fn hb_asc(s: HbValue) -> HbValue {
    match s {
        HbValue::String(t) => match t.bytes().next() {
            Some(b) => HbValue::Integer(b as i64),
            None    => HbValue::Integer(0),
        },
        _ => HbValue::Integer(0),
    }
}

// ---------------------------------------------------------------------------
// Len
// ---------------------------------------------------------------------------

/// `LEN(cStr | aArray)` — length of string (bytes) or number of array elements.
pub fn hb_len(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => HbValue::Integer(s.len() as i64),
        HbValue::Array(a)  => HbValue::Integer(a.len() as i64),
        _                  => HbValue::Integer(0),
    }
}

// ---------------------------------------------------------------------------
// Val
// ---------------------------------------------------------------------------

/// `VAL(cStr)` — converts a numeric string to its numeric value.
///
/// Tries integer first, then float; returns `0` for non-numeric input.
pub fn hb_val(s: HbValue) -> HbValue {
    match s {
        HbValue::String(t) => {
            let t = t.trim();
            if let Ok(i) = t.parse::<i64>() {
                HbValue::Integer(i)
            } else if let Ok(f) = t.parse::<f64>() {
                HbValue::Float(f)
            } else {
                HbValue::Integer(0)
            }
        }
        HbValue::Integer(n) => HbValue::Integer(n),
        HbValue::Float(f)   => HbValue::Float(f),
        _                   => HbValue::Integer(0),
    }
}

// ---------------------------------------------------------------------------
// Space( nSize ) — string of n blank spaces
// ---------------------------------------------------------------------------

/// `SPACE(nSize)` — returns a string of `nSize` blank spaces.
pub fn hb_space(n: HbValue) -> HbValue {
    match n {
        HbValue::Integer(n) if n >= 0 => HbValue::String(" ".repeat(n as usize)),
        _ => HbValue::String(String::new()),
    }
}

// ---------------------------------------------------------------------------
// Replicate( cStr, nN ) — repeat string n times
// ---------------------------------------------------------------------------

/// `REPLICATE(cStr, nN)` — repeats `cStr` exactly `nN` times.
pub fn hb_replicate(s: HbValue, n: HbValue) -> HbValue {
    match (s, n) {
        (HbValue::String(s), HbValue::Integer(n)) if n >= 0 => {
            HbValue::String(s.repeat(n as usize))
        }
        _ => HbValue::String(String::new()),
    }
}

// ---------------------------------------------------------------------------
// NativeFunction implementations
// ---------------------------------------------------------------------------

/// Registry struct for `SPACE`.
pub struct Space;
impl NativeFunction<HbValue> for Space {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_space(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "SPACE" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `REPLICATE`.
pub struct Replicate;
impl NativeFunction<HbValue> for Replicate {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        let s = it.next().unwrap_or(HbValue::Nil);
        let n = it.next().unwrap_or(HbValue::Nil);
        hb_replicate(s, n)
    }
    fn name(&self) -> &'static str { "REPLICATE" }
    fn arity(&self) -> (usize, usize) { (2, 2) }
}

/// Registry struct for `ALLTRIM`.
pub struct AllTrim;
impl NativeFunction<HbValue> for AllTrim {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_alltrim(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "ALLTRIM" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `LTRIM`.
pub struct LTrim;
impl NativeFunction<HbValue> for LTrim {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_ltrim(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "LTRIM" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `RTRIM`.
pub struct RTrim;
impl NativeFunction<HbValue> for RTrim {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_rtrim(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "RTRIM" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `TRIM` (alias for `RTRIM`).
pub struct Trim;
impl NativeFunction<HbValue> for Trim {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_rtrim(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "TRIM" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `UPPER`.
pub struct Upper;
impl NativeFunction<HbValue> for Upper {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_upper(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "UPPER" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `LOWER`.
pub struct Lower;
impl NativeFunction<HbValue> for Lower {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_lower(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "LOWER" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `LEFT`.
pub struct Left;
impl NativeFunction<HbValue> for Left {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        let s = it.next().unwrap_or(HbValue::Nil);
        let n = it.next().unwrap_or(HbValue::Nil);
        hb_left(s, n)
    }
    fn name(&self) -> &'static str { "LEFT" }
    fn arity(&self) -> (usize, usize) { (2, 2) }
}

/// Registry struct for `RIGHT`.
pub struct Right;
impl NativeFunction<HbValue> for Right {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        let s = it.next().unwrap_or(HbValue::Nil);
        let n = it.next().unwrap_or(HbValue::Nil);
        hb_right(s, n)
    }
    fn name(&self) -> &'static str { "RIGHT" }
    fn arity(&self) -> (usize, usize) { (2, 2) }
}

/// Registry struct for `SUBSTR`.
pub struct SubStr;
impl NativeFunction<HbValue> for SubStr {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        let s     = it.next().unwrap_or(HbValue::Nil);
        let start = it.next().unwrap_or(HbValue::Nil);
        let len   = it.next().unwrap_or(HbValue::Nil);
        hb_substr(s, start, len)
    }
    fn name(&self) -> &'static str { "SUBSTR" }
    fn arity(&self) -> (usize, usize) { (2, 3) }
}

/// Registry struct for `AT`.
pub struct At;
impl NativeFunction<HbValue> for At {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        let needle   = it.next().unwrap_or(HbValue::Nil);
        let haystack = it.next().unwrap_or(HbValue::Nil);
        hb_at(needle, haystack)
    }
    fn name(&self) -> &'static str { "AT" }
    fn arity(&self) -> (usize, usize) { (2, 2) }
}

/// Registry struct for `ASC`.
pub struct Asc;
impl NativeFunction<HbValue> for Asc {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_asc(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "ASC" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `LEN`.
pub struct Len;
impl NativeFunction<HbValue> for Len {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_len(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "LEN" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `VAL`.
pub struct Val;
impl NativeFunction<HbValue> for Val {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_val(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "VAL" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

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

    // ── AllTrim / LTrim / RTrim / Trim ───────────────────────────────────────

    #[test]
    fn alltrim_both_sides() {
        assert_eq!(hb_alltrim(s("  hi  ")), s("hi"));
    }

    #[test]
    fn ltrim_only_leading() {
        assert_eq!(hb_ltrim(s("  hi  ")), s("hi  "));
    }

    #[test]
    fn rtrim_only_trailing() {
        assert_eq!(hb_rtrim(s("  hi  ")), s("  hi"));
    }

    #[test]
    fn trim_is_rtrim() {
        assert_eq!(Trim.call(vec![s("  hi  ")]), s("  hi"));
        assert_eq!(Trim.name(), "TRIM");
    }

    #[test]
    fn trim_nil_passthrough() {
        assert_eq!(hb_alltrim(sp()), sp());
    }

    // ── Upper / Lower ─────────────────────────────────────────────────────────

    #[test]
    fn upper_basic() {
        assert_eq!(hb_upper(s("hello")), s("HELLO"));
    }

    #[test]
    fn lower_basic() {
        assert_eq!(hb_lower(s("WORLD")), s("world"));
    }

    #[test]
    fn upper_nil_passthrough() {
        assert_eq!(hb_upper(sp()), sp());
    }

    // ── Left / Right ──────────────────────────────────────────────────────────

    #[test]
    fn left_basic() {
        assert_eq!(hb_left(s("Harbour"), n(3)), s("Har"));
    }

    #[test]
    fn left_longer_than_string() {
        assert_eq!(hb_left(s("Hi"), n(10)), s("Hi"));
    }

    #[test]
    fn left_zero() {
        assert_eq!(hb_left(s("Hi"), n(0)), s(""));
    }

    #[test]
    fn right_basic() {
        assert_eq!(hb_right(s("Harbour"), n(4)), s("bour"));
    }

    #[test]
    fn right_longer_than_string() {
        assert_eq!(hb_right(s("Hi"), n(10)), s("Hi"));
    }

    #[test]
    fn right_zero() {
        assert_eq!(hb_right(s("Hi"), n(0)), s(""));
    }

    #[test]
    fn native_fn_left() {
        assert_eq!(Left.call(vec![s("abc"), n(2)]), s("ab"));
        assert_eq!(Left.name(), "LEFT");
        assert_eq!(Left.arity(), (2, 2));
    }

    #[test]
    fn native_fn_right() {
        assert_eq!(Right.call(vec![s("abc"), n(2)]), s("bc"));
        assert_eq!(Right.name(), "RIGHT");
    }

    // ── SubStr ────────────────────────────────────────────────────────────────

    #[test]
    fn substr_with_len() {
        assert_eq!(hb_substr(s("Harbour"), n(2), n(3)), s("arb"));
    }

    #[test]
    fn substr_to_end() {
        assert_eq!(hb_substr(s("Harbour"), n(4), sp()), s("bour"));
    }

    #[test]
    fn substr_start_beyond_end() {
        assert_eq!(hb_substr(s("Hi"), n(10), sp()), s(""));
    }

    #[test]
    fn native_fn_substr() {
        assert_eq!(SubStr.call(vec![s("Hello"), n(2), n(3)]), s("ell"));
        assert_eq!(SubStr.name(), "SUBSTR");
        assert_eq!(SubStr.arity(), (2, 3));
    }

    // ── At / Asc ──────────────────────────────────────────────────────────────

    #[test]
    fn at_found() {
        assert_eq!(hb_at(s("bo"), s("Harbour")), n(4));
    }

    #[test]
    fn at_not_found() {
        assert_eq!(hb_at(s("xyz"), s("Harbour")), n(0));
    }

    #[test]
    fn at_empty_needle() {
        // empty needle always found at position 1
        assert_eq!(hb_at(s(""), s("abc")), n(1));
    }

    #[test]
    fn asc_basic() {
        assert_eq!(hb_asc(s("A")), n(65));
        assert_eq!(hb_asc(s("ABC")), n(65)); // first char only
    }

    #[test]
    fn asc_empty_string() {
        assert_eq!(hb_asc(s("")), n(0));
    }

    #[test]
    fn native_fn_at() {
        assert_eq!(At.call(vec![s("ar"), s("Harbour")]), n(2));
        assert_eq!(At.name(), "AT");
    }

    #[test]
    fn native_fn_asc() {
        assert_eq!(Asc.call(vec![s("Z")]), n(90));
        assert_eq!(Asc.name(), "ASC");
    }

    // ── Len ───────────────────────────────────────────────────────────────────

    #[test]
    fn len_string() {
        assert_eq!(hb_len(s("hello")), n(5));
    }

    #[test]
    fn len_empty_string() {
        assert_eq!(hb_len(s("")), n(0));
    }

    #[test]
    fn len_array() {
        use swed_rt::HbArray;
        let mut a = HbArray::new();
        a.hb_aadd(HbValue::Integer(1));
        a.hb_aadd(HbValue::Integer(2));
        assert_eq!(hb_len(HbValue::Array(a)), n(2));
    }

    #[test]
    fn len_nil_returns_zero() {
        assert_eq!(hb_len(sp()), n(0));
    }

    #[test]
    fn native_fn_len() {
        assert_eq!(Len.call(vec![s("abcde")]), n(5));
        assert_eq!(Len.name(), "LEN");
        assert_eq!(Len.arity(), (1, 1));
    }

    // ── Val ───────────────────────────────────────────────────────────────────

    #[test]
    fn val_integer_string() {
        assert_eq!(hb_val(s("42")), n(42));
    }

    #[test]
    fn val_float_string() {
        assert_eq!(hb_val(s("3.14")), HbValue::Float(3.14));
    }

    #[test]
    fn val_non_numeric() {
        assert_eq!(hb_val(s("abc")), n(0));
    }

    #[test]
    fn val_with_spaces() {
        assert_eq!(hb_val(s("  99  ")), n(99));
    }

    #[test]
    fn val_integer_passthrough() {
        assert_eq!(hb_val(n(7)), n(7));
    }

    #[test]
    fn native_fn_val() {
        assert_eq!(Val.call(vec![s("123")]), n(123));
        assert_eq!(Val.name(), "VAL");
        assert_eq!(Val.arity(), (1, 1));
    }

    // ── Space ────────────────────────────────────────────────────────────────

    #[test]
    fn space_basic() {
        assert_eq!(hb_space(n(5)), s("     "));
    }

    #[test]
    fn space_zero() {
        assert_eq!(hb_space(n(0)), s(""));
    }

    #[test]
    fn space_nil_returns_empty() {
        assert_eq!(hb_space(sp()), s(""));
    }

    #[test]
    fn native_fn_space() {
        assert_eq!(Space.call(vec![n(3)]), s("   "));
        assert_eq!(Space.name(), "SPACE");
        assert_eq!(Space.arity(), (1, 1));
    }

    // ── Replicate ────────────────────────────────────────────────────────────

    #[test]
    fn replicate_basic() {
        assert_eq!(hb_replicate(s("ab"), n(3)), s("ababab"));
    }

    #[test]
    fn replicate_zero_times() {
        assert_eq!(hb_replicate(s("x"), n(0)), s(""));
    }

    #[test]
    fn replicate_nil_n_returns_empty() {
        assert_eq!(hb_replicate(s("x"), sp()), s(""));
    }

    #[test]
    fn replicate_nil_str_returns_empty() {
        assert_eq!(hb_replicate(sp(), n(3)), s(""));
    }

    #[test]
    fn native_fn_replicate() {
        assert_eq!(Replicate.call(vec![s("hi"), n(2)]), s("hihi"));
        assert_eq!(Replicate.name(), "REPLICATE");
        assert_eq!(Replicate.arity(), (2, 2));
    }
}
