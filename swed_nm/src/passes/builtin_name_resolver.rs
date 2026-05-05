//! Renames Harbour built-in call sites to their `HB_XXX` canonical form.
//!
//! The codegen's generic branch lowercases and snake-cases the callee name,
//! so `HB_CHR` → `hb_chr(...)`, `HB_MAX` → `hb_max(...)`, etc.
//!
//! Built-ins that already have dedicated codegen arms (AADD, LEN, STR, VAL,
//! SUBSTR, AT, ALLTRIM, LTRIM, RTRIM, ASIZE, QOUT) are intentionally excluded
//! to avoid interfering with their special-case rendering.

use swed_co::ast::{CallExpr, Expr, ExprKind, Program, Stmt, StmtKind};

use crate::pass::NormPass;
use crate::walker::{walk_exprs_in_stmt, walk_program};

/// Renames Harbour built-in identifiers in call positions to their `HB_`
/// prefixed form so the codegen emits the correct `hb_*` Rust functions.
pub struct BuiltinNameResolver;

impl NormPass for BuiltinNameResolver {
    fn name(&self) -> &'static str {
        "BuiltinNameResolver"
    }

    fn run(&self, program: &mut Program) {
        walk_program(program, &mut |stmt: &mut Stmt| {
            // Rename call sites at statement level (StmtKind::Call)
            if let StmtKind::Call(c) = &mut stmt.kind {
                rename(c);
            }
            // Rename call sites inside expressions within this statement
            walk_exprs_in_stmt(stmt, &mut |expr: &mut Expr| {
                if let ExprKind::Call(c) = &mut expr.kind {
                    rename(c);
                }
            });
        });
    }
}

/// Map a Harbour built-in UPPERCASE name to its `HB_XXX` form.
/// Returns `None` for names that are either user-defined or already
/// handled by dedicated codegen arms (so we leave them untouched).
fn resolve(name: &str) -> Option<&'static str> {
    match name {
        // ── String ────────────────────────────────────────────────────────
        "CHR"        => Some("HB_CHR"),
        "ASC"        => Some("HB_ASC"),
        "SPACE"      => Some("HB_SPACE"),
        "REPLICATE"  => Some("HB_REPLICATE"),
        "UPPER"      => Some("HB_UPPER"),
        "LOWER"      => Some("HB_LOWER"),
        "LEFT"       => Some("HB_LEFT"),
        "RIGHT"      => Some("HB_RIGHT"),
        "PADC"       => Some("HB_PADC"),
        "STRTRAN"    => Some("HB_STRTRAN"),
        "STRZERO"    => Some("HB_STRZERO"),
        "HARDCR"     => Some("HB_HARDCR"),
        "RAT"        => Some("HB_RAT"),
        "TRIM"       => Some("HB_RTRIM"),   // alias

        // ── Numeric ───────────────────────────────────────────────────────
        "ABS"        => Some("HB_ABS"),
        "INT"        => Some("HB_INT"),
        "MAX"        => Some("HB_MAX"),
        "MIN"        => Some("HB_MIN"),
        "ROUND"      => Some("HB_ROUND"),
        "MOD"        => Some("HB_MOD"),
        "SQRT"       => Some("HB_SQRT"),
        "EXP"        => Some("HB_EXP"),
        "LOG"        => Some("HB_LOG"),

        // ── Date / time ───────────────────────────────────────────────────
        "DATE"       => Some("HB_DATE"),
        "YEAR"       => Some("HB_YEAR"),
        "MONTH"      => Some("HB_MONTH"),
        "DAY"        => Some("HB_DAY"),
        "DTOS"       => Some("HB_DTOS"),
        "STOD"       => Some("HB_STOD"),
        "DTOC"       => Some("HB_DTOC"),
        "CTOD"       => Some("HB_CTOD"),
        "TIME"       => Some("HB_TIME"),
        "SECONDS"    => Some("HB_SECONDS"),

        // ── Type / value ──────────────────────────────────────────────────
        "VALTYPE"    => Some("HB_VALTYPE"),
        "TYPE"       => Some("HB_TYPE"),
        "EMPTY"      => Some("HB_EMPTY"),
        "ISNIL"      => Some("HB_ISNIL"),
        "ISALPHA"    => Some("HB_ISALPHA"),
        "ISDIGIT"    => Some("HB_ISDIGIT"),
        "ISLOWER"    => Some("HB_ISLOWER"),
        "ISUPPER"    => Some("HB_ISUPPER"),

        // ── Array ─────────────────────────────────────────────────────────
        // AADD / LEN / ASIZE handled by codegen — skip
        "ARRAY"      => Some("HB_ARRAY"),
        "ATAIL"      => Some("HB_ATAIL"),
        "ASORT"      => Some("HB_ASORT"),
        "AEVAL"      => Some("HB_AEVAL"),
        "AFILL"      => Some("HB_AFILL"),
        "ADEL"       => Some("HB_ADEL"),
        "AINS"       => Some("HB_AINS"),
        "ACLONE"     => Some("HB_ACLONE"),

        // ── Screen / TUI ──────────────────────────────────────────────────
        "SETCOLOR"   => Some("HB_SETCOLOR"),
        "SETMODE"    => Some("HB_SETMODE"),
        "ROW"        => Some("HB_ROW"),
        "COL"        => Some("HB_COL"),
        "MAXROW"     => Some("HB_MAXROW"),
        "MAXCOL"     => Some("HB_MAXCOL"),
        "INKEY"      => Some("HB_INKEY"),
        "LASTKEY"    => Some("HB_LASTKEY"),
        "NEXTKEY"    => Some("HB_NEXTKEY"),
        "TONE"       => Some("HB_TONE"),

        // ── Misc ──────────────────────────────────────────────────────────
        "OS"         => Some("HB_OS"),
        "VERSION"    => Some("HB_VERSION"),

        // Everything else: user-defined or already handled → leave alone
        _ => None,
    }
}

fn rename(call: &mut CallExpr) {
    if let Some(new_name) = resolve(&call.callee) {
        call.callee = new_name.to_owned();
    }
}
