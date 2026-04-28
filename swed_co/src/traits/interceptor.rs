//! `ErrorInterceptor` — observer for Critical-severity errors.

use crate::error::SwedError;

/// Observes `Critical`-severity errors before they propagate.
///
/// Implemented by `swed_kn` to emit hex dumps and patch suggestions.
///
/// `V` is the runtime value type (concretely `HbValue` in `swed_rt`).
pub trait ErrorInterceptor<V> {
    /// Called when a `SeverityLevel::Critical` error is raised.
    ///
    /// `val` is the `HbValue` involved in the failing operation.
    /// The interceptor must **not** panic — it is called from error-handling paths.
    fn on_critical(&self, err: &SwedError, val: &V);
}
