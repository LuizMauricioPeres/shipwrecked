//! Harbour low-level file I/O: FCreate, FOpen, FClose, FWrite, FErase, FRename, FError.
//!
//! File handles are stored in a process-wide registry (`OnceLock<Mutex<…>>`).
//! Handle IDs start at 3 to avoid colliding with stdin/stdout/stderr.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, OnceLock};

use swed_co::NativeFunction;
use swed_rt::HbValue;

// ---------------------------------------------------------------------------
// Handle registry
// ---------------------------------------------------------------------------

struct HandleTable {
    map:     HashMap<i32, File>,
    next_id: i32,
}

static HANDLES: OnceLock<Mutex<HandleTable>> = OnceLock::new();
static LAST_ERR: OnceLock<Mutex<i32>> = OnceLock::new();

fn handles() -> &'static Mutex<HandleTable> {
    HANDLES.get_or_init(|| {
        Mutex::new(HandleTable { map: HashMap::new(), next_id: 3 })
    })
}

fn set_error(code: i32) {
    *LAST_ERR.get_or_init(|| Mutex::new(0)).lock().unwrap() = code;
}

fn os_error(e: &std::io::Error) -> i32 {
    e.raw_os_error().unwrap_or(-1)
}

// ---------------------------------------------------------------------------
// FCreate( cFile [, nAttr] ) → nHandle
// ---------------------------------------------------------------------------

/// `FCREATE(cFile [, nAttr])` — create or truncate a file.
///
/// Returns a numeric file handle on success, `-1` on error.
/// `FError()` is set to the OS error code on failure.
///
/// `nAttr` is accepted for Harbour compatibility but ignored (file permissions
/// use the OS default).
pub fn hb_fcreate(name: HbValue, _attr: HbValue) -> HbValue {
    let path = match name {
        HbValue::String(s) => s,
        _ => return HbValue::Integer(-1),
    };
    match File::create(&path) {
        Ok(f) => {
            set_error(0);
            let mut h = handles().lock().unwrap();
            let id = h.next_id;
            h.next_id += 1;
            h.map.insert(id, f);
            HbValue::Integer(id as i64)
        }
        Err(e) => {
            set_error(os_error(&e));
            HbValue::Integer(-1)
        }
    }
}

// ---------------------------------------------------------------------------
// FOpen( cFile [, nMode] ) → nHandle
// ---------------------------------------------------------------------------

/// `FOPEN(cFile [, nMode])` — open an existing file.
///
/// `nMode`:
/// - `0` (default) — read-only
/// - `1` — write-only
/// - `2` — read/write
///
/// Returns a handle on success, `-1` on error.
pub fn hb_fopen(name: HbValue, mode: HbValue) -> HbValue {
    let path = match name {
        HbValue::String(s) => s,
        _ => return HbValue::Integer(-1),
    };
    let (read, write) = match mode {
        HbValue::Integer(1) => (false, true),
        HbValue::Integer(2) => (true,  true),
        _                   => (true,  false), // 0 or omitted → read-only
    };
    match OpenOptions::new().read(read).write(write).open(&path) {
        Ok(f) => {
            set_error(0);
            let mut h = handles().lock().unwrap();
            let id = h.next_id;
            h.next_id += 1;
            h.map.insert(id, f);
            HbValue::Integer(id as i64)
        }
        Err(e) => {
            set_error(os_error(&e));
            HbValue::Integer(-1)
        }
    }
}

// ---------------------------------------------------------------------------
// FClose( nHandle ) → lSuccess
// ---------------------------------------------------------------------------

/// `FCLOSE(nHandle)` — close an open file handle.
///
/// The `File` is dropped (and thus flushed + closed) when removed from the
/// registry. Returns `.T.` on success, `.F.` if the handle was not found.
pub fn hb_fclose(handle: HbValue) -> HbValue {
    let id = match handle {
        HbValue::Integer(n) => n as i32,
        _ => return HbValue::Logical(false),
    };
    let removed = handles().lock().unwrap().map.remove(&id).is_some();
    set_error(if removed { 0 } else { 6 }); // 6 = ERROR_INVALID_HANDLE
    HbValue::Logical(removed)
}

// ---------------------------------------------------------------------------
// FWrite( nHandle, cStr [, nBytes] ) → nBytesWritten
// ---------------------------------------------------------------------------

