//! `ModuleComponent` — lifecycle hooks for SWed modules.

/// Lifecycle hooks that allow optional modules (`swed_kn`, `swed_db`, etc.)
/// to plug into the engine without modifying `swed_rt`.
pub trait ModuleComponent {
    /// Called once when the module is loaded into the engine.
    fn on_init(&mut self);

    /// Called when the engine shuts down (clean exit or critical error).
    fn on_shutdown(&mut self);
}
