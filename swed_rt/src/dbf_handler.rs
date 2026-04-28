// swed_rt/src/dbf_handler.rs
//! Low-level dBase III/IV reader with `DataNavigator` integration.
//!
//! ## Wire Format
//!
//! | Offset      | Size     | Content                                              |
//! |-------------|----------|------------------------------------------------------|
//! | 0           | 1        | Version: `0x03` = dBase III, `0x04` = dBase IV       |
//! | 1–3         | 3        | Last update (YY offset from 1900, MM, DD)            |
//! | 4–7         | 4        | Record count (u32 LE, includes deleted records)      |
//! | 8–9         | 2        | Header size in bytes (u16 LE)                        |
//! | 10–11       | 2        | Record size in bytes (u16 LE, deletion flag + fields)|
//! | 12–31       | 20       | Reserved                                             |
//! | 32+         | 32 × n   | Field descriptors (32 bytes each)                    |
//! | (variable)  | 1        | Header terminator `0x0D`                             |
//! | `header_size` | `record_size × n` | Records                             |
//!
//! Each record begins with a deletion byte: `0x20` = active, `0x2A` = deleted.
//!
//! ## Concurrency
//!
//! Files are opened with `read(true).write(false)` — no exclusive lock is
//! acquired on POSIX systems, so a legacy dBase/Harbour process can hold the
//! file open in shared mode simultaneously.
//!
//! ## Memory model
//!
//! `DbfReader` uses a 64 KiB `BufReader` for sequential reads within each
//! record.  Only the current record is kept in RAM; the rest of the file
//! is read on demand via `Seek`.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::Arc;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::row::{DataNavigator, FieldMeta, Row, RowProxy, RowSchema};
use crate::value::HbValue;

// ── Error ─────────────────────────────────────────────────────────────────────

/// All errors that can originate from the DBF reader layer.
#[derive(Debug, thiserror::Error)]
pub enum DbfError {
    /// Underlying OS / filesystem I/O error.
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),

    /// Byte 0 of the file is not `0x03` (dBase III) or `0x04` (dBase IV).
    #[error("unsupported DBF version 0x{0:02X} (expected 0x03 or 0x04)")]
    UnsupportedVersion(u8),

    /// Header geometry is impossible: `header_size < 32` or `record_size == 0`.
    #[error("invalid header geometry: header_size={0}, record_size={1}")]
    InvalidHeader(u16, u16),

    /// Requested 0-based record index is beyond `record_count`.
    #[error("record index {0} out of range (file has {1} records)")]
    OutOfRange(usize, u32),

    /// Record deletion byte is `0x2A`.  The public `read_record` API surfaces
    /// this as an error so callers can implement `SET DELETED ON/OFF` logic.
    #[error("record {0} is marked for deletion (flag 0x2A)")]
    RecordDeleted(usize),

    /// A field value could not be decoded to the expected type.
    #[error("field decode error: {0}")]
    Decode(String),
}

// ── DbfHeader ─────────────────────────────────────────────────────────────────

/// Parsed dBase III/IV file header.
///
/// Parsed manually via `byteorder` (Little Endian) rather than
/// `#[repr(C, packed)]`, which would require `unsafe` for field access.
#[derive(Debug, Clone)]
pub struct DbfHeader {
    /// `0x03` = dBase III, `0x04` = dBase IV.
    pub version: u8,
    /// Last update as `(full_year, month, day)` — raw byte + 1900 for year.
    pub last_update: (u16, u8, u8),
    /// Total records in file, **including** deleted ones.
    pub record_count: u32,
    /// Byte offset of the first record; also the total header block size.
    pub header_size: u16,
    /// Size of one record in bytes (deletion flag + sum of all field widths).
    pub record_size: u16,
}

