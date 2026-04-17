// swed_rt/src/unwrap.rs
// Traits de "Unwrapping Seguro" — converte HbValue em tipos Rust primitivos.
//
// Inspirado na API do Harbour (hb_parni, hb_parnd, hb_parc, hb_retni…)
// mas com ergonomia Rust: Result<T, UnwrapError> e Option<T>.
//
// Uso nos blocos #BEGINDUMP:
//
//   // Harbour passa um HbValue::Float para o bloco Rust
//   fn calcular_juros(principal: HbValue, taxa: HbValue) -> HbValue {
//       let p: f64 = principal.try_as_f64()?;  // unwrap seguro
//       let t: f64 = taxa.try_as_f64()?;
//       HbValue::Float(p * t)
//   }
//
// Também fornece ScopeStack para acesso a variáveis do xBase
// dentro dos blocos inline.

use crate::value::HbValue;
use std::fmt;

// ---------------------------------------------------------------------------
// UnwrapError
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct UnwrapError {
    pub expected: &'static str,
    pub got: String,
}

impl fmt::Display for UnwrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tipo esperado `{}`, encontrado `{}`", self.expected, self.got)
    }
}

impl std::error::Error for UnwrapError {}

// ---------------------------------------------------------------------------
// TryAsRust trait — conversão segura HbValue → tipo Rust
// ---------------------------------------------------------------------------

pub trait TryAsRust {
    /// Converte para i64. Aceita Integer e Float (truncado).
    fn try_as_i64(&self) -> Result<i64, UnwrapError>;

    /// Converte para f64. Aceita Integer e Float.
    fn try_as_f64(&self) -> Result<f64, UnwrapError>;

    /// Converte para bool. Aceita Logical.
    fn try_as_bool(&self) -> Result<bool, UnwrapError>;

    /// Converte para &str. Aceita String.
    fn try_as_str(&self) -> Result<&str, UnwrapError>;

    /// Converte para String clonado. Aceita String e Integer/Float (via Display).
    fn try_as_string(&self) -> Result<String, UnwrapError>;

    /// Retorna None para Nil, Some(self) para qualquer outro tipo.
    fn as_option(&self) -> Option<&HbValue>;

    // ── Versões que retornam default em vez de erro (hb_parni estilo) ─────

    /// Equivalente ao `hb_parni` do Harbour: retorna 0 se não for numérico.
    fn as_i64_or(&self, default: i64) -> i64 {
        self.try_as_i64().unwrap_or(default)
    }

    /// Equivalente ao `hb_parnd` do Harbour: retorna 0.0 se não for numérico.
    fn as_f64_or(&self, default: f64) -> f64 {
        self.try_as_f64().unwrap_or(default)
    }

    /// Equivalente ao `hb_parc` do Harbour: retorna "" se não for string.
    fn as_str_or<'a>(&'a self, default: &'a str) -> &'a str {
        self.try_as_str().unwrap_or(default)
    }

    /// Equivalente ao `hb_parl` do Harbour: retorna false se não for lógico.
    fn as_bool_or(&self, default: bool) -> bool {
        self.try_as_bool().unwrap_or(default)
    }
}

impl TryAsRust for HbValue {
    fn try_as_i64(&self) -> Result<i64, UnwrapError> {
        match self {
            HbValue::Integer(n) => Ok(*n),
            HbValue::Float(f)   => Ok(*f as i64),
            other => Err(UnwrapError {
                expected: "Integer or Float",
                got: other.val_type().to_string(),
            }),
        }
    }

    fn try_as_f64(&self) -> Result<f64, UnwrapError> {
        match self {
            HbValue::Integer(n) => Ok(*n as f64),
            HbValue::Float(f)   => Ok(*f),
            other => Err(UnwrapError {
                expected: "Integer or Float",
                got: other.val_type().to_string(),
            }),
        }
    }

    fn try_as_bool(&self) -> Result<bool, UnwrapError> {
        match self {
            HbValue::Logical(b) => Ok(*b),
            other => Err(UnwrapError {
                expected: "Logical",
                got: other.val_type().to_string(),
            }),
        }
    }

    fn try_as_str(&self) -> Result<&str, UnwrapError> {
        match self {
            HbValue::String(s) => Ok(s.as_str()),
            other => Err(UnwrapError {
                expected: "String",
                got: other.val_type().to_string(),
            }),
        }
    }

