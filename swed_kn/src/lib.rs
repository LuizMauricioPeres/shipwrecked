//! `swed_kn` — Knife Tools: diagnostic and debugging utilities for SWed.
//!
//! Implements `ErrorInterceptor` from `swed_co`. Observes `Critical`-severity
//! errors without modifying `swed_rt` or generated code.
//! Depends on `swed_co` and `swed_rt`. Optional — not required in production.
//!
//! # Planned modules
//!
//! - `interceptor` — `ErrorInterceptor<HbValue>` implementation
//! - `hex_dump`    — `HbValue` → formatted hex dump (16 bytes/row, ASCII sidebar)
//! - `patch`       — type-coercion patch suggestion engine
//! - `logger`      — append-only JSON logger to `swed_kn.log`
//! - `traits::inspector` — `Inspector` trait for custom diagnostic backends
