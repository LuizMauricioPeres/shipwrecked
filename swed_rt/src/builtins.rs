// swed_rt/src/builtins.rs
// Harbour built-in functions used by generated Rust code.
//
// Naming convention: all public fns are prefixed with `hb_` so they read
// naturally in generated code and never shadow Rust std items.

use crate::value::HbValue;
use crate::array::HbArray;

// ---------------------------------------------------------------------------
// String functions
// ---------------------------------------------------------------------------

/// `LEN(cStr)` / `LEN(aArr)` — length of string or array.
pub fn hb_len(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => HbValue::Integer(s.len() as i64),
        HbValue::Array(a)  => HbValue::Integer(a.len() as i64),
        _ => HbValue::Integer(0),
    }
}

/// `ALLTRIM(cStr)` — trim leading and trailing spaces.
pub fn hb_alltrim(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => HbValue::String(s.trim().to_owned()),
        other => other,
    }
}

/// `LTRIM(cStr)` — trim leading spaces.
pub fn hb_ltrim(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => HbValue::String(s.trim_start().to_owned()),
        other => other,
    }
}

/// `RTRIM(cStr)` / `TRIM(cStr)` — trim trailing spaces.
pub fn hb_rtrim(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => HbValue::String(s.trim_end().to_owned()),
        other => other,
    }
}

/// `UPPER(cStr)` — convert to uppercase.
pub fn hb_upper(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => HbValue::String(s.to_uppercase()),
        other => other,
    }
}

/// `LOWER(cStr)` — convert to lowercase.
pub fn hb_lower(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => HbValue::String(s.to_lowercase()),
        other => other,
    }
}

/// `SUBSTR(cStr, nStart [, nLen])` — extract substring (1-based).
pub fn hb_substr(s: HbValue, start: HbValue, len: HbValue) -> HbValue {
    let text = match s {
        HbValue::String(t) => t,
        _ => return HbValue::Nil,
    };
    let start = match start {
        HbValue::Integer(n) if n >= 1 => (n - 1) as usize,
        _ => return HbValue::Nil,
    };
    if start >= text.len() {
        return HbValue::String(String::new());
    }
    let slice = &text[start..];
    let result = match len {
        HbValue::Integer(n) if n > 0 => {
            let n = n as usize;
            &slice[..n.min(slice.len())]
        }
        HbValue::Nil | HbValue::Integer(_) => slice,
        _ => slice,
    };
    HbValue::String(result.to_owned())
}

/// `AT(cNeedle, cHaystack)` — 1-based position or 0 if not found.
pub fn hb_at(needle: HbValue, haystack: HbValue) -> HbValue {
    match (needle, haystack) {
        (HbValue::String(n), HbValue::String(h)) => {
            match h.find(&*n) {
                Some(pos) => HbValue::Integer((pos + 1) as i64),
                None      => HbValue::Integer(0),
            }
        }
        _ => HbValue::Integer(0),
    }
}

/// `RAT(cNeedle, cHaystack)` — last occurrence, 1-based or 0.
pub fn hb_rat(needle: HbValue, haystack: HbValue) -> HbValue {
    match (needle, haystack) {
        (HbValue::String(n), HbValue::String(h)) => {
            match h.rfind(&*n) {
                Some(pos) => HbValue::Integer((pos + 1) as i64),
                None      => HbValue::Integer(0),
            }
        }
        _ => HbValue::Integer(0),
    }
}

/// `STR(nNum [, nLen [, nDec]])` — numeric to string.
pub fn hb_str(num: HbValue, len: HbValue, dec: HbValue) -> HbValue {
    let n = match num {
        HbValue::Integer(i) => i as f64,
        HbValue::Float(f)   => f,
        _ => return HbValue::String(" ".repeat(10)),
    };
    let decimals = match dec {
        HbValue::Integer(d) if d >= 0 => d as usize,
        _ => 0,
    };
    let s = if decimals > 0 {
        format!("{n:.decimals$}")
    } else {
        format!("{:.0}", n)
    };
    let total_len = match len {
        HbValue::Integer(l) if l > 0 => l as usize,
        _ => s.len(),
    };
    if s.len() < total_len {
        HbValue::String(format!("{:>total_len$}", s))
    } else {
        HbValue::String(s)
    }
}

