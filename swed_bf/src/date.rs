//! Harbour date functions: Date(), Year(), Month(), Day(), DToS(), SToD().

use std::time::{SystemTime, UNIX_EPOCH};
use swed_co::NativeFunction;
use swed_rt::HbValue;

// ---------------------------------------------------------------------------
// Internal: days-since-epoch ↔ (year, month, day)  [Hinnant algorithm]
// ---------------------------------------------------------------------------

/// Decomposes days-since-1970-01-01 into `(year, month, day)`.
fn civil_from_days(days: i32) -> (i32, u32, u32) {
    let z   = days as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y   = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d   = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m   = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y   = if m <= 2 { y + 1 } else { y } as i32;
    (y, m, d)
}

/// Parses `"YYYYMMDD"` into days-since-1970-01-01. Returns `None` on bad input.
fn days_from_str(s: &str) -> Option<i32> {
    if s.len() != 8 { return None; }
    let y: i32 = s[0..4].parse().ok()?;
    let m: u32 = s[4..6].parse().ok()?;
    let d: u32 = s[6..8].parse().ok()?;
    if m < 1 || m > 12 || d < 1 || d > 31 { return None; }
    // Days since epoch using the inverse of Hinnant's algorithm
    let y = if m <= 2 { y - 1 } else { y } as i64;
    let m = if m <= 2 { m + 9 } else { m - 3 } as i64;
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * m + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some((era * 146_097 + doe - 719_468) as i32)
}

// ---------------------------------------------------------------------------
// Date()
// ---------------------------------------------------------------------------

/// `DATE()` — today's date as `HbValue::Date(days_since_1970)`.
pub fn hb_date() -> HbValue {
    let days = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 86_400;
    HbValue::Date(days as i32)
}

// ---------------------------------------------------------------------------
// Year() / Month() / Day()
// ---------------------------------------------------------------------------

/// `YEAR(dDate)` — four-digit year as `HbValue::Integer`.
pub fn hb_year(val: HbValue) -> HbValue {
    match val {
        HbValue::Date(d) => HbValue::Integer(civil_from_days(d).0 as i64),
        _ => HbValue::Integer(0),
    }
}

/// `MONTH(dDate)` — month 1-12 as `HbValue::Integer`.
pub fn hb_month(val: HbValue) -> HbValue {
    match val {
        HbValue::Date(d) => HbValue::Integer(civil_from_days(d).1 as i64),
        _ => HbValue::Integer(0),
    }
}

/// `DAY(dDate)` — day 1-31 as `HbValue::Integer`.
pub fn hb_day(val: HbValue) -> HbValue {
    match val {
        HbValue::Date(d) => HbValue::Integer(civil_from_days(d).2 as i64),
        _ => HbValue::Integer(0),
    }
}

// ---------------------------------------------------------------------------
// DToS() / SToD()
// ---------------------------------------------------------------------------

/// `DTOS(dDate)` — date to `"YYYYMMDD"` string (sortable format).
pub fn hb_dtos(val: HbValue) -> HbValue {
    match val {
        HbValue::Date(d) => {
            let (y, m, day) = civil_from_days(d);
            HbValue::String(format!("{y:04}{m:02}{day:02}"))
        }
        _ => HbValue::String(String::new()),
    }
}

/// `STOD(cStr)` — `"YYYYMMDD"` string to `HbValue::Date`. Returns `Nil` on bad input.
pub fn hb_stod(val: HbValue) -> HbValue {
    match val {
        HbValue::String(s) => match days_from_str(&s) {
            Some(d) => HbValue::Date(d),
            None    => HbValue::Nil,
        },
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// NativeFunction implementations
// ---------------------------------------------------------------------------

/// Registry struct for `DATE`.
pub struct Date;
impl NativeFunction<HbValue> for Date {
    fn call(&self, _args: Vec<HbValue>) -> HbValue { hb_date() }
    fn name(&self) -> &'static str { "DATE" }
    fn arity(&self) -> (usize, usize) { (0, 0) }
}

/// Registry struct for `YEAR`.
pub struct Year;
impl NativeFunction<HbValue> for Year {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_year(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "YEAR" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `MONTH`.
pub struct Month;
impl NativeFunction<HbValue> for Month {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_month(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "MONTH" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `DAY`.
pub struct Day;
impl NativeFunction<HbValue> for Day {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_day(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "DAY" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `DTOS`.
pub struct DToS;
impl NativeFunction<HbValue> for DToS {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_dtos(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "DTOS" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `STOD`.
pub struct SToD;
impl NativeFunction<HbValue> for SToD {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_stod(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "STOD" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // 2024-03-24 = 19 806 days since 1970-01-01
    // python: (date(2024,3,24) - date(1970,1,1)).days == 19806
    const TEST_DATE: i32 = 19_806;

    #[test]
    fn civil_decomposition() {
        let (y, m, d) = civil_from_days(TEST_DATE);
        assert_eq!((y, m, d), (2024, 3, 24));
    }

    #[test]
    fn civil_epoch_origin() {
        let (y, m, d) = civil_from_days(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn hb_year_month_day() {
        let date = HbValue::Date(TEST_DATE);
        assert_eq!(hb_year(date.clone()),  HbValue::Integer(2024));
        assert_eq!(hb_month(date.clone()), HbValue::Integer(3));
        assert_eq!(hb_day(date),           HbValue::Integer(24));
    }

    #[test]
    fn hb_dtos_roundtrip() {
        let date = HbValue::Date(TEST_DATE);
        let s = hb_dtos(date);
        assert_eq!(s, HbValue::String("20240324".into()));
        let back = hb_stod(s);
        assert_eq!(back, HbValue::Date(TEST_DATE));
    }

    #[test]
    fn hb_stod_bad_input() {
        assert_eq!(hb_stod(HbValue::String("not-a-date".into())), HbValue::Nil);
        assert_eq!(hb_stod(HbValue::Nil), HbValue::Nil);
    }

    #[test]
    fn hb_date_returns_date_variant() {
        // We can't assert the exact day (it depends on when the test runs),
        // but we can assert the variant and that it's a plausible value.
        match hb_date() {
            HbValue::Date(d) => assert!(d > 19_000, "date suspiciously small: {d}"),
            other => panic!("expected Date, got {other:?}"),
        }
    }

    #[test]
    fn native_fn_trait_date() {
        assert_eq!(Date.name(), "DATE");
        assert_eq!(Date.arity(), (0, 0));
        assert!(matches!(Date.call(vec![]), HbValue::Date(_)));
    }

    #[test]
    fn native_fn_trait_year() {
        let r = Year.call(vec![HbValue::Date(TEST_DATE)]);
        assert_eq!(r, HbValue::Integer(2024));
    }
}
