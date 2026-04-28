//! `swed_co` — Core types and traits shared across the SWed workspace.
//!
//! No runtime logic. No external dependencies. All other crates depend on this one.

pub mod error;
pub mod hb_type;
pub mod traits;

pub use error::{SeverityLevel, SwedError};
pub use hb_type::HbType;
pub use traits::{ErrorInterceptor, FunctionResolver, ModuleComponent, NativeFunction};
