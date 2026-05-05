//! [`NormPass`] trait — implemented by every normalization pass.

use swed_co::ast::Program;

/// A single AST rewrite pass.
///
/// Passes receive the whole `Program` and mutate it in place.
/// They must be idempotent: running a pass twice should produce the same
/// result as running it once.
pub trait NormPass {
    /// Apply this pass to `program`.
    fn run(&self, program: &mut Program);

    /// Human-readable name used in debug output.
    fn name(&self) -> &'static str;
}
