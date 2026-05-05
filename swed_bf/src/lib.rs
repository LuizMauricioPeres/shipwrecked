//! `swed_bf` — Harbour Built-in Functions backed by `HbValue`.
//!
//! Provides 56 Harbour standard functions organised in five modules:
//! [`array`], [`date`], [`math`], [`misc`], [`numeric`], [`string`].
//!
//! Each function exists as both a plain Rust free function (for generated code)
//! and a [`swed_co::NativeFunction`] implementor (for the interpreter registry).
//! [`all_functions`] returns the full registry as a flat list.
//!
//! All functions return [`swed_rt::HbValue`] and propagate `Nil` on type
//! mismatch — no panics, no `Result`.
//!
//! # Dependency
//!
//! Depends on `swed_co` (trait) and `swed_rt` (types). Does **not** depend on
//! any compiler crate (`swed`, `swed_mkh`), making it safe in production runtimes.

pub mod array;
pub mod date;
pub mod math;
pub mod misc;
pub mod numeric;
pub mod string;

// ── Free functions ────────────────────────────────────────────────────────────

pub use array::{
    hb_aadd, hb_adel, hb_afill, hb_ains, hb_aclone, hb_array_new, hb_ascan, hb_asize, hb_atail,
};
pub use date::{hb_date, hb_day, hb_dtos, hb_month, hb_stod, hb_year};
pub use math::{hb_abs, hb_exp, hb_int, hb_log, hb_max, hb_min, hb_mod, hb_round, hb_sqrt};
pub use misc::{hb_empty, hb_pcount, hb_type};
pub use numeric::{hb_chr, hb_ntos, hb_str, hb_strzero};
pub use string::{
    hb_alltrim, hb_asc, hb_at, hb_left, hb_len, hb_lower, hb_ltrim,
    hb_padc, hb_padl, hb_padr, hb_replicate, hb_right, hb_rtrim,
    hb_space, hb_substr, hb_upper, hb_val,
};

// ── NativeFunction registry structs ──────────────────────────────────────────

pub use array::{AAdd, ADel, AFill, AIns, AClone, Array, AScan, ASize, ATail};
pub use date::{Date, Day, DToS, Month, SToD, Year};
pub use math::{Abs, Exp, Int, Log, Max, Min, Mod, Round, Sqrt};
pub use misc::{Empty, PCount, Type, ValType};
pub use numeric::{Chr, NToS, Str, StrZero};
pub use string::{
    AllTrim, Asc, At, Left, Len, Lower, LTrim,
    PadC, PadL, PadR, Replicate, Right, RTrim, Space, SubStr, Trim, Upper, Val,
};

// ── Registry builder ─────────────────────────────────────────────────────────

use swed_co::NativeFunction;
use swed_rt::HbValue;

/// Returns all functions implemented in this crate as a flat list of
/// `(name, Box<dyn NativeFunction<HbValue>>)` pairs.
///
/// Intended for use with a `FunctionResolver` implementation.
pub fn all_functions() -> Vec<(&'static str, Box<dyn NativeFunction<HbValue>>)> {
    vec![
        // Array
        ("ARRAY",    Box::new(Array)),
        ("AADD",     Box::new(AAdd)),
        ("ASIZE",    Box::new(ASize)),
        ("AFILL",    Box::new(AFill)),
        ("ASCAN",    Box::new(AScan)),
        ("ATAIL",    Box::new(ATail)),
        ("ADEL",     Box::new(ADel)),
        ("AINS",     Box::new(AIns)),
        ("ACLONE",   Box::new(AClone)),
        // DateTime
        ("DATE",     Box::new(Date)),
        ("YEAR",     Box::new(Year)),
        ("MONTH",    Box::new(Month)),
        ("DAY",      Box::new(Day)),
        ("DTOS",     Box::new(DToS)),
        ("STOD",     Box::new(SToD)),
        // Core
        ("TYPE",     Box::new(Type)),
        ("VALTYPE",  Box::new(ValType)),
        ("EMPTY",    Box::new(Empty)),
        ("PCOUNT",   Box::new(PCount)),
        // String — search / slice
        ("ALLTRIM",  Box::new(AllTrim)),
        ("LTRIM",    Box::new(LTrim)),
        ("RTRIM",    Box::new(RTrim)),
        ("TRIM",     Box::new(Trim)),
        ("UPPER",    Box::new(Upper)),
        ("LOWER",    Box::new(Lower)),
        ("LEFT",     Box::new(Left)),
        ("RIGHT",    Box::new(Right)),
        ("SUBSTR",   Box::new(SubStr)),
        ("AT",       Box::new(At)),
        ("ASC",      Box::new(Asc)),
        ("LEN",      Box::new(Len)),
        ("VAL",      Box::new(Val)),
        // String — padding / repeat
        ("PADL",     Box::new(PadL)),
        ("PADR",     Box::new(PadR)),
        ("PADC",     Box::new(PadC)),
        ("SPACE",    Box::new(Space)),
        ("REPLICATE",Box::new(Replicate)),
        // Math
        ("ABS",      Box::new(Abs)),
        ("INT",      Box::new(Int)),
        ("ROUND",    Box::new(Round)),
        ("MAX",      Box::new(Max)),
        ("MIN",      Box::new(Min)),
        ("MOD",      Box::new(Mod)),
        ("SQRT",     Box::new(Sqrt)),
        ("EXP",      Box::new(Exp)),
        ("LOG",      Box::new(Log)),
        // Numeric → string
        ("STR",      Box::new(Str)),
        ("STRZERO",  Box::new(StrZero)),
        ("HB_NTOS",  Box::new(NToS)),
        ("CHR",      Box::new(Chr)),
    ]
}
