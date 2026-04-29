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
    /// Seed the table with all standard Harbour functions implemented in the
    /// SWed workspace (`swed_rt` + `swed_bf`), so the transpiler works
    /// out-of-the-box without a full hbdocs.json.
    pub fn with_builtins() -> Self {
        let seed_json = r#"[
          { "name": "AADD",      "returns": "A", "is_procedure": false,
            "params": [{"name": "aArray", "hb_type": "A", "optional": false},
                       {"name": "xValue", "hb_type": "U", "optional": false}]},
          { "name": "ASIZE",     "returns": "A", "is_procedure": false,
            "params": [{"name": "aArray", "hb_type": "A", "optional": false},
                       {"name": "nSize",  "hb_type": "N", "optional": false}]},
          { "name": "ASCAN",     "returns": "N", "is_procedure": false,
            "params": [{"name": "aArray", "hb_type": "A", "optional": false},
                       {"name": "xValue", "hb_type": "U", "optional": false}]},
          { "name": "AEVAL",     "returns": "A", "is_procedure": false,
            "params": [{"name": "aArray", "hb_type": "A", "optional": false},
                       {"name": "bBlock", "hb_type": "U", "optional": false}]},
          { "name": "ASORT",     "returns": "A", "is_procedure": false,
            "params": [{"name": "aArray", "hb_type": "A", "optional": false},
                       {"name": "nStart", "hb_type": "N", "optional": true},
                       {"name": "nCount", "hb_type": "N", "optional": true},
                       {"name": "bBlock", "hb_type": "U", "optional": true}]},
          { "name": "LEN",       "returns": "N", "is_procedure": false,
            "params": [{"name": "xExpr",  "hb_type": "U", "optional": false}]},
          { "name": "QOUT",      "returns": "U", "is_procedure": true,
            "params": [{"name": "xExpr",  "hb_type": "U", "optional": true}]},
          { "name": "QQOUT",     "returns": "U", "is_procedure": true,
            "params": [{"name": "xExpr",  "hb_type": "U", "optional": true}]},
          { "name": "ALLTRIM",   "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false}]},
          { "name": "LTRIM",     "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false}]},
          { "name": "RTRIM",     "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false}]},
          { "name": "TRIM",      "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false}]},
          { "name": "UPPER",     "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false}]},
          { "name": "LOWER",     "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false}]},
          { "name": "LEFT",      "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false},
                       {"name": "nLen",   "hb_type": "N", "optional": false}]},
          { "name": "RIGHT",     "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false},
                       {"name": "nLen",   "hb_type": "N", "optional": false}]},
          { "name": "SUBSTR",    "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false},
                       {"name": "nStart", "hb_type": "N", "optional": false},
                       {"name": "nLen",   "hb_type": "N", "optional": true}]},
          { "name": "AT",        "returns": "N", "is_procedure": false,
            "params": [{"name": "cSearch","hb_type": "C", "optional": false},
                       {"name": "cStr",   "hb_type": "C", "optional": false}]},
          { "name": "RAT",       "returns": "N", "is_procedure": false,
            "params": [{"name": "cSearch","hb_type": "C", "optional": false},
                       {"name": "cStr",   "hb_type": "C", "optional": false}]},
          { "name": "STRTRAN",   "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false},
                       {"name": "cSearch","hb_type": "C", "optional": false},
                       {"name": "cRepl",  "hb_type": "C", "optional": true}]},
          { "name": "SPACE",     "returns": "C", "is_procedure": false,
            "params": [{"name": "nLen",   "hb_type": "N", "optional": false}]},
          { "name": "REPLICATE", "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false},
                       {"name": "nTimes", "hb_type": "N", "optional": false}]},
          { "name": "CHR",       "returns": "C", "is_procedure": false,
            "params": [{"name": "nAscii", "hb_type": "N", "optional": false}]},
          { "name": "ASC",       "returns": "N", "is_procedure": false,
            "params": [{"name": "cChar",  "hb_type": "C", "optional": false}]},
          { "name": "PADL",      "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false},
                       {"name": "nLen",   "hb_type": "N", "optional": false},
                       {"name": "cPad",   "hb_type": "C", "optional": true}]},
          { "name": "PADR",      "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false},
                       {"name": "nLen",   "hb_type": "N", "optional": false},
                       {"name": "cPad",   "hb_type": "C", "optional": true}]},
          { "name": "PADC",      "returns": "C", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false},
                       {"name": "nLen",   "hb_type": "N", "optional": false},
                       {"name": "cPad",   "hb_type": "C", "optional": true}]},
          { "name": "STR",       "returns": "C", "is_procedure": false,
            "params": [{"name": "nNum",   "hb_type": "N", "optional": false},
                       {"name": "nLen",   "hb_type": "N", "optional": true},
                       {"name": "nDec",   "hb_type": "N", "optional": true}]},
          { "name": "STRZERO",   "returns": "C", "is_procedure": false,
            "params": [{"name": "nNum",   "hb_type": "N", "optional": false},
                       {"name": "nLen",   "hb_type": "N", "optional": false},
                       {"name": "nDec",   "hb_type": "N", "optional": true}]},
          { "name": "HB_NTOS",   "returns": "C", "is_procedure": false,
            "params": [{"name": "nNum",   "hb_type": "N", "optional": false}]},
          { "name": "VAL",       "returns": "N", "is_procedure": false,
            "params": [{"name": "cStr",   "hb_type": "C", "optional": false}]},
          { "name": "INT",       "returns": "N", "is_procedure": false,
            "params": [{"name": "nNum",   "hb_type": "N", "optional": false}]},
          { "name": "ABS",       "returns": "N", "is_procedure": false,
            "params": [{"name": "nNum",   "hb_type": "N", "optional": false}]},
          { "name": "ROUND",     "returns": "N", "is_procedure": false,
            "params": [{"name": "nNum",   "hb_type": "N", "optional": false},
                       {"name": "nDec",   "hb_type": "N", "optional": false}]},
          { "name": "MAX",       "returns": "U", "is_procedure": false,
            "params": [{"name": "xA",     "hb_type": "U", "optional": false},
                       {"name": "xB",     "hb_type": "U", "optional": false}]},
          { "name": "MIN",       "returns": "U", "is_procedure": false,
            "params": [{"name": "xA",     "hb_type": "U", "optional": false},
                       {"name": "xB",     "hb_type": "U", "optional": false}]},
          { "name": "SQRT",      "returns": "N", "is_procedure": false,
            "params": [{"name": "nNum",   "hb_type": "N", "optional": false}]},
          { "name": "TYPE",      "returns": "C", "is_procedure": false,
            "params": [{"name": "xExpr",  "hb_type": "U", "optional": false}]},
          { "name": "VALTYPE",   "returns": "C", "is_procedure": false,
            "params": [{"name": "xExpr",  "hb_type": "U", "optional": false}]},
          { "name": "EMPTY",     "returns": "L", "is_procedure": false,
            "params": [{"name": "xExpr",  "hb_type": "U", "optional": false}]},
          { "name": "ISNIL",     "returns": "L", "is_procedure": false,
            "params": [{"name": "xExpr",  "hb_type": "U", "optional": false}]},
          { "name": "IIF",       "returns": "U", "is_procedure": false,
            "params": [{"name": "lCond",  "hb_type": "L", "optional": false},
                       {"name": "xTrue",  "hb_type": "U", "optional": false},
                       {"name": "xFalse", "hb_type": "U", "optional": false}]},
          { "name": "DATE",      "returns": "D", "is_procedure": false,
            "params": []},
          { "name": "YEAR",      "returns": "N", "is_procedure": false,
            "params": [{"name": "dDate",  "hb_type": "D", "optional": false}]},
          { "name": "MONTH",     "returns": "N", "is_procedure": false,
            "params": [{"name": "dDate",  "hb_type": "D", "optional": false}]},
          { "name": "DAY",       "returns": "N", "is_procedure": false,
            "params": [{"name": "dDate",  "hb_type": "D", "optional": false}]},
          { "name": "DTOS",      "returns": "C", "is_procedure": false,
            "params": [{"name": "dDate",  "hb_type": "D", "optional": false}]},
          { "name": "DTOC",      "returns": "C", "is_procedure": false,
            "params": [{"name": "dDate",  "hb_type": "D", "optional": false}]},
          { "name": "STOD",      "returns": "D", "is_procedure": false,
            "params": [{"name": "cDate",  "hb_type": "C", "optional": false}]},
          { "name": "CTOD",      "returns": "D", "is_procedure": false,
            "params": [{"name": "cDate",  "hb_type": "C", "optional": false}]},
          { "name": "DIRECTORY", "returns": "A", "is_procedure": false,
            "params": [{"name": "cMask",  "hb_type": "C", "optional": true},
                       {"name": "cAttr",  "hb_type": "C", "optional": true}]},
          { "name": "FCREATE",   "returns": "N", "is_procedure": false,
            "params": [{"name": "cFile",  "hb_type": "C", "optional": false},
                       {"name": "nAttr",  "hb_type": "N", "optional": true}]},
          { "name": "FOPEN",     "returns": "N", "is_procedure": false,
            "params": [{"name": "cFile",  "hb_type": "C", "optional": false},
                       {"name": "nMode",  "hb_type": "N", "optional": true}]},
          { "name": "FCLOSE",    "returns": "L", "is_procedure": false,
            "params": [{"name": "nHandle","hb_type": "N", "optional": false}]},
          { "name": "FWRITE",    "returns": "N", "is_procedure": false,
            "params": [{"name": "nHandle","hb_type": "N", "optional": false},
                       {"name": "cStr",   "hb_type": "C", "optional": false},
                       {"name": "nBytes", "hb_type": "N", "optional": true}]},
          { "name": "FREAD",     "returns": "N", "is_procedure": false,
            "params": [{"name": "nHandle","hb_type": "N", "optional": false},
                       {"name": "cBuf",   "hb_type": "C", "optional": false},
                       {"name": "nBytes", "hb_type": "N", "optional": false}]},
          { "name": "FREADSTR",  "returns": "C", "is_procedure": false,
            "params": [{"name": "nHandle","hb_type": "N", "optional": false},
                       {"name": "nBytes", "hb_type": "N", "optional": false}]},
          { "name": "FSEEK",     "returns": "N", "is_procedure": false,
            "params": [{"name": "nHandle","hb_type": "N", "optional": false},
                       {"name": "nOffset","hb_type": "N", "optional": false},
                       {"name": "nOrigin","hb_type": "N", "optional": true}]},
          { "name": "FERASE",    "returns": "N", "is_procedure": false,
            "params": [{"name": "cFile",  "hb_type": "C", "optional": false}]},
          { "name": "FRENAME",   "returns": "N", "is_procedure": false,
            "params": [{"name": "cOld",   "hb_type": "C", "optional": false},
                       {"name": "cNew",   "hb_type": "C", "optional": false}]},
          { "name": "FERROR",    "returns": "N", "is_procedure": false,
            "params": []},
          { "name": "FILE",      "returns": "L", "is_procedure": false,
            "params": [{"name": "cFile",  "hb_type": "C", "optional": false}]}
        ]"#;

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