    fn try_as_string(&self) -> Result<String, UnwrapError> {
        match self {
            HbValue::String(s)  => Ok(s.clone()),
            HbValue::Integer(n) => Ok(n.to_string()),
            HbValue::Float(f)   => Ok(format!("{f}")),
            other => Err(UnwrapError {
                expected: "String, Integer or Float",
                got: other.val_type().to_string(),
            }),
        }
    }

    fn as_option(&self) -> Option<&HbValue> {
        match self {
            HbValue::Nil => None,
            other => Some(other),
        }
    }
}

// ---------------------------------------------------------------------------
// IntoHbValue trait — conversão segura tipo Rust → HbValue (hb_ret*)
// ---------------------------------------------------------------------------

pub trait IntoHbValue {
    fn into_hb(self) -> HbValue;
}

impl IntoHbValue for i64   { fn into_hb(self) -> HbValue { HbValue::Integer(self) } }
impl IntoHbValue for i32   { fn into_hb(self) -> HbValue { HbValue::Integer(self as i64) } }
impl IntoHbValue for usize { fn into_hb(self) -> HbValue { HbValue::Integer(self as i64) } }
impl IntoHbValue for f64   { fn into_hb(self) -> HbValue { HbValue::Float(self) } }
impl IntoHbValue for f32   { fn into_hb(self) -> HbValue { HbValue::Float(self as f64) } }
impl IntoHbValue for bool  { fn into_hb(self) -> HbValue { HbValue::Logical(self) } }
impl IntoHbValue for String { fn into_hb(self) -> HbValue { HbValue::String(self) } }
impl IntoHbValue for &str  { fn into_hb(self) -> HbValue { HbValue::String(self.to_owned()) } }
impl IntoHbValue for Vec<u8> {
    fn into_hb(self) -> HbValue {
        // Bytes → String UTF-8 lossy (para buffers de telemetria/imagem)
        HbValue::String(String::from_utf8_lossy(&self).into_owned())
    }
}
impl<T: IntoHbValue> IntoHbValue for Option<T> {
    fn into_hb(self) -> HbValue {
        self.map(|v| v.into_hb()).unwrap_or(HbValue::Nil)
    }
}
impl<T: IntoHbValue, E: fmt::Display> IntoHbValue for Result<T, E> {
    fn into_hb(self) -> HbValue {
        match self {
            Ok(v)  => v.into_hb(),
            Err(e) => HbValue::String(format!("ERROR: {e}")),
        }
    }
}

