// swed_rt/src/work_area.rs
// Global Work Area Manager — gerencia o estado Alias→WorkArea
// protegido por RwLock (leitura concorrente, escrita exclusiva).
//
// Modela o mecanismo de Work Areas do Harbour/Clipper:
//   SELECT CUST   →  work_area_manager.select("CUST")
//   CUST->NAME    →  work_area_manager.field("CUST", "NAME")
//   DBGOBOTTOM    →  work_area_manager.goto_bottom("CUST")
//
// O estado global é thread-safe e acessível em blocos #BEGINDUMP
// via work_areas() → &WorkAreaManager.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use crate::value::HbValue;
use crate::row::{DataNavigator, FieldIndex, Row, RowProxy, RowSchema};

// ---------------------------------------------------------------------------
// WorkArea — um slot de tabela aberta
// ---------------------------------------------------------------------------

pub struct WorkArea {
    /// Alias UPPERCASE (ex: "CUST", "ITEMS")
    pub alias: String,
    /// Driver da fonte de dados (DBF, SQLite, InMemory…)
    pub navigator: Box<dyn DataNavigator>,
    /// Cache de resolução nome→índice para esta work area.
    /// Evita chamadas repetidas a schema().resolve() em loops.
    field_cache: HashMap<String, FieldIndex>,
}

impl WorkArea {
    pub fn new(alias: impl Into<String>, navigator: Box<dyn DataNavigator>) -> Self {
        let alias = alias.into().to_ascii_uppercase();
        WorkArea {
            alias,
            navigator,
            field_cache: HashMap::new(),
        }
    }

    /// Resolve nome→FieldIndex com cache local (O(1) após primeira chamada).
    pub fn resolve_field(&mut self, name: &str) -> Option<FieldIndex> {
        let key = name.to_ascii_uppercase();
        if let Some(&idx) = self.field_cache.get(&key) {
            return Some(idx);
        }
        if let Some(idx) = self.navigator.schema().resolve(&key) {
            self.field_cache.insert(key, idx);
            return Some(idx);
        }
        None
    }

    /// Lê um campo da linha atual por nome (com cache de índice).
    pub fn get_field(&mut self, name: &str) -> HbValue {
        let idx = match self.resolve_field(name) {
            Some(i) => i,
            None => return HbValue::Nil,
        };
        self.navigator
            .current_row()
            .and_then(|r| r.get(idx.0).cloned())
            .unwrap_or(HbValue::Nil)
    }

    /// Escreve um campo da linha atual por nome.
    pub fn set_field(&mut self, name: &str, val: HbValue) {
        if let Some(mut proxy) = self.navigator.current_row_mut() {
            proxy.set_by_name(name, val);
        }
    }

    pub fn record_count(&self) -> usize { self.navigator.record_count() }
    pub fn current_pos(&self)  -> usize { self.navigator.current_position() }
    pub fn is_eof(&self)       -> bool  { self.navigator.is_eof() }
    pub fn is_bof(&self)       -> bool  { self.navigator.is_bof() }

    pub fn skip(&mut self, n: i64) -> bool { self.navigator.skip(n) }
    pub fn goto(&mut self, pos: usize) -> bool { self.navigator.goto(pos) }
    pub fn goto_top(&mut self)    { self.navigator.goto(0); }
    pub fn goto_bottom(&mut self) {
        let last = self.navigator.record_count().saturating_sub(1);
        self.navigator.goto(last);
    }
}

// ---------------------------------------------------------------------------
// WorkAreaManager — gerenciador global protegido por RwLock
// ---------------------------------------------------------------------------

pub struct WorkAreaManager {
    /// Mapa alias → WorkArea
    areas: RwLock<HashMap<String, WorkArea>>,
    /// Area selecionada no momento (alias UPPERCASE ou vazio)
    selected: RwLock<String>,
}

impl WorkAreaManager {
    pub fn new() -> Self {
        WorkAreaManager {
            areas: RwLock::new(HashMap::new()),
            selected: RwLock::new(String::new()),
        }
    }

    // ── Gestão de areas ───────────────────────────────────────────────────