impl DbfHeader {
    fn parse<R: Read>(r: &mut R) -> Result<Self, DbfError> {
        let version = r.read_u8()?;
        if version != 0x03 && version != 0x04 {
            return Err(DbfError::UnsupportedVersion(version));
        }

        let yy = r.read_u8()?;
        let mm = r.read_u8()?;
        let dd = r.read_u8()?;

        let record_count = r.read_u32::<LittleEndian>()?;
        let header_size  = r.read_u16::<LittleEndian>()?;
        let record_size  = r.read_u16::<LittleEndian>()?;

        if header_size < 32 || record_size == 0 {
            return Err(DbfError::InvalidHeader(header_size, record_size));
        }

        // Skip reserved bytes 12–31 (20 bytes).
        let mut reserved = [0u8; 20];
        r.read_exact(&mut reserved)?;

        Ok(DbfHeader {
            version,
            last_update: (1900 + u16::from(yy), mm, dd),
            record_count,
            header_size,
            record_size,
        })
    }
}

// ── DbfField ──────────────────────────────────────────────────────────────────

/// Metadata for a single column from the DBF header (32-byte descriptor block).
#[derive(Debug, Clone)]
pub struct DbfField {
    /// Field name in ASCII-uppercase, stripped of null-padding.
    pub name: String,
    /// xBase type code: `C` = Character, `N` = Numeric, `L` = Logical,
    /// `D` = Date.  Unrecognised types (e.g. `M` Memo) decode to `Nil`.
    pub field_type: char,
    /// Total column width in bytes as declared in the header.
    pub length: u8,
    /// Decimal places (meaningful only for `N` fields).
    pub decimals: u8,
}

impl DbfField {
    /// Returns `Ok(None)` when the header terminator byte `0x0D` is encountered.
    fn parse<R: Read>(r: &mut R) -> Result<Option<Self>, DbfError> {
        let mut name_buf = [0u8; 11];
        r.read_exact(&mut name_buf)?;

        if name_buf[0] == 0x0D {
            return Ok(None); // header terminator
        }

        let nul = name_buf.iter().position(|&b| b == 0).unwrap_or(11);
        let name = std::str::from_utf8(&name_buf[..nul])
            .unwrap_or("")
            .trim()
            .to_ascii_uppercase();

        let field_type = r.read_u8()? as char;

        let mut addr = [0u8; 4]; // field data address — reserved in dBase III
        r.read_exact(&mut addr)?;

        let length   = r.read_u8()?;
        let decimals = r.read_u8()?;

        let mut reserved = [0u8; 14];
        r.read_exact(&mut reserved)?;

        Ok(Some(DbfField { name, field_type, length, decimals }))
    }
}

// ── FieldValue ────────────────────────────────────────────────────────────────

/// Native Rust representation of a decoded DBF field value.
///
/// Intentionally separate from `HbValue` so the low-level parser layer
/// carries no Harbour runtime semantics and can be tested independently.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    /// `C` — character string; trailing spaces stripped; CP-1252 → UTF-8.
    Character(String),
    /// `N` — numeric; always widened to `f64` regardless of decimal count.
    Numeric(f64),
    /// `L` — logical (`T`/`Y` = true, `F`/`N` = false).
    Logical(bool),
    /// `D` — date encoded as days since 1970-01-01, matching `HbValue::Date`.
    Date(i32),
    /// Empty / undefined: blank numeric, empty date, unrecognised type.
    Nil,
}

impl From<FieldValue> for HbValue {
    fn from(fv: FieldValue) -> Self {
        match fv {
            FieldValue::Character(s) => HbValue::String(s),
            FieldValue::Numeric(n)   => HbValue::Float(n),
            FieldValue::Logical(b)   => HbValue::Logical(b),
            FieldValue::Date(d)      => HbValue::Date(d),
            FieldValue::Nil          => HbValue::Nil,
        }
    }
}

// ── DbfReader ─────────────────────────────────────────────────────────────────

/// Low-level dBase III/IV file reader.
///
/// Seek-on-demand: only one record is read per call to `read_record`; the
/// rest of the file stays on disk.  A 64 KiB `BufReader` amortises the
/// sequential reads within each record's field data.
pub struct DbfReader {
    reader: BufReader<File>,
    /// Parsed file header (version, geometry, record count).
    pub header: DbfHeader,
    /// Ordered field descriptors — same order as stored on disk.
    pub fields: Vec<DbfField>,
}

