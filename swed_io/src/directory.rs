//! Harbour `DIRECTORY()` — list directory contents.
//!
//! Returns an array of sub-arrays, each with 5 elements:
//! `{ cFileName, nSize, cDate, cTime, cAttr }`.

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use swed_co::NativeFunction;
use swed_rt::{HbArray, HbValue};

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Decompose days-since-1970-01-01 → (year, month, day) [Hinnant].
fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z   = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y   = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d   = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m   = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y   = if m <= 2 { y + 1 } else { y } as i32;
    (y, m, d)
}

/// Format a `SystemTime` as `("DD/MM/YYYY", "HH:MM:SS")`.
fn format_mtime(t: SystemTime) -> (String, String) {
    let secs = t.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let days = (secs / 86_400) as i64;
    let rem  = secs % 86_400;
    let (y, m, d) = civil_from_days(days);
    let h  = rem / 3600;
    let mi = (rem % 3600) / 60;
    let s  = rem % 60;
    (format!("{d:02}/{m:02}/{y:04}"), format!("{h:02}:{mi:02}:{s:02}"))
}

/// Simple glob: supports `*`, `*.ext`, `prefix*`.
/// Case-insensitive on all platforms for Harbour compatibility.
fn matches_glob(name: &str, pattern: &str) -> bool {
    let name    = name.to_ascii_lowercase();
    let pattern = pattern.to_ascii_lowercase();

    if pattern == "*" || pattern == "*.*" {
        return true;
    }
    if let Some(ext) = pattern.strip_prefix("*.") {
        return name.ends_with(&format!(".{ext}"));
    }
    if let Some(prefix) = pattern.strip_suffix('*') {
        return name.starts_with(prefix);
    }
    name == pattern
}

/// Build one `{ cFileName, nSize, cDate, cTime, cAttr }` sub-array.
fn entry_array(name: &str, size: u64, mtime: SystemTime, is_dir: bool) -> HbValue {
    let (date_s, time_s) = format_mtime(mtime);
    let attr = if is_dir { "D" } else { "A" };

    let mut a = HbArray::new();
    a.hb_aadd(HbValue::String(name.to_owned()));
    a.hb_aadd(HbValue::Integer(size as i64));
    a.hb_aadd(HbValue::String(date_s));
    a.hb_aadd(HbValue::String(time_s));
    a.hb_aadd(HbValue::String(attr.to_owned()));
    HbValue::Array(a)
}

// ---------------------------------------------------------------------------
// Directory( [cMask [, cAttr]] ) → aDirectory
// ---------------------------------------------------------------------------

/// `DIRECTORY([cMask [, cAttr]])` — list directory entries.
///
/// `cMask` supports simple glob patterns: `"*"`, `"*.prg"`, `"data*"`.
/// If omitted, defaults to `"*"` (all files in the current directory).
///
/// `cAttr` is accepted for Harbour compatibility; when it contains `"D"`,
/// subdirectories are included in the result.
///
/// Each element of the returned array is a 5-element sub-array:
/// ```text
/// { cFileName, nSize, cDate "DD/MM/YYYY", cTime "HH:MM:SS", cAttr }
/// ```
pub fn hb_directory(mask: HbValue, attr: HbValue) -> HbValue {
    let pattern = match mask {
        HbValue::String(s) if !s.is_empty() => s,
        _ => "*".to_owned(),
    };
    let include_dirs = matches!(&attr, HbValue::String(s) if s.to_ascii_uppercase().contains('D'));

    // Split pattern into directory prefix and file mask
    let (dir_path, file_mask) = {
        let p = pattern.replace('\\', "/");
        match p.rfind('/') {
            Some(pos) => (p[..=pos].to_owned(), p[pos + 1..].to_owned()),
            None      => (".".to_owned(),        p),
        }
    };

    let mut result = HbArray::new();

    let entries = match fs::read_dir(&dir_path) {
        Ok(e)  => e,
        Err(_) => return HbValue::Array(result),
    };

    for entry in entries.flatten() {
        let fname = entry.file_name().to_string_lossy().into_owned();
        if !matches_glob(&fname, &file_mask) {
            continue;
        }
        let meta = match entry.metadata() {
            Ok(m)  => m,
            Err(_) => continue,
        };
        let is_dir = meta.is_dir();
        if is_dir && !include_dirs {
            continue;
        }
        let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        result.hb_aadd(entry_array(&fname, meta.len(), mtime, is_dir));
    }

    HbValue::Array(result)
}

