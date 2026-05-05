CRATE: swed_io v0.2.0
TYPE: library (lib)
ROLE: File I/O and encoding — reads .prg with CP1252/UTF-8 auto-detection; writes output
STATUS: file_io + directory functional; config (swed.json) pending; traits stub

DEPS:
  swed_co     — SwedError, SeverityLevel
  swed_rt     — HbValue (config values surfaced as HbValue for SET command parity)
  encoding_rs — Windows-1252 → UTF-8 conversion

SOURCE_FILES:
  lib.rs
  file_io.rs      — read_prg, write_output (implemented)
  directory.rs    — recursive .prg discovery (implemented)
  traits/         — Encoder trait (stub)

FUNCTIONS (file_io.rs):
  read_prg(path:&str) -> Result<String, SwedError>
    1. fs::read(path) → Vec<u8>
    2. str::from_utf8 — if Ok, return as String
    3. else → encoding_rs::WINDOWS_1252.decode(&bytes).0.into_owned()
    4. emits SeverityLevel::Notice if CP1252 fallback triggered
    Always yields valid UTF-8 regardless of source encoding

  write_output(path:&str, content:&str) -> Result<(), SwedError>
    creates parent directories if absent
    writes UTF-8 content atomically (temp file + rename — TBD)

FUNCTIONS (directory.rs):
  find_prg_files(root:&str) -> Vec<PathBuf>
    walkdir recursive; filters *.prg (case-insensitive on Windows)
    used by swed_mkh analyser for batch manifest generation

ENCODING_STRATEGY:
  detection order: UTF-8 → CP1252 (no BOM detection yet)
  encoding_rs WINDOWS_1252.decode() is lossless (replacement chars for unmapped bytes)
  Notice diagnostic carries source path and byte offset of first non-UTF-8 byte

TRAIT (traits/encoder.rs — stub):
  Encoder:
    fn encode(raw:&[u8]) -> Result<String, SwedError>
    fn name(&self) -> &'static str
  Utf8Encoder — passthrough impl
  Cp1252Encoder — encoding_rs impl
  Intended use: pluggable encoding for future ISO-8859-1, EUC-JP Harbour source

PENDING_CONFIG (not yet implemented):
  Config struct from swed.json:
    date_format: String   — "DD/MM/YYYY" default
    century: bool         — true = 4-digit year
    decimal: u8           — decimal places default
    output_dir: String    — target directory for .rs output
  fn read_config(path:&str) -> Result<Config, SwedError>
  Mirrors Harbour SET commands (SET DATE, SET CENTURY, SET DECIMAL)

PENDING (this crate):
  read_config / Config struct                                (priority: M)
  Encoder trait wired to concrete impls                     (priority: L)
  BOM detection (UTF-8 BOM, UTF-16 BOM)                    (priority: L)
  Atomic write (temp + rename) for write_output             (priority: L)

INTEGRATES_WITH:
  swed (binary): main.rs calls read_prg before lexer; write_output after codegen
  swed_mkh: find_prg_files used by batch analyser
  swed_co: returns SwedError on all failures
