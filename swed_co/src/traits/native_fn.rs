//! `NativeFunction` — contract for Harbour built-in implementations.

/// A Harbour built-in function backed by a Rust implementation.
///
/// `V` is the runtime value type (concretely `HbValue` in `swed_rt`).
pub trait NativeFunction<V> {
    /// Execute the function with the given positional arguments.
    fn call(&self, args: Vec<V>) -> V;

    /// Canonical UPPERCASE Harbour name (e.g. `"ALLTRIM"`).
    fn name(&self) -> &'static str;

    /// `(min_args, max_args)` — used for arity validation at call sites.
    fn arity(&self) -> (usize, usize);
}
