//! Desugars `x++` / `x--` from rvalue position into explicit swap blocks.
//!
//! The parser emits `ExprKind::UnOp(PostIncrement | PostDecrement, ident)`.
//! When used as the RHS of an assignment or inside an expression, the
//! codegen wraps this in a block like:
//!   `{ let old = x.clone(); x += 1; old }`
//!
//! This pass is currently a stub — the codegen already handles `PostIncrement`
//! / `PostDecrement` via `__EXPR_STMT__` at the statement level. Full rvalue
//! desugaring (inside complex expressions) will be implemented in a
//! follow-up.
//!
//! Priority: P2.

use swed_co::ast::Program;

use crate::pass::NormPass;

/// Desugars `x++` / `x--` in rvalue position (stub — P2).
pub struct IncrDecrNorm;

impl NormPass for IncrDecrNorm {
    fn name(&self) -> &'static str {
        "IncrDecrNorm"
    }

    fn run(&self, _program: &mut Program) {
        // TODO P2: walk expressions; when ExprKind::UnOp(PostIncrement | PostDecrement, e)
        // appears inside a non-statement context, rewrite to a block expression node.
    }
}