/// `VAL(cStr)` — string to numeric.
pub fn hb_val(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => {
            let trimmed = s.trim();
            if let Ok(i) = trimmed.parse::<i64>() {
                HbValue::Integer(i)
            } else if let Ok(f) = trimmed.parse::<f64>() {
                HbValue::Float(f)
            } else {
                HbValue::Integer(0)
            }
        }
        HbValue::Integer(n) => HbValue::Integer(n),
        HbValue::Float(f)   => HbValue::Float(f),
        _ => HbValue::Integer(0),
    }
}

/// `CHR(nAscii)` — ASCII code to single-char string.
pub fn hb_chr(val: HbValue) -> HbValue {
    match val {
        HbValue::Integer(n) if n >= 0 && n <= 255 => {
            HbValue::String((n as u8 as char).to_string())
        }
        _ => HbValue::String(String::new()),
    }
}

/// `ASC(cChar)` — first char to ASCII code.
pub fn hb_asc(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => match s.bytes().next() {
            Some(b) => HbValue::Integer(b as i64),
            None    => HbValue::Integer(0),
        },
        _ => HbValue::Integer(0),
    }
}

/// `SPACE(n)` — string of n spaces.
pub fn hb_space(val: HbValue) -> HbValue {
    match val {
        HbValue::Integer(n) if n >= 0 => HbValue::String(" ".repeat(n as usize)),
        _ => HbValue::String(String::new()),
    }
}

/// `REPLICATE(cStr, n)` — repeat string n times.
pub fn hb_replicate(s: HbValue, n: HbValue) -> HbValue {
    match (s, n) {
        (HbValue::String(s), HbValue::Integer(n)) if n >= 0 => {
            HbValue::String(s.repeat(n as usize))
        }
        _ => HbValue::String(String::new()),
    }
}

/// `PADR(cStr, nLen [, cPad])` — pad right to length.
pub fn hb_padr(s: HbValue, len: HbValue, pad: HbValue) -> HbValue {
    let text = match s { HbValue::String(t) => t, _ => return HbValue::Nil };
    let n    = match len { HbValue::Integer(n) if n > 0 => n as usize, _ => return HbValue::Nil };
    let ch   = match pad { HbValue::String(p) => p.chars().next().unwrap_or(' '), _ => ' ' };
    if text.len() >= n {
        HbValue::String(text[..n].to_owned())
    } else {
        let extra = n - text.len();
        HbValue::String(text + &ch.to_string().repeat(extra))
    }
}

/// `PADL(cStr, nLen [, cPad])` — pad left to length.
pub fn hb_padl(s: HbValue, len: HbValue, pad: HbValue) -> HbValue {
    let text = match s { HbValue::String(t) => t, _ => return HbValue::Nil };
    let n    = match len { HbValue::Integer(n) if n > 0 => n as usize, _ => return HbValue::Nil };
    let ch   = match pad { HbValue::String(p) => p.chars().next().unwrap_or(' '), _ => ' ' };
    if text.len() >= n {
        HbValue::String(text[text.len() - n..].to_owned())
    } else {
        let extra = n - text.len();
        HbValue::String(ch.to_string().repeat(extra) + &text)
    }
}

// ---------------------------------------------------------------------------
// Numeric functions
// ---------------------------------------------------------------------------

/// `INT(nNum)` — truncate to integer part.
pub fn hb_int(val: HbValue) -> HbValue {
    match val {
        HbValue::Integer(n) => HbValue::Integer(n),
        HbValue::Float(f)   => HbValue::Integer(f.trunc() as i64),
        _ => HbValue::Integer(0),
    }
}

/// `ABS(nNum)` — absolute value.
pub fn hb_abs(val: HbValue) -> HbValue {
    match val {
        HbValue::Integer(n) => HbValue::Integer(n.abs()),
        HbValue::Float(f)   => HbValue::Float(f.abs()),
        _ => HbValue::Nil,
    }
}