impl DbfReader {
    /// Open a `.dbf` file, validate its header, and parse all field descriptors.
    ///
    /// The file descriptor is opened with `read(true)` only — no write or
    /// create flags — so no exclusive OS-level lock is held on POSIX systems.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DbfError> {
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path)?;

        let mut reader = BufReader::with_capacity(65_536, file);
        let header = DbfHeader::parse(&mut reader)?;

        // Upper bound: field slots between the 32-byte fixed header and
        // header_size.  The loop breaks early on the 0x0D terminator.
        let max_fields = usize::from(header.header_size.saturating_sub(32)) / 32;
        let mut fields = Vec::with_capacity(max_fields);

        for _ in 0..max_fields {
            match DbfField::parse(&mut reader)? {
                Some(f) => fields.push(f),
                None    => break,
            }
        }

        Ok(DbfReader { reader, header, fields })
    }

    /// Read one record by 0-based index, returning its decoded field map.
    ///
    /// Returns `Err(DbfError::RecordDeleted)` when the deletion byte is
    /// `0x2A`.  Callers that implement `SET DELETED OFF` should use
    /// `read_record_raw` (available to crate-internal code) instead.
    pub fn read_record(&mut self, index: usize) -> Result<HashMap<String, FieldValue>, DbfError> {
        let (deleted, map) = self.read_record_raw(index)?;
        if deleted {
            Err(DbfError::RecordDeleted(index))
        } else {
            Ok(map)
        }
    }

    /// Build a `RowSchema` from the parsed field descriptors.
    ///
    /// Call this once during `WorkArea` registration and cache the result
    /// via `Arc` for O(1) field resolution inside hot navigation loops.
    pub fn build_schema(&self) -> RowSchema {
        RowSchema::new(
            self.fields.iter().map(|f| FieldMeta {
                name:     f.name.clone(),
                dbf_type: f.field_type,
                size:     u16::from(f.length),
                decimals: f.decimals,
            }).collect(),
        )
    }

    /// Reads a record including deleted ones, returning `(is_deleted, fields)`.
    ///
    /// Used internally by `DbfNavigator` to populate the look-aside cache
    /// while still exposing the deletion flag via `is_deleted()`.
    pub(crate) fn read_record_raw(
        &mut self,
        index: usize,
    ) -> Result<(bool, HashMap<String, FieldValue>), DbfError> {
        if index as u64 >= u64::from(self.header.record_count) {
            return Err(DbfError::OutOfRange(index, self.header.record_count));
        }

        let offset = u64::from(self.header.header_size)
            + index as u64 * u64::from(self.header.record_size);

        self.reader.seek(SeekFrom::Start(offset))?;

        let deletion_byte = self.reader.read_u8()?;
        let is_deleted = deletion_byte == 0x2A;

        let mut map = HashMap::with_capacity(self.fields.len());
        for field in &self.fields {
            let mut buf = vec![0u8; usize::from(field.length)];
            self.reader.read_exact(&mut buf)?;
            map.insert(field.name.clone(), decode_field(field, &buf)?);
        }

        Ok((is_deleted, map))
    }
}

// ── Field decoders ────────────────────────────────────────────────────────────

fn decode_field(field: &DbfField, raw: &[u8]) -> Result<FieldValue, DbfError> {
    match field.field_type {
        'C' => decode_character(raw),
        'N' => decode_numeric(raw),
        'L' => Ok(decode_logical(raw)),
        'D' => decode_date(raw),
        _   => Ok(FieldValue::Nil), // M (Memo), B (Blob) — Sprint 4+
    }
}

fn decode_character(raw: &[u8]) -> Result<FieldValue, DbfError> {
    let s = if raw.is_ascii() {
        std::str::from_utf8(raw)
            .unwrap_or("")
            .trim_end_matches(' ')
            .to_owned()
    } else {
        // Inline CP-1252 → UTF-8 decode; avoids adding encoding_rs to swed_rt.
        cp1252_decode(raw).trim_end_matches(' ').to_owned()
    };
    Ok(FieldValue::Character(s))
}

fn decode_numeric(raw: &[u8]) -> Result<FieldValue, DbfError> {
    let s = std::str::from_utf8(raw)
        .map_err(|_| DbfError::Decode("N field contains non-ASCII bytes".into()))?
        .trim();
    if s.is_empty() || s.starts_with('*') {
        // '*' is the dBase overflow marker; empty means uninitialized.
        return Ok(FieldValue::Nil);
    }
    s.parse::<f64>()
        .map(FieldValue::Numeric)
        .map_err(|e| DbfError::Decode(format!("N field `{s}` is not a number: {e}")))
}