/// `FWRITE(nHandle, cStr [, nBytes])` — write bytes to an open file.
///
/// If `nBytes` is omitted or zero, writes the entire string.
/// Returns the number of bytes actually written.
pub fn hb_fwrite(handle: HbValue, data: HbValue, bytes: HbValue) -> HbValue {
    let id = match handle {
        HbValue::Integer(n) => n as i32,
        _ => return HbValue::Integer(0),
    };
    let text = match data {
        HbValue::String(s) => s,
        _ => return HbValue::Integer(0),
    };
    let buf = text.as_bytes();
    let slice = match bytes {
        HbValue::Integer(n) if n > 0 && (n as usize) < buf.len() => &buf[..n as usize],
        _ => buf,
    };
    let mut h = handles().lock().unwrap();
    match h.map.get_mut(&id) {
        Some(f) => match f.write(slice) {
            Ok(n)  => { set_error(0); HbValue::Integer(n as i64) }
            Err(e) => { set_error(os_error(&e)); HbValue::Integer(0) }
        },
        None => {
            set_error(6);
            HbValue::Integer(0)
        }
    }
}

// ---------------------------------------------------------------------------
// FErase( cFile ) → 0 | -1
// ---------------------------------------------------------------------------

/// `FERASE(cFile)` — delete a file from disk.
///
/// Returns `0` on success, `-1` on error (Harbour convention).
pub fn hb_ferase(name: HbValue) -> HbValue {
    let path = match name {
        HbValue::String(s) => s,
        _ => return HbValue::Integer(-1),
    };
    match std::fs::remove_file(&path) {
        Ok(()) => { set_error(0); HbValue::Integer(0) }
        Err(e) => { set_error(os_error(&e)); HbValue::Integer(-1) }
    }
}

// ---------------------------------------------------------------------------
// FRename( cOldName, cNewName ) → 0 | -1
// ---------------------------------------------------------------------------

/// `FRENAME(cOldName, cNewName)` — rename or move a file.
///
/// Returns `0` on success, `-1` on error.
pub fn hb_frename(old_name: HbValue, new_name: HbValue) -> HbValue {
    let old = match old_name { HbValue::String(s) => s, _ => return HbValue::Integer(-1) };
    let new = match new_name { HbValue::String(s) => s, _ => return HbValue::Integer(-1) };
    match std::fs::rename(&old, &new) {
        Ok(()) => { set_error(0); HbValue::Integer(0) }
        Err(e) => { set_error(os_error(&e)); HbValue::Integer(-1) }
    }
}

// ---------------------------------------------------------------------------
// FError() → nErrorCode
// ---------------------------------------------------------------------------

/// `FERROR()` — return the OS error code from the last file operation.
///
/// `0` means no error. Mirrors `errno` on POSIX systems.
pub fn hb_ferror() -> HbValue {
    let code = *LAST_ERR.get_or_init(|| Mutex::new(0)).lock().unwrap();
    HbValue::Integer(code as i64)
}

// ---------------------------------------------------------------------------
// NativeFunction implementations
// ---------------------------------------------------------------------------

/// Registry struct for `FCREATE`.
pub struct FCreate;
impl NativeFunction<HbValue> for FCreate {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        hb_fcreate(it.next().unwrap_or(HbValue::Nil), it.next().unwrap_or(HbValue::Nil))
    }
    fn name(&self)  -> &'static str      { "FCREATE" }
    fn arity(&self) -> (usize, usize) { (1, 2) }
}

/// Registry struct for `FOPEN`.
pub struct FOpen;
impl NativeFunction<HbValue> for FOpen {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        hb_fopen(it.next().unwrap_or(HbValue::Nil), it.next().unwrap_or(HbValue::Nil))
    }
    fn name(&self)  -> &'static str      { "FOPEN" }
    fn arity(&self) -> (usize, usize) { (1, 2) }
}

/// Registry struct for `FCLOSE`.
pub struct FClose;
impl NativeFunction<HbValue> for FClose {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_fclose(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self)  -> &'static str      { "FCLOSE" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `FWRITE`.
pub struct FWrite;
impl NativeFunction<HbValue> for FWrite {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        hb_fwrite(
            it.next().unwrap_or(HbValue::Nil),
            it.next().unwrap_or(HbValue::Nil),
            it.next().unwrap_or(HbValue::Nil),
        )
    }
    fn name(&self)  -> &'static str      { "FWRITE" }
    fn arity(&self) -> (usize, usize) { (2, 3) }
}

