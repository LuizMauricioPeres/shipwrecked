// swed/src/hb_array.rs
// HbArray — safe Rust wrapper for Harbour's 1-indexed dynamic arrays.
//
// Transpilation example:
//   Harbour:  AAdd( aTeste, [Texto] )
//   Rust:     a_teste.push(HbValue::String("Texto".into()))
//             (or the typed helper: a_teste.hb_aadd(HbValue::String(...)))

use std::fmt;

// ---------------------------------------------------------------------------
// HbValue — the Harbour "any" type (simplified for this skeleton)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum HbValue {
    Nil,
    Logical(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(HbArray),
}

impl fmt::Display for HbValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HbValue::Nil => write!(f, "NIL"),
            HbValue::Logical(b) => write!(f, "{}", if *b { ".T." } else { ".F." }),
            HbValue::Integer(n) => write!(f, "{n}"),
            HbValue::Float(n) => write!(f, "{n}"),
            HbValue::String(s) => write!(f, "{s}"),
            HbValue::Array(a) => write!(f, "Array({} items)", a.len()),
        }
    }
}

// ---------------------------------------------------------------------------
// HbArray — 1-indexed wrapper around Vec<HbValue>
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HbArray(Vec<HbValue>);

impl HbArray {
    pub fn new() -> Self {
        HbArray(Vec::new())
    }

    pub fn with_capacity(n: usize) -> Self {
        HbArray(Vec::with_capacity(n))
    }

    /// Harbour LEN( aArray ) — number of elements.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // ------------------------------------------------------------------
    // AAdd( aArray, xValue ) → appends and returns the array
    // ------------------------------------------------------------------

    /// Transpilation target for `AAdd( aTeste, [Texto] )`:
    ///
    /// ```rust
    /// a_teste.hb_aadd(HbValue::String("Texto".into()));
    /// ```
    ///
    /// Returns `&mut Self` for chaining (mirrors Harbour returning the array).
    pub fn hb_aadd(&mut self, value: HbValue) -> &mut Self {
        self.0.push(value);
        self
    }

    // ------------------------------------------------------------------
    // 1-indexed access — mirrors Harbour semantics
    // ------------------------------------------------------------------

    /// `aArray[n]` in Harbour (1-based).  Returns `None` for out-of-bounds.
    pub fn hb_get(&self, index: usize) -> Option<&HbValue> {
        if index == 0 {
            return None; // Harbour arrays start at 1
        }
        self.0.get(index - 1)
    }

    /// Mutable 1-indexed access.
    pub fn hb_get_mut(&mut self, index: usize) -> Option<&mut HbValue> {
        if index == 0 {
            return None;
        }
        self.0.get_mut(index - 1)
    }

    /// `aArray[n] := value` — panics on out-of-bounds, matching Harbour runtime.
    pub fn hb_set(&mut self, index: usize, value: HbValue) {
        assert!(index >= 1 && index <= self.0.len(),
            "Array index {index} out of bounds (array has {} elements)", self.0.len());
        self.0[index - 1] = value;
    }

    // ------------------------------------------------------------------
    // ASize( aArray, nSize ) — resize (truncate or grow with Nil)
    // ------------------------------------------------------------------
    pub fn hb_asize(&mut self, new_size: usize) {
        if new_size < self.0.len() {
            self.0.truncate(new_size);
        } else {
            self.0.resize(new_size, HbValue::Nil);
        }
    }

    // ------------------------------------------------------------------
    // Iterator convenience
    // ------------------------------------------------------------------
    pub fn iter(&self) -> impl Iterator<Item = &HbValue> {
        self.0.iter()
    }
}

// ---------------------------------------------------------------------------
// Code-generation helpers
// ---------------------------------------------------------------------------

/// Represents a Harbour `AAdd(array_name, value_expr)` call at the AST level.
pub struct AAddCall {
    pub array_name: String,  // Harbour identifier, e.g. "aTeste"
    pub value_expr: AAddArg, // the second argument
}

/// The second argument to AAdd can be a bracket-string, a quoted string,
/// a numeric literal, or an arbitrary expression (represented as raw source).
pub enum AAddArg {
    BracketString(String), // `[Texto]`   →  HbValue::String("Texto")
    QuotedString(String),  // `"Texto"`   →  HbValue::String("Texto")
    Integer(i64),          // `42`        →  HbValue::Integer(42)
    Float(f64),            // `3.14`      →  HbValue::Float(3.14)
    Bool(bool),            // `.T.`/`.F.` →  HbValue::Logical(true/false)
    Nil,                   // `NIL`       →  HbValue::Nil
    Expr(String),          // fallback: raw Rust expression string
}

impl AAddCall {
    /// Emit the Rust source for this AAdd call.
    ///
    /// Example output for `AAdd( aTeste, [Texto] )`:
    /// ```text
    /// a_teste.hb_aadd(HbValue::String("Texto".into()));
    /// ```
    pub fn to_rust(&self, rust_array_ident: &str) -> String {
        let value = match &self.value_expr {
            AAddArg::BracketString(s) | AAddArg::QuotedString(s) => {
                format!("HbValue::String(\"{}\".into())", s.replace('"', "\\\""))
            }
            AAddArg::Integer(n) => format!("HbValue::Integer({n})"),
            AAddArg::Float(f)   => format!("HbValue::Float({f})"),
            AAddArg::Bool(b)    => format!("HbValue::Logical({b})"),
            AAddArg::Nil        => "HbValue::Nil".to_string(),
            AAddArg::Expr(e)    => e.clone(),
        };
        format!("{rust_array_ident}.hb_aadd({value});")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aadd_and_len() {
        let mut arr = HbArray::new();
        arr.hb_aadd(HbValue::String("Texto".into()));
        arr.hb_aadd(HbValue::Integer(42));
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_1_indexed_get() {
        let mut arr = HbArray::new();
        arr.hb_aadd(HbValue::String("first".into()));
        assert_eq!(arr.hb_get(1), Some(&HbValue::String("first".into())));
        assert_eq!(arr.hb_get(0), None);  // 0 is invalid in Harbour
    }

    #[test]
    fn test_asize_grow_with_nil() {
        let mut arr = HbArray::new();
        arr.hb_aadd(HbValue::Integer(1));
        arr.hb_asize(3);
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.hb_get(2), Some(&HbValue::Nil));
    }

    #[test]
    fn test_code_gen_bracket_string() {
        let call = AAddCall {
            array_name: "aTeste".into(),
            value_expr: AAddArg::BracketString("Texto".into()),
        };
        let rust_code = call.to_rust("a_teste");
        assert_eq!(
            rust_code,
            r#"a_teste.hb_aadd(HbValue::String("Texto".into()));"#
        );
    }

    #[test]
    fn test_code_gen_nil() {
        let call = AAddCall {
            array_name: "aBuf".into(),
            value_expr: AAddArg::Nil,
        };
        assert_eq!(call.to_rust("a_buf"), "a_buf.hb_aadd(HbValue::Nil);");
    }
}
