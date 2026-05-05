CRATE: swed_db v0.2.0
TYPE: library (lib)
ROLE: Replaceable Database Driver (RDD) — DBF/xBase file access layer
STATUS: functional (DbfHandler, WorkArea, Row); registry and trait-swap pending

DEPS:
  swed_rt   — HbValue (field values in/out)
  thiserror — error derive
  byteorder — DBF binary reading (little-endian)

DESIGN: Isolates all DBF I/O from swed_rt. WorkArea = Harbour cursor concept.
        Rdd trait allows driver swap (e.g. SQL backend) without changing generated code.
        Global work-area table mirrors Harbour's 255 simultaneous areas.

SOURCE_FILES:
  lib.rs
  dbf/
    dbf_handler.rs  — low-level DBF read/write (dbase crate or byteorder direct)
    row.rs          — single record buffer: Vec<(String,HbValue)>
  sql/              — placeholder for future SQL RDD
  traits/
    rdd.rs          — Rdd trait (swappable driver contract)
  work_area.rs      — WorkArea cursor + navigation
  registry.rs       — PENDING: global SELECT/ALIAS table

SOURCE_LAYOUT (current actual):
  lib.rs
  dbf/  (dir with dbf_handler + row)
  sql/  (dir placeholder)
  traits/ (rdd trait)

TYPES:
  WorkArea — Harbour work area cursor
    path: String
    alias: String
    records: Vec<Row>
    position: usize        — 0-based; BOF=0, EOF=records.len()
    open: bool
  methods:
    WorkArea::open(path:&str, alias:&str) -> Result<WorkArea, SwedError>
    close(&mut self)
    go_top(&mut self)
    go_bottom(&mut self)
    skip(&mut self, n:i64)
    is_eof(&self) -> bool
    is_bof(&self) -> bool
    recno(&self) -> HbValue::Integer   — 1-based
    field_get(&self, name:&str) -> HbValue
    field_set(&mut self, name:&str, val:HbValue) -> Result<(),SwedError>
    append_blank(&mut self) -> Result<(),SwedError>

  DbfHandler — low-level DBF I/O
    fn open(path:&str) -> Result<DbfHandler,SwedError>
    fn read_all(&mut self) -> Result<Vec<Row>,SwedError>
    fn write_record(&mut self, recno:usize, row:&Row) -> Result<(),SwedError>

  Row — single DBF record buffer
    fields: HashMap<String, HbValue>
    fn get(&self, name:&str) -> HbValue
    fn set(&mut self, name:&str, val:HbValue)

TRAIT (traits/rdd.rs):
  Rdd:
    fn open(&mut self, path:&str) -> Result<(),SwedError>
    fn close(&mut self)
    fn go_top(&mut self)
    fn skip(&mut self, n:i64)
    fn field_get(&self, name:&str) -> HbValue
    fn field_set(&mut self, name:&str, val:HbValue) -> Result<(),SwedError>
    fn is_eof(&self) -> bool
    fn append_blank(&mut self) -> Result<(),SwedError>

HARBOUR_MAPPING:
  USE file ALIAS a        → WorkArea::open(path, alias)
  CLOSE / CLOSE ALL       → WorkArea::close()
  GO TOP / GO BOTTOM      → go_top() / go_bottom()
  SKIP n                  → skip(n)
  FIELD->name             → field_get("NAME")
  REPLACE field WITH v    → field_set("FIELD", val)
  APPEND BLANK            → append_blank()
  EOF() / BOF()           → is_eof() / is_bof()
  RECNO()                 → recno()
  SELECT area             → PENDING: work_area::select(alias)

DBF_TYPE_MAPPING:
  Character 'C'  → HbValue::String
  Numeric   'N'  → HbValue::Float (or Integer if decimals=0)
  Logical   'L'  → HbValue::Logical
  Date      'D'  → HbValue::Date

INVARIANTS:
  - recno() is always 1-based (Harbour convention)
  - field names normalized to UPPERCASE before lookup
  - FIELD->ALIAS resolved via global registry (pending)
  - No unsafe; byteorder reads only

PENDING (this crate):
  registry.rs: global 255-area table; SELECT/ALIAS resolution     (priority: H)
  SQL RDD placeholder in sql/                                       (priority: L)
  impl Rdd for WorkArea (wire trait to concrete type)              (priority: M)
  Index support (SEEK / FIND via .ntx/.cdx)                        (priority: L)
  DBEVAL / FOR / WHILE filter on navigation                        (priority: L)

INTEGRATES_WITH:
  swed_rt: HbValue flows in/out of all field operations
  swed (binary): dbf feature flag wires swed_db into codegen
  swed_co: Rdd implementations return SwedError on failure
