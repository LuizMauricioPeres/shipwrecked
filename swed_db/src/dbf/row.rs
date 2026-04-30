// swed_db/src/dbf/row.rs
#![allow(missing_docs)]
// RowSchema + RowProxy — indexed field access for DBF/SQL/in-memory rows.
//
// O problema central do xBase em Rust:
//   CUST->NAME  →  work_area.get("NAME")  →  HashMap lookup  →  O(log n) / hash
//
// Solução: o mapeamento nominal→numérico acontece UMA VEZ na abertura da tabela.
// Todo acesso posterior em loop/Browse usa offset usize → Vec index → O(1).
//
//   ┌─────────────────────────────────────────────┐
//   │  RowSchema  (alocado uma vez por tabela)     │
//   │  "NAME" → 0,  "SALARY" → 1,  "ACTIVE" → 2  │
//   └─────────────┬───────────────────────────────┘
//                 │  resolve() retorna FieldIndex (usize)
//                 ▼
//   ┌─────────────────────────────────────────────┐
//   │  RowProxy  (referência para uma linha)       │
//   │  row[0]  row[1]  row[2]   → &HbValue         │
//   └─────────────────────────────────────────────┘

use std::collections::HashMap;
use std::sync::Arc;
use swed_rt::HbValue;

// ---------------------------------------------------------------------------
// FieldIndex — newtype para evitar confusão com índices genéricos
// ---------------------------------------------------------------------------

/// Índice resolvido de um campo. Opaco para o código xBase,
/// armazenado em variáveis LOCAL após a primeira resolução.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldIndex(pub usize);

// ---------------------------------------------------------------------------
// FieldMeta — metadados por campo
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FieldMeta {
    /// Nome canônico UPPERCASE (ex: "SALARY")
    pub name: String,
    /// Tipo DBF: C / N / L / D / M / B (memo/blob)
    pub dbf_type: char,
    /// Tamanho declarado no cabeçalho DBF
    pub size: u16,
    /// Casas decimais (para tipo N)
    pub decimals: u8,
}

// ---------------------------------------------------------------------------
// RowSchema — construído na abertura da tabela, compartilhado via Arc
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct RowSchema {
    /// Mapeamento nome → índice (lookup único na abertura)
    index_map: HashMap<String, FieldIndex>,
    /// Metadados ordenados pelo índice físico
    fields: Vec<FieldMeta>,
}

impl RowSchema {
    /// Constrói o schema a partir de uma lista ordenada de campos.
    /// Chamado uma vez na abertura do arquivo DBF/SQL.
    pub fn new(fields: Vec<FieldMeta>) -> Self {
        let index_map = fields
            .iter()
            .enumerate()
            .map(|(i, f)| (f.name.to_ascii_uppercase(), FieldIndex(i)))
            .collect();
        RowSchema { index_map, fields }
    }

    /// Resolve nome → índice. Retorna None se o campo não existe.
    pub fn resolve(&self, name: &str) -> Option<FieldIndex> {
        self.index_map.get(&name.to_ascii_uppercase()).copied()
    }

    /// Resolve com pânico — para uso em código onde o campo é garantido
    /// (ex: gerado pelo transpilador após validação semântica).
    pub fn resolve_or_panic(&self, name: &str) -> FieldIndex {
        self.resolve(name)
            .unwrap_or_else(|| panic!("Campo `{name}` não existe no schema"))
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    pub fn field_meta(&self, idx: FieldIndex) -> Option<&FieldMeta> {
        self.fields.get(idx.0)
    }

    pub fn fields(&self) -> &[FieldMeta] {
        &self.fields
    }
}

// ---------------------------------------------------------------------------
// Row — linha de dados bruta (Vec alocado pelo driver DBF/SQL/mem)
// ---------------------------------------------------------------------------

pub type Row = Vec<HbValue>;

// ---------------------------------------------------------------------------
// RowProxy — acesso indexado a uma linha com referência ao schema
// ---------------------------------------------------------------------------

/// Proxy de leitura/escrita para uma linha.
pub struct RowProxy<'a> {
    schema: Arc<RowSchema>,
    row: &'a mut Row,
}