    /// Abre/registra uma work area. Equivalente ao USE ... ALIAS em Harbour.
    pub fn open(&self, alias: &str, navigator: Box<dyn DataNavigator>) {
        let key = alias.to_ascii_uppercase();
        if let Ok(mut m) = self.areas.write() {
            m.insert(key.clone(), WorkArea::new(&key, navigator));
        }
        // Seleciona automaticamente a area recém-aberta
        if let Ok(mut sel) = self.selected.write() {
            *sel = key;
        }
    }

    /// Fecha uma work area. Equivalente ao USE (sem arquivo) em Harbour.
    pub fn close(&self, alias: &str) {
        if let Ok(mut m) = self.areas.write() {
            m.remove(&alias.to_ascii_uppercase());
        }
    }

    /// Seleciona a work area ativa. Equivalente ao SELECT em Harbour.
    pub fn select(&self, alias: &str) {
        if let Ok(mut sel) = self.selected.write() {
            *sel = alias.to_ascii_uppercase();
        }
    }

    /// Retorna o alias da area selecionada.
    pub fn selected_alias(&self) -> String {
        self.selected.read().map(|s| s.clone()).unwrap_or_default()
    }

    // ── Acesso a dados ────────────────────────────────────────────────────

    /// `ALIAS->FIELD` — lê campo de uma area específica.
    /// Ex: `CUST->NAME` → `wam.field("CUST", "NAME")`
    pub fn field(&self, alias: &str, field: &str) -> HbValue {
        let key = alias.to_ascii_uppercase();
        if let Ok(mut m) = self.areas.write() {
            if let Some(area) = m.get_mut(&key) {
                return area.get_field(field);
            }
        }
        HbValue::Nil
    }

    /// Lê campo da area SELECIONADA.
    pub fn field_current(&self, field: &str) -> HbValue {
        let alias = self.selected_alias();
        if alias.is_empty() { return HbValue::Nil; }
        self.field(&alias, field)
    }

    /// Escreve campo de uma área específica.
    pub fn field_set(&self, alias: &str, field: &str, val: HbValue) {
        let key = alias.to_ascii_uppercase();
        if let Ok(mut m) = self.areas.write() {
            if let Some(area) = m.get_mut(&key) {
                area.set_field(field, val);
            }
        }
    }

    /// Escreve campo da área SELECIONADA.
    pub fn field_set_current(&self, field: &str, val: HbValue) {
        let alias = self.selected_alias();
        if !alias.is_empty() {
            self.field_set(&alias, field, val);
        }
    }

    // ── Navegação ─────────────────────────────────────────────────────────

    pub fn skip(&self, alias: &str, n: i64) -> bool {
        let key = alias.to_ascii_uppercase();
        self.areas.write().ok()
            .and_then(|mut m| m.get_mut(&key).map(|a| a.skip(n)))
            .unwrap_or(false)
    }

    pub fn goto_top(&self, alias: &str) {
        let key = alias.to_ascii_uppercase();
        if let Ok(mut m) = self.areas.write() {
            if let Some(a) = m.get_mut(&key) { a.goto_top(); }
        }
    }

    pub fn goto_bottom(&self, alias: &str) {
        let key = alias.to_ascii_uppercase();
        if let Ok(mut m) = self.areas.write() {
            if let Some(a) = m.get_mut(&key) { a.goto_bottom(); }
        }
    }

    pub fn goto(&self, alias: &str, pos: usize) -> bool {
        let key = alias.to_ascii_uppercase();
        self.areas.write().ok()
            .and_then(|mut m| m.get_mut(&key).map(|a| a.goto(pos)))
            .unwrap_or(false)
    }

    pub fn is_eof(&self, alias: &str) -> bool {
        let key = alias.to_ascii_uppercase();
        self.areas.read().ok()
            .and_then(|m| m.get(&key).map(|a| a.is_eof()))
            .unwrap_or(true)
    }

    pub fn record_count(&self, alias: &str) -> usize {
        let key = alias.to_ascii_uppercase();
        self.areas.read().ok()
            .and_then(|m| m.get(&key).map(|a| a.record_count()))
            .unwrap_or(0)
    }

    pub fn is_open(&self, alias: &str) -> bool {
        self.areas.read().ok()
            .map(|m| m.contains_key(&alias.to_ascii_uppercase()))
            .unwrap_or(false)
    }
}

