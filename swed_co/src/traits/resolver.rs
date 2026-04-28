//! `FunctionResolver` — maps function names to runtime implementations.

use super::NativeFunction;

/// Resolves a Harbour function name to its `NativeFunction` implementation.
///
/// Transpiled code receives a `FunctionResolver` via dependency injection,
/// keeping the generated output decoupled from concrete `swed_bf` internals.
///
/// `V` is the runtime value type (concretely `HbValue` in `swed_rt`).
pub trait FunctionResolver<V> {
    /// Look up a function by its UPPERCASE Harbour name.
    ///
    /// Returns `None` when the name is not registered (caller should fall back
    /// to user-defined functions or emit a runtime error).
    fn resolve(&self, name: &str) -> Option<&dyn NativeFunction<V>>;

    /// Returns `true` if the resolver knows the given function name.
    fn contains(&self, name: &str) -> bool {
        self.resolve(name).is_some()
    }
}
