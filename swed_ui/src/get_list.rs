// swed_ui/src/get_list.rs
// GetList — coleção ordenada de campos GET com navegação por teclado.

use crate::get_element::GetElement;
use swed_rt::HbValue;

/// Coleção de campos [`GetElement`] que formam um formulário Harbour.
///
/// Gerencia o campo ativo e a navegação Tab/Shift-Tab entre os campos.
/// Criada antes do `READ` e descartada após o término do ciclo.
pub struct GetList {
    elements:     Vec<Box<dyn GetElement>>,
    active_index: Option<usize>,
}

impl GetList {
    /// Cria uma GetList vazia.
    pub fn new() -> Self {
        GetList { elements: Vec::new(), active_index: None }
    }

    /// Adiciona um campo ao final da lista.
    pub fn add(&mut self, element: Box<dyn GetElement>) {
        self.elements.push(element);
    }

    /// Número de campos na lista.
    pub fn len(&self) -> usize { self.elements.len() }

    /// `true` se a lista não tem campos.
    pub fn is_empty(&self) -> bool { self.elements.is_empty() }

    /// Índice do campo atualmente com foco, ou `None` se nenhum.
    pub fn active_index(&self) -> Option<usize> { self.active_index }

    /// Ativa o campo `idx`, desativando o campo anterior.
    pub fn set_active(&mut self, idx: usize) {
        if let Some(old) = self.active_index {
            if let Some(e) = self.elements.get_mut(old) {
                e.set_focus(false);
            }
        }
        if idx < self.elements.len() {
            self.active_index = Some(idx);
            self.elements[idx].set_focus(true);
        }
    }

    /// Avança para o próximo campo (circular).
    pub fn next(&mut self) {
        if self.elements.is_empty() { return; }
        let n    = self.elements.len();
        let next = self.active_index.map_or(0, |i| (i + 1) % n);
        self.set_active(next);
    }

    /// Recua para o campo anterior (circular).
    pub fn prev(&mut self) {
        if self.elements.is_empty() { return; }
        let n    = self.elements.len();
        let prev = self.active_index.map_or(0, |i| if i == 0 { n - 1 } else { i - 1 });
        self.set_active(prev);
    }

    /// Referência imutável ao campo ativo, ou `None`.
    pub fn active_element(&self) -> Option<&dyn GetElement> {
        self.active_index
            .and_then(|i| self.elements.get(i))
            .map(Box::as_ref)
    }

    /// Referência mutável ao campo ativo, ou `None`.
    pub fn active_element_mut(&mut self) -> Option<&mut Box<dyn GetElement>> {
        self.active_index.and_then(|i| self.elements.get_mut(i))
    }

    /// Slice de todos os campos (para renderização).
    pub fn elements(&self) -> &[Box<dyn GetElement>] { &self.elements }

    /// Confirma todos os campos (chamado no ESC + Enter no último campo).
    pub fn commit_all(&mut self) {
        for e in &mut self.elements { e.commit(); }
    }

    /// Cancela todos os campos (chamado no ESC).
    pub fn cancel_all(&mut self) {
        for e in &mut self.elements { e.cancel(); }
    }

    /// Lê o valor do campo `idx`. Retorna `HbValue::Nil` se fora do range.
    pub fn get_value(&self, idx: usize) -> HbValue {
        self.elements.get(idx).map_or(HbValue::Nil, |e| e.var_get())
    }
}

impl Default for GetList {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_element::GetWidget;

    fn widget(label: &str, val: &str) -> Box<dyn GetElement> {
        Box::new(GetWidget::new(label, HbValue::String(val.into()), None))
    }

    #[test]
    fn test_add_e_len() {
        let mut list = GetList::new();
        list.add(widget("A", ""));
        list.add(widget("B", ""));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_set_active_foca_elemento() {
        let mut list = GetList::new();
        list.add(widget("A", "x"));
        list.add(widget("B", "y"));
        list.set_active(1);
        assert_eq!(list.active_index(), Some(1));
    }

    #[test]
    fn test_next_circular() {
        let mut list = GetList::new();
        list.add(widget("A", ""));
        list.add(widget("B", ""));
        list.set_active(0);
        list.next();
        assert_eq!(list.active_index(), Some(1));
        list.next(); // wrap
        assert_eq!(list.active_index(), Some(0));
    }

    #[test]
    fn test_prev_circular() {
        let mut list = GetList::new();
        list.add(widget("A", ""));
        list.add(widget("B", ""));
        list.set_active(0);
        list.prev(); // wrap para o final
        assert_eq!(list.active_index(), Some(1));
    }

    #[test]
    fn test_get_value() {
        let mut list = GetList::new();
        list.add(Box::new(GetWidget::new("N", HbValue::Integer(99), None)));
        assert_eq!(list.get_value(0), HbValue::Integer(99));
        assert_eq!(list.get_value(99), HbValue::Nil);
    }

    #[test]
    fn test_cancel_all_restaura_valores() {
        let mut list = GetList::new();
        list.add(Box::new(GetWidget::new("X", HbValue::String("original".into()), None)));
        list.set_active(0);
        list.active_element_mut().unwrap().handle_char('Z');
        list.cancel_all();
        assert_eq!(list.get_value(0), HbValue::String("original".into()));
    }
}
