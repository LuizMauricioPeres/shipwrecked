//! Campo de data — máscara DD/MM/AAAA com validação civil.

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

/// Campo de data com máscara `DD/MM/AAAA`.
///
/// Apenas dígitos são aceitos; as barras são inseridas automaticamente.
/// Valida se a data é civil (mês 1-12, dia válido para o mês/ano).
#[derive(Debug, Clone)]
pub struct DateInput {
    label: String,
    /// Apenas os 8 dígitos (sem barras): DDMMAAAA.
    digits: [char; 8],
    /// Posição do cursor em dígitos (0-7).
    digit_pos: usize,
    done: bool,
}

impl DateInput {
    /// Cria um campo de data com o rótulo indicado.
    pub fn new(label: impl Into<String>) -> Self {
        DateInput {
            label: label.into(),
            digits: ['_'; 8],
            digit_pos: 0,
            done: false,
        }
    }

    /// Formata os dígitos como `DD/MM/AAAA` para exibição.
    fn display_text(&self) -> String {
        format!(
            "{}{}/{}{}/{}{}{}{}",
            self.digits[0],
            self.digits[1],
            self.digits[2],
            self.digits[3],
            self.digits[4],
            self.digits[5],
            self.digits[6],
            self.digits[7],
        )
    }

    /// Posição visual do cursor dentro da string `DD/MM/AAAA` (10 chars).
    fn cursor_visual_pos(&self) -> usize {
        match self.digit_pos {
            0 | 1 => self.digit_pos,
            2 | 3 => self.digit_pos + 1, // pula a primeira '/'
            4..=7 => self.digit_pos + 2, // pula as duas '/'
            _ => 9,
        }
    }

    fn is_valid_date(&self) -> bool {
        if self.digits.iter().any(|&c| c == '_') {
            return false;
        }
        let d: u32 = format!("{}{}", self.digits[0], self.digits[1])
            .parse()
            .unwrap_or(0);
        let m: u32 = format!("{}{}", self.digits[2], self.digits[3])
            .parse()
            .unwrap_or(0);
        let y: i32 = format!(
            "{}{}{}{}",
            self.digits[4], self.digits[5], self.digits[6], self.digits[7]
        )
        .parse()
        .unwrap_or(0);

        if m == 0 || m > 12 || d == 0 {
            return false;
        }
        let max_day = days_in_month(m, y);
        d <= max_day
    }

    /// Converte para dias desde 1970-01-01 (formato `HbValue::Date`).
    fn to_epoch_days(&self) -> Option<i32> {
        if !self.is_valid_date() {
            return None;
        }
        let d: i64 = format!("{}{}", self.digits[0], self.digits[1])
            .parse()
            .ok()?;
        let m: i64 = format!("{}{}", self.digits[2], self.digits[3])
            .parse()
            .ok()?;
        let y: i64 = format!(
            "{}{}{}{}",
            self.digits[4], self.digits[5], self.digits[6], self.digits[7]
        )
        .parse()
        .ok()?;

        // Algoritmo de Howard Hinnant (inverso do usado em HbValue::format_date)
        let (y, m) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = y - era * 400;
        let doy = (153 * m + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        Some((era * 146_097 + doe - 719_468) as i32)
    }
}

fn days_in_month(m: u32, y: i32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

impl GetElement for DateInput {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let text = self.display_text();
        let vis = self.cursor_visual_pos();

        let (before, rest) = text.split_at(vis);
        let mut chars = rest.chars();
        let cursor_ch = chars.next().unwrap_or(' ');
        let after: String = chars.collect();

        let valid_style = if self.is_valid_date() || self.digits.iter().all(|&c| c == '_') {
            Style::default()
        } else {
            Style::default().fg(Color::Red)
        };

        let line = Line::from(vec![
            Span::styled(before.to_string(), valid_style),
            Span::styled(
                cursor_ch.to_string(),
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(after, valid_style),
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
                if self.digit_pos > 0 {
                    self.digit_pos -= 1;
                    self.digits[self.digit_pos] = '_';
                }
                return true;
            }
            KeyCode::Left => {
                if self.digit_pos > 0 {
                    self.digit_pos -= 1;
                }
                return true;
            }
            KeyCode::Right => {
                if self.digit_pos < 7 {
                    self.digit_pos += 1;
                }
                return true;
            }
            KeyCode::Char(ch)
                if ch.is_ascii_digit() && !key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                self.digits[self.digit_pos] = ch;
                if self.digit_pos < 7 {
                    self.digit_pos += 1;
                }
                return true;
            }
            _ => {}
        }
        false
    }

    fn value(&self) -> HbValue {
        match self.to_epoch_days() {
            Some(days) => HbValue::Date(days),
            None => HbValue::Nil,
        }
    }

    fn set_value(&mut self, val: HbValue) {
        if let HbValue::Date(days) = val {
            // Reutiliza format_date do HbValue para obter DD/MM/AAAA
            let s = HbValue::Date(days).format_date();
            // s = "DD/MM/YYYY"
            let nums: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
            let chars: Vec<char> = nums.chars().collect();
            for (i, ch) in chars.iter().enumerate().take(8) {
                self.digits[i] = *ch;
            }
        } else {
            self.digits = ['_'; 8];
        }
        self.digit_pos = 0;
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
