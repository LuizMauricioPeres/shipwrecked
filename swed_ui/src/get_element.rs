// swed_ui/src/get_element.rs
// GetElement trait + GetWidget — campo editável do ciclo GET/READ do Harbour.

use swed_rt::HbValue;

// ---------------------------------------------------------------------------
// Trait público
// ---------------------------------------------------------------------------

/// Interface de um elemento GET: um campo editável em um formulário Harbour.
///
/// O ciclo `GET x PICTURE "999" / READ` cria um ou mais `GetElement`s,
/// empilha-os em um [`GetList`] e chama [`hb_read`] para entrar no modo interativo.
///
/// [`GetList`]: super::get_list::GetList
/// [`hb_read`]: super::read::hb_read
pub trait GetElement: Send + Sync {
    /// Representação visual completa (label + valor), usada fora do modo edição.
    fn display(&self) -> String;
    /// Ativa ou desativa o foco visual.
    fn set_focus(&mut self, active: bool);
    /// Lê o valor Harbour atual.
    fn var_get(&self) -> HbValue;
    /// Escreve um valor Harbour (substitui valor e reinicia buffer).
    fn var_put(&mut self, val: HbValue);
    /// Largura visual do campo em colunas de terminal.
    fn width(&self) -> u16;
    /// Rótulo exibido antes do campo.
    fn label(&self) -> &str;
    /// Conteúdo atual do buffer de edição (o que o usuário está digitando).
    fn edit_buf(&self) -> &str;
    /// Processa um caractere digitado. Retorna `true` se aceito pelo PICTURE.
    fn handle_char(&mut self, ch: char) -> bool;
    /// Remove o último caractere do buffer (Backspace).
    fn handle_backspace(&mut self);
    /// Confirma a edição: converte buffer → `HbValue` e persiste.
    fn commit(&mut self);
    /// Cancela a edição: restaura o valor anterior ao foco.
    fn cancel(&mut self);
}

// ---------------------------------------------------------------------------
// GetWidget — implementação concreta
// ---------------------------------------------------------------------------

/// Campo editável concreto com suporte à cláusula PICTURE do Harbour.
///
/// # Flags PICTURE suportadas
/// - `@!` — converte entrada para maiúsculas
/// - Template: `9` dígito, `A` letra, `L` lógico, `X`/`!` qualquer char, literal passado direto
pub struct GetWidget {
    label:    String,
    value:    HbValue,
    picture:  Option<String>,
    focused:  bool,
    edit_buf: String,
    original: HbValue,
}

impl GetWidget {
    /// Cria um novo campo com rótulo, valor inicial e PICTURE opcional.
    pub fn new(label: impl Into<String>, value: HbValue, picture: Option<String>) -> Self {
        let buf      = hb_to_edit_str(&value);
        let original = value.clone();
        GetWidget { label: label.into(), value, picture, focused: false, edit_buf: buf, original }
    }

    fn computed_width(&self) -> u16 {
        if let Some(pic) = &self.picture {
            pic_template(pic).len().max(1) as u16
        } else {
            match &self.value {
                HbValue::Integer(_) => 10,
                HbValue::Float(_)   => 14,
                HbValue::String(s)  => s.len().max(10) as u16,
                HbValue::Logical(_) => 1,
                HbValue::Date(_)    => 10,
                _                   => 10,
            }
        }
    }

    fn accepts_char_at(&self, ch: char, pos: usize) -> bool {
        if let Some(pic) = &self.picture {
            match pic_template(pic).chars().nth(pos) {
                Some(mask) => pic_match(mask, ch),
                None       => false,
            }
        } else {
            true
        }
    }

    fn apply_flags(&self, ch: char) -> char {
        if self.picture.as_deref().map_or(false, |p| p.contains("@!")) {
            ch.to_ascii_uppercase()
        } else {
            ch
        }
    }
}

impl GetElement for GetWidget {
    fn display(&self) -> String {
        let w   = self.computed_width() as usize;
        let val = format!("{:<width$}", hb_to_edit_str(&self.value), width = w);
        if self.label.is_empty() { val } else { format!("{}: {val}", self.label) }
    }

    fn set_focus(&mut self, active: bool) {
        self.focused = active;
        if active {
            self.edit_buf = hb_to_edit_str(&self.value);
        }
    }

    fn var_get(&self) -> HbValue { self.value.clone() }

    fn var_put(&mut self, val: HbValue) {
        self.edit_buf = hb_to_edit_str(&val);
        self.original = val.clone();
        self.value    = val;
    }

    fn width(&self) -> u16   { self.computed_width() }
    fn label(&self) -> &str  { &self.label }
    fn edit_buf(&self) -> &str { &self.edit_buf }

    fn handle_char(&mut self, ch: char) -> bool {
        let pos = self.edit_buf.len();
        if pos >= self.computed_width() as usize { return false; }
        if !self.accepts_char_at(ch, pos)        { return false; }
        self.edit_buf.push(self.apply_flags(ch));
        true
    }

