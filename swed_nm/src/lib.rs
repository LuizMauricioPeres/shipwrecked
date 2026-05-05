//! `swed_nm` — Semantic Normalizer for the SWed transpiler.
//!
//! Runs a chain of AST rewrite passes between the Semantic Analyzer and the
//! Codegen. Each pass receives a `&mut Program` and mutates it in place.
//!
//! # Pass execution order (P1 first)
//!
//! 1. [`passes::BuiltinNameResolver`] — renames Harbour built-in callees to
//!    their `HB_XXX` canonical form (e.g. `CHR` → `HB_CHR`).
//! 2. [`passes::IndexAssignNorm`] — rewrites `a[i] := v` assignments into
//!    `HB_SET_VAL(a, i, v)` calls, fixing `E0070` in generated Rust.
//! 3. [`passes::IncrDecrNorm`] — desugars `x++` / `x--` in rvalue position
//!    into explicit temporary-swap blocks.

pub mod pass;
pub mod passes;
mod walker;

use swed_co::ast::Program;

pub use pass::NormPass;

/// Run all normalization passes over `program` in priority order.
///
/// This is the single entry point called by the `swed` binary after parsing
/// and before code generation.
pub fn normalize(program: &mut Program) {
    let chain: &[&dyn NormPass] = &[
        &passes::BuiltinNameResolver,
        &passes::IndexAssignNorm,
        &passes::IncrDecrNorm,
    ];
    for pass in chain {
        pass.run(program);
    }
}
