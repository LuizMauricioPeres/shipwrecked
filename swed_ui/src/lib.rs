//! `swed_ui` — TUI layer for SWed.
//!
//! Translates Harbour screen commands (`@..SAY`, `@..GET`, `READ`) into
//! interactive Ratatui widgets. Depends on `swed_co` and `swed_rt`.
//! Does **not** depend on compiler crates.

pub mod app_state;
pub mod say;
pub mod traits;
pub mod widgets;

pub use app_state::{AppState, ReadResult};
pub use traits::GetElement;
pub use widgets::{CharInput, DateInput, LogicalToggle, NumericInput};
