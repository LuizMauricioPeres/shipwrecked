//! `swed_ui` — TUI layer for SWed.
//!
//! Translates Harbour screen commands (`@..SAY`, `@..GET`, `READ`) into
//! interactive Ratatui widgets. Depends on `swed_co` and `swed_rt`.
//! Does **not** depend on compiler crates.
//!
//! # Systems
//!
//! **AppState system** (`app_state`, `traits`, `widgets`): ratatui-widget-based,
//! each widget renders itself. Used by the codegen `@..SAY`/`@..GET`/`READ` blocks.
//!
//! **GetList system** (`get_element`, `get_list`, `read`): char-input-based,
//! Harbour-faithful with PICTURE mask support and commit/cancel semantics.

pub mod app_state;
pub mod get_element;
pub mod get_list;
pub mod read;
pub mod say;
pub mod traits;
pub mod widgets;

// ── AppState system ───────────────────────────────────────────────────────────
pub use app_state::{AppState, ReadResult};
pub use traits::GetElement;
pub use widgets::{CharInput, DateInput, LogicalToggle, NumericInput};

// ── GetList system ────────────────────────────────────────────────────────────
pub use get_element::GetWidget;
pub use get_list::GetList;
pub use read::hb_read;

use std::cell::RefCell;

thread_local! {
    /// Campo GET atualmente com foco durante um ciclo READ ativo.
    /// Permite que callbacks Harbour acessem o elemento ativo sem referência direta.
    pub static GETLIST_ACTIVE: RefCell<Option<Box<dyn get_element::GetElement>>>
        = RefCell::new(None);
}

/// Retorna o valor do campo GET atualmente ativo, ou `None` se não há READ em andamento.
pub fn active_get_value() -> Option<swed_rt::HbValue> {
    GETLIST_ACTIVE.with(|cell| {
        cell.borrow().as_ref().map(|e| e.var_get())
    })
}