impl Default for WorkAreaManager {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// Global singleton — thread_local para isolamento por thread (Browse reativo)
// ---------------------------------------------------------------------------

/// Acesso ao WorkAreaManager global via thread_local.
/// Cada thread tem sua própria instância — sem contention em acessos
/// de leitura single-thread (Browse principal + threads de background
/// trabalham em instâncias separadas).
thread_local! {
    static WORK_AREAS: WorkAreaManager = WorkAreaManager::new();
}

/// Executa uma closure com acesso ao WorkAreaManager da thread atual.
/// Padrão de uso gerado pelo transpilador:
///
///   with_work_areas(|wam| wam.field("CUST", "NAME"))
pub fn with_work_areas<F, R>(f: F) -> R
where
    F: FnOnce(&WorkAreaManager) -> R,
{
    WORK_AREAS.with(f)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::row::{FieldMeta, InMemoryTable, RowSchema};

    fn make_cust_table() -> Box<dyn DataNavigator> {
        let schema = RowSchema::new(vec![
            FieldMeta { name: "NAME".into(),   dbf_type: 'C', size: 40, decimals: 0 },
            FieldMeta { name: "BALANCE".into(), dbf_type: 'N', size: 12, decimals: 2 },
        ]);
        let mut table = InMemoryTable::new(schema, vec![]);
        table.push_row(vec![HbValue::String("Alice".into()), HbValue::Float(1500.0)]);
        table.push_row(vec![HbValue::String("Bob".into()),   HbValue::Float(2200.0)]);
        table.push_row(vec![HbValue::String("Carol".into()), HbValue::Float(3100.0)]);
        Box::new(table)
    }

    #[test]
    fn test_open_and_read_field() {
        let wam = WorkAreaManager::new();
        wam.open("CUST", make_cust_table());

        assert_eq!(wam.field("CUST", "NAME"),    HbValue::String("Alice".into()));
        assert_eq!(wam.field("CUST", "BALANCE"), HbValue::Float(1500.0));
    }

    #[test]
    fn test_navigation() {
        let wam = WorkAreaManager::new();
        wam.open("CUST", make_cust_table());

        assert_eq!(wam.record_count("CUST"), 3);
        assert!(!wam.is_eof("CUST"));

        wam.skip("CUST", 1);
        assert_eq!(wam.field("CUST", "NAME"), HbValue::String("Bob".into()));

        wam.goto_bottom("CUST");
        assert_eq!(wam.field("CUST", "NAME"), HbValue::String("Carol".into()));

        wam.goto_top("CUST");
        assert_eq!(wam.field("CUST", "NAME"), HbValue::String("Alice".into()));
    }

    #[test]
    fn test_selected_alias() {
        let wam = WorkAreaManager::new();
        wam.open("CUST", make_cust_table());
        wam.select("CUST");
        assert_eq!(wam.field_current("NAME"), HbValue::String("Alice".into()));
    }

    #[test]
    fn test_field_cache_pattern() {
        // Simula o padrão de resolução único + acesso em loop:
        //   SELECT CUST
        //   DBGOTOP()
        //   DO WHILE !EOF()
        //      ? CUST->NAME
        //      DBSKIP()
        //   ENDDO
        let wam = WorkAreaManager::new();
        wam.open("CUST", make_cust_table());

        let mut names = vec![];
        // O field_cache dentro de WorkArea garante que "NAME" só é
        // resolvido UMA vez mesmo neste loop:
        wam.goto_top("CUST");
        loop {
            if wam.is_eof("CUST") { break; }
            names.push(wam.field("CUST", "NAME"));
            wam.skip("CUST", 1);
        }

        assert_eq!(names, vec![
            HbValue::String("Alice".into()),
            HbValue::String("Bob".into()),
            HbValue::String("Carol".into()),
        ]);
    }

    #[test]
    fn test_is_open_close() {
        let wam = WorkAreaManager::new();
        assert!(!wam.is_open("CUST"));
        wam.open("CUST", make_cust_table());
        assert!(wam.is_open("CUST"));
        wam.close("CUST");
        assert!(!wam.is_open("CUST"));
    }

    #[test]
    fn test_thread_local_work_areas() {
        with_work_areas(|wam| {
            wam.open("CUST", make_cust_table());
            assert_eq!(
                wam.field("CUST", "NAME"),
                HbValue::String("Alice".into())
            );
        });
    }
}