/// Registry struct for `FERASE`.
pub struct FErase;
impl NativeFunction<HbValue> for FErase {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_ferase(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self)  -> &'static str      { "FERASE" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `FRENAME`.
pub struct FRename;
impl NativeFunction<HbValue> for FRename {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let mut it = args.into_iter();
        hb_frename(it.next().unwrap_or(HbValue::Nil), it.next().unwrap_or(HbValue::Nil))
    }
    fn name(&self)  -> &'static str      { "FRENAME" }
    fn arity(&self) -> (usize, usize) { (2, 2) }
}

/// Registry struct for `FERROR`.
pub struct FError;
impl NativeFunction<HbValue> for FError {
    fn call(&self, _args: Vec<HbValue>) -> HbValue { hb_ferror() }
    fn name(&self)  -> &'static str      { "FERROR" }
    fn arity(&self) -> (usize, usize) { (0, 0) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn tmp(name: &str) -> String {
        std::env::temp_dir().join(name).to_string_lossy().into_owned()
    }

    fn s(v: &str) -> HbValue { HbValue::String(v.into()) }
    fn i(n: i64)  -> HbValue { HbValue::Integer(n) }

    #[test]
    fn fcreate_and_fclose() {
        let path = tmp("swed_io_test_create.tmp");
        let h = hb_fcreate(s(&path), HbValue::Nil);
        assert!(matches!(h, HbValue::Integer(n) if n >= 3));
        let ok = hb_fclose(h);
        assert_eq!(ok, HbValue::Logical(true));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn fwrite_creates_content() {
        let path = tmp("swed_io_test_write.tmp");
        let h = hb_fcreate(s(&path), HbValue::Nil);
        let written = hb_fwrite(h.clone(), s("Hello, SWed!"), HbValue::Nil);
        assert_eq!(written, HbValue::Integer(12));
        hb_fclose(h);

        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "Hello, SWed!");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn fwrite_partial_bytes() {
        let path = tmp("swed_io_test_partial.tmp");
        let h = hb_fcreate(s(&path), HbValue::Nil);
        let written = hb_fwrite(h.clone(), s("Hello"), i(3));
        assert_eq!(written, HbValue::Integer(3));
        hb_fclose(h);

        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "Hel");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn ferase_deletes_file() {
        let path = tmp("swed_io_test_erase.tmp");
        std::fs::write(&path, b"x").unwrap();
        let r = hb_ferase(s(&path));
        assert_eq!(r, HbValue::Integer(0));
        assert!(!std::path::Path::new(&path).exists());
    }

    #[test]
    fn ferase_missing_file_returns_minus1() {
        let r = hb_ferase(s("/tmp/swed_io_does_not_exist_xyz.tmp"));
        assert_eq!(r, HbValue::Integer(-1));
        assert!(matches!(hb_ferror(), HbValue::Integer(n) if n != 0));
    }

    #[test]
    fn frename_moves_file() {
        let src = tmp("swed_io_rename_src.tmp");
        let dst = tmp("swed_io_rename_dst.tmp");
        std::fs::write(&src, b"data").unwrap();
        let r = hb_frename(s(&src), s(&dst));
        assert_eq!(r, HbValue::Integer(0));
        assert!(!std::path::Path::new(&src).exists());
        assert!(std::path::Path::new(&dst).exists());
        let _ = std::fs::remove_file(&dst);
    }

    #[test]
    fn fopen_read_existing() {
        let path = tmp("swed_io_test_fopen.tmp");
        std::fs::write(&path, b"hello").unwrap();
        let h = hb_fopen(s(&path), HbValue::Nil); // mode 0 = read
        assert!(matches!(h, HbValue::Integer(n) if n >= 3));
        // Verify we can read the file directly (handle holds the fd)
        {
            let mut table = handles().lock().unwrap();
            let id = match h { HbValue::Integer(n) => n as i32, _ => panic!() };
            let f = table.map.get_mut(&id).unwrap();
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            assert_eq!(buf, "hello");
        }
        hb_fclose(h);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn fclose_invalid_handle_returns_false() {
        let r = hb_fclose(HbValue::Integer(9999));
        assert_eq!(r, HbValue::Logical(false));
    }

    #[test]
    fn ferror_zero_after_success() {
        let path = tmp("swed_io_test_err.tmp");
        let h = hb_fcreate(s(&path), HbValue::Nil);
        hb_fclose(h);
        assert_eq!(hb_ferror(), HbValue::Integer(0));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn native_fn_trait_fcreate() {
        assert_eq!(FCreate.name(), "FCREATE");
        assert_eq!(FCreate.arity(), (1, 2));
    }

    #[test]
    fn native_fn_trait_frename() {
        assert_eq!(FRename.name(), "FRENAME");
        assert_eq!(FRename.arity(), (2, 2));
    }
}
