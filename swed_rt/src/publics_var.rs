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
// Codegen API — funções livres emitidas pelo transpilador
// ---------------------------------------------------------------------------
//
// O Codegen emite `use swed_rt::{pub_declare, pub_get, pub_set, memvar_assign, memvar_get};`
// e chama essas funções diretamente no código gerado. Todas usam o mesmo STORE singleton.
//
// PUBLIC e MEMVAR/PRIVATE têm escopos diferentes em Harbour, mas no código transpilado
// (onde não existe scoping dinâmico) ambos mapeiam para o mesmo HashMap global.

/// `PUBLIC name` — declara variável PUBLIC com valor inicial.
/// Sobrescreve se já existir (comportamento Harbour para re-declaração).
pub fn pub_declare(name: &str, init: HbValue) {
    public_store().write().unwrap().set(name, init);
}

/// Lê uma variável PUBLIC. Retorna `Nil` se não declarada.
pub fn pub_get(name: &str) -> HbValue {
    public_store().read().unwrap().get(name)
}

/// Escreve uma variável PUBLIC (atribuição).
pub fn pub_set(name: &str, val: HbValue) {
    public_store().write().unwrap().set(name, val);
}

/// `MEMVAR name` ou `m->name := val` — declara ou sobrescreve uma MEMVAR/PRIVATE.
pub fn memvar_assign(name: &str, val: HbValue) {
    public_store().write().unwrap().set(name, val);
}

/// Lê uma MEMVAR/PRIVATE, incluindo acesso via `m->name`.
/// Retorna `Nil` se não declarada.
pub fn memvar_get(name: &str) -> HbValue {
    public_store().read().unwrap().get(name)
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

    // ── Codegen API ────────────────────────────────────────────────────────────

    #[test]
    fn test_pub_declare_cria_variavel() {
        fresh();
        pub_declare("NEMPRESA", HbValue::Nil);
        assert!(public_store().read().unwrap().contains("NEMPRESA"));
        assert_eq!(pub_get("NEMPRESA"), HbValue::Nil);
    }

    #[test]
    fn test_pub_declare_sobrescreve_existente() {
        fresh();
        pub_declare("NEMPRESA", HbValue::Integer(1));
        pub_declare("NEMPRESA", HbValue::Integer(99));
        assert_eq!(pub_get("NEMPRESA"), HbValue::Integer(99));
    }

    #[test]
    fn test_pub_set_e_pub_get_round_trip() {
        fresh();
        pub_declare("CUSUARIO", HbValue::Nil);
        pub_set("CUSUARIO", HbValue::String("ADMIN".into()));
        assert_eq!(pub_get("CUSUARIO"), HbValue::String("ADMIN".into()));
    }

    #[test]
    fn test_pub_get_ausente_retorna_nil() {
        fresh();
        assert_eq!(pub_get("INEXISTENTE"), HbValue::Nil);
    }

    #[test]
    fn test_memvar_assign_declara_e_sobrescreve() {
        fresh();
        memvar_assign("CNAME", HbValue::Nil);
        assert_eq!(memvar_get("CNAME"), HbValue::Nil);
        memvar_assign("CNAME", HbValue::String("Alice".into()));
        assert_eq!(memvar_get("CNAME"), HbValue::String("Alice".into()));
    }

    #[test]
    fn test_memvar_get_ausente_retorna_nil() {
        fresh();
        assert_eq!(memvar_get("NINEXISTENTE"), HbValue::Nil);
    }

    #[test]
    fn test_m_arrow_semantics_via_memvar_get() {
        // m->nCounter em Harbour → memvar_get("NCOUNTER") no código gerado.
        // Deve retornar o valor armazenado independente do escopo LOCAL.
        fresh();
        memvar_assign("NCOUNTER", HbValue::Integer(42));
        assert_eq!(memvar_get("NCOUNTER"), HbValue::Integer(42));
    }

    #[test]
    fn test_pub_e_memvar_compartilham_store() {
        // PUBLIC e MEMVAR usam o mesmo armazém — nomes distintos não colidem.
        fresh();
        pub_declare("NPUB", HbValue::Integer(1));
        memvar_assign("NMEM", HbValue::Integer(2));
        assert_eq!(pub_get("NPUB"), HbValue::Integer(1));
        assert_eq!(memvar_get("NMEM"), HbValue::Integer(2));
        // Mesmo nome sobrescreve (armazém único)
        pub_set("NMEM", HbValue::Integer(99));
        assert_eq!(memvar_get("NMEM"), HbValue::Integer(99));
    }
}