fn decode_logical(raw: &[u8]) -> FieldValue {
    match raw.first().copied().unwrap_or(b' ') {
        b'T' | b't' | b'Y' | b'y' => FieldValue::Logical(true),
        b'F' | b'f' | b'N' | b'n' => FieldValue::Logical(false),
        _ => FieldValue::Nil, // '?' = uninitialized in dBase
    }
}

/// Parses an 8-byte ASCII date (`YYYYMMDD`) to days since 1970-01-01.
fn decode_date(raw: &[u8]) -> Result<FieldValue, DbfError> {
    if raw.len() < 8 {
        return Ok(FieldValue::Nil);
    }
    let s = std::str::from_utf8(&raw[..8])
        .map_err(|_| DbfError::Decode("D field contains non-ASCII bytes".into()))?
        .trim();
    if s.len() < 8 || s == "00000000" || s.chars().all(|c| c == ' ') {
        return Ok(FieldValue::Nil);
    }
    let y: i64 = s[..4].parse()
        .map_err(|_| DbfError::Decode(format!("date year in `{s}`")))?;
    let m: i64 = s[4..6].parse()
        .map_err(|_| DbfError::Decode(format!("date month in `{s}`")))?;
    let d: i64 = s[6..8].parse()
        .map_err(|_| DbfError::Decode(format!("date day in `{s}`")))?;

    let days = civil_to_epoch_days(y, m, d);
    #[allow(clippy::cast_possible_truncation)]
    Ok(FieldValue::Date(days as i32))
}

/// Howard Hinnant civil-date algorithm: `(year, month 1–12, day 1–31)` →
/// days since 1970-01-01.  Works for any proleptic Gregorian date.
///
/// Mirrors the inverse algorithm already used in `HbValue::format_date()`,
/// keeping `swed_rt` free of `chrono` as a dependency.
fn civil_to_epoch_days(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Decodes a CP-1252 (Windows-1252) byte slice to UTF-8 without external crates.
///
/// * `0x00–0x7F` — identical to ASCII / UTF-8.
/// * `0x80–0x9F` — special Unicode mappings via lookup table.
/// * `0xA0–0xFF` — Latin-1 Supplement; byte value equals the Unicode code point.
fn cp1252_decode(raw: &[u8]) -> String {
    // Unicode values for CP-1252 bytes 0x80–0x9F.
    const MAP: [char; 32] = [
        '€',        '\u{FFFD}', '‚',        'ƒ',        '„',        '…',
        '†',        '‡',        'ˆ',        '‰',        'Š',        '‹',
        'Œ',        '\u{FFFD}', 'Ž',        '\u{FFFD}', '\u{FFFD}', '\u{2018}',
        '\u{2019}', '\u{201C}', '\u{201D}', '•',        '–',        '—',
        '˜',        '™',        'š',        '›',        'œ',        '\u{FFFD}',
        'ž',        'Ÿ',
    ];
    raw.iter().map(|&b| match b {
        0x00..=0x7F => b as char,
        0x80..=0x9F => MAP[usize::from(b - 0x80)],
        _           => b as char, // 0xA0–0xFF: byte == Unicode scalar
    }).collect()
}

// ── DbfNavigator — DataNavigator implementation ────────────────────────────────

/// `DataNavigator` backed by a real `.dbf` file on disk.
///
/// Navigation is seek-based: `goto()` and `skip()` seek to the target record,
/// read it into a one-record look-aside cache, and return.  `current_row()`
/// clones from that cache — no I/O on field access inside hot loops.
///
/// Deleted records (flag `0x2A`) have their field data fully decoded and
/// available via `current_row()`, enabling `SET DELETED OFF` semantics.
/// Use `is_deleted()` to test the deletion flag for the current position.
///
/// Write-back (`REPLACE`, `APPEND`) is planned for Sprint 4.
pub struct DbfNavigator {
    reader:  DbfReader,
    schema:  Arc<RowSchema>,
    pos:     usize,
    /// Cached row for the current position — populated by `goto` / `skip`.
    current: Option<Row>,
    /// True when the record at `pos` has the deletion flag set.
    deleted: bool,
}

impl DbfNavigator {
    /// Open a `.dbf` file and position the cursor at record 0.
    ///
    /// # Errors
    ///
    /// Returns `DbfError` if the file cannot be opened, has an invalid header,
    /// or the first record cannot be read.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DbfError> {
        let reader = DbfReader::open(path)?;
        let schema = Arc::new(reader.build_schema());
        let mut nav = DbfNavigator {
            reader,
            schema,
            pos: 0,
            current: None,
            deleted: false,
        };
        // Pre-load record 0 so `current_row()` works without a prior `goto()`.
        if nav.reader.header.record_count > 0 {
            nav.refresh_current();
        }
        Ok(nav)
    }

    /// Returns `true` when the record at the current position is marked deleted.
    ///
    /// Mirrors the Harbour `DELETED()` function.  The corresponding row is
    /// still accessible via `current_row()` (all-`Nil` values) so that
    /// callers implementing `SET DELETED OFF` can still display field data.
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }

    /// Seek to `self.pos`, read the record, and populate `self.current`.
    /// Called after every navigation operation that changes `self.pos`.
    fn refresh_current(&mut self) {
        match self.reader.read_record_raw(self.pos) {
            Ok((is_deleted, map)) => {
                self.deleted = is_deleted;
                self.current = Some(self.map_to_row(&map));
            }
            Err(_) => {
                self.deleted = false;
                self.current = None;
            }
        }
    }

    fn map_to_row(&self, map: &HashMap<String, FieldValue>) -> Row {
        self.schema
            .fields()
            .iter()
            .map(|f| map.get(&f.name).cloned().map(HbValue::from).unwrap_or(HbValue::Nil))
            .collect()
    }
}

