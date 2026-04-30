//! `@row,col SAY` — exibe texto estático na posição indicada.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
    Frame,
};
use swed_rt::HbValue;

/// Representa um `@row,col SAY <expr>` do Harbour.
///
/// Em Ratatui o posicionamento absoluto por linha/coluna é feito via
/// `Rect { x: col, y: row, width, height: 1 }`.
#[derive(Debug, Clone)]
pub struct Say {
    row: u16,
    col: u16,
    text: String,
    color: Color,
}

impl Say {
    /// Cria um SAY a partir de um `HbValue` (converte via `Display`).
    pub fn new(row: u16, col: u16, val: &HbValue) -> Self {
        Say {
            row,
            col,
            text: val.to_string(),
            color: Color::White,
        }
    }

    /// Define a cor do texto (equivale a parte do `SETCOLOR` do Harbour).
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Renderiza o texto no frame na posição absoluta.
    pub fn render(&self, frame: &mut Frame) {
        let area = Rect {
            x: self.col,
            y: self.row,
            width: self.text.len() as u16,
            height: 1,
        };
        let span = Span::styled(self.text.clone(), Style::default().fg(self.color));
        frame.render_widget(Paragraph::new(span), area);
    }
}
