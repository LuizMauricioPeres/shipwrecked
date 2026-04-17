// swed_rt/src/value.rs
// HbValue — the universal Harbour "any" type.
//
// Harbour variables are dynamically typed at runtime.  Every value that
// crosses a procedure boundary or lives in a LOCAL/MEMVAR slot is one of
// these variants.  The transpiler emits `HbValue` for all untyped slots
// and specialises to Rust primitives only when type inference is certain.

use std::fmt;
use crate::array::HbArray;

// ---------------------------------------------------------------------------
// HbValue
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum HbValue {
    /// NIL — the zero-value; returned by procedures, uninitialized vars.
    Nil,
    /// .T. / .F.
    Logical(bool),
    /// Integer numeric (Harbour has no distinct int/float at source level,
    /// but we split for Rust efficiency).
    Integer(i64),
    /// Floating-point numeric.
    Float(f64),
    /// Character string (Harbour strings are byte strings; we use UTF-8).
    String(String),
    /// Array — 1-indexed, dynamic, heterogeneous.
    Array(HbArray),
    /// Date — stored as days since 1970-01-01 (simple representation).
    Date(i32),
}

// ---------------------------------------------------------------------------
// Display (mirrors Harbour's default output)
// ---------------------------------------------------------------------------

impl fmt::Display for HbValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HbValue::Nil           => write!(f, ""),
            HbValue::Logical(true) => write!(f, ".T."),
            HbValue::Logical(false)=> write!(f, ".F."),
            HbValue::Integer(n)    => write!(f, "{n}"),
            HbValue::Float(n)      => {
                // Harbour prints up to 10 significant digits, no trailing zeros
                let s = format!("{n:.10}");
                let s = s.trim_end_matches('0');
                let s = s.trim_end_matches('.');
                write!(f, "{s}")
            }
            HbValue::String(s)     => write!(f, "{s}"),
            HbValue::Array(a)      => write!(f, "Array({} items)", a.len()),
            HbValue::Date(d)       => {
                // Convert days-since-epoch to YYYY-MM-DD
                let base = 719_163i32; // days from year 0 to 1970-01-01
                let jd = d + base;
                let y = jd / 365;
                write!(f, "Date({y}+{d})")
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Arithmetic operators — mirror Harbour's runtime coercions
// ---------------------------------------------------------------------------

impl std::ops::Add for HbValue {
    type Output = HbValue;
    fn add(self, rhs: HbValue) -> HbValue {
        match (self, rhs) {
            (HbValue::Integer(a), HbValue::Integer(b)) => HbValue::Integer(a + b),
            (HbValue::Float(a),   HbValue::Float(b))   => HbValue::Float(a + b),
            (HbValue::Integer(a), HbValue::Float(b))   => HbValue::Float(a as f64 + b),
            (HbValue::Float(a),   HbValue::Integer(b)) => HbValue::Float(a + b as f64),
            // String concatenation via `+`
            (HbValue::String(a),  HbValue::String(b))  => HbValue::String(a + &b),
            _ => HbValue::Nil,
        }
    }
}

impl std::ops::Sub for HbValue {
    type Output = HbValue;
    fn sub(self, rhs: HbValue) -> HbValue {
        match (self, rhs) {
            (HbValue::Integer(a), HbValue::Integer(b)) => HbValue::Integer(a - b),
            (HbValue::Float(a),   HbValue::Float(b))   => HbValue::Float(a - b),
            (HbValue::Integer(a), HbValue::Float(b))   => HbValue::Float(a as f64 - b),
            (HbValue::Float(a),   HbValue::Integer(b)) => HbValue::Float(a - b as f64),
            _ => HbValue::Nil,
        }
    }
}

impl std::ops::Mul for HbValue {
    type Output = HbValue;
    fn mul(self, rhs: HbValue) -> HbValue {
        match (self, rhs) {
            (HbValue::Integer(a), HbValue::Integer(b)) => HbValue::Integer(a * b),
            (HbValue::Float(a),   HbValue::Float(b))   => HbValue::Float(a * b),
            (HbValue::Integer(a), HbValue::Float(b))   => HbValue::Float(a as f64 * b),
            (HbValue::Float(a),   HbValue::Integer(b)) => HbValue::Float(a * b as f64),
            _ => HbValue::Nil,
        }
    }
}

impl std::ops::Div for HbValue {
    type Output = HbValue;
    fn div(self, rhs: HbValue) -> HbValue {
        match (self, rhs) {
            (_, HbValue::Integer(0))   => HbValue::Nil, // Harbour: division by zero → runtime error; we return Nil
            (_, HbValue::Float(b)) if b == 0.0 => HbValue::Nil,
            (HbValue::Integer(a), HbValue::Integer(b)) => HbValue::Float(a as f64 / b as f64),
            (HbValue::Float(a),   HbValue::Float(b))   => HbValue::Float(a / b),
            (HbValue::Integer(a), HbValue::Float(b))   => HbValue::Float(a as f64 / b),
            (HbValue::Float(a),   HbValue::Integer(b)) => HbValue::Float(a / b as f64),
            _ => HbValue::Nil,
        }
    }
}

impl std::ops::Rem for HbValue {
    type Output = HbValue;
    fn rem(self, rhs: HbValue) -> HbValue {
        match (self, rhs) {
            (HbValue::Integer(a), HbValue::Integer(b)) if b != 0 => HbValue::Integer(a % b),
            (HbValue::Float(a),   HbValue::Float(b))   => HbValue::Float(a % b),
            _ => HbValue::Nil,
        }
    }
}

impl std::ops::Neg for HbValue {
    type Output = HbValue;
    fn neg(self) -> HbValue {
        match self {
            HbValue::Integer(n) => HbValue::Integer(-n),
            HbValue::Float(f)   => HbValue::Float(-f),
            _ => HbValue::Nil,
        }
    }
}

impl std::ops::Not for HbValue {
    type Output = HbValue;
    fn not(self) -> HbValue {
        match self {
            HbValue::Logical(b) => HbValue::Logical(!b),
            _ => HbValue::Nil,
        }
    }
}

// ---------------------------------------------------------------------------
// Comparison operators
// ---------------------------------------------------------------------------

impl PartialOrd for HbValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (HbValue::Integer(a), HbValue::Integer(b)) => a.partial_cmp(b),
            (HbValue::Float(a),   HbValue::Float(b))   => a.partial_cmp(b),
            (HbValue::Integer(a), HbValue::Float(b))   => (*a as f64).partial_cmp(b),
            (HbValue::Float(a),   HbValue::Integer(b)) => a.partial_cmp(&(*b as f64)),
            (HbValue::String(a),  HbValue::String(b))  => a.partial_cmp(b),
            (HbValue::Date(a),    HbValue::Date(b))     => a.partial_cmp(b),
            _ => None,
        }
    }
}

