//! Harbour type-inspection functions: Type() / ValType().

use swed_co::NativeFunction;
use swed_rt::HbValue;

// ---------------------------------------------------------------------------
// Type() / ValType()
// ---------------------------------------------------------------------------

/// `TYPE(x)` / `VALTYPE(x)` — returns the Harbour type character.
///
/// | Return | Harbour type      |
/// |--------|-------------------|
/// | `"C"`  | Character string  |
/// | `"N"`  | Numeric           |
/// | `"L"`  | Logical           |
/// | `"D"`  | Date              |
/// | `"A"`  | Array             |
/// | `"U"`  | NIL / undefined   |
pub fn hb_type(val: HbValue) -> HbValue {
    HbValue::String(val.val_type().to_owned())
}

// ---------------------------------------------------------------------------
// NativeFunction implementation
// ---------------------------------------------------------------------------

/// Registry struct for `TYPE` / `VALTYPE`.
pub struct Type;
impl NativeFunction<HbValue> for Type {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_type(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "TYPE" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_character() {
        assert_eq!(hb_type(HbValue::String("hi".into())), HbValue::String("C".into()));
    }

    #[test]
    fn type_numeric_integer() {
        assert_eq!(hb_type(HbValue::Integer(42)), HbValue::String("N".into()));
    }

    #[test]
    fn type_numeric_float() {
        assert_eq!(hb_type(HbValue::Float(3.14)), HbValue::String("N".into()));
    }

    #[test]
    fn type_logical() {
        assert_eq!(hb_type(HbValue::Logical(true)), HbValue::String("L".into()));
    }

    #[test]
    fn type_date() {
        assert_eq!(hb_type(HbValue::Date(0)), HbValue::String("D".into()));
    }

    #[test]
    fn type_nil() {
        assert_eq!(hb_type(HbValue::Nil), HbValue::String("U".into()));
    }

    #[test]
    fn native_fn_trait_type() {
        let r = Type.call(vec![HbValue::Integer(1)]);
        assert_eq!(r, HbValue::String("N".into()));
        assert_eq!(Type.name(), "TYPE");
        assert_eq!(Type.arity(), (1, 1));
    }
}
