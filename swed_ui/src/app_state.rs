//! `AppState` — loop principal do `READ` do Harbour.
//!
//! Gerencia um conjunto de campos `GetElement`, navega entre eles com Tab /
//! Shift+Tab / setas e retorna os valores confirmados ou sinaliza cancelamento.

use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
    Terminal,
};
use swed_rt::HbValue;

use crate::traits::GetElement;

/// Resultado do `READ` loop.
#[derive(Debug)]
pub enum ReadResult {
    /// Usuário confirmou todos os campos (Enter no último ou Tab além do último).
    Confirmed(Vec<HbValue>),
    /// Usuário cancelou com Esc.
    Cancelled,
}

/// Estado do loop `READ`.
pub struct AppState {
    fields: Vec<Box<dyn GetElement>>,
    /// Índice do campo com foco atual.
    focused: usize,
    /// Mensagem de status exibida na linha inferior.
    status: String,
}

impl AppState {
    /// Cria um `AppState` com a lista de campos em ordem de tabulação.
    pub fn new(fields: Vec<Box<dyn GetElement>>) -> Self {
        AppState {
            fields,
            focused: 0,
            status: String::from("Tab: próximo  Shift+Tab: anterior  Esc: cancelar"),
        }
    }

    /// Executa o loop de eventos Ratatui e retorna o resultado do READ.
    ///
    /// Inicializa o terminal em modo raw, alterna para tela alternativa e
    /// restaura o estado original ao sair (seja por confirmação ou cancelamento).
    pub fn run(mut self) -> io::Result<ReadResult> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.event_loop(&mut terminal);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }

    fn event_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<ReadResult> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.handle_global_key(key) {
                    Some(result) => return Ok(result),
                    None => self.dispatch_to_focused(key),
                }
            }

            // Verifica se o campo focado confirmou e avança
            if self.fields[self.focused].is_done() {
                self.fields[self.focused].reset_done();
                if self.focused + 1 < self.fields.len() {
                    self.focused += 1;
                } else {
                    // Último campo confirmado — encerra com sucesso
                    let values = self.fields.iter().map(|f| f.value()).collect();
                    return Ok(ReadResult::Confirmed(values));
                }
            }
        }
    }

    fn render(&self, frame: &mut ratatui::Frame) {
        let total = self.fields.len();
        // Uma linha por campo + 2 (título + status)
        let constraints: Vec<Constraint> = (0..total)
            .map(|_| Constraint::Length(3))
            .chain(std::iter::once(Constraint::Min(1)))
            .collect();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints)
            .split(frame.area());

        for (i, field) in self.fields.iter().enumerate() {
            // Destaca o campo com foco com borda colorida diferente
            // (cada widget desenha a si mesmo; AppState só fornece a área)
            let area = chunks[i];
            if i == self.focused {
                // Overlay de indicação de foco — borda amarela
                let focus_hint = ratatui::widgets::Block::default()
                    .border_style(Style::default().fg(Color::Yellow));
                frame.render_widget(focus_hint, area);
            }
            field.render(frame, area);
        }

        // Linha de status
        let status_area = chunks[total];
        let status_span = Span::styled(
            self.status.as_str(),
            Style::default().fg(Color::DarkGray),
        );
        frame.render_widget(Paragraph::new(status_span), status_area);
    }

    /// Trata teclas globais (Esc, Shift+Tab retroativo).
    /// Retorna `Some(ReadResult)` se o loop deve encerrar.
    fn handle_global_key(&mut self, key: KeyEvent) -> Option<ReadResult> {
        if key.code == KeyCode::Esc {
            return Some(ReadResult::Cancelled);
        }
        // Shift+Tab no primeiro campo não faz nada
        if key.code == KeyCode::BackTab
            || (key.code == KeyCode::Tab
                && key.modifiers.contains(KeyModifiers::SHIFT))
        {
            if self.focused > 0 {
                self.focused -= 1;
            }
            self.status = format!(
                "Campo {}/{} — Tab: próximo  Shift+Tab: anterior  Esc: cancelar",
                self.focused + 1,
                self.fields.len()
            );
            return None; // não encerra, só navega
        }
        None
    }

    fn dispatch_to_focused(&mut self, key: KeyEvent) {
        self.fields[self.focused].handle_key(key);
    }
}