// ---------------------------------------------------------------------------
// NativeFunction implementation
// ---------------------------------------------------------------------------

/// Registry struct for `DIRECTORY`.
pub struct Directory;
impl NativeFunction<HbValue> for Directory {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        hb_directory(
            it.next().unwrap_or(HbValue::Nil),
            it.next().unwrap_or(HbValue::Nil),
        )
    }
    fn name(&self)  -> &'static str      { "DIRECTORY" }
    fn arity(&self) -> (usize, usize) { (0, 2) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> HbValue { HbValue::String(v.into()) }

    #[test]
    fn directory_returns_array() {
        let r = hb_directory(HbValue::Nil, HbValue::Nil);
        assert!(matches!(r, HbValue::Array(_)));
    }

    #[test]
    fn directory_entries_have_5_fields() {
        // List the workspace root — at minimum Cargo.toml exists
        let r = hb_directory(s("../../*"), HbValue::Nil);
        if let HbValue::Array(arr) = r {
            for entry in arr.iter() {
                if let HbValue::Array(sub) = entry {
                    assert_eq!(sub.len(), 5, "each entry must have 5 fields");
                }
            }
        }
    }

    #[test]
    fn directory_glob_prg_extension() {
        // Create a temp .prg file and verify it appears
        let tmp_dir = std::env::temp_dir();
        let prg = tmp_dir.join("swed_test_dir.prg");
        std::fs::write(&prg, b"").unwrap();

        let mask = format!("{}/*.prg", tmp_dir.display());
        let r = hb_directory(s(&mask), HbValue::Nil);

        let found = if let HbValue::Array(arr) = r {
            arr.iter().any(|e| {
                if let HbValue::Array(sub) = e {
                    matches!(sub.hb_get(1), HbValue::String(n) if n.ends_with(".prg"))
                } else { false }
            })
        } else { false };

        let _ = std::fs::remove_file(&prg);
        assert!(found, "*.prg glob should have matched swed_test_dir.prg");
    }

    #[test]
    fn directory_entry_fields_types() {
        // Create a known file
        let tmp = std::env::temp_dir().join("swed_dir_fields.tmp");
        std::fs::write(&tmp, b"hello").unwrap();

        let mask = format!("{}/swed_dir_fields.tmp", std::env::temp_dir().display());
        let r = hb_directory(s(&mask), HbValue::Nil);

        if let HbValue::Array(arr) = r {
            if let Some(HbValue::Array(sub)) = arr.iter().next() {
                assert!(matches!(sub.hb_get(1), HbValue::String(_)),  "[1] must be string (name)");
                assert!(matches!(sub.hb_get(2), HbValue::Integer(_)), "[2] must be integer (size)");
                assert!(matches!(sub.hb_get(3), HbValue::String(_)),  "[3] must be string (date)");
                assert!(matches!(sub.hb_get(4), HbValue::String(_)),  "[4] must be string (time)");
                assert!(matches!(sub.hb_get(5), HbValue::String(_)),  "[5] must be string (attr)");
            }
        }
        let _ = std::fs::remove_file(&tmp);
    }

    #[test]
    fn native_fn_trait_directory() {
        assert_eq!(Directory.name(), "DIRECTORY");
        assert_eq!(Directory.arity(), (0, 2));
    }
}
