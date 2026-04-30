//! `GetElement` — contrato que todo widget de entrada Harbour deve satisfazer.

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};
use swed_rt::HbValue;

/// Trait central do sistema `@..GET` / `READ`.
///
/// Cada tipo de campo (texto, numérico, data, lógico) implementa esta trait.
/// `AppState` mantém um `Vec<Box<dyn GetElement>>` e dirige o loop de eventos.
pub trait GetElement {
    /// Renderiza o widget dentro da área indicada.
    fn render(&self, frame: &mut Frame, area: Rect);

    /// Processa uma tecla. Retorna `true` se o campo consumiu o evento.
    fn handle_key(&mut self, key: KeyEvent) -> bool;

    /// Valor atual do campo como `HbValue`.
    fn value(&self) -> HbValue;

    /// Define o valor inicial (chamado antes de entrar no READ loop).
    fn set_value(&mut self, val: HbValue);

    /// `true` quando o usuário confirmou a entrada (Enter / Tab) e o foco
    /// deve passar para o próximo campo.
    fn is_done(&self) -> bool;

    /// Reseta o flag `done` ao receber foco novamente (Tab reverso ou retorno).
    fn reset_done(&mut self);

    /// Rótulo descritivo exibido à esquerda do campo (equivalente ao SAY).
    fn label(&self) -> &str;
}
