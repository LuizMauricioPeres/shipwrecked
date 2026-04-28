# swed_db — Database Engine (RDD)

Replaceable Database Driver layer for SWed. Encapsulates all DBF/xBase file access, isolating it from the general runtime (`swed_rt`). Depends on `swed_co` and `swed_rt`.

## Responsibilities

- Open, close, index, and navigate DBF files via the `dbase` crate
- Expose `WorkArea` — Harbour's concept of a database cursor with alias, BOF/EOF, and record pointer
- Map DBF field types to `HbValue` (Character → `String`, Numeric → `Float`/`Integer`, Logical → `Logical`, Date → `Date`)
- Resolve `FIELD->alias` references for multi-table joins
- Manage the global work-area registry (up to 255 areas, Harbour-compatible)

## Harbour commands → swed_db API

| Harbour | swed_db |
|---|---|
| `USE file ALIAS a` | `WorkArea::open(path, alias)` |
| `CLOSE` / `CLOSE ALL` | `WorkArea::close()` |
| `GO TOP` / `GO BOTTOM` | `WorkArea::go_top()` / `go_bottom()` |
| `SKIP n` | `WorkArea::skip(n)` |
| `FIELD->name` | `WorkArea::field_get("NAME")` |
| `REPLACE field WITH v` | `WorkArea::field_set("FIELD", val)` |
| `APPEND BLANK` | `WorkArea::append_blank()` |
| `EOF()` / `BOF()` | `WorkArea::is_eof()` / `is_bof()` |
| `RECNO()` | `WorkArea::recno()` |
| `SELECT area` | `work_area::select(area_or_alias)` |

## Rdd trait

The `Rdd` trait allows swapping the underlying driver without changing transpiled code:

```rust
pub trait Rdd {
    fn open(&mut self, path: &str) -> Result<(), SwedError>;
    fn close(&mut self);
    fn go_top(&mut self);
    fn skip(&mut self, n: i64);
    fn field_get(&self, name: &str) -> HbValue;
    fn field_set(&mut self, name: &str, val: HbValue) -> Result<(), SwedError>;
    fn is_eof(&self) -> bool;
    fn append_blank(&mut self) -> Result<(), SwedError>;
}
```

## Source layout

```
swed_db/src/
├── lib.rs
├── work_area.rs    ← cursor + navigation (migrated from swed_rt)
├── dbf_handler.rs  ← low-level DBF I/O (migrated from swed_rt)
├── row.rs          ← record buffer (migrated from swed_rt)
├── registry.rs     ← global work-area table (SELECT / ALIAS)
└── traits/
    └── rdd.rs      ← Rdd trait (swappable drivers)
```