/// `ROUND(nNum, nDec)` — round to n decimal places.
pub fn hb_round(val: HbValue, dec: HbValue) -> HbValue {
    let f = match val {
        HbValue::Integer(n) => n as f64,
        HbValue::Float(f)   => f,
        _ => return HbValue::Nil,
    };
    let d = match dec {
        HbValue::Integer(d) => d,
        _ => 0,
    };
    let factor = 10f64.powi(d as i32);
    let rounded = (f * factor).round() / factor;
    if d <= 0 {
        HbValue::Integer(rounded as i64)
    } else {
        HbValue::Float(rounded)
    }
}

/// `MAX(x, y)` — maximum of two values.
pub fn hb_max(a: HbValue, b: HbValue) -> HbValue {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Less) => b,
        _ => a,
    }
}

/// `MIN(x, y)` — minimum of two values.
pub fn hb_min(a: HbValue, b: HbValue) -> HbValue {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Greater) => b,
        _ => a,
    }
}

// ---------------------------------------------------------------------------
// Type-checking / conversion
// ---------------------------------------------------------------------------

/// `VALTYPE(x)` — returns "C", "N", "L", "A", "D", "U" etc.
pub fn hb_valtype(val: HbValue) -> HbValue {
    HbValue::String(val.val_type().to_owned())
}

/// `EMPTY(x)` — `.T.` if value is empty/zero/nil.
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

/// `ISNIL(x)` — `.T.` if value is NIL.
pub fn hb_isnil(val: HbValue) -> HbValue {
    HbValue::Logical(matches!(val, HbValue::Nil))
}

// ---------------------------------------------------------------------------
// FOR loop range iterator
// ---------------------------------------------------------------------------

/// Used by generated FOR loops:
///   `FOR i := start TO end STEP step`
///   → `for i in hb_range(start, end, step) { ... }`
///
/// Handles both positive and negative STEP correctly.
pub fn hb_range(start: HbValue, end: HbValue, step: HbValue) -> HbRangeIter {
    let s = to_f64(&start).unwrap_or(0.0);
    let e = to_f64(&end).unwrap_or(0.0);
    let st = to_f64(&step).unwrap_or(1.0);
    HbRangeIter { current: s, end: e, step: st }
}

fn to_f64(v: &HbValue) -> Option<f64> {
    match v {
        HbValue::Integer(n) => Some(*n as f64),
        HbValue::Float(f)   => Some(*f),
        _ => None,
    }
}

/// Iterator returned by `hb_range`.
pub struct HbRangeIter {
    current: f64,
    end: f64,
    step: f64,
}

impl Iterator for HbRangeIter {
    type Item = HbValue;
    fn next(&mut self) -> Option<HbValue> {
        if self.step > 0.0 && self.current > self.end { return None; }
        if self.step < 0.0 && self.current < self.end { return None; }
        if self.step == 0.0 { return None; }

        let val = if self.current.fract() == 0.0 {
            HbValue::Integer(self.current as i64)
        } else {
            HbValue::Float(self.current)
        };
        self.current += self.step;
        Some(val)
    }
}

// ---------------------------------------------------------------------------
// Macro substitution placeholder
// ---------------------------------------------------------------------------

/// `&varName` — macro substitution; in Harbour this evaluates the string
/// content of a variable as an expression.  In the transpiler we represent
/// it as a runtime call for now.
pub fn hb_macro(val: HbValue) -> HbValue {
    // Full macro evaluation requires an embedded parser — out of scope for v0.2.
    // For now, return the value as-is with a warning annotation.
    val
}

// ---------------------------------------------------------------------------
// Console I/O
// ---------------------------------------------------------------------------

/// `QOUT(x)` / `? x` — print value followed by newline.
pub fn hb_qout(val: HbValue) {
    println!("{val}");
}

/// `QQOUT(x)` / `?? x` — print value without newline.
pub fn hb_qqout(val: HbValue) {
    print!("{val}");
}

// ---------------------------------------------------------------------------
// Array convenience wrappers (free-function form matches Harbour call syntax)
// ---------------------------------------------------------------------------

