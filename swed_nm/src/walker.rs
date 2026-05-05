//! Shared mutable AST walk utilities used by multiple passes.
//!
//! Each `walk_*` function visits a node and all its children, calling the
//! provided closures. Passes use these helpers instead of duplicating the
//! traversal logic.

use swed_co::ast::{
    Expr, ExprKind, Stmt, StmtKind, TopLevel, Program,
    IfStmt, DoWhileStmt, ForStmt,
};

// ---------------------------------------------------------------------------
// Program / top-level
// ---------------------------------------------------------------------------

/// Walk every statement body in the program, calling `f` on each `Stmt`.
pub fn walk_program<F>(program: &mut Program, f: &mut F)
where
    F: FnMut(&mut Stmt),
{
    for unit in &mut program.units {
        match unit {
            TopLevel::Procedure(p) => walk_stmts(&mut p.body, f),
            TopLevel::Function(fu) => walk_stmts(&mut fu.body, f),
            TopLevel::Class(c) => {
                for m in &mut c.methods {
                    walk_stmts(&mut m.body, f);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Statement list
// ---------------------------------------------------------------------------

/// Walk a slice of statements, calling `f` on each and recursing into
/// all nested statement lists (if/while/for bodies).
pub fn walk_stmts<F>(stmts: &mut Vec<Stmt>, f: &mut F)
where
    F: FnMut(&mut Stmt),
{
    // Two-pass: first recurse into children, then call f on this stmt.
    // This makes it safe for f to replace stmt.kind without missing children.
    for stmt in stmts.iter_mut() {
        recurse_stmt(stmt, f);
        f(stmt);
    }
}

/// Recurse into nested statement lists inside a statement (does NOT call f
/// on `stmt` itself — that is done by the caller in `walk_stmts`).
fn recurse_stmt<F>(stmt: &mut Stmt, f: &mut F)
where
    F: FnMut(&mut Stmt),
{
    match &mut stmt.kind {
        StmtKind::If(i) => recurse_if(i, f),
        StmtKind::DoWhile(w) => recurse_while(w, f),
        StmtKind::For(fo) => recurse_for(fo, f),
        _ => {}
    }
}

fn recurse_if<F>(i: &mut IfStmt, f: &mut F)
where
    F: FnMut(&mut Stmt),
{
    walk_stmts(&mut i.then_body, f);
    for (_, body) in &mut i.elseif_branches {
        walk_stmts(body, f);
    }
    if let Some(body) = &mut i.else_body {
        walk_stmts(body, f);
    }
}

fn recurse_while<F>(w: &mut DoWhileStmt, f: &mut F)
where
    F: FnMut(&mut Stmt),
{
    walk_stmts(&mut w.body, f);
}

fn recurse_for<F>(fo: &mut ForStmt, f: &mut F)
where
    F: FnMut(&mut Stmt),
{
    walk_stmts(&mut fo.body, f);
}

// ---------------------------------------------------------------------------
// Expression walker
// ---------------------------------------------------------------------------

/// Recursively walk all expressions in a statement, calling `f` on each.
pub fn walk_exprs_in_stmt<F>(stmt: &mut Stmt, f: &mut F)
where
    F: FnMut(&mut Expr),
{
    match &mut stmt.kind {
        StmtKind::Assign(a) => {
            walk_expr(&mut a.target, f);
            walk_expr(&mut a.value, f);
        }
        StmtKind::Call(c) => {
            for arg in &mut c.args {
                walk_expr(arg, f);
            }
        }
        StmtKind::Print(e) | StmtKind::Return(Some(e)) => walk_expr(e, f),
        StmtKind::Store(s) => walk_expr(&mut s.value, f),
        StmtKind::If(i) => {
            walk_expr(&mut i.condition, f);
            for (cond, _) in &mut i.elseif_branches {
                walk_expr(cond, f);
            }
        }
        StmtKind::DoWhile(w) => walk_expr(&mut w.condition, f),
        StmtKind::For(fo) => {
            walk_expr(&mut fo.start, f);
            walk_expr(&mut fo.end, f);
            if let Some(s) = &mut fo.step {
                walk_expr(s, f);
            }
        }
        StmtKind::VarDecl(v) | StmtKind::StaticDecl(v) | StmtKind::PublicDecl(v) => {
            for init in &mut v.vars {
                if let Some(e) = &mut init.init {
                    walk_expr(e, f);
                }
            }
        }
        StmtKind::AtSay(s) => {
            walk_expr(&mut s.row, f);
            walk_expr(&mut s.col, f);
            walk_expr(&mut s.expr, f);
        }
        StmtKind::AtGet(g) => {
            walk_expr(&mut g.row, f);
            walk_expr(&mut g.col, f);
        }
        _ => {}
    }
}

/// Recursively walk `expr` and all sub-expressions, calling `f` on each.
pub fn walk_expr<F>(expr: &mut Expr, f: &mut F)
where
    F: FnMut(&mut Expr),
{
    // Recurse into children first (bottom-up), then call f.
    match &mut expr.kind {
        ExprKind::Call(c) => {
            for arg in &mut c.args {
                walk_expr(arg, f);
            }
        }
        ExprKind::Index(a, b) => {
            walk_expr(a, f);
            walk_expr(b, f);
        }
        ExprKind::Field(e, _) => walk_expr(e, f),
        ExprKind::BinOp(_, l, r) => {
            walk_expr(l, f);
            walk_expr(r, f);
        }
        ExprKind::UnOp(_, e) => walk_expr(e, f),
        ExprKind::Iif(c, t, fa) => {
            walk_expr(c, f);
            walk_expr(t, f);
            walk_expr(fa, f);
        }
        ExprKind::ArrayLit(elems) => {
            for e in elems {
                walk_expr(e, f);
            }
        }
        _ => {}
    }
    f(expr);
}
