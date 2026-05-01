// swed_ui/src/read.rs
// hb_read() — implementação do comando READ do Harbour via ratatui.
//
// Fluxo Harbour:
//   GET x PICTURE "999"   → cria GetWidget, insere em GetList
//   GET y                  → idem
//   READ                   → chama hb_read(&mut list); bloqueia até o usuário sair
//
// Navegação:
//   Tab / Enter  → próximo campo (Enter no último = confirma e sai)
//   Shift-Tab    → campo anterior
//   ESC          → cancela todos os campos e sai (retorna false)
//   Backspace    → apaga último char do buffer
//   Qualquer char → enviado para handle_char() do campo ativo

use std::io;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::get_list::GetList;

/// Executa o ciclo READ do Harbour.
///
/// Toma controle do terminal, renderiza os campos com ratatui e processa
/// a entrada do usuário até que o formulário seja confirmado (retorna `true`)
/// ou cancelado com ESC (retorna `false`).
///
/// # Errors
/// Propaga erros de I/O do terminal (crossterm/ratatui).
pub fn hb_read(list: &mut GetList) -> io::Result<bool> {
    if list.is_empty() {
        return Ok(false);
    }

    list.set_active(0);

    let mut terminal = ratatui::init();
    let result       = run_loop(&mut terminal, list);
    ratatui::restore();
    result
}

fn run_loop(
    terminal: &mut ratatui::DefaultTerminal,
    list:     &mut GetList,
) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| render(frame, list))?;

        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                // ESC — cancela tudo
                (KeyCode::Esc, _) => {
                    list.cancel_all();
                    return Ok(false);
                }

                // Enter — confirma campo atual; se for o último, encerra
                (KeyCode::Enter, _) => {
                    if let Some(e) = list.active_element_mut() { e.commit(); }
                    let next = list.active_index().map_or(0, |i| i + 1);
                    if next >= list.len() {
                        list.commit_all();
                        return Ok(true);
                    }
                    list.set_active(next);
                }

                // Tab — próximo campo (circular)
                (KeyCode::Tab, mods) if !mods.contains(KeyModifiers::SHIFT) => {
                    if let Some(e) = list.active_element_mut() { e.commit(); }
                    list.next();
                }

                // Shift-Tab / BackTab — campo anterior
                (KeyCode::BackTab, _) | (KeyCode::Tab, KeyModifiers::SHIFT) => {
                    if let Some(e) = list.active_element_mut() { e.commit(); }
                    list.prev();
                }

                // Backspace
                (KeyCode::Backspace, _) => {
                    if let Some(e) = list.active_element_mut() { e.handle_backspace(); }
                }

                // Caractere imprimível
                (KeyCode::Char(c), _) => {
                    if let Some(e) = list.active_element_mut() { e.handle_char(c); }
                }

                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Renderização
// ---------------------------------------------------------------------------

fn render(frame: &mut Frame, list: &GetList) {
    let area   = frame.area();
    let n      = list.len();
    let active = list.active_index();

    // Uma linha por campo
    let constraints: Vec<Constraint> = (0..n).map(|_| Constraint::Length(1)).collect();
    let chunks = Layout::vertical(constraints).split(area);

    for (i, elem) in list.elements().iter().enumerate() {
        let Some(chunk) = chunks.get(i) else { break };
        let is_active   = active == Some(i);

        let label_style  = Style::default().add_modifier(Modifier::BOLD);
        let value_style  = if is_active {
            Style::default().fg(Color::Black).bg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let label = elem.label();
        let w     = elem.width() as usize;
        let buf   = if is_active {
            elem.edit_buf()
        } else {
            elem.edit_buf() // display usa mesmo buf fora do foco
        };
        let padded = format!("{:<width$}", buf, width = w);

        let line = if label.is_empty() {
            Line::from(Span::styled(padded, value_style))
        } else {
            Line::from(vec![
                Span::styled(format!("{label}: "), label_style),
                Span::styled(padded, value_style),
            ])
        };

        frame.render_widget(Paragraph::new(line), *chunk);
    }
}
