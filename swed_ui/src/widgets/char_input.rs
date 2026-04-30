//! Campo de texto — equivale a `@row,col GET cVar PICTURE "@!"`.

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

/// Máscara de picture do Harbour (subconjunto implementado).
#[derive(Debug, Clone, Default)]
pub enum Picture {
    /// Sem máscara — aceita qualquer caractere.
    #[default]
    None,
    /// `@!` — converte tudo para maiúsculo.
    UpperCase,
    /// `@K` — limpa o campo na primeira tecla (kill-on-first-key).
    KillOnFirst,
}

/// Widget de entrada de texto com comprimento máximo e picture mask.
#[derive(Debug, Clone)]
pub struct CharInput {
    label: String,
    buffer: Vec<char>,
    cursor: usize,
    max_len: usize,
    picture: Picture,
    done: bool,
    /// Indica se a próxima tecla deve limpar o buffer (modo `@K`).
    kill_pending: bool,
}

impl CharInput {
    /// `label` — texto do SAY; `max_len` — comprimento máximo do campo.
    pub fn new(label: impl Into<String>, max_len: usize) -> Self {
        CharInput {
            label: label.into(),
            buffer: Vec::new(),
            cursor: 0,
            max_len: max_len.max(1),
            picture: Picture::default(),
            done: false,
            kill_pending: false,
        }
    }

    /// Define a máscara de picture para o campo.
    pub fn with_picture(mut self, picture: Picture) -> Self {
        if matches!(picture, Picture::KillOnFirst) {
            self.kill_pending = true;
        }
        self.picture = picture;
        self
    }

    fn apply_picture(&self, ch: char) -> char {
        match self.picture {
            Picture::UpperCase => ch.to_uppercase().next().unwrap_or(ch),
            _ => ch,
        }
    }

    fn current_text(&self) -> String {
        self.buffer.iter().collect()
    }
}

impl GetElement for CharInput {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let text = self.current_text();
        // Pad com espaços até max_len para exibir o campo completo
        let padded = format!("{:<width$}", text, width = self.max_len);

        // Coloca cursor visual via split do texto
        let (before, rest) = padded.split_at(self.cursor.min(padded.len()));
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
        // Ctrl+C / Esc — não consome, deixa AppState decidir
        if key.code == KeyCode::Esc {
            return false;
        }

        match key.code {
            KeyCode::Enter | KeyCode::Tab => {
                self.done = true;
                return true;
            }
            KeyCode::BackTab => {
                // Shift+Tab — sinaliza ao AppState para voltar
                self.done = true;
                return true;
            }
            KeyCode::Backspace => {
                if self.kill_pending {
                    self.buffer.clear();
                    self.cursor = 0;
                    self.kill_pending = false;
                } else if self.cursor > 0 {
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
                self.kill_pending = false;
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                return true;
            }
            KeyCode::Right => {
                self.kill_pending = false;
                if self.cursor < self.buffer.len() {
                    self.cursor += 1;
                }
                return true;
            }
            KeyCode::Home => {
                self.kill_pending = false;
                self.cursor = 0;
                return true;
            }
            KeyCode::End => {
                self.kill_pending = false;
                self.cursor = self.buffer.len();
                return true;
            }
            KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.kill_pending {
                    self.buffer.clear();
                    self.cursor = 0;
                    self.kill_pending = false;
                }
                if self.buffer.len() < self.max_len {
                    let ch = self.apply_picture(ch);
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
        HbValue::String(self.current_text())
    }

    fn set_value(&mut self, val: HbValue) {
        let s = match val {
            HbValue::String(s) => s,
            other => other.to_string(),
        };
        self.buffer = s.chars().take(self.max_len).collect();
        self.cursor = self.buffer.len();
        if matches!(self.picture, Picture::KillOnFirst) {
            self.kill_pending = true;
        }
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
