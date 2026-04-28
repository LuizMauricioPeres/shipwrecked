//! Harbour type system — canonical definition for the entire SWed workspace.
//!
//! # Hungarian-notation inference
//!
//! Detection rule: `first_char.is_lowercase() && second_char.is_uppercase()`.
//! This identifies camelCase Hungarian (`nValor`, `cNome`) without false-positives
//! on Harbour constants (`NVALOR`) or plain names (`valor`).

/// Harbour type — used for Hungarian-notation inference, scope annotations, and
/// Rust type mapping in transpiled code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HbType {
    /// `i` — integer → `i64` (reserved; no active Hungarian prefix yet)
    Integer,
    /// `n` / `f` — numeric → `f64`.
    /// `n` is the transitional wildcard (Harbour uses f64 internally).
    Float,
    /// `c` — character string → `String`
    Character,
    /// `l` — logical (.T./.F.) → `bool`
    Logical,
    /// `d` — date → `HbValue::Date`
    Date,
    /// `a` — array → `Vec<HbValue>`
    Array,
    /// `o` — object (class instance) → `HbValue`
    Object,
    /// `h` — hash / associative array → `HbValue`
    Hash,
    /// `b` — code block (`{ || ... }`) → `HbValue`
    Block,
    /// NIL — the zero value returned by procedures and uninitialized variables.
    Nil,
    /// Untyped / Any — used by the scope resolver when the type cannot be inferred.
    Unknown,
}

impl Default for HbType {
    fn default() -> Self {
        HbType::Unknown
    }
}

impl HbType {
    /// Infer type from a Hungarian-notation identifier.
    ///
    /// Returns `None` when the rule does not apply (e.g. `NVALOR`, `valor`).
    pub fn from_hungarian(name: &str) -> Option<Self> {
        let mut chars = name.chars();
        let first = chars.next()?;
        let second = chars.next()?;

        if !first.is_lowercase() || !second.is_uppercase() {
            return None;
        }

        match first {
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

    /// Corresponding native Rust type for code generation.
    ///
    /// Types without a concrete Rust counterpart return `"HbValue"`.
    pub fn to_rust_type(&self) -> &'static str {
        match self {
            Self::Integer   => "i64",
            Self::Float     => "f64",
            Self::Character => "String",
            Self::Logical   => "bool",
            Self::Array     => "Vec<HbValue>",
            Self::Date | Self::Object | Self::Hash | Self::Block
            | Self::Nil | Self::Unknown => "HbValue",
        }
    }

    /// Single-character type code matching Harbour's `ValType()` / Hungarian prefix.
    pub fn type_char(&self) -> char {
        match self {
            Self::Integer   => 'i',
            Self::Float     => 'n',
            Self::Character => 'c',
            Self::Logical   => 'l',
            Self::Date      => 'd',
            Self::Array     => 'a',
            Self::Object    => 'o',
            Self::Hash      => 'h',
            Self::Block     => 'b',
            Self::Nil       => 'u',
            Self::Unknown   => 'u',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hungarian_all_prefixes() {
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
    fn from_hungarian_rejects_non_camel() {
        assert_eq!(HbType::from_hungarian("NVALOR"), None);
        assert_eq!(HbType::from_hungarian("valor"),  None);
        assert_eq!(HbType::from_hungarian("nvalor"), None);
        assert_eq!(HbType::from_hungarian("n"),      None);
        assert_eq!(HbType::from_hungarian(""),       None);
        assert_eq!(HbType::from_hungarian("xRaw"),   None);
    }

    #[test]
    fn to_rust_type_mapping() {
        assert_eq!(HbType::Integer.to_rust_type(),   "i64");
        assert_eq!(HbType::Float.to_rust_type(),     "f64");
        assert_eq!(HbType::Character.to_rust_type(), "String");
        assert_eq!(HbType::Logical.to_rust_type(),   "bool");
        assert_eq!(HbType::Array.to_rust_type(),     "Vec<HbValue>");
        assert_eq!(HbType::Object.to_rust_type(),    "HbValue");
        assert_eq!(HbType::Nil.to_rust_type(),       "HbValue");
        assert_eq!(HbType::Unknown.to_rust_type(),   "HbValue");
    }

    #[test]
    fn default_is_unknown() {
        assert_eq!(HbType::default(), HbType::Unknown);
    }
}