// Implement bool ops for use in `if` / `while` conditions
impl HbValue {
    /// Truthy test — mirrors Harbour's implicit boolean coercion.
    pub fn is_truthy(&self) -> bool {
        match self {
            HbValue::Nil            => false,
            HbValue::Logical(b)     => *b,
            HbValue::Integer(n)     => *n != 0,
            HbValue::Float(f)       => *f != 0.0,
            HbValue::String(s)      => !s.is_empty(),
            HbValue::Array(a)       => !a.is_empty(),
            HbValue::Date(_)        => true,
        }
    }

    /// Exponentiation: `^` operator in Harbour.
    pub fn pow_hb(self, exp: HbValue) -> HbValue {
        match (self, exp) {
            (HbValue::Integer(a), HbValue::Integer(b)) if b >= 0 => {
                HbValue::Integer(a.pow(b as u32))
            }
            (HbValue::Integer(a), HbValue::Float(b))   => HbValue::Float((a as f64).powf(b)),
            (HbValue::Float(a),   HbValue::Integer(b)) => HbValue::Float(a.powi(b as i32)),
            (HbValue::Float(a),   HbValue::Float(b))   => HbValue::Float(a.powf(b)),
            _ => HbValue::Nil,
        }
    }

    /// Comprimento interno (para bounds checks em código gerado).
    pub fn len(&self) -> usize {
        match self {
            HbValue::String(s) => s.len(),
            HbValue::Array(a)  => a.len(),
            _ => 0,
        }
    }

    /// Coerção para string — espelho de `cValToChar()` do Harbour.
    pub fn to_hb_str(&self) -> String {
        self.to_string()
    }

    // -----------------------------------------------------------------------
    // Numéricos
    // -----------------------------------------------------------------------

