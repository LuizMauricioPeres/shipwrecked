//! `swed_ui` — TUI layer for SWed.
//!
//! Translates Harbour screen commands (`@..SAY`, `@..GET`, `READ`) into
//! interactive Ratatui widgets. Depends on `swed_co` and `swed_rt`.
//! Does **not** depend on compiler crates.
//!
//! # Planned modules
//!
//! - `app_state`      — READ loop + field cursor (`AppState::run()`)
//! - `say`            — `@..SAY` → `Paragraph` widget
//! - `traits::get_element` — `GetElement` trait (render + handle_key + value)
//! - `widgets::char_input`     — text field with Picture mask
//! - `widgets::numeric_input`  — numeric field with decimal precision
//! - `widgets::date_input`     — DD/MM/YYYY mask with calendar validation
//! - `widgets::logical_toggle` — `.T.`/`.F.` toggled by Y / N / Space