impl<'a> RowProxy<'a> {
    pub fn new(schema: Arc<RowSchema>, row: &'a mut Row) -> Self {
        RowProxy { schema, row }
    }

    // ── Acesso por FieldIndex (O(1) — caminho hot) ────────────────────────

    #[inline(always)]
    pub fn get(&self, idx: FieldIndex) -> &HbValue {
        self.row.get(idx.0).unwrap_or(&HbValue::Nil)
    }

    #[inline(always)]
    pub fn set(&mut self, idx: FieldIndex, value: HbValue) {
        if let Some(slot) = self.row.get_mut(idx.0) {
            *slot = value;
        }
    }

    // ── Acesso por nome (com resolve — use apenas fora de loops) ──────────

    pub fn get_by_name(&self, name: &str) -> &HbValue {
        match self.schema.resolve(name) {
            Some(idx) => self.get(idx),
            None => &HbValue::Nil,
        }
    }

    pub fn set_by_name(&mut self, name: &str, value: HbValue) {
        if let Some(idx) = self.schema.resolve(name) {
            self.set(idx, value);
        }
    }

    // ── Iteração ──────────────────────────────────────────────────────────

    pub fn iter(&self) -> impl Iterator<Item = (&FieldMeta, &HbValue)> {
        self.schema.fields().iter().zip(self.row.iter())
    }

    pub fn schema(&self) -> &Arc<RowSchema> {
        &self.schema
    }
}

// ---------------------------------------------------------------------------
// DataNavigator trait — contrato para Browse reativo
// ---------------------------------------------------------------------------

pub trait DataNavigator: Send + Sync {
    /// Schema da fonte atual (compartilhado via Arc — zero cópia).
    fn schema(&self) -> Arc<RowSchema>;

    /// Total de registros (pode ser aproximado para fontes remotas).
    fn record_count(&self) -> usize;

    /// Posição atual (0-based).
    fn current_position(&self) -> usize;

    /// Move para posição absoluta. Retorna false se fora dos limites.
    fn goto(&mut self, pos: usize) -> bool;

    /// Avança um registro. Retorna false no EOF.
    fn skip(&mut self, n: i64) -> bool;

    /// Lê a linha atual como Row clonado.
    fn current_row(&self) -> Option<Row>;

    /// Lê a linha atual como RowProxy mutável.
    fn current_row_mut(&mut self) -> Option<RowProxy<'_>>;

    /// Persiste modificações na linha atual (flush para DBF/SQL).
    fn commit(&mut self) -> Result<(), String>;

    /// Verifica se está no início.
    fn is_bof(&self) -> bool { self.current_position() == 0 }

    /// Verifica se está além do último registro.
    fn is_eof(&self) -> bool { self.current_position() >= self.record_count() }
}

// ---------------------------------------------------------------------------
// InMemoryTable — implementação de DataNavigator para testes e staging
// ---------------------------------------------------------------------------

pub struct InMemoryTable {
    schema: Arc<RowSchema>,
    rows: Vec<Row>,
    pos: usize,
}

impl InMemoryTable {
    pub fn new(schema: RowSchema, rows: Vec<Row>) -> Self {
        InMemoryTable {
            schema: Arc::new(schema),
            rows,
            pos: 0,
        }
    }

    pub fn push_row(&mut self, row: Row) {
        self.rows.push(row);
    }
}

impl DataNavigator for InMemoryTable {
    fn schema(&self) -> Arc<RowSchema> { Arc::clone(&self.schema) }
    fn record_count(&self) -> usize    { self.rows.len() }
    fn current_position(&self) -> usize { self.pos }

    fn goto(&mut self, pos: usize) -> bool {
        if pos < self.rows.len() {
            self.pos = pos;
            true
        } else {
            self.pos = self.rows.len();
            false
        }
    }

    fn skip(&mut self, n: i64) -> bool {
        let next = self.pos as i64 + n;
        self.goto(next.max(0) as usize)
    }