impl DataNavigator for DbfNavigator {
    fn schema(&self) -> Arc<RowSchema> {
        Arc::clone(&self.schema)
    }

    fn record_count(&self) -> usize {
        self.reader.header.record_count as usize
    }

    fn current_position(&self) -> usize {
        self.pos
    }

    fn goto(&mut self, pos: usize) -> bool {
        if pos < self.record_count() {
            self.pos = pos;
            self.refresh_current();
            true
        } else {
            self.pos = self.record_count(); // EOF sentinel
            self.current = None;
            self.deleted = false;
            false
        }
    }

    fn skip(&mut self, n: i64) -> bool {
        let current = i64::try_from(self.pos).unwrap_or(i64::MAX);
        let next = current.saturating_add(n).max(0);
        self.goto(usize::try_from(next).unwrap_or(usize::MAX))
    }

    fn current_row(&self) -> Option<Row> {
        self.current.clone()
    }

    /// Write-back to `.dbf` (REPLACE / APPEND) is planned for Sprint 4.
    fn current_row_mut(&mut self) -> Option<RowProxy<'_>> {
        None
    }

    /// Write-back to `.dbf` is not yet implemented.
    fn commit(&mut self) -> Result<(), String> {
        Err("DbfNavigator is read-only in this version".into())
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // ── Unit: field decoders ──────────────────────────────────────────────────

    #[test]
    fn test_decode_character_ascii() {
        let fv = decode_character(b"Alice     ").unwrap();
        assert_eq!(fv, FieldValue::Character("Alice".into()));
    }

    #[test]
    fn test_decode_character_cp1252() {
        // 0xE3 = 'ã' (U+00E3) in CP-1252 — Latin-1 Supplement range (0xA0–0xFF)
        let raw = b"S\xE3o Paulo  ";
        let fv = decode_character(raw).unwrap();
        assert_eq!(fv, FieldValue::Character("São Paulo".into()));
    }

    #[test]
    fn test_decode_numeric_integer() {
        let fv = decode_numeric(b"  42").unwrap();
        assert_eq!(fv, FieldValue::Numeric(42.0));
    }

    #[test]
    fn test_decode_numeric_decimal() {
        let fv = decode_numeric(b" 9500.50").unwrap();
        assert_eq!(fv, FieldValue::Numeric(9500.5));
    }

    #[test]
    fn test_decode_numeric_empty_is_nil() {
        assert_eq!(decode_numeric(b"      ").unwrap(), FieldValue::Nil);
    }

    #[test]
    fn test_decode_numeric_overflow_marker_is_nil() {
        assert_eq!(decode_numeric(b"*******").unwrap(), FieldValue::Nil);
    }

    #[test]
    fn test_decode_logical() {
        assert_eq!(decode_logical(b"T"), FieldValue::Logical(true));
        assert_eq!(decode_logical(b"Y"), FieldValue::Logical(true));
        assert_eq!(decode_logical(b"F"), FieldValue::Logical(false));
        assert_eq!(decode_logical(b"N"), FieldValue::Logical(false));
        assert_eq!(decode_logical(b"?"), FieldValue::Nil);
        assert_eq!(decode_logical(b" "), FieldValue::Nil);
    }

    #[test]
    fn test_decode_date_valid() {
        // 2024-03-24 → 19806 days since 1970-01-01
        let fv = decode_date(b"20240324").unwrap();
        assert_eq!(fv, FieldValue::Date(19_806));
    }

    #[test]
    fn test_decode_date_epoch() {
        let fv = decode_date(b"19700101").unwrap();
        assert_eq!(fv, FieldValue::Date(0));
    }

    #[test]
    fn test_decode_date_blank_is_nil() {
        assert_eq!(decode_date(b"        ").unwrap(), FieldValue::Nil);
        assert_eq!(decode_date(b"00000000").unwrap(), FieldValue::Nil);
    }

    #[test]
    fn test_civil_to_epoch_days() {
        // Verified against Python: (date(2024,3,24) - date(1970,1,1)).days == 19806
        assert_eq!(civil_to_epoch_days(2024, 3, 24), 19_806);
        assert_eq!(civil_to_epoch_days(1970, 1, 1),  0);
        assert_eq!(civil_to_epoch_days(2000, 1, 1),  10_957);
    }

    #[test]
    fn test_cp1252_euro_sign() {
        // 0x80 = € in CP-1252
        assert_eq!(cp1252_decode(&[0x80]), "€");
    }

    #[test]
    fn test_cp1252_latin1_supplement() {
        // 0xE9 = é in both CP-1252 and Latin-1 / Unicode
        assert_eq!(cp1252_decode(&[b'p', b'r', 0xE9, b'c', b'o']), "préco");
    }

    // ── Integration: synthetic DBF buffer ────────────────────────────────────

    /// Builds a minimal valid dBase III buffer with two fields (NAME C/10, AGE N/3)
    /// and two records (one active, one deleted) into a temp file.
    fn write_synthetic_dbf(path: &std::path::Path) {
        let mut buf: Vec<u8> = Vec::new();

        // ── Header (32 bytes) ─────────────────────────────────────────────────
        buf.push(0x03);          // version: dBase III
        buf.extend([0x7A, 0x04, 0x1B]); // last update: 2022-04-27
        buf.extend(2u32.to_le_bytes()); // record_count = 2
        // header_size = 32 (fixed) + 2×32 (fields) + 1 (terminator) = 97
        buf.extend(97u16.to_le_bytes());
        // record_size = 1 (flag) + 10 (NAME) + 3 (AGE) = 14
        buf.extend(14u16.to_le_bytes());
        buf.extend([0u8; 20]); // reserved

        // ── Field 1: NAME C/10 ───────────────────────────────────────────────
        let mut name = [0u8; 11];
        name[..4].copy_from_slice(b"NAME");
        buf.extend(name);
        buf.push(b'C');          // type
        buf.extend([0u8; 4]);   // reserved addr
        buf.push(10);            // length
        buf.push(0);             // decimals
        buf.extend([0u8; 14]);  // reserved

        // ── Field 2: AGE N/3 ─────────────────────────────────────────────────
        let mut age = [0u8; 11];
        age[..3].copy_from_slice(b"AGE");
        buf.extend(age);
        buf.push(b'N');
        buf.extend([0u8; 4]);
        buf.push(3);
        buf.push(0);
        buf.extend([0u8; 14]);

        buf.push(0x0D); // header terminator

        // ── Record 0: active ─────────────────────────────────────────────────
        buf.push(0x20);                   // active
        buf.extend(b"Alice     ");        // NAME (10 bytes)
        buf.extend(b" 30");              // AGE  (3 bytes)

        // ── Record 1: deleted ────────────────────────────────────────────────
        buf.push(0x2A);                   // deleted
        buf.extend(b"Bob       ");        // NAME
        buf.extend(b" 25");              // AGE

        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(&buf).unwrap();
    }

    #[test]
    fn test_dbf_reader_open_and_read() {
        let dir  = std::env::temp_dir();
        let path = dir.join("swed_test_reader.dbf");
        write_synthetic_dbf(&path);

        let mut rdr = DbfReader::open(&path).unwrap();
        assert_eq!(rdr.header.version,      0x03);
        assert_eq!(rdr.header.record_count, 2);
        assert_eq!(rdr.header.header_size,  97);
        assert_eq!(rdr.header.record_size,  14);
        assert_eq!(rdr.fields.len(),         2);

        let rec0 = rdr.read_record(0).unwrap();
        assert_eq!(rec0["NAME"], FieldValue::Character("Alice".into()));
        assert_eq!(rec0["AGE"],  FieldValue::Numeric(30.0));

        assert!(matches!(rdr.read_record(1), Err(DbfError::RecordDeleted(1))));
        assert!(matches!(rdr.read_record(5), Err(DbfError::OutOfRange(5, 2))));

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_dbf_navigator_integration() {
        let dir  = std::env::temp_dir();
        let path = dir.join("swed_test_nav.dbf");
        write_synthetic_dbf(&path);

        let mut nav = DbfNavigator::open(&path).unwrap();

        assert_eq!(nav.record_count(),    2);
        assert_eq!(nav.current_position(), 0);
        assert!(!nav.is_deleted());

        let row0 = nav.current_row().unwrap();
        assert_eq!(row0[0], HbValue::String("Alice".into()));
        assert_eq!(row0[1], HbValue::Float(30.0));

        nav.skip(1);
        assert_eq!(nav.current_position(), 1);
        assert!(nav.is_deleted()); // record 1 is marked deleted
        // Field data is still decoded for deleted records (SET DELETED OFF semantics).
        let row1 = nav.current_row().unwrap();
        assert_eq!(row1[0], HbValue::String("Bob".into()));
        assert_eq!(row1[1], HbValue::Float(25.0));

        nav.goto(0);
        assert_eq!(nav.current_position(), 0);
        assert!(!nav.is_eof());

        nav.skip(99); // beyond EOF
        assert!(nav.is_eof());

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_build_schema_matches_fields() {
        let dir  = std::env::temp_dir();
        let path = dir.join("swed_test_schema.dbf");
        write_synthetic_dbf(&path);

        let rdr    = DbfReader::open(&path).unwrap();
        let schema = rdr.build_schema();

        assert_eq!(schema.field_count(), 2);
        assert!(schema.resolve("NAME").is_some());
        assert!(schema.resolve("AGE").is_some());
        assert!(schema.resolve("GHOST").is_none());

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_invalid_version_rejected() {
        let dir  = std::env::temp_dir();
        let path = dir.join("swed_test_bad_ver.dbf");
        {
            let mut f = std::fs::File::create(&path).unwrap();
            let mut buf = vec![0u8; 97 + 28]; // enough bytes to not hit EOF first
            buf[0] = 0x07; // unsupported version
            f.write_all(&buf).unwrap();
        }
        assert!(matches!(
            DbfReader::open(&path),
            Err(DbfError::UnsupportedVersion(0x07))
        ));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_field_value_into_hbvalue() {
        assert_eq!(HbValue::from(FieldValue::Character("ok".into())), HbValue::String("ok".into()));
        assert_eq!(HbValue::from(FieldValue::Numeric(1.5)),            HbValue::Float(1.5));
        assert_eq!(HbValue::from(FieldValue::Logical(true)),           HbValue::Logical(true));
        assert_eq!(HbValue::from(FieldValue::Date(19806)),             HbValue::Date(19806));
        assert_eq!(HbValue::from(FieldValue::Nil),                     HbValue::Nil);
    }
}
