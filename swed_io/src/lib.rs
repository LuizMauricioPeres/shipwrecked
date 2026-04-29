//! `swed_io` — File I/O and encoding utilities for SWed.
//!
//! Depends on `swed_co` (traits), `swed_rt` (HbValue), and `encoding_rs`.
//! Does **not** depend on compiler crates — safe to ship in production runtimes.

pub mod directory;
pub mod file_io;

// ── Free functions ────────────────────────────────────────────────────────────

pub use directory::hb_directory;
pub use file_io::{
    hb_fcreate, hb_fclose, hb_ferase, hb_ferror, hb_fopen, hb_frename, hb_fwrite,
};

// ── NativeFunction registry structs ──────────────────────────────────────────

pub use directory::Directory;
pub use file_io::{FClose, FCreate, FErase, FError, FOpen, FRename, FWrite};

// ── Registry builder ─────────────────────────────────────────────────────────

use swed_co::NativeFunction;
use swed_rt::HbValue;

/// Returns all I/O functions as `(name, Box<dyn NativeFunction<HbValue>>)` pairs.
pub fn all_functions() -> Vec<(&'static str, Box<dyn NativeFunction<HbValue>>)> {
    vec![
        ("DIRECTORY", Box::new(Directory)),
        ("FCREATE",   Box::new(FCreate)),
        ("FOPEN",     Box::new(FOpen)),
        ("FCLOSE",    Box::new(FClose)),
        ("FWRITE",    Box::new(FWrite)),
        ("FERASE",    Box::new(FErase)),
        ("FRENAME",   Box::new(FRename)),
        ("FERROR",    Box::new(FError)),
    ]
}
