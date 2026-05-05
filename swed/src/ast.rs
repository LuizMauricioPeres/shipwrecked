// swed/src/ast.rs
// AST types moved to swed_co::ast so that swed_nm can share them without
// a circular dependency. Re-export everything here so that the rest of the
// swed crate compiles unchanged.
pub use swed_co::ast::*;

// logos::Span is Range<usize>, same as swed_co::ast::SrcSpan — re-export so
// that parser.rs / codegen.rs can still write `use crate::ast::SrcSpan`.
pub use swed_co::ast::SrcSpan;
