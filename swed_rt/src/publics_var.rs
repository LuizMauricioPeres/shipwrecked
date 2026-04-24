// swed_rt/src/publics_var.rs
// Singleton para variáveis PUBLIC e MEMVAR do Harbour.
//
// Thread-safe: OnceLock<RwLock<PublicVars>> — uma instância por processo.
// Todos os nomes são normalizados para UPPERCASE (Harbour é case-insensitive).
//
// Leitura:  `public_store().read().unwrap().get("N_EMPRESA")`
// Escrita:  `public_store().write().unwrap().set("N_EMPRESA", val)`
// Teardown: `public_store().write().unwrap().clear()`

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use crate::HbValue;

// ---------------------------------------------------------------------------
// PublicVars — armazém dinâmico
// ---------------------------------------------------------------------------

/// Armazém dinâmico de variáveis PUBLIC e MEMVAR.
/// Nomes são normalizados para UPPERCASE internamente.
pub struct PublicVars {
    vars: HashMap<String, HbValue>,
}

impl PublicVars {
    fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    /// Lê uma variável. Retorna `Nil` se não declarada.
    pub fn get(&self, name: &str) -> HbValue {
        self.vars
            .get(&name.to_ascii_uppercase())
            .cloned()
            .unwrap_or(HbValue::Nil)
    }

    /// Escreve uma variável (cria ou sobrescreve).
    pub fn set(&mut self, name: &str, val: HbValue) {
        self.vars.insert(name.to_ascii_uppercase(), val);
    }

    /// Verifica se a variável foi declarada.
    pub fn contains(&self, name: &str) -> bool {
        self.vars.contains_key(&name.to_ascii_uppercase())
    }

    /// Remove a variável — equivalente a `RELEASE name` no Harbour.
    pub fn release(&mut self, name: &str) -> HbValue {
        self.vars
            .remove(&name.to_ascii_uppercase())
            .unwrap_or(HbValue::Nil)
    }

    /// Zera o armazém — usado em teardown de testes.
    pub fn clear(&mut self) {
        self.vars.clear();
    }

    /// Lista todos os nomes declarados (diagnóstico / debug).
    pub fn names(&self) -> Vec<&str> {
        self.vars.keys().map(String::as_str).collect()
    }
}

// ---------------------------------------------------------------------------
// Singleton
// ---------------------------------------------------------------------------

static STORE: OnceLock<RwLock<PublicVars>> = OnceLock::new();

/// Retorna referência ao armazém global de variáveis PUBLIC/MEMVAR.
pub fn public_store() -> &'static RwLock<PublicVars> {
    STORE.get_or_init(|| RwLock::new(PublicVars::new()))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() {
        public_store().write().unwrap().clear();
    }

    #[test]
    fn test_set_and_get() {
        fresh();
        public_store().write().unwrap().set("N_EMPRESA", HbValue::Integer(1));
        assert_eq!(
            public_store().read().unwrap().get("N_EMPRESA"),
            HbValue::Integer(1)
        );
    }

    #[test]
    fn test_get_missing_returns_nil() {
        fresh();
        assert_eq!(
            public_store().read().unwrap().get("NAO_EXISTE"),
            HbValue::Nil
        );
    }

    #[test]
    fn test_case_insensitive() {
        fresh();
        public_store().write().unwrap().set("cUsuario", HbValue::String("ADMIN".into()));
        // Qualquer capitalização deve funcionar
        assert_eq!(
            public_store().read().unwrap().get("CUSUARIO"),
            HbValue::String("ADMIN".into())
        );
        assert_eq!(
            public_store().read().unwrap().get("cusuario"),
            HbValue::String("ADMIN".into())
        );
    }

    #[test]
    fn test_overwrite() {
        fresh();
        public_store().write().unwrap().set("X", HbValue::Integer(1));
        public_store().write().unwrap().set("X", HbValue::Integer(2));
        assert_eq!(public_store().read().unwrap().get("X"), HbValue::Integer(2));
    }

    #[test]
    fn test_release() {
        fresh();
        public_store().write().unwrap().set("TEMP", HbValue::Logical(true));
        let released = public_store().write().unwrap().release("TEMP");
        assert_eq!(released, HbValue::Logical(true));
        assert_eq!(public_store().read().unwrap().get("TEMP"), HbValue::Nil);
    }

    #[test]
    fn test_contains() {
        fresh();
        assert!(!public_store().read().unwrap().contains("Y"));
        public_store().write().unwrap().set("Y", HbValue::Nil);
        assert!(public_store().read().unwrap().contains("Y"));
    }

    #[test]
    fn test_names() {
        fresh();
        public_store().write().unwrap().set("A", HbValue::Integer(1));
        public_store().write().unwrap().set("B", HbValue::Integer(2));
        let mut names = public_store().read().unwrap().names()
            .iter().map(|s| s.to_string()).collect::<Vec<_>>();
        names.sort();
        assert_eq!(names, vec!["A", "B"]);
    }

    #[test]
    fn test_nil_written_explicitly() {
        fresh();
        // PUBLIC x sem init → valor Nil, mas a chave deve existir
        public_store().write().unwrap().set("P_VAR", HbValue::Nil);
        assert!(public_store().read().unwrap().contains("P_VAR"));
        assert_eq!(public_store().read().unwrap().get("P_VAR"), HbValue::Nil);
    }
}
