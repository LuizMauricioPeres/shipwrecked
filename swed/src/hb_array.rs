// swed/src/hb_array.rs
// Codegen helpers for AAdd calls.
//
// HbValue and HbArray live in swed_rt. This module contains only the
// compile-time representation of AAdd arguments used during code emission.

/// Represents a Harbour `AAdd(array_name, value_expr)` call at the AST level.
pub struct AAddCall {
    /// Harbour identifier, e.g. `"aTeste"`.
    pub array_name: String,
    /// The second argument.
    pub value_expr: AAddArg,
}

/// The second argument to AAdd — each variant maps to one `HbValue` constructor.
pub enum AAddArg {
    BracketString(String), // `[Texto]`   →  HbValue::String("Texto")
    QuotedString(String),  // `"Texto"`   →  HbValue::String("Texto")
    Integer(i64),          // `42`        →  HbValue::Integer(42)
    Float(f64),            // `3.14`      →  HbValue::Float(3.14)
    Bool(bool),            // `.T.`/`.F.` →  HbValue::Logical(true/false)
    Nil,                   //  NIL        →  HbValue::Nil
    Expr(String),          // fallback: raw Rust expression string
}

impl AAddCall {
    /// Emit the Rust source line for this AAdd call.
    ///
    /// ```text
    /// // AAdd( aTeste, [Texto] )  →
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_gen_bracket_string() {
        let call = AAddCall {
            array_name: "aTeste".into(),
            value_expr: AAddArg::BracketString("Texto".into()),
        };
        assert_eq!(
            call.to_rust("a_teste"),
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
