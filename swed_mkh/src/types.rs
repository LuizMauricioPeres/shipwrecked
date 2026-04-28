//! Tipos centrais do manifesto de símbolos Harbour (.mkh).

// `HbType` agora vive em `swed_co` — re-exportado aqui para compatibilidade.
pub use swed_co::HbType;

// ── Parameter definition ──────────────────────────────────────────────────────

/// Parâmetro formal de uma função, procedimento ou método.
#[derive(Debug, Clone)]
pub struct ParamDef {
    /// Nome original preservando case (`"nValor"`) — usado pelo Rust codegen.
    pub original_name: String,
    /// Nome normalizado em UPPERCASE (`"NVALOR"`) — chave nas tabelas de símbolos Harbour.
    pub normalized_name: String,
    /// Tipo inferido via Hungarian notation. `None` se a regra não se aplicar.
    pub hb_type: Option<HbType>,
}

impl ParamDef {
    /// Constrói um `ParamDef` a partir do nome **com case original preservado**.
    ///
    /// A inferência de tipo aplica a regra camelCase húngara:
    /// `first.is_lowercase() && second.is_uppercase()`.
    pub fn new(original: impl Into<String>) -> Self {
        let original_name = original.into();
        let hb_type = HbType::from_hungarian(&original_name);
        let normalized_name = original_name.to_ascii_uppercase();
        Self { original_name, normalized_name, hb_type }
    }
}

// ── Symbol kinds ──────────────────────────────────────────────────────────────

/// Tipo de símbolo extraído do fonte Harbour.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    /// `FUNCTION`
    Function,
    /// `PROCEDURE`
    Procedure,
    /// `METHOD`
    Method,
    /// `PUBLIC var`
    Public,
    /// `STATIC var`
    Static,
    /// `MEMVAR var`
    Memvar,
    /// `VAR name [EXPORTED|HIDDEN|PROTECTED]` dentro de classe
    ClassVar {
        /// Visibilidade do membro
        visibility: Visibility,
    },
    /// `ACCESS name` dentro de classe
    Access,
    /// `ASSIGN name` dentro de classe
    Assign,
    /// `CLASS name`
    Class,
}

/// Visibilidade de membros de classe Harbour.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    /// `EXPORTED` (padrão)
    Exported,
    /// `HIDDEN`
    Hidden,
    /// `PROTECTED`
    Protected,
}

// ── Symbol ────────────────────────────────────────────────────────────────────

/// Um símbolo definido no fonte.
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Nome do símbolo em UPPERCASE (chave nas tabelas de símbolos Harbour).
    pub name: String,
    /// Tipo do símbolo.
    pub kind: SymbolKind,
    /// Escopo onde o símbolo foi declarado (nome da função/classe ou `"GLOBAL"`).
    pub scope: String,
    /// Linha de declaração (1-based).
    pub line: usize,
    /// Atributos adicionais (ex.: `"CONDITIONAL"`).
    pub attributes: Vec<String>,
    /// `true` se declarado dentro de bloco `#IFDEF`/`#IFNDEF`.
    pub conditional: bool,
    /// Tipo Harbour inferido via Hungarian notation aplicada ao nome **original**
    /// (antes de normalizar para UPPERCASE).
    ///
    /// `None` quando o nome não satisfaz a regra camelCase húngara (`first` minúsculo
    /// + `second` maiúsculo) ou quando o prefixo não tem mapeamento de tipo.
    /// Preenchido para Function, Procedure, Public, Memvar e Static.
    pub hb_type: Option<HbType>,
    /// Parâmetros formais — preenchidos para Function/Procedure/Method.
    pub params: Vec<ParamDef>,
}

// ── Usage ─────────────────────────────────────────────────────────────────────

/// Um uso de símbolo externo (chamada de função não definida localmente).
#[derive(Debug, Clone)]
pub struct Usage {
    /// Nome do símbolo em UPPERCASE.
    pub name: String,
    /// Linha de uso (1-based).
    pub line: usize,
    /// Coluna de início (1-based).
    pub col: usize,
}

// ── Manifest ──────────────────────────────────────────────────────────────────

/// Manifesto completo de um arquivo `.prg`.
#[derive(Debug)]
pub struct Manifest {
    /// Caminho do arquivo fonte.
    pub source_path: String,
    /// MD5 dos bytes brutos do arquivo (encoding preservado).
    pub md5: String,
    /// Todos os símbolos definidos no arquivo.
    pub symbols: Vec<Symbol>,
    /// Usos de símbolos externos (não definidos localmente).
    pub usages: Vec<Usage>,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_def_preserves_original_and_normalizes() {
        let p = ParamDef::new("nValor");
        assert_eq!(p.original_name,   "nValor");
        assert_eq!(p.normalized_name, "NVALOR");
        assert_eq!(p.hb_type, Some(HbType::Float)); // n → f64 até divisão i/f
    }

    #[test]
    fn param_def_no_type_for_allcaps() {
        let p = ParamDef::new("NVALOR");
        assert_eq!(p.original_name,   "NVALOR");
        assert_eq!(p.normalized_name, "NVALOR");
        assert_eq!(p.hb_type, None);
    }

    #[test]
    fn param_def_no_type_for_lowercase() {
        let p = ParamDef::new("valor");
        assert_eq!(p.hb_type, None);
    }

    #[test]
    fn symbol_kind_equality() {
        assert_eq!(SymbolKind::Function, SymbolKind::Function);
        assert_ne!(SymbolKind::Function, SymbolKind::Procedure);
    }

    #[test]
    fn visibility_equality() {
        assert_eq!(Visibility::Exported, Visibility::Exported);
        assert_ne!(Visibility::Exported, Visibility::Hidden);
    }

    #[test]
    fn manifest_empty() {
        let m = Manifest {
            source_path: "test.prg".into(),
            md5: "abc".into(),
            symbols: vec![],
            usages: vec![],
        };
        assert!(m.symbols.is_empty());
        assert!(m.usages.is_empty());
    }
}
