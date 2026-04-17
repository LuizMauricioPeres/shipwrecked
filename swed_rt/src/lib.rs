// swed_rt/src/lib.rs
// Shipwrecked Runtime Library — API pública flat.

pub mod array;
pub mod builtins;
pub mod row;
pub mod unwrap;
pub mod value;
pub mod work_area;

// ── Core types ───────────────────────────────────────────────────────────────
pub use array::HbArray;
pub use value::HbValue;

// ── Row / DataNavigator ───────────────────────────────────────────────────────
pub use row::{
    DataNavigator, FieldIndex, FieldMeta,
    InMemoryTable, Row, RowProxy, RowSchema,
};

// ── Unwrap traits ─────────────────────────────────────────────────────────────
pub use unwrap::{
    IntoHbValue, ScopeStack, TryAsRust, UnwrapError,
};

// ── Work Area Manager ─────────────────────────────────────────────────────────
pub use work_area::{with_work_areas, WorkArea, WorkAreaManager};

// ── Built-in functions ────────────────────────────────────────────────────────
pub use builtins::{
    hb_abs, hb_aadd, hb_asc, hb_ascan, hb_asize,
    hb_alltrim, hb_at, hb_chr, hb_empty,
    hb_int, hb_isnil, hb_len, hb_lower, hb_ltrim,
    hb_macro, hb_max, hb_min, hb_padr, hb_padl,
    hb_qqout, hb_qout, hb_range, hb_rat,
    hb_replicate, hb_round, hb_rtrim, hb_space,
    hb_str, hb_substr, hb_upper, hb_val, hb_valtype,
};

// ── hb_instr ──────────────────────────────────────────────────────────────────
pub fn hb_instr(needle: HbValue, haystack: HbValue) -> HbValue {
    needle.hb_instr_contains(&haystack)
}

// ── hb_array! macro ───────────────────────────────────────────────────────────
#[macro_export]
macro_rules! hb_array {
    ( $($e:expr),* $(,)? ) => {{
        let mut __a = $crate::HbArray::new();
        $( __a.hb_aadd($e); )*
        __a
    }};
}

// ── Integration tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hb_array_macro() {
        let a = hb_array![HbValue::Integer(1), HbValue::String("two".into())];
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn test_hb_instr() {
        assert_eq!(
            hb_instr(HbValue::String("lo".into()), HbValue::String("Hello".into())),
            HbValue::Logical(true)
        );
    }

    #[test]
    fn test_full_row_pipeline() {
        // Schema → RowProxy → FieldIndex → O(1) access
        let schema = RowSchema::new(vec![
            FieldMeta { name: "NOME".into(), dbf_type: 'C', size: 40, decimals: 0 },
            FieldMeta { name: "SALDO".into(), dbf_type: 'N', size: 12, decimals: 2 },
        ]);
        let idx_saldo = schema.resolve("SALDO").unwrap();

        let mut table = InMemoryTable::new(schema, vec![]);
        table.push_row(vec![HbValue::String("Alice".into()), HbValue::Float(9500.0)]);

        let row = table.current_row().unwrap();
        assert_eq!(row.get(idx_saldo.0), Some(&HbValue::Float(9500.0)));
    }

    #[test]
    fn test_scope_and_unwrap_integration() {
        let scope = ScopeStack::new();
        scope.inject("NVAL", HbValue::Float(42.5));

        let v = scope.get("NVAL");
        let f: f64 = v.try_as_f64().unwrap();
        assert_eq!(f, 42.5);

        let result = (f * 2.0).into_hb();
        assert_eq!(result, HbValue::Float(85.0));
    }

    #[test]
    fn test_work_area_integration() {
        with_work_areas(|wam| {
            let schema = RowSchema::new(vec![
                FieldMeta { name: "COD".into(), dbf_type: 'C', size: 10, decimals: 0 },
            ]);
            let mut table = InMemoryTable::new(schema, vec![]);
            table.push_row(vec![HbValue::String("001".into())]);
            wam.open("TEST", Box::new(table));
            assert_eq!(wam.field("TEST", "COD"), HbValue::String("001".into()));
        });
    }
}
