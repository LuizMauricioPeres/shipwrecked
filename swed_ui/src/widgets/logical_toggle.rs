//! Campo lógico — `.T.`/`.F.` alternado por `Y`, `N`, `S`, `N` ou `Space`.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use swed_rt::HbValue;

use crate::traits::GetElement;

/// Campo lógico Harbour. Aceita:
/// - `Y` / `S` (Sim) → `.T.`
/// - `N` → `.F.`
/// - `Space` → alterna
/// - `Enter` / `Tab` → confirma
#[derive(Debug, Clone)]
pub struct LogicalToggle {
    label: String,
    value: bool,
    done: bool,
}

impl LogicalToggle {
    /// Cria um campo lógico com o rótulo e valor inicial indicados.
    pub fn new(label: impl Into<String>, initial: bool) -> Self {
        LogicalToggle {
            label: label.into(),
            value: initial,
            done: false,
        }
    }
}

impl GetElement for LogicalToggle {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let (text, style) = if self.value {
            (
                " .T. ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            (
                " .F. ",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            )
        };

        let line = Line::from(vec![Span::styled(text, style)]);

        let block = Block::default()
            .title(self.label.as_str())
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Cyan));

        frame.render_widget(Paragraph::new(line).block(block), area);
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        if key.code == KeyCode::Esc {
            return false;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            return false;
        }
        match key.code {
            KeyCode::Enter | KeyCode::Tab => {
                self.done = true;
                true
            }
            KeyCode::Char(' ') => {
                self.value = !self.value;
                true
            }
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Char('s') | KeyCode::Char('S') => {
                self.value = true;
                true
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.value = false;
                true
            }
            _ => false,
        }
    }

    fn value(&self) -> HbValue {
        HbValue::Logical(self.value)
    }

    fn set_value(&mut self, val: HbValue) {
        self.value = matches!(val, HbValue::Logical(true));
        self.done = false;
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn reset_done(&mut self) {
        self.done = false;
    }

    fn label(&self) -> &str {
        &self.label
    }
}
