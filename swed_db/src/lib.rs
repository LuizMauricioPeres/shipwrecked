//! `swed_db` — Replaceable Database Driver (RDD) layer for SWed.
//!
//! Encapsulates all DBF/xBase file access. Depends on `swed_co` and `swed_rt`.
//! Does **not** depend on compiler crates — safe to ship in production runtimes.
//!
//! # Planned modules
//!
//! - `work_area` — DBF cursor: open / close / navigate / field access
//! - `dbf_handler` — low-level DBF I/O (migrated from `swed_rt`)
//! - `row` — single DBF record buffer (migrated from `swed_rt`)
//! - `registry` — global work-area table (SELECT / ALIAS, up to 255 areas)
//! - `traits::rdd` — `Rdd` trait for swappable drivers
