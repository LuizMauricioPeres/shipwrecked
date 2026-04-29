//! `swed_bf` — Harbour Basic Functions backed by `HbValue`.
//!
//! Each function is available both as a free function (for direct use in
//! generated code) and as a `NativeFunction<HbValue>` implementor (for the
//! `FunctionResolver` registry).
//!
//! # Dependency
//!
//! This crate depends on `swed_co` (for the `NativeFunction` trait) and
//! `swed_rt` (for `HbValue`). It does **not** depend on the compiler crates
//! (`swed`, `swed_mkh`), making it safe to ship in production runtimes.

pub mod date;
pub mod misc;
pub mod numeric;
pub mod string;

// ── Free functions ────────────────────────────────────────────────────────────

pub use date::{hb_date, hb_day, hb_dtos, hb_month, hb_stod, hb_year};
pub use misc::hb_type;
pub use numeric::{hb_chr, hb_ntos, hb_str, hb_strzero};
pub use string::{hb_padc, hb_padl, hb_padr};

// ── NativeFunction registry structs ──────────────────────────────────────────

pub use date::{Date, Day, DToS, Month, SToD, Year};
pub use misc::Type;
pub use numeric::{Chr, NToS, Str, StrZero};
pub use string::{PadC, PadL, PadR};

// ── Registry builder ─────────────────────────────────────────────────────────

use swed_co::NativeFunction;
use swed_rt::HbValue;

/// Returns all functions implemented in this crate as a flat list of
/// `(name, Box<dyn NativeFunction<HbValue>>)` pairs.
///
/// Intended for use with a `FunctionResolver` implementation.
pub fn all_functions() -> Vec<(&'static str, Box<dyn NativeFunction<HbValue>>)> {
    vec![
        ("DATE",     Box::new(Date)),
        ("YEAR",     Box::new(Year)),
        ("MONTH",    Box::new(Month)),
        ("DAY",      Box::new(Day)),
        ("DTOS",     Box::new(DToS)),
        ("STOD",     Box::new(SToD)),
        ("TYPE",     Box::new(Type)),
        ("PADL",     Box::new(PadL)),
        ("PADR",     Box::new(PadR)),
        ("PADC",     Box::new(PadC)),
        ("STR",      Box::new(Str)),
        ("STRZERO",  Box::new(StrZero)),
        ("HB_NTOS",  Box::new(NToS)),
        ("CHR",      Box::new(Chr)),
    ]
}