    fn handle_backspace(&mut self) { self.edit_buf.pop(); }

    fn commit(&mut self) {
        let raw = self.edit_buf.trim_end().to_string();
        self.value = match &self.value {
            HbValue::Integer(_) => raw.parse::<i64>()
                .map(HbValue::Integer).unwrap_or(HbValue::Integer(0)),
            HbValue::Float(_)   => raw.parse::<f64>()
                .map(HbValue::Float).unwrap_or(HbValue::Float(0.0)),
            HbValue::Logical(_) => HbValue::Logical(
                matches!(raw.to_ascii_uppercase().as_str(), "T" | "Y" | "S" | ".T.")
            ),
            _ => HbValue::String(raw),
        };
        self.original = self.value.clone();
    }

    fn cancel(&mut self) {
        self.value    = self.original.clone();
        self.edit_buf = hb_to_edit_str(&self.value);
    }
}

// ---------------------------------------------------------------------------
// Helpers privados
// ---------------------------------------------------------------------------

fn hb_to_edit_str(val: &HbValue) -> String {
    match val {
        HbValue::String(s)  => s.clone(),
        HbValue::Integer(n) => n.to_string(),
        HbValue::Float(f)   => format!("{f}"),
        HbValue::Logical(b) => if *b { "T".into() } else { "F".into() },
        HbValue::Date(d)    => format!("{d}"),
        HbValue::Nil        => String::new(),
        HbValue::Array(_)   => String::new(),
    }
}

/// Extrai o template de máscara da PICTURE, descartando flags `@...`.
/// `"@! 999-999"` → `"999-999"`.  `"9999"` → `"9999"`.
fn pic_template(pic: &str) -> &str {
    if let Some(rest) = pic.strip_prefix('@') {
        rest.trim_start_matches(|c: char| c != ' ').trim_start_matches(' ')
    } else {
        pic
    }
}

/// Verifica se `ch` é aceito pelo char de máscara `mask`.
fn pic_match(mask: char, ch: char) -> bool {
    match mask {
        '9'       => ch.is_ascii_digit() || ch == '-' || ch == '.',
        'A'       => ch.is_ascii_alphabetic(),
        'L'       => matches!(ch, 'T' | 'F' | 't' | 'f' | 'Y' | 'N' | 'y' | 'n'),
        'X' | '!' => true,
        _         => ch == mask, // literal: '-', '/', '.', etc.
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_picture_9_aceita_digito_rejeita_letra() {
        let mut w = GetWidget::new("Cod", HbValue::String(String::new()), Some("999".into()));
        assert!(w.handle_char('5'));
        assert!(!w.handle_char('A'));
        assert_eq!(w.edit_buf(), "5");
    }

    #[test]
    fn test_picture_a_rejeita_digito() {
        let mut w = GetWidget::new("Nome", HbValue::String(String::new()), Some("AAA".into()));
        assert!(w.handle_char('Z'));
        assert!(!w.handle_char('1'));
    }

    #[test]
    fn test_flag_exclamation_uppercases() {
        let mut w = GetWidget::new("", HbValue::String(String::new()), Some("@! AAA".into()));
        w.handle_char('a');
        assert_eq!(w.edit_buf(), "A");
    }

    #[test]
    fn test_commit_integer() {
        let mut w = GetWidget::new("", HbValue::Integer(0), None);
        w.handle_char('4');
        w.handle_char('2');
        w.commit();
        assert_eq!(w.var_get(), HbValue::Integer(42));
    }

    #[test]
    fn test_cancel_restaura_original() {
        let mut w = GetWidget::new("", HbValue::String("Alice".into()), None);
        w.set_focus(true);
        w.handle_char('B');
        w.cancel();
        assert_eq!(w.var_get(), HbValue::String("Alice".into()));
    }

    #[test]
    fn test_width_vem_do_picture() {
        let w = GetWidget::new("", HbValue::String(String::new()), Some("999-999".into()));
        assert_eq!(w.width(), 7);
    }

    #[test]
    fn test_backspace_remove_ultimo_char() {
        let mut w = GetWidget::new("", HbValue::String(String::new()), None);
        w.handle_char('A');
        w.handle_char('B');
        w.handle_backspace();
        assert_eq!(w.edit_buf(), "A");
    }

    #[test]
    fn test_overflow_rejeitado() {
        let mut w = GetWidget::new("", HbValue::String(String::new()), Some("99".into()));
        w.handle_char('1');
        w.handle_char('2');
        assert!(!w.handle_char('3')); // largura = 2, não cabe
        assert_eq!(w.edit_buf(), "12");
    }

    #[test]
    fn test_pic_template_strips_flags() {
        assert_eq!(pic_template("@! 999"), "999");
        assert_eq!(pic_template("@R 999-999"), "999-999");
        assert_eq!(pic_template("9999"), "9999");
    }
}