    /// Trunca sem arredondar, preservando N casas decimais.
    /// Equivalente a `INT()` quando `decimals = 0`, ou ao padrão
    /// `hb_numTrunc(n, nDec)` do Harbour para casas > 0.
    /// Ex: `hb_trunc(1.999, 2)` → `1.99`
    pub fn hb_trunc(&self, decimals: i32) -> HbValue {
        let f = match self {
            HbValue::Integer(n) => return HbValue::Integer(*n),
            HbValue::Float(f)   => *f,
            _                   => return HbValue::Nil,
        };
        if decimals <= 0 {
            return HbValue::Integer(f.trunc() as i64);
        }
        let factor = 10f64.powi(decimals);
        HbValue::Float((f * factor).trunc() / factor)
    }

    /// Arredonda para N casas decimais.
    /// Equivalente a `ROUND(n, nDec)` do Harbour.
    /// Ex: `hb_round(1.995, 2)` → `2.00`
    pub fn hb_round(&self, decimals: i32) -> HbValue {
        let f = match self {
            HbValue::Integer(n) => *n as f64,
            HbValue::Float(f)   => *f,
            _                   => return HbValue::Nil,
        };
        let factor = 10f64.powi(decimals);
        let rounded = (f * factor).round() / factor;
        if decimals <= 0 {
            HbValue::Integer(rounded as i64)
        } else {
            HbValue::Float(rounded)
        }
    }

    /// Formata número com largura total e casas decimais, alinhado à direita.
    /// Equivalente a `STR(n, nWidth, nDec)` do Harbour.
    /// Ex: `STR(1234.5, 10, 2)` → `"   1234.50"`
    pub fn hb_str_format(&self, width: i32, dec: i32) -> String {
        let f = match self {
            HbValue::Integer(n) => *n as f64,
            HbValue::Float(f)   => *f,
            _                   => return " ".repeat(width.max(0) as usize),
        };
        let dec = dec.max(0) as usize;
        let s = if dec > 0 {
            format!("{f:.dec$}")
        } else {
            format!("{:.0}", f)
        };
        let width = width.max(0) as usize;
        if s.len() < width { format!("{s:>width$}") } else { s }
    }

    // -----------------------------------------------------------------------
    // Data
    // -----------------------------------------------------------------------

    /// Formata `Date` no padrão `dd/mm/yyyy`.
    /// Equivalente a `DTOC(d)` com formato brasileiro.
    /// Ex: `Date(19814)` → `"24/03/2024"`
    ///
    /// Usa o algoritmo de Howard Hinnant para conversão de dias desde
    /// 1970-01-01 sem dependências externas.
    pub fn format_date(&self) -> String {
        let days = match self {
            HbValue::Date(d) => *d as i64,
            _                => return String::new(),
        };
        // Algoritmo civil de Hinnant: shift para era iniciando em 1 Mar 0000
        let z   = days + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;                          // dia na era [0, 146096]
        let yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365; // ano na era [0, 399]
        let y   = yoe + era * 400;
        let doy = doe - (365*yoe + yoe/4 - yoe/100);         // dia no ano [0, 365]
        let mp  = (5*doy + 2) / 153;                          // mês interno [0, 11]
        let d   = doy - (153*mp + 2)/5 + 1;                  // dia [1, 31]
        let m   = if mp < 10 { mp + 3 } else { mp - 9 };     // mês [1, 12]
        let y   = if m <= 2 { y + 1 } else { y };
        format!("{:02}/{:02}/{:04}", d, m, y)
    }

    /// Harbour `$` (substring test): `"bc" $ "abcd"` → .T.
    pub fn hb_instr_contains(&self, haystack: &HbValue) -> HbValue {
        match (self, haystack) {
            (HbValue::String(needle), HbValue::String(hay)) => {
                HbValue::Logical(hay.contains(needle.as_str()))
            }
            _ => HbValue::Logical(false),
        }
    }

