//! Normalization passes — each implements [`crate::pass::NormPass`].

mod builtin_name_resolver;
mod incr_decr_norm;
mod index_assign_norm;

pub use builtin_name_resolver::BuiltinNameResolver;
pub use incr_decr_norm::IncrDecrNorm;
pub use index_assign_norm::IndexAssignNorm;