    fn current_row(&self) -> Option<Row> {
        self.rows.get(self.pos).cloned()
    }

    fn current_row_mut(&mut self) -> Option<RowProxy<'_>> {
        let schema = Arc::clone(&self.schema);
        self.rows.get_mut(self.pos).map(|r| RowProxy::new(schema, r))
    }

    fn commit(&mut self) -> Result<(), String> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_schema() -> RowSchema {
        RowSchema::new(vec![
            FieldMeta { name: "NAME".into(),   dbf_type: 'C', size: 40, decimals: 0 },
            FieldMeta { name: "SALARY".into(), dbf_type: 'N', size: 10, decimals: 2 },
            FieldMeta { name: "ACTIVE".into(), dbf_type: 'L', size: 1,  decimals: 0 },
        ])
    }

    #[test]
    fn test_schema_resolve() {
        let schema = make_schema();
        assert_eq!(schema.resolve("NAME"),   Some(FieldIndex(0)));
        assert_eq!(schema.resolve("SALARY"), Some(FieldIndex(1)));
        assert_eq!(schema.resolve("name"),   Some(FieldIndex(0))); // case-insensitive
        assert_eq!(schema.resolve("GHOST"),  None);
    }

    #[test]
    fn test_row_proxy_indexed_access() {
        let schema = Arc::new(make_schema());
        let mut row: Row = vec![
            HbValue::String("Alice".into()),
            HbValue::Float(9500.0),
            HbValue::Logical(true),
        ];
        let proxy = RowProxy::new(Arc::clone(&schema), &mut row);

        let idx_name   = schema.resolve("NAME").unwrap();
        let idx_salary = schema.resolve("SALARY").unwrap();

        assert_eq!(proxy.get(idx_name),   &HbValue::String("Alice".into()));
        assert_eq!(proxy.get(idx_salary), &HbValue::Float(9500.0));
    }

    #[test]
    fn test_row_proxy_set() {
        let schema = Arc::new(make_schema());
        let mut row: Row = vec![HbValue::Nil, HbValue::Nil, HbValue::Nil];
        let mut proxy = RowProxy::new(Arc::clone(&schema), &mut row);

        let idx = schema.resolve("SALARY").unwrap();
        proxy.set(idx, HbValue::Float(12000.0));
        assert_eq!(proxy.get(idx), &HbValue::Float(12000.0));
    }

    #[test]
    fn test_in_memory_table_navigation() {
        let schema = make_schema();
        let mut table = InMemoryTable::new(schema, vec![]);
        table.push_row(vec![HbValue::String("Alice".into()), HbValue::Float(9500.0), HbValue::Logical(true)]);
        table.push_row(vec![HbValue::String("Bob".into()),   HbValue::Float(8200.0), HbValue::Logical(false)]);

        assert_eq!(table.record_count(), 2);
        assert_eq!(table.current_position(), 0);

        let row = table.current_row().unwrap();
        assert_eq!(row[0], HbValue::String("Alice".into()));

        assert!(table.skip(1));
        let row2 = table.current_row().unwrap();
        assert_eq!(row2[0], HbValue::String("Bob".into()));

        assert!(!table.skip(1)); // EOF
        assert!(table.is_eof());
    }

    #[test]
    fn test_schema_resolve_caches_index_pattern() {
        let schema = Arc::new(make_schema());
        let idx_salary = schema.resolve("SALARY").unwrap();

        let mut rows: Vec<Row> = (0..1000)
            .map(|i| vec![
                HbValue::String(format!("emp_{i}")),
                HbValue::Float(i as f64 * 100.0),
                HbValue::Logical(true),
            ])
            .collect();

        let mut total = 0.0f64;
        for row in &mut rows {
            let proxy = RowProxy::new(Arc::clone(&schema), row);
            if let HbValue::Float(s) = proxy.get(idx_salary) {
                total += s;
            }
        }
        assert_eq!(total, (0..1000).map(|i| i as f64 * 100.0).sum::<f64>());
    }
}
