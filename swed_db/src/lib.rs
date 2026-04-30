//! `swed_db` — Replaceable Database Driver (RDD) layer for SWed.
//!
//! Encapsula todo o acesso DBF/xBase e (futuramente) SQL.
//! Depende de `swed_rt` para `HbValue` e `HbArray`; não depende
//! dos crates do compilador — seguro para produção.
//!
//! # Estrutura
//!
//! - [`dbf`] — driver dBase III/IV: I/O de arquivo, esquema de linha, work areas
//! - [`sql`] — stub para driver SQL (Sprint 4+)

pub mod dbf;
pub mod sql;

// ── Re-exports públicos ──────────────────────────────────────────────────────

pub use dbf::{
    // DataNavigator + abstrações de linha
    DataNavigator, FieldIndex, FieldMeta, InMemoryTable, Row, RowProxy, RowSchema,
    // Work area
    with_work_areas, WorkArea, WorkAreaManager,
    // DBF I/O
    DbfError, DbfField, DbfHeader, DbfNavigator, DbfReader, FieldValue,
};

// ── Helpers de campo gerados pelo transpilador ───────────────────────────────

use swed_rt::HbValue;

/// Lê campo da área de trabalho atualmente selecionada.
/// Gerado pelo transpilador para cada variável declarada com FIELD.
pub fn field_get(name: &str) -> HbValue {
    with_work_areas(|wam| wam.field_current(name))
}

/// Escreve campo da área de trabalho atualmente selecionada.
pub fn field_set(name: &str, val: HbValue) {
    with_work_areas(|wam| wam.field_set_current(name, val));
}

/// Lê campo de uma área específica (ALIAS->FIELD).
pub fn field_get_alias(alias: &str, name: &str) -> HbValue {
    with_work_areas(|wam| wam.field(alias, name))
}

/// Escreve campo de uma área específica.
pub fn field_set_alias(alias: &str, name: &str, val: HbValue) {
    with_work_areas(|wam| wam.field_set(alias, name, val));
}

// ── Integration tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_row_pipeline() {
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
    fn test_work_area_integration() {
        with_work_areas(|wam| {
            let schema = RowSchema::new(vec![
                FieldMeta { name: "COD".into(), dbf_type: 'C', size: 10, decimals: 0 },
            ]);
            let mut table = InMemoryTable::new(schema, vec![]);
            table.push_row(vec![HbValue::String("001".into())]);
            wam.open("TEST2", Box::new(table));
            assert_eq!(wam.field("TEST2", "COD"), HbValue::String("001".into()));
        });
    }

    #[test]
    fn test_field_helpers() {
        with_work_areas(|wam| {
            let schema = RowSchema::new(vec![
                FieldMeta { name: "X".into(), dbf_type: 'C', size: 1, decimals: 0 },
            ]);
            let mut table = InMemoryTable::new(schema, vec![]);
            table.push_row(vec![HbValue::String("A".into())]);
            wam.open("FTEST", Box::new(table));
            wam.select("FTEST");
        });
        assert_eq!(field_get("X"), HbValue::String("A".into()));
    }
}