/// `AAdd(aArr, xVal)` — free-function wrapper (also available as method).
pub fn hb_aadd(arr: &mut HbArray, val: HbValue) -> HbValue {
    arr.hb_aadd(val);
    HbValue::Array(arr.clone())
}

/// `ASize(aArr, nSize)`
pub fn hb_asize(arr: &mut HbArray, size: HbValue) {
    arr.hb_asize(size);
}

/// `AScan(aArr, xVal)` → 1-based index or 0
pub fn hb_ascan(arr: &HbArray, val: HbValue) -> HbValue {
    arr.hb_ascan(&val)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hb_len_string() {
        assert_eq!(hb_len(HbValue::String("hello".into())), HbValue::Integer(5));
    }

    #[test]
    fn test_hb_alltrim() {
        let r = hb_alltrim(HbValue::String("  hi  ".into()));
        assert_eq!(r, HbValue::String("hi".into()));
    }

    #[test]
    fn test_hb_substr() {
        let r = hb_substr(
            HbValue::String("Harbour".into()),
            HbValue::Integer(1),
            HbValue::Integer(3),
        );
        assert_eq!(r, HbValue::String("Har".into()));
    }

    #[test]
    fn test_hb_at() {
        // "Harbour" → H(1) a(2) r(3) b(4) o(5) u(6) r(7)
        // "bo" starts at position 4
        let r = hb_at(
            HbValue::String("bo".into()),
            HbValue::String("Harbour".into()),
        );
        assert_eq!(r, HbValue::Integer(4));
    }

    #[test]
    fn test_hb_str_int() {
        let r = hb_str(HbValue::Integer(42), HbValue::Nil, HbValue::Nil);
        assert_eq!(r, HbValue::String("42".into()));
    }

    #[test]
    fn test_hb_val() {
        assert_eq!(hb_val(HbValue::String("123".into())), HbValue::Integer(123));
        assert_eq!(hb_val(HbValue::String("3.14".into())), HbValue::Float(3.14));
        assert_eq!(hb_val(HbValue::String("abc".into())), HbValue::Integer(0));
    }

    #[test]
    fn test_hb_range_step1() {
        let v: Vec<HbValue> = hb_range(
            HbValue::Integer(1),
            HbValue::Integer(3),
            HbValue::Integer(1),
        ).collect();
        assert_eq!(v, vec![
            HbValue::Integer(1),
            HbValue::Integer(2),
            HbValue::Integer(3),
        ]);
    }

    #[test]
    fn test_hb_range_negative_step() {
        let v: Vec<HbValue> = hb_range(
            HbValue::Integer(3),
            HbValue::Integer(1),
            HbValue::Integer(-1),
        ).collect();
        assert_eq!(v, vec![
            HbValue::Integer(3),
            HbValue::Integer(2),
            HbValue::Integer(1),
        ]);
    }

    #[test]
    fn test_hb_round() {
        let r = hb_round(HbValue::Float(3.456), HbValue::Integer(2));
        assert_eq!(r, HbValue::Float(3.46));
    }

    #[test]
    fn test_hb_empty() {
        assert_eq!(hb_empty(HbValue::Nil),                  HbValue::Logical(true));
        assert_eq!(hb_empty(HbValue::String("".into())),    HbValue::Logical(true));
        assert_eq!(hb_empty(HbValue::String("x".into())),   HbValue::Logical(false));
        assert_eq!(hb_empty(HbValue::Integer(0)),            HbValue::Logical(true));
    }

    #[test]
    fn test_hb_max_min() {
        assert_eq!(hb_max(HbValue::Integer(3), HbValue::Integer(7)), HbValue::Integer(7));
        assert_eq!(hb_min(HbValue::Integer(3), HbValue::Integer(7)), HbValue::Integer(3));
    }

    #[test]
    fn test_hb_padr() {
        let r = hb_padr(
            HbValue::String("Hi".into()),
            HbValue::Integer(5),
            HbValue::String(" ".into()),
        );
        assert_eq!(r, HbValue::String("Hi   ".into()));
    }
}
