//! Tipos centrais do manifesto de símbolos Harbour (.mkh).

// ── Hungarian-notation type inference ─────────────────────────────────────────

/// Tipo Harbour inferido via prefixo de notação húngara personalizada.
///
/// **Regra de detecção:** `first_char.is_lowercase() && second_char.is_uppercase()`.
/// Isso identifica camelCase húngaro (`nValor`, `cNome`) sem falso-positivo em
/// constantes Harbour (`NVALOR`) ou nomes sem convenção (`valor`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HbType {
    /// `i` — inteiro  →  `i64`  (reservado; ainda sem prefixo ativo)
    Integer,
    /// `n` / `f` — numérico  →  `f64`
    /// `n` é coringa transitório (Harbour usa f64 internamente).
    /// Futuramente `n` será substituído por `i` (inteiro) e `f` (float).
    Float,
    /// `c` — string de caracteres  →  `String`
    Character,
    /// `l` — lógico (.T./.F.)  →  `bool`
    Logical,
    /// `d` — data  →  `HbValue` (até `HbDate` ser estabilizado)
    Date,
    /// `a` — array  →  `Vec<HbValue>`
    Array,
    /// `o` — objeto (instância de classe)  →  `HbValue`
    Object,
    /// `h` — hash / associative array  →  `HbValue`
    Hash,
    /// `b` — bloco de código (`{ || ... }`)  →  `HbValue`
    Block,
    /// `u` — qualquer / NIL-safe  →  `HbValue`
    Unknown,
}

impl HbType {
    /// Detecta Hungarian notation e infere o tipo.
    ///
    /// Regra: `first.is_lowercase() && second.is_uppercase()`.
    /// Retorna `None` se o nome não satisfizer a regra (ex.: `NVALOR`, `valor`).
    pub fn from_hungarian(name: &str) -> Option<Self> {
        let mut chars = name.chars();
        let first = chars.next()?;
        let second = chars.next()?;

        // Apenas camelCase húngaro: primeiro minúsculo, segundo maiúsculo
        if !first.is_lowercase() || !second.is_uppercase() {
            return None;
        }

        match first {
            // 'n' é coringa numérico → f64 (Harbour usa f64 internamente).
            // Futuramente: 'i' → Integer (i64), 'f' → Float (f64), 'n' deprecated.
            'n' | 'f' => Some(Self::Float),
            'c' => Some(Self::Character),
            'l' => Some(Self::Logical),
            'd' => Some(Self::Date),
            'a' => Some(Self::Array),
            'o' => Some(Self::Object),
            'h' => Some(Self::Hash),
            'b' => Some(Self::Block),
            'u' => Some(Self::Unknown),
            _ => None,
        }
    }

    /// Tipo nativo Rust correspondente.
    ///
    /// Tipos sem mapeamento direto retornam `"HbValue"` — serão refinados
    /// conforme os tipos concretos (`HbDate`, etc.) forem estabilizados.
    pub fn to_rust_type(&self) -> &'static str {
        match self {
            Self::Integer   => "i64",
            Self::Float     => "f64",
            Self::Character => "String",
            Self::Logical   => "bool",
            Self::Array     => "Vec<HbValue>",
            // ainda sem tipo nativo concreto:
            Self::Date | Self::Object | Self::Hash | Self::Block | Self::Unknown => "HbValue",
        }
    }
}

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
    fn hungarian_camel_case_rule() {
        // Detecta: primeiro minúsculo + segundo maiúsculo
        // n e f são ambos coringa numérico → Float (f64) até a divisão i/f
        assert_eq!(HbType::from_hungarian("nValor"),  Some(HbType::Float));
        assert_eq!(HbType::from_hungarian("fPreco"),  Some(HbType::Float));
        assert_eq!(HbType::from_hungarian("cNome"),   Some(HbType::Character));
        assert_eq!(HbType::from_hungarian("lAtivo"),  Some(HbType::Logical));
        assert_eq!(HbType::from_hungarian("dNasc"),   Some(HbType::Date));
        assert_eq!(HbType::from_hungarian("aItens"),  Some(HbType::Array));
        assert_eq!(HbType::from_hungarian("oUser"),   Some(HbType::Object));
        assert_eq!(HbType::from_hungarian("hConfig"), Some(HbType::Hash));
        assert_eq!(HbType::from_hungarian("bAction"), Some(HbType::Block));
        assert_eq!(HbType::from_hungarian("uVal"),    Some(HbType::Unknown));
    }

    #[test]
    fn hungarian_rejects_non_camel() {
        // NVALOR: primeiro maiúsculo → nega
        assert_eq!(HbType::from_hungarian("NVALOR"), None);
        // valor: segundo não é maiúsculo → nega
        assert_eq!(HbType::from_hungarian("valor"), None);
        // nvalor: todos minúsculos → nega
        assert_eq!(HbType::from_hungarian("nvalor"), None);
        // n: só um char → nega (sem segundo)
        assert_eq!(HbType::from_hungarian("n"), None);
        assert_eq!(HbType::from_hungarian(""), None);
        // prefixo não mapeado, mas camelCase válido → nega
        assert_eq!(HbType::from_hungarian("xRaw"), None);
    }

    #[test]
    fn hb_type_to_rust_type() {
        assert_eq!(HbType::Integer.to_rust_type(),   "i64");
        assert_eq!(HbType::Float.to_rust_type(),     "f64");
        assert_eq!(HbType::Character.to_rust_type(), "String");
        assert_eq!(HbType::Logical.to_rust_type(),   "bool");
        assert_eq!(HbType::Array.to_rust_type(),     "Vec<HbValue>");
        assert_eq!(HbType::Object.to_rust_type(),    "HbValue");
        assert_eq!(HbType::Date.to_rust_type(),      "HbValue");
    }

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
