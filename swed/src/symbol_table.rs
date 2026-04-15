// swed/src/symbol_table.rs
// Symbol Table — loads hbdocs.json and validates function call signatures
// during the semantic analysis pass.

use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;
use thiserror::Error;

// ---------------------------------------------------------------------------
// JSON schema for hbdocs.json
// ---------------------------------------------------------------------------

/// One parameter descriptor from the docs.
#[derive(Debug, Deserialize, Clone)]
pub struct ParamDef {
    pub name: String,
    /// Harbour type hint: "A" = Array, "C" = String, "N" = Numeric,
    /// "L" = Logical, "O" = Object, "D" = Date, "U" = Undefined/Any
    pub hb_type: String,
    #[serde(default)]
    pub optional: bool,
}

/// One function/procedure signature from the docs.
#[derive(Debug, Deserialize, Clone)]
pub struct FunctionSig {
    /// Canonical uppercase name (e.g. "AADD")
    pub name: String,
    pub params: Vec<ParamDef>,
    /// Return type hint
    pub returns: String,
    #[serde(default)]
    pub is_procedure: bool,
}

// ---------------------------------------------------------------------------
// SymbolTable
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct SymbolTable {
    /// Built-in Harbour functions loaded from hbdocs.json.
    /// Keys are UPPERCASE function names.
    builtins: HashMap<String, FunctionSig>,

    /// User-defined symbols discovered during parsing.
    user_symbols: HashMap<String, UserSymbol>,
}

#[derive(Debug, Clone)]
pub struct UserSymbol {
    pub kind: SymbolKind,
    pub scope: ScopeLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Procedure,
    Class,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeLevel {
    Local,
    Static,
    Public,
    Private, // MEMVAR / PRIVATE
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum SymbolError {
    #[error("IO error loading hbdocs: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error in hbdocs: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Unknown function `{0}`")]
    UnknownFunction(String),

    #[error("Arity mismatch for `{name}`: expected {expected} args, got {got}")]
    ArityMismatch {
        name: String,
        expected: usize,
        got: usize,
    },
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl SymbolTable {
    /// Create an empty table (no hbdocs loaded).
    pub fn new() -> Self {
        Self::default()
    }

    /// Load function signatures from `hbdocs.json`.
    /// The file must contain a JSON array of `FunctionSig` objects.
    pub fn load_hbdocs<P: AsRef<Path>>(path: P) -> Result<Self, SymbolError> {
        let content = fs::read_to_string(path)?;
        let sigs: Vec<FunctionSig> = serde_json::from_str(&content)?;

        let builtins = sigs
            .into_iter()
            .map(|sig| (sig.name.to_ascii_uppercase(), sig))
            .collect();

        Ok(Self {
            builtins,
            user_symbols: HashMap::new(),
        })
    }

    /// Validate a function call: existence + arity.
    /// `name` will be normalized to uppercase internally.
    pub fn validate_call(
        &self,
        name: &str,
        arg_count: usize,
    ) -> Result<&FunctionSig, SymbolError> {
        let key = name.to_ascii_uppercase();
        let sig = self
            .builtins
            .get(&key)
            .ok_or_else(|| SymbolError::UnknownFunction(key.clone()))?;

        let required = sig.params.iter().filter(|p| !p.optional).count();
        let total = sig.params.len();

        if arg_count < required || arg_count > total {
            return Err(SymbolError::ArityMismatch {
                name: key,
                expected: required,
                got: arg_count,
            });
        }

        Ok(sig)
    }

    /// Register a user-defined symbol (discovered while parsing).
    pub fn define(&mut self, name: &str, kind: SymbolKind, scope: ScopeLevel) {
        self.user_symbols.insert(
            name.to_ascii_uppercase(),
            UserSymbol { kind, scope },
        );
    }

    /// Look up a user symbol (case-insensitive).
    pub fn lookup(&self, name: &str) -> Option<&UserSymbol> {
        self.user_symbols.get(&name.to_ascii_uppercase())
    }
}

// ---------------------------------------------------------------------------
// Minimal built-in seed (used when hbdocs.json is absent)
// ---------------------------------------------------------------------------

impl SymbolTable {
    /// Seed the table with the most common Harbour array/string functions
    /// so the transpiler works out-of-the-box without a full hbdocs.json.
    pub fn with_builtins() -> Self {
        let seed_json = r#"
        [
          { "name": "AADD",   "returns": "A", "is_procedure": false,
            "params": [
              {"name": "aArray",  "hb_type": "A", "optional": false},
              {"name": "xValue",  "hb_type": "U", "optional": false}
            ]},
          { "name": "ASIZE",  "returns": "A", "is_procedure": false,
            "params": [
              {"name": "aArray",  "hb_type": "A", "optional": false},
              {"name": "nSize",   "hb_type": "N", "optional": false}
            ]},
          { "name": "LEN",    "returns": "N", "is_procedure": false,
            "params": [
              {"name": "xExpr",   "hb_type": "U", "optional": false}
            ]},
          { "name": "QOUT",   "returns": "U", "is_procedure": true,
            "params": [
              {"name": "xExpr",   "hb_type": "U", "optional": true}
            ]},
          { "name": "ALLTRIM","returns": "C", "is_procedure": false,
            "params": [
              {"name": "cStr",    "hb_type": "C", "optional": false}
            ]}
        ]
        "#;

        let sigs: Vec<FunctionSig> = serde_json::from_str(seed_json).unwrap();
        let builtins = sigs
            .into_iter()
            .map(|s| (s.name.clone(), s))
            .collect();

        Self {
            builtins,
            user_symbols: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> SymbolTable {
        SymbolTable::with_builtins()
    }

    #[test]
    fn test_aadd_valid() {
        assert!(table().validate_call("AAdd", 2).is_ok());
    }

    #[test]
    fn test_aadd_arity_error() {
        assert!(matches!(
            table().validate_call("AAdd", 3),
            Err(SymbolError::ArityMismatch { .. })
        ));
    }

    #[test]
    fn test_unknown_fn() {
        assert!(matches!(
            table().validate_call("Xpto", 1),
            Err(SymbolError::UnknownFunction(_))
        ));
    }

    #[test]
    fn test_case_insensitive_lookup() {
        assert!(table().validate_call("aadd", 2).is_ok());
        assert!(table().validate_call("AADD", 2).is_ok());
    }
}
