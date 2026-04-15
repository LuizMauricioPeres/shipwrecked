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

// ---------------------------------------------------------------------------
// Variable metadata
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum HbType {
    Any,       // U — untyped (most Harbour vars)
    Numeric,   // N
    String,    // C
    Logical,   // L
    Array,     // A
    Object,    // O
    Date,      // D
    Nil,
}

impl Default for HbType {
    fn default() -> Self {
        HbType::Any
    }
}

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
    Private = 2,  // PRIVATE / MEMVAR — dynamic scope stack
    Public = 3,   // PUBLIC — global memvar pool
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
        env.declare_public("X", HbType::Numeric);
        env.declare_local("X", HbType::String);

        let info = env.resolve("X").unwrap();
        assert_eq!(info.scope, VarScope::Local);
        assert_eq!(info.hb_type, HbType::String);
    }

    #[test]
    fn test_static_shadows_private() {
        let mut env = ScopeEnv::new("mymod");
        env.declare_private("COUNTER", HbType::Numeric);
        env.declare_static("COUNTER", HbType::Numeric);

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
        env.declare_local("X", HbType::Numeric);
        assert!(env.resolve("X").is_some());

        env.push_frame();
        // X is not visible in new frame (strict LOCAL scope per procedure)
        assert!(env.resolve("X").is_none());

        env.pop_frame();
        assert!(env.resolve("X").is_some());
    }
}
