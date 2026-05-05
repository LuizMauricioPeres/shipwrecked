//! Rewrites array-element assignment into `HB_SET_VAL` / `HB_SET_NESTED` calls.
//!
//! Harbour:
//! ```harbour
//! aArray[i]    := value       -- single index
//! aArray[i][j] := value       -- chained (nested array)
//! ```
//!
//! The parser produces `StmtKind::Assign { target: ExprKind::Index(...), value }`.
//! Rust rejects this pattern (`E0070`) because `hb_get_val` returns by value.
//!
//! This pass rewrites it to `StmtKind::Call(HB_SET_VAL / HB_SET_NESTED)`.
//! The codegen then emits the method-call form:
//!   `base.hb_set_val(idx.clone(), val)`
//!   `base.hb_set_nested(i.clone(), j.clone(), val)`

use swed_co::ast::{
    AssignStmt, CallExpr, ExprKind, Program, Stmt, StmtKind,
};

use crate::pass::NormPass;
use crate::walker::walk_program;

/// Fixes `E0070`: replaces `a[i] := v` with `HB_SET_VAL(a, i, v)`.
pub struct IndexAssignNorm;

impl NormPass for IndexAssignNorm {
    fn name(&self) -> &'static str {
        "IndexAssignNorm"
    }

    fn run(&self, program: &mut Program) {
        walk_program(program, &mut rewrite_stmt);
    }
}

fn rewrite_stmt(stmt: &mut Stmt) {
    let span = stmt.span.clone();
    let new_kind = match &stmt.kind {
        StmtKind::Assign(a) => rewrite_assign(a, span),
        _ => return,
    };
    if let Some(kind) = new_kind {
        stmt.kind = kind;
    }
}

/// Returns `Some(new_kind)` when the assignment should be replaced,
/// `None` when it should be left unchanged.
fn rewrite_assign(
    a: &AssignStmt,
    span: std::ops::Range<usize>,
) -> Option<StmtKind> {
    match &a.target.kind {
        // ── Chained: a[i][j] := v → HB_SET_NESTED(a, i, j, v) ────────────
        ExprKind::Index(outer, j_expr)
            if matches!(outer.kind, ExprKind::Index(..)) =>
        {
            if let ExprKind::Index(base, i_expr) = &outer.kind {
                let call = CallExpr {
                    callee: "HB_SET_NESTED".to_owned(),
                    args: vec![
                        (**base).clone(),
                        (**i_expr).clone(),
                        (**j_expr).clone(),
                        a.value.clone(),
                    ],
                    span,
                };
                Some(StmtKind::Call(call))
            } else {
                None // unreachable, but satisfies the compiler
            }
        }

        // ── Simple: a[i] := v → HB_SET_VAL(a, i, v) ──────────────────────
        ExprKind::Index(base, idx_expr) => {
            let call = CallExpr {
                callee: "HB_SET_VAL".to_owned(),
                args: vec![
                    (**base).clone(),
                    (**idx_expr).clone(),
                    a.value.clone(),
                ],
                span,
            };
            Some(StmtKind::Call(call))
        }

        // ── Not an index assignment — leave unchanged ─────────────────────
        _ => None,
    }
}