    /// Type name — mirrors `ValType()` in Harbour.
    pub fn val_type(&self) -> &'static str {
        match self {
            HbValue::Nil       => "U",
            HbValue::Logical(_)=> "L",
            HbValue::Integer(_)
            | HbValue::Float(_)=> "N",
            HbValue::String(_) => "C",
            HbValue::Array(_)  => "A",
            HbValue::Date(_)   => "D",
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions — convenience for Rust literals in generated code
// ---------------------------------------------------------------------------

impl From<i64> for HbValue {
    fn from(n: i64) -> Self { HbValue::Integer(n) }
}
impl From<i32> for HbValue {
    fn from(n: i32) -> Self { HbValue::Integer(n as i64) }
}
impl From<f64> for HbValue {
    fn from(f: f64) -> Self { HbValue::Float(f) }
}
impl From<bool> for HbValue {
    fn from(b: bool) -> Self { HbValue::Logical(b) }
}
impl From<String> for HbValue {
    fn from(s: String) -> Self { HbValue::String(s) }
}
impl From<&str> for HbValue {
    fn from(s: &str) -> Self { HbValue::String(s.to_owned()) }
}
impl From<HbArray> for HbValue {
    fn from(a: HbArray) -> Self { HbValue::Array(a) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_integers() {
        let r = HbValue::Integer(3) + HbValue::Integer(4);
        assert_eq!(r, HbValue::Integer(7));
    }

    #[test]
    fn test_add_strings_concat() {
        let r = HbValue::String("Hello".into()) + HbValue::String(", World".into());
        assert_eq!(r, HbValue::String("Hello, World".into()));
    }

    #[test]
    fn test_div_by_zero_returns_nil() {
        let r = HbValue::Integer(10) / HbValue::Integer(0);
        assert_eq!(r, HbValue::Nil);
    }

    #[test]
    fn test_pow_hb() {
        let r = HbValue::Integer(2).pow_hb(HbValue::Integer(8));
        assert_eq!(r, HbValue::Integer(256));
    }

    #[test]
    fn test_truthy() {
        assert!( HbValue::Logical(true).is_truthy());
        assert!(!HbValue::Logical(false).is_truthy());
        assert!(!HbValue::Nil.is_truthy());
        assert!( HbValue::Integer(1).is_truthy());
        assert!(!HbValue::Integer(0).is_truthy());
    }

    #[test]
    fn test_display_float_no_trailing_zeros() {
        let s = format!("{}", HbValue::Float(3.14));
        assert_eq!(s, "3.14");
    }

    #[test]
    fn test_len() {
        assert_eq!(HbValue::String("hello".into()).len(), 5);
        assert_eq!(HbValue::Nil.len(), 0);
        assert_eq!(HbValue::Integer(99).len(), 0);
    }

    #[test]
    fn test_to_hb_str() {
        assert_eq!(HbValue::String("abc".into()).to_hb_str(), "abc");
        assert_eq!(HbValue::Integer(42).to_hb_str(), "42");
        assert_eq!(HbValue::Logical(true).to_hb_str(), ".T.");
        assert_eq!(HbValue::Nil.to_hb_str(), "");
    }

    #[test]
    fn test_instr() {
        let needle  = HbValue::String("bc".into());
        let haystack = HbValue::String("abcd".into());
        assert_eq!(needle.hb_instr_contains(&haystack), HbValue::Logical(true));
    }

    #[test]
    fn test_val_type() {
        assert_eq!(HbValue::Nil.val_type(),        "U");
        assert_eq!(HbValue::Integer(1).val_type(), "N");
        assert_eq!(HbValue::String("".into()).val_type(), "C");
    }

    #[test]
    fn test_hb_trunc() {
        assert_eq!(HbValue::Float(1.999).hb_trunc(2), HbValue::Float(1.99));
        assert_eq!(HbValue::Float(1.999).hb_trunc(0), HbValue::Integer(1));
        assert_eq!(HbValue::Integer(5).hb_trunc(2),   HbValue::Integer(5));
        assert_eq!(HbValue::Float(-1.999).hb_trunc(2), HbValue::Float(-1.99));
    }

    #[test]
    fn test_hb_round() {
        assert_eq!(HbValue::Float(1.995).hb_round(2), HbValue::Float(2.0));
        assert_eq!(HbValue::Float(1.994).hb_round(2), HbValue::Float(1.99));
        assert_eq!(HbValue::Float(1.5).hb_round(0),   HbValue::Integer(2));
        assert_eq!(HbValue::Integer(7).hb_round(2),   HbValue::Float(7.0));
    }

    #[test]
    fn test_hb_str_format() {
        assert_eq!(HbValue::Float(1234.5).hb_str_format(10, 2), "   1234.50");
        assert_eq!(HbValue::Integer(42).hb_str_format(6, 0),    "    42");
        assert_eq!(HbValue::Float(3.1).hb_str_format(5, 2),     " 3.10");
    }

    #[test]
    fn test_format_date() {
        // 2024-03-24 = 19_806 dias desde 1970-01-01
        // python: (date(2024,3,24) - date(1970,1,1)).days == 19806
        assert_eq!(HbValue::Date(19806).format_date(), "24/03/2024");
        // Epoch origin
        assert_eq!(HbValue::Date(0).format_date(), "01/01/1970");
        assert_eq!(HbValue::Nil.format_date(), "");
    }
}
