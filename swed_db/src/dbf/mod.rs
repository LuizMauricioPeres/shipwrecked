//! DBF (dBase III/IV) driver — low-level I/O, row abstraction, work areas.

pub mod handler;
/// Row abstraction, `DataNavigator` trait and in-memory table.
pub mod row;
/// Work area cursor and global `WorkAreaManager`.
pub mod work_area;

pub use handler::{DbfError, DbfField, DbfHeader, DbfNavigator, DbfReader, FieldValue};
pub use row::{DataNavigator, FieldIndex, FieldMeta, InMemoryTable, Row, RowProxy, RowSchema};
pub use work_area::{with_work_areas, WorkArea, WorkAreaManager};