// ---------------------------------------------------------------------------
// ScopeStack — acesso a variáveis xBase dentro de blocos #BEGINDUMP
//
// O transpilador injeta um &ScopeStack no início de cada bloco inline.
// O código Rust acessa variáveis do scope xBase via:
//
//   let n: f64 = scope.get("NSALARIO").try_as_f64()?;
//   scope.set("CRESULT", resultado.into_hb());
// ---------------------------------------------------------------------------

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Camada de variáveis do scope xBase acessível em blocos inline.
/// Protegida por RwLock para suportar acesso concorrente no futuro.
#[derive(Debug, Clone, Default)]
pub struct ScopeStack {
    vars: Arc<RwLock<HashMap<String, HbValue>>>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack {
            vars: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Injeta uma variável do scope xBase (chamado pelo runtime antes do bloco).
    pub fn inject(&self, name: &str, value: HbValue) {
        if let Ok(mut m) = self.vars.write() {
            m.insert(name.to_ascii_uppercase(), value);
        }
    }

    /// Lê uma variável. Retorna HbValue::Nil se não encontrada.
    /// Equivalente ao `hb_param()` do Harbour.
    pub fn get(&self, name: &str) -> HbValue {
        self.vars
            .read()
            .ok()
            .and_then(|m| m.get(&name.to_ascii_uppercase()).cloned())
            .unwrap_or(HbValue::Nil)
    }

    /// Escreve de volta ao scope xBase.
    /// Equivalente ao `hb_stor*()` do Harbour.
    pub fn set(&self, name: &str, value: HbValue) {
        if let Ok(mut m) = self.vars.write() {
            m.insert(name.to_ascii_uppercase(), value);
        }
    }

    /// Número de variáveis no scope (equivalente a `hb_pcount()`).
    pub fn count(&self) -> usize {
        self.vars.read().map(|m| m.len()).unwrap_or(0)
    }

    /// Drena as variáveis modificadas de volta para o scope xBase.
    /// Retorna uma snapshot para o transpilador sincronizar os LOCALs.
    pub fn drain_modified(&self) -> HashMap<String, HbValue> {
        self.vars
            .read()
            .map(|m| m.clone())
            .unwrap_or_default()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_as_i64() {
        assert_eq!(HbValue::Integer(42).try_as_i64(), Ok(42));
        assert_eq!(HbValue::Float(3.9).try_as_i64(),  Ok(3));
        assert!(HbValue::String("x".into()).try_as_i64().is_err());
    }

    #[test]
    fn test_try_as_f64() {
        assert_eq!(HbValue::Float(3.14).try_as_f64(), Ok(3.14));
        assert_eq!(HbValue::Integer(2).try_as_f64(),  Ok(2.0));
        assert!(HbValue::Nil.try_as_f64().is_err());
    }

    #[test]
    fn test_try_as_bool() {
        assert_eq!(HbValue::Logical(true).try_as_bool(),  Ok(true));
        assert_eq!(HbValue::Logical(false).try_as_bool(), Ok(false));
        assert!(HbValue::Integer(1).try_as_bool().is_err());
    }

    #[test]
    fn test_try_as_str() {
        assert_eq!(HbValue::String("hello".into()).try_as_str(), Ok("hello"));
        assert!(HbValue::Nil.try_as_str().is_err());
    }

    #[test]
    fn test_as_defaults() {
        assert_eq!(HbValue::Nil.as_i64_or(-1),    -1);
        assert_eq!(HbValue::Nil.as_f64_or(0.5),   0.5);
        assert_eq!(HbValue::Nil.as_str_or("def"), "def");
        assert_eq!(HbValue::Nil.as_bool_or(true), true);
    }

    #[test]
    fn test_as_option() {
        assert!(HbValue::Nil.as_option().is_none());
        assert!(HbValue::Integer(1).as_option().is_some());
    }

    #[test]
    fn test_into_hb() {
        assert_eq!(42i64.into_hb(),         HbValue::Integer(42));
        assert_eq!(3.14f64.into_hb(),       HbValue::Float(3.14));
        assert_eq!(true.into_hb(),          HbValue::Logical(true));
        assert_eq!("hi".into_hb(),          HbValue::String("hi".into()));
        assert_eq!(None::<i64>.into_hb(),   HbValue::Nil);
        assert_eq!(Some(7i64).into_hb(),    HbValue::Integer(7));
        let r: Result<i64, &str> = Err("falha");
        assert!(r.into_hb().try_as_str().unwrap().starts_with("ERROR:"));
    }

    #[test]
    fn test_scope_stack() {
        let scope = ScopeStack::new();
        scope.inject("NSALARIO", HbValue::Float(9500.0));
        scope.inject("CNOME",    HbValue::String("Alice".into()));

        assert_eq!(scope.get("NSALARIO"), HbValue::Float(9500.0));
        assert_eq!(scope.get("cnome"),    HbValue::String("Alice".into())); // case-insensitive
        assert_eq!(scope.get("GHOST"),    HbValue::Nil);

        scope.set("NRESULT", HbValue::Float(9500.0 * 1.1));
        let snap = scope.drain_modified();
        assert!(snap.contains_key("NRESULT"));
    }

    #[test]
    fn test_scope_inline_usage_pattern() {
        // Simula o padrão gerado pelo transpilador para #BEGINDUMP:
        //
        //   LOCAL nSalario := 9500.00
        //   LOCAL nBonus   := CalcBonus( nSalario )
        //
        // #BEGINDUMP
        //   fn CalcBonus(scope: &ScopeStack) -> HbValue {
        //       let s = scope.get("NSALARIO").as_f64_or(0.0);
        //       let b = s * 0.15;
        //       b.into_hb()
        //   }
        // #ENDDUMP

        let scope = ScopeStack::new();
        scope.inject("NSALARIO", HbValue::Float(9500.0));

        // bloco inline simulado:
        let bonus = {
            let s = scope.get("NSALARIO").as_f64_or(0.0);
            (s * 0.15).into_hb()
        };

        assert_eq!(bonus, HbValue::Float(1425.0));
    }
}
