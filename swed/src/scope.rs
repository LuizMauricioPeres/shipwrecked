// swed/src/scope.rs
// Scope Environment — models Harbour variable precedence:
//
//   LOCAL  >  STATIC  >  PRIVATE (MEMVAR)  >  PUBLIC
//
// Harbour resolves a bare name at runtime by walking that chain.
// During transpilation we replicate the resolution statically so we can
// emit the correct Rust binding (stack-local let, static OnceCell, or
// a thread-local "memvar pool").

use std::collections::HashMap;
use swed_co::HbType;

// ---------------------------------------------------------------------------
// Variable metadata
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct VarInfo {
    pub hb_type: HbType,
    /// Harbour declaration level.
    pub scope: VarScope,
    /// Suggested Rust identifier (snake_case transformation applied).
    pub rust_ident: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VarScope {
    Local = 0,    // LOCAL  — innermost, highest priority
    Static = 1,   // STATIC — file-scoped, persists between calls
    DbField = 2,  // FIELD  — DB column, higher priority than MEMVAR
    Private = 3,  // PRIVATE / MEMVAR — dynamic scope stack
    Public = 4,   // PUBLIC — global memvar pool
}

// ---------------------------------------------------------------------------
// ScopeFrame — one activation record
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct ScopeFrame {
    locals: HashMap<String, VarInfo>,
}

// ---------------------------------------------------------------------------
// ScopeEnv — the full environment
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct ScopeEnv {
    /// Stack of activation frames (procedures/functions).
    /// The last element is the current (innermost) frame.
    call_stack: Vec<ScopeFrame>,

    /// STATIC variables are keyed by (module_name, var_name).
    statics: HashMap<(String, String), VarInfo>,

    /// PRIVATE / MEMVAR dynamic stack (mimics Harbour's memvar pool).
    memvar_stack: Vec<HashMap<String, VarInfo>>,

    /// PUBLIC globals.
    publics: HashMap<String, VarInfo>,

    /// Current module name (for STATIC scoping).
    current_module: String,
}

impl ScopeEnv {
    pub fn new(module: impl Into<String>) -> Self {
        Self {
            call_stack: vec![ScopeFrame::default()],
            statics: HashMap::new(),
            memvar_stack: vec![HashMap::new()],
            publics: HashMap::new(),
            current_module: module.into(),
        }
    }

    // ------------------------------------------------------------------
    // Frame management
    // ------------------------------------------------------------------

    /// Enter a new procedure/function scope.
    pub fn push_frame(&mut self) {
        self.call_stack.push(ScopeFrame::default());
        self.memvar_stack.push(HashMap::new());
    }

    /// Leave the current procedure/function scope.
    pub fn pop_frame(&mut self) {
        if self.call_stack.len() > 1 {
            self.call_stack.pop();
        }
        if self.memvar_stack.len() > 1 {
            self.memvar_stack.pop();
        }
    }

    // ------------------------------------------------------------------
    // Variable declaration
    // ------------------------------------------------------------------

    pub fn declare_local(&mut self, name: &str, hb_type: HbType) {
        let info = VarInfo {
            hb_type,
            scope: VarScope::Local,
            rust_ident: to_snake_case(name),
        };
        if let Some(frame) = self.call_stack.last_mut() {
            frame.locals.insert(name.to_ascii_uppercase(), info);
        }
    }

    pub fn declare_static(&mut self, name: &str, hb_type: HbType) {
        let info = VarInfo {
            hb_type,
            scope: VarScope::Static,
            rust_ident: to_snake_case(name),
        };
        let key = (self.current_module.clone(), name.to_ascii_uppercase());
        self.statics.insert(key, info);
    }

    pub fn declare_field(&mut self, name: &str) {
        let info = VarInfo {
            hb_type: HbType::Unknown,
            scope: VarScope::DbField,
            rust_ident: to_snake_case(name),
        };
        // Fields are stored in the current LOCAL frame so resolution
        // checks them after LOCAL/STATIC but before MEMVAR/PUBLIC.
        if let Some(frame) = self.call_stack.last_mut() {
            frame.locals.insert(name.to_ascii_uppercase(), info);
        }
    }

    pub fn declare_private(&mut self, name: &str, hb_type: HbType) {
        let info = VarInfo {
            hb_type,
            scope: VarScope::Private,
            rust_ident: to_snake_case(name),
        };
        if let Some(frame) = self.memvar_stack.last_mut() {
            frame.insert(name.to_ascii_uppercase(), info);
        }
    }

    pub fn declare_public(&mut self, name: &str, hb_type: HbType) {
        let info = VarInfo {
            hb_type,
            scope: VarScope::Public,
            rust_ident: to_snake_case(name),
        };
        self.publics.insert(name.to_ascii_uppercase(), info);
    }

    // ------------------------------------------------------------------
    // Resolution — LOCAL > STATIC > PRIVATE > PUBLIC
    // ------------------------------------------------------------------

    /// Resolve a variable name following Harbour's precedence rules.
    /// Returns `None` if the variable is undeclared (Harbour would create
    /// it as PRIVATE at runtime; we flag it as a warning during transpilation).
    pub fn resolve(&self, name: &str) -> Option<&VarInfo> {
        let key = name.to_ascii_uppercase();

        // 1. LOCAL — innermost frame
        if let Some(frame) = self.call_stack.last() {
            if let Some(info) = frame.locals.get(&key) {
                return Some(info);
            }
        }

        // 2. STATIC — current module
        let static_key = (self.current_module.clone(), key.clone());
        if let Some(info) = self.statics.get(&static_key) {
            return Some(info);
        }

        // 3. PRIVATE / MEMVAR — innermost to outermost dynamic frame
        for frame in self.memvar_stack.iter().rev() {
            if let Some(info) = frame.get(&key) {
                return Some(info);
            }
        }

        // 4. PUBLIC — global pool
        self.publics.get(&key)
    }

    /// Returns `true` if the name resolves to a LOCAL variable.
    pub fn is_local(&self, name: &str) -> bool {
        matches!(
            self.resolve(name).map(|v| &v.scope),
            Some(VarScope::Local)
        )
    }
}

// ---------------------------------------------------------------------------
// Utility: convert HARBOUR_IDENT to rust_snake_case
// ---------------------------------------------------------------------------

pub fn to_snake_case(s: &str) -> String {
    // Harbour identifiers may use PascalCase (aMyArray) or ALL_CAPS (NCOUNT).
    // Insert `_` before each uppercase letter that follows a lowercase letter,
    // then lowercase everything.
    let mut out = String::with_capacity(s.len() + 4);
    let chars: Vec<char> = s.chars().collect();
    for (i, &ch) in chars.iter().enumerate() {
        let prev_lower = i > 0 && chars[i - 1].is_ascii_lowercase();
        if ch.is_ascii_uppercase() && prev_lower {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    // Replace non-alphanumeric runs with single `_`
    let mut result = String::with_capacity(out.len());
    let mut prev_under = false;
    for ch in out.chars() {
        if ch.is_alphanumeric() {
            result.push(ch);
            prev_under = false;
        } else if !prev_under {
            result.push('_');
            prev_under = true;
        }
    }
    result.trim_matches('_').to_string()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_shadows_public() {
        let mut env = ScopeEnv::new("test_mod");
        env.declare_public("X", HbType::Float);
        env.declare_local("X", HbType::Character);

        let info = env.resolve("X").unwrap();
        assert_eq!(info.scope, VarScope::Local);
        assert_eq!(info.hb_type, HbType::Character);
    }

    #[test]
    fn test_static_shadows_private() {
        let mut env = ScopeEnv::new("mymod");
        env.declare_private("COUNTER", HbType::Float);
        env.declare_static("COUNTER", HbType::Float);

        let info = env.resolve("COUNTER").unwrap();
        assert_eq!(info.scope, VarScope::Static);
    }

    #[test]
    fn test_undeclared_returns_none() {
        let env = ScopeEnv::new("mod");
        assert!(env.resolve("GHOST").is_none());
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("aMyArray"), "a_my_array");
        assert_eq!(to_snake_case("NCOUNT"), "ncount");
        assert_eq!(to_snake_case("cFullName"), "c_full_name");
    }

    #[test]
    fn test_frame_push_pop() {
        let mut env = ScopeEnv::new("mod");
        env.declare_local("X", HbType::Float);
        assert!(env.resolve("X").is_some());

        env.push_frame();
        // X is not visible in new frame (strict LOCAL scope per procedure)
        assert!(env.resolve("X").is_none());

        env.pop_frame();
        assert!(env.resolve("X").is_some());
    }
}

// ---------------------------------------------------------------------------
// IndexedFrame — extensão do ScopeFrame para acesso O(1) por usize
//
// Usado pela camada de geração de código quando o transpilador pode
// provar estaticamente a ordem das variáveis LOCALs em uma procedure.
// ---------------------------------------------------------------------------

/// Um frame com resolução dupla: por nome (HashMap) e por índice (Vec).
/// O índice corresponde à ordem de declaração das variáveis LOCAL.
#[derive(Debug, Default)]
pub struct IndexedFrame {
    /// Mapeamento nome → slot index (construído na declaração)
    name_to_idx: HashMap<String, usize>,
    /// Valores indexados por slot (Vec → O(1))
    slots: Vec<VarInfo>,
}

impl IndexedFrame {
    pub fn new() -> Self {
        IndexedFrame::default()
    }

    /// Declara uma variável e retorna seu índice de slot.
    /// Chamado pelo transpilador na ordem de declaração.
    pub fn declare(&mut self, name: &str, info: VarInfo) -> usize {
        let key = name.to_ascii_uppercase();
        if let Some(&idx) = self.name_to_idx.get(&key) {
            // Redeclaração: atualiza mas mantém o índice existente
            self.slots[idx] = info;
            return idx;
        }
        let idx = self.slots.len();
        self.slots.push(info);
        self.name_to_idx.insert(key, idx);
        idx
    }

    /// Acesso por nome (O(hash)) — usado na análise semântica.
    pub fn get_by_name(&self, name: &str) -> Option<(usize, &VarInfo)> {
        let key = name.to_ascii_uppercase();
        self.name_to_idx.get(&key).map(|&idx| (idx, &self.slots[idx]))
    }

    /// Acesso por índice (O(1)) — usado no código gerado.
    pub fn get_by_index(&self, idx: usize) -> Option<&VarInfo> {
        self.slots.get(idx)
    }

    /// Número de variáveis declaradas.
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }
}

/// Slot index de uma variável LOCAL — newtype para clareza no codegen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalSlot(pub usize);

/// Extensão do ScopeEnv com suporte a frames indexados.
/// Coexiste com o ScopeEnv existente — adotado incrementalmente.
#[derive(Debug, Default)]
pub struct IndexedScopeEnv {
    frames: Vec<IndexedFrame>,
    current_module: String,
}

impl IndexedScopeEnv {
    pub fn new(module: impl Into<String>) -> Self {
        IndexedScopeEnv {
            frames: vec![IndexedFrame::new()],
            current_module: module.into(),
        }
    }

    pub fn push_frame(&mut self) {
        self.frames.push(IndexedFrame::new());
    }

    pub fn pop_frame(&mut self) {
        if self.frames.len() > 1 {
            self.frames.pop();
        }
    }

    /// Declara uma LOCAL e retorna o LocalSlot para uso no codegen.
    pub fn declare_local(&mut self, name: &str, hb_type: HbType) -> LocalSlot {
        let info = VarInfo {
            hb_type,
            scope: VarScope::Local,
            rust_ident: to_snake_case(name),
        };
        let idx = self.frames.last_mut()
            .map(|f| f.declare(name, info))
            .unwrap_or(0);
        LocalSlot(idx)
    }

    /// Resolve por nome → (LocalSlot, &VarInfo)
    pub fn resolve_by_name(&self, name: &str) -> Option<(LocalSlot, &VarInfo)> {
        self.frames.last()
            .and_then(|f| f.get_by_name(name))
            .map(|(idx, info)| (LocalSlot(idx), info))
    }

    /// Resolve por slot (O(1)) — usado no código gerado em loops.
    pub fn resolve_by_slot(&self, slot: LocalSlot) -> Option<&VarInfo> {
        self.frames.last()
            .and_then(|f| f.get_by_index(slot.0))
    }

    /// Número de LOCALs no frame atual — útil para gerar o array de slots.
    pub fn local_count(&self) -> usize {
        self.frames.last().map(|f| f.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod indexed_tests {
    use super::*;

    #[test]
    fn test_indexed_frame_declare_and_get() {
        let mut frame = IndexedFrame::new();
        let info = VarInfo {
            hb_type: HbType::Float,
            scope: VarScope::Local,
            rust_ident: "n_salary".into(),
        };
        let idx = frame.declare("NSALARY", info.clone());
        assert_eq!(idx, 0);

        let (slot, got) = frame.get_by_name("NSALARY").unwrap();
        assert_eq!(slot, 0);
        assert_eq!(got.rust_ident, "n_salary");

        // Acesso O(1) por índice
        let got2 = frame.get_by_index(0).unwrap();
        assert_eq!(got2.rust_ident, "n_salary");
    }

    #[test]
    fn test_indexed_scope_env() {
        let mut env = IndexedScopeEnv::new("Calcular");
        let slot_a = env.declare_local("NSALARIO", HbType::Float);
        let slot_b = env.declare_local("CNAME",    HbType::Character);

        assert_eq!(slot_a, LocalSlot(0));
        assert_eq!(slot_b, LocalSlot(1));

        // Resolve por nome
        let (s, info) = env.resolve_by_name("NSALARIO").unwrap();
        assert_eq!(s, LocalSlot(0));
        assert_eq!(info.hb_type, HbType::Float);

        // Resolve por slot (O(1))
        let info2 = env.resolve_by_slot(LocalSlot(1)).unwrap();
        assert_eq!(info2.hb_type, HbType::Character);

        assert_eq!(env.local_count(), 2);
    }

    #[test]
    fn test_indexed_frame_push_pop_isolation() {
        let mut env = IndexedScopeEnv::new("Main");
        env.declare_local("X", HbType::Float);
        assert_eq!(env.local_count(), 1);

        env.push_frame();
        assert_eq!(env.local_count(), 0); // novo frame vazio
        env.declare_local("Y", HbType::Character);
        assert_eq!(env.local_count(), 1);

        env.pop_frame();
        assert_eq!(env.local_count(), 1); // volta ao frame anterior
        // Y não existe no frame original
        assert!(env.resolve_by_name("Y").is_none());
    }
}
