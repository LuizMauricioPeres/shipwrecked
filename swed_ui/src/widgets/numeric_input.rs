//! Campo numérico — equivale a `@row,col GET nVar PICTURE "9999.99"`.

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

/// Campo numérico com largura total e casas decimais configuráveis.
#[derive(Debug, Clone)]
pub struct NumericInput {
    label: String,
    /// Texto bruto em edição (dígitos + ponto decimal).
    buffer: String,
    cursor: usize,
    /// Número de casas decimais (0 = inteiro).
    decimals: usize,
    /// Largura total do campo (inclui sinal e ponto).
    width: usize,
    done: bool,
}

impl NumericInput {
    /// Cria um campo numérico com rótulo, largura total e casas decimais.
    pub fn new(label: impl Into<String>, width: usize, decimals: usize) -> Self {
        NumericInput {
            label: label.into(),
            buffer: String::new(),
            cursor: 0,
            decimals,
            width: width.max(1),
            done: false,
        }
    }

    fn is_valid_char(&self, ch: char) -> bool {
        if ch.is_ascii_digit() {
            return true;
        }
        if ch == '-' && self.buffer.is_empty() {
            return true;
        }
        if ch == '.' && self.decimals > 0 && !self.buffer.contains('.') {
            return true;
        }
        false
    }

    fn parse_value(&self) -> f64 {
        self.buffer.parse::<f64>().unwrap_or(0.0)
    }
}

impl GetElement for NumericInput {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let padded = format!("{:>width$}", self.buffer, width = self.width);
        let cursor_pos = self.cursor.min(padded.len());
        let (before, rest) = padded.split_at(cursor_pos);
        let mut chars = rest.chars();
        let cursor_ch = chars.next().unwrap_or(' ');
        let after: String = chars.collect();

        let line = Line::from(vec![
            Span::raw(before.to_string()),
            Span::styled(
                cursor_ch.to_string(),
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(after),
        ]);

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
        match key.code {
            KeyCode::Enter | KeyCode::Tab => {
                self.done = true;
                return true;
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.buffer.remove(self.cursor);
                }
                return true;
            }
            KeyCode::Delete => {
                if self.cursor < self.buffer.len() {
                    self.buffer.remove(self.cursor);
                }
                return true;
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                return true;
            }
            KeyCode::Right => {
                if self.cursor < self.buffer.len() {
                    self.cursor += 1;
                }
                return true;
            }
            KeyCode::Home => {
                self.cursor = 0;
                return true;
            }
            KeyCode::End => {
                self.cursor = self.buffer.len();
                return true;
            }
            KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.is_valid_char(ch) && self.buffer.len() < self.width {
                    self.buffer.insert(self.cursor, ch);
                    self.cursor += 1;
                }
                return true;
            }
            _ => {}
        }
        false
    }

    fn value(&self) -> HbValue {
        let f = self.parse_value();
        if self.decimals == 0 {
            HbValue::Integer(f as i64)
        } else {
            HbValue::Float(f)
        }
    }

    fn set_value(&mut self, val: HbValue) {
        self.buffer = match val {
            HbValue::Integer(n) => n.to_string(),
            HbValue::Float(f) => {
                if self.decimals > 0 {
                    format!("{:.dec$}", f, dec = self.decimals)
                } else {
                    format!("{:.0}", f)
                }
            }
            _ => String::new(),
        };
        self.cursor = self.buffer.len();
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
