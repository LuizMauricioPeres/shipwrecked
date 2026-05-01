// swed/src/parser.rs
// Recursive-descent parser: Vec<Token> → ast::Program
//
// Harbour quirks handled here:
//   - Context-sensitive `[`: string if preceded by `=`/`(`/`{`/`[`, else array index.
//   - `;` as both inline separator and line-continuation.
//   - `=` as assignment (legacy) OR comparison — resolved by position.
//   - Case-insensitive keywords (source pre-normalized to UPPER by lexer::normalize).
//   - `?` / `??` as QOUT / QQOUT shorthand.

use crate::ast::*;
use crate::lexer::Token;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token {got:?} at position {pos}, expected {expected}")]
    Unexpected {
        got: String,
        expected: String,
        pos: usize,
    },
    #[error("Unexpected end of input, expected {0}")]
    Eof(String),
    #[error("{0}")]
    Other(String),
}

// ---------------------------------------------------------------------------
// Parser state
// ---------------------------------------------------------------------------

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    // ── Cursor helpers ───────────────────────────────────────────────────

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek2(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Some(Token::Newline) | Some(Token::Semicolon)) {
            self.advance();
        }
    }

    fn expect_newline_or_semi(&mut self) {
        // Consume statement terminator(s); lenient — don't error on EOF.
        while matches!(self.peek(), Some(Token::Newline) | Some(Token::Semicolon)) {
            self.advance();
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
        match self.peek() {
            Some(tok) if std::mem::discriminant(tok) == std::mem::discriminant(expected) => {
                self.advance();
                Ok(())
            }
            Some(tok) => Err(ParseError::Unexpected {
                got: format!("{tok:?}"),
                expected: format!("{expected:?}"),
                pos: self.pos,
            }),
            None => Err(ParseError::Eof(format!("{expected:?}"))),
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.advance() {
            Some(Token::Ident(s)) => Ok(s),
            Some(other) => Err(ParseError::Unexpected {
                got: format!("{other:?}"),
                expected: "identifier".into(),
                pos: self.pos,
            }),
            None => Err(ParseError::Eof("identifier".into())),
        }
    }

    /// Fake zero-length span (pos, pos) — adequate for a skeleton parser.
    fn span(&self) -> SrcSpan {
        self.pos..self.pos
    }

    /// The `BracketString` regex `\[[^\]]*\]` stops at the first `]`, so nested
    /// indexing like `arr[inner[i]]` produces `BracketString(" inner[i ")` + `RBracket`
    /// in the token stream.  This method appends one `]` per consumed `RBracket`
    /// until open and close counts balance, enabling recursive sub-parsing.
    fn balance_bracket_content(&mut self, mut content: String) -> String {
        let open  = content.chars().filter(|&c| c == '[').count();
        let close = content.chars().filter(|&c| c == ']').count();
        let mut needed = open.saturating_sub(close);
        while needed > 0 && matches!(self.peek(), Some(Token::RBracket)) {
            self.advance();
            content.push(']');
            needed -= 1;
        }
        content
    }

    // -----------------------------------------------------------------------
    // Entry point
    // -----------------------------------------------------------------------

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut units = Vec::new();
        self.skip_newlines();
        while self.peek().is_some() {
            // MEMVAR/PRIVATE at file level is a compiler hint — consume and discard.
            if matches!(self.peek(), Some(Token::Memvar) | Some(Token::Private)) {
                self.skip_top_level_memvar();
                self.skip_newlines();
                continue;
            }
            // FIELD at file level: module-scoped DB field declaration.
            // Parsed but not promoted to a TopLevel unit — handled as a
            // file-scope FieldDecl by storing in a pre-pass (codegen scans it).
            // We skip it here to avoid "expected PROCEDURE" errors.
            if matches!(self.peek(), Some(Token::FieldKw)) {
                self.skip_top_level_memvar(); // same shape: keyword ident[,ident]* [IN ident]
                self.skip_newlines();
                continue;
            }
            units.push(self.parse_top_level()?);
            self.skip_newlines();
        }
        Ok(Program { units })
    }

    fn skip_top_level_memvar(&mut self) {
        self.advance(); // consume MEMVAR / PRIVATE
        // Discard idents and commas until end of line
        loop {
            match self.peek() {
                Some(Token::Ident(_)) | Some(Token::Comma) => { self.advance(); }
                _ => break,
            }
        }
    }

    // -----------------------------------------------------------------------
    // Top-level: PROCEDURE / FUNCTION / CLASS
    // -----------------------------------------------------------------------

    fn parse_top_level(&mut self) -> Result<TopLevel, ParseError> {
        match self.peek() {
            Some(Token::Static) => {
                self.advance(); // consume STATIC
                match self.peek() {
                    Some(Token::Procedure) => {
                        // Não consumir aqui; parse_proc() vai fazer isso
                        Ok(TopLevel::Procedure(self.parse_proc()?))
                    }
                    Some(Token::Function) => {
                        // Não consumir aqui; parse_func() vai fazer isso
                        Ok(TopLevel::Function(self.parse_func()?))
                    }
                    Some(other) => Err(ParseError::Unexpected {
                        got: format!("{other:?}"),
                        expected: "PROCEDURE or FUNCTION after STATIC".into(),
                        pos: self.pos,
                    }),
                    None => Err(ParseError::Eof("top-level declaration".into())),
                }
            }
            Some(Token::Procedure) => Ok(TopLevel::Procedure(self.parse_proc()?)),
            Some(Token::Function) => Ok(TopLevel::Function(self.parse_func()?)),
            Some(Token::Class) => Ok(TopLevel::Class(self.parse_class()?)),
            Some(other) => Err(ParseError::Unexpected {
                got: format!("{other:?}"),
                expected: "PROCEDURE, FUNCTION or CLASS".into(),
                pos: self.pos,
            }),
            None => Err(ParseError::Eof("top-level declaration".into())),
        }
    }

    fn parse_proc(&mut self) -> Result<ProcDef, ParseError> {
        let span_start = self.pos;
        self.advance(); // consume PROCEDURE
        let name = self.expect_ident()?;
        let params = self.parse_param_list()?;
        self.expect_newline_or_semi();
        let body = self.parse_body_until_return_or_proc()?;
        Ok(ProcDef {
            name,
            params,
            body,
            span: span_start..self.pos,
        })
    }

    fn parse_func(&mut self) -> Result<FuncDef, ParseError> {
        let span_start = self.pos;
        self.advance(); // consume FUNCTION
        let name = self.expect_ident()?;
        let params = self.parse_param_list()?;
        self.expect_newline_or_semi();
        let body = self.parse_body_until_return_or_proc()?;
        Ok(FuncDef {
            name,
            params,
            body,
            span: span_start..self.pos,
        })
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ParseError> {
        // Optional: ( param1, param2, ... )
        if !matches!(self.peek(), Some(Token::LParen)) {
            return Ok(vec![]);
        }
        self.advance(); // (
        let mut params = Vec::new();
        while !matches!(self.peek(), Some(Token::RParen) | None) {
            let sp = self.pos;
            let name = self.expect_ident()?;
            params.push(Param { name, span: sp..self.pos });
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(&Token::RParen)?;
        Ok(params)
    }

    fn parse_class(&mut self) -> Result<ClassDef, ParseError> {
        let span_start = self.pos;
        self.advance(); // CLASS
        let name = self.expect_ident()?;

        // Optional: FROM SuperClass
        let superclass = if matches!(self.peek(), Some(Token::Ident(s)) if s == "FROM") {
            self.advance();
            Some(self.expect_ident()?)
        } else {
            None
        };

        self.expect_newline_or_semi();

        let mut data = Vec::new();
        let mut methods = Vec::new();

        loop {
            self.skip_newlines();
            match self.peek() {
                Some(Token::EndClass) | None => {
                    self.advance();
                    break;
                }
                Some(Token::Data) => {
                    self.advance();
                    let sp = self.pos;
                    let n = self.expect_ident()?;
                    data.push(DataDecl { name: n, span: sp..self.pos });
                    self.expect_newline_or_semi();
                }
                Some(Token::Method) => {
                    methods.push(self.parse_method()?);
                }
                _ => {
                    // Skip unknown tokens inside CLASS body for now
                    self.advance();
                }
            }
        }

        Ok(ClassDef {
            name,
            superclass,
            data,
            methods,
            span: span_start..self.pos,
        })
    }

    fn parse_method(&mut self) -> Result<MethodDef, ParseError> {
        let sp = self.pos;
        self.advance(); // METHOD
        let name = self.expect_ident()?;
        let params = self.parse_param_list()?;
        self.expect_newline_or_semi();
        let body = self.parse_body_until_return_or_proc()?;
        Ok(MethodDef { name, params, body, span: sp..self.pos })
    }

    // -----------------------------------------------------------------------
    // Body: sequence of statements until RETURN / next top-level / EOF
    // -----------------------------------------------------------------------

    fn parse_body_until_return_or_proc(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        loop {
            self.skip_newlines();
            match self.peek() {
                None
                | Some(Token::Procedure)
                | Some(Token::Function)
                | Some(Token::Class)
                | Some(Token::Static)
                | Some(Token::EndClass) => break,
                Some(Token::Return) => {
                    stmts.push(self.parse_return()?);
                    self.expect_newline_or_semi();
                    // Continue parsing — unreachable stmts are collected for
                    // semantic analysis, but stop at the next top-level decl.
                    // We break only when we see a new top-level boundary above.
                }
                _ => {
                    stmts.push(self.parse_stmt()?);
                    self.expect_newline_or_semi();
                }
            }
        }
        Ok(stmts)
    }

    // -----------------------------------------------------------------------
    // Statements
    // -----------------------------------------------------------------------

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let sp = self.pos;
        match self.peek().cloned() {
            Some(Token::Local) => self.parse_var_decl(false, sp),
            Some(Token::Static) => self.parse_var_decl(true, sp),
            Some(Token::Memvar) | Some(Token::Private) => self.parse_memvar(sp),
            Some(Token::Public) => self.parse_public(sp),
            Some(Token::FieldKw) => self.parse_field(sp),
            Some(Token::If) => self.parse_if(sp),
            Some(Token::Do) => self.parse_do_while(sp),
            Some(Token::While) => self.parse_while(sp),
            Some(Token::For) => self.parse_for(sp),
            Some(Token::Store) => self.parse_store(sp),
            Some(Token::Exit) => {
                self.advance();
                Ok(Stmt::new(StmtKind::Exit, sp..self.pos))
            }
            Some(Token::Loop) => {
                self.advance();
                Ok(Stmt::new(StmtKind::Loop, sp..self.pos))
            }
            Some(Token::Cls) => {
                self.advance();
                Ok(Stmt::new(StmtKind::Cls, sp..self.pos))
            }
            Some(Token::Return) => self.parse_return(),
            // Agora o Parser reconhece o VIP que o Lexer enviou
            Some(Token::Question) => {
                self.advance(); // Consome o '?'
                // No Harbour, o '?' pode não ter nada na frente (pula linha)
                // ou ter uma expressão.
                let expr = if matches!(self.peek(), Some(Token::Newline) | Some(Token::Semicolon) | None) {
                    // Se for fim de linha, o comando '?' imprime vazio (nil)
                    Expr::nil(sp..self.pos)
                } else {
                    self.parse_expr()?
                };
                Ok(Stmt::new(StmtKind::Print(expr), sp..self.pos))
            }

            Some(Token::QuestionQuestion) => {
                self.advance(); // Consome o '??'
                let expr = self.parse_expr()?;
                // Aqui você passaria para um StmtKind::PrintInline ou similar
                Ok(Stmt::new(StmtKind::Print(expr), sp..self.pos))
            }

            Some(Token::At) => self.parse_at_stmt(sp),

            Some(Token::Read) => {
                self.advance();
                Ok(Stmt::new(StmtKind::Read, sp..self.pos))
            }

            Some(Token::Ident(_)) => self.parse_ident_stmt(sp),
            Some(other) => Err(ParseError::Unexpected {
                got: format!("{other:?}"),
                expected: "statement".into(),
                pos: sp,
            }),
            None => Err(ParseError::Eof("statement".into())),
        }
    }

    // ── @ row, col SAY/GET ───────────────────────────────────────────────

    fn parse_at_stmt(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // consume `@`
        let row = self.parse_expr()?;
        self.skip_newlines();
        self.expect(&Token::Comma)?;
        self.skip_newlines();
        let col = self.parse_expr()?;

        match self.peek().cloned() {
            Some(Token::Say) => {
                self.advance();
                let expr = self.parse_expr()?;
                // PICTURE clause is accepted but ignored on SAY
                self.try_consume_picture();
                Ok(Stmt::new(StmtKind::AtSay(AtSayStmt { row, col, expr }), sp..self.pos))
            }
            Some(Token::Get) => {
                self.advance();
                let var = self.expect_ident()?;
                let picture = self.try_consume_picture();
                Ok(Stmt::new(StmtKind::AtGet(AtGetStmt { row, col, var, picture }), sp..self.pos))
            }
            Some(other) => Err(ParseError::Unexpected {
                got: format!("{other:?}"),
                expected: "SAY or GET after @row,col".into(),
                pos: self.pos,
            }),
            None => Err(ParseError::Eof("SAY or GET".into())),
        }
    }

    /// Consome `PICTURE "mask"` se presente; retorna a string da máscara.
    fn try_consume_picture(&mut self) -> Option<String> {
        if !matches!(self.peek(), Some(Token::Picture)) {
            return None;
        }
        self.advance(); // consume PICTURE
        match self.advance() {
            Some(Token::StringLit(s)) | Some(Token::StringLitSingle(s)) => Some(s),
            Some(Token::BracketString(s)) => Some(s),
            _ => None,
        }
    }

    fn parse_var_decl(&mut self, is_static: bool, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // LOCAL or STATIC
        let mut vars = Vec::new();
        loop {
            let vsp = self.pos;
            let name = self.expect_ident()?;
            let init = if matches!(self.peek(), Some(Token::Assign)) {
                self.advance(); // :=
                Some(self.parse_expr()?)
            } else if matches!(self.peek(), Some(Token::Eq)) {
                // Legacy `=` used as assignment in declaration
                self.advance();
                Some(self.parse_expr()?)
            } else {
                None
            };
            vars.push(VarInit { name, init, span: vsp..self.pos });
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
            } else {
                break;
            }
        }
        let kind = if is_static {
            StmtKind::StaticDecl(VarDeclStmt { vars })
        } else {
            StmtKind::VarDecl(VarDeclStmt { vars })
        };
        Ok(Stmt::new(kind, sp..self.pos))
    }

    fn parse_memvar(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // MEMVAR or PRIVATE
        let mut names = Vec::new();
        loop {
            names.push(self.expect_ident()?);
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(Stmt::new(StmtKind::MemvarDecl(names), sp..self.pos))
    }

    fn parse_field(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // FIELD
        let mut names = Vec::new();
        loop {
            names.push(self.expect_ident()?);
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
            } else {
                break;
            }
        }
        // Optional: IN alias
        let alias = if matches!(self.peek(), Some(Token::Ident(s)) if s == "IN") {
            self.advance(); // IN
            Some(self.expect_ident()?)
        } else {
            None
        };
        Ok(Stmt::new(StmtKind::FieldDecl { names, alias }, sp..self.pos))
    }

    fn parse_public(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // PUBLIC
        let mut vars = Vec::new();
        loop {
            let vsp = self.pos;
            let name = self.expect_ident()?;
            let init = if matches!(self.peek(), Some(Token::Assign)) {
                self.advance(); // :=
                Some(self.parse_expr()?)
            } else if matches!(self.peek(), Some(Token::Eq)) {
                self.advance(); // legacy =
                Some(self.parse_expr()?)
            } else {
                None
            };
            vars.push(VarInit { name, init, span: vsp..self.pos });
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(Stmt::new(StmtKind::PublicDecl(VarDeclStmt { vars }), sp..self.pos))
    }

    fn parse_if(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // IF
        let cond = self.parse_expr()?;
        self.expect_newline_or_semi();
        let then_body = self.parse_stmts_until(&[
            Token::Else, Token::ElseIf, Token::EndIf,
        ])?;

        let mut elseif_branches = Vec::new();
        let mut else_body = None;

        loop {
            match self.peek().cloned() {
                Some(Token::ElseIf) => {
                    self.advance();
                    let c = self.parse_expr()?;
                    self.expect_newline_or_semi();
                    let b = self.parse_stmts_until(&[
                        Token::Else, Token::ElseIf, Token::EndIf,
                    ])?;
                    elseif_branches.push((c, b));
                }
                Some(Token::Else) => {
                    self.advance();
                    self.expect_newline_or_semi();
                    else_body = Some(self.parse_stmts_until(&[Token::EndIf])?);
                    self.expect(&Token::EndIf)?;
                    break;
                }
                Some(Token::EndIf) => {
                    self.advance();
                    break;
                }
                _ => break,
            }
        }

        Ok(Stmt::new(
            StmtKind::If(IfStmt { condition: cond, then_body, elseif_branches, else_body }),
            sp..self.pos,
        ))
    }

    fn parse_stmts_until(&mut self, terminators: &[Token]) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        loop {
            self.skip_newlines();
            if self.peek().is_none() {
                break;
            }
            if let Some(tok) = self.peek() {
                if terminators.iter().any(|t| std::mem::discriminant(t) == std::mem::discriminant(tok)) {
                    break;
                }
            }
            stmts.push(self.parse_stmt()?);
            self.expect_newline_or_semi();
        }
        Ok(stmts)
    }

    fn parse_do_while(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // DO
        self.expect(&Token::While)?;
        let cond = self.parse_expr()?;
        self.expect_newline_or_semi();
        let body = self.parse_stmts_until(&[Token::EndDo])?;
        self.expect(&Token::EndDo)?;
        Ok(Stmt::new(
            StmtKind::DoWhile(DoWhileStmt { condition: cond, body }),
            sp..self.pos,
        ))
    }

    fn parse_while(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // WHILE
        let cond = self.parse_expr()?;
        self.expect_newline_or_semi();
        let body = self.parse_stmts_until(&[Token::EndDo])?;
        self.expect(&Token::EndDo)?;
        Ok(Stmt::new(
            StmtKind::DoWhile(DoWhileStmt { condition: cond, body }),
            sp..self.pos,
        ))
    }

    fn parse_for(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // FOR
        let var = self.expect_ident()?;
        // := or =
        if matches!(self.peek(), Some(Token::Assign) | Some(Token::Eq)) {
            self.advance();
        }
        let start = self.parse_expr()?;
        self.expect(&Token::To)?;
        let end = self.parse_expr()?;
        let step = if matches!(self.peek(), Some(Token::Step)) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect_newline_or_semi();
        let body = self.parse_stmts_until(&[Token::Next])?;
        self.expect(&Token::Next)?;
        Ok(Stmt::new(
            StmtKind::For(ForStmt { var, start, end, step, body }),
            sp..self.pos,
        ))
    }

    fn parse_store(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        self.advance(); // STORE
        let value = self.parse_expr()?;
        self.expect(&Token::To)?;
        let mut vars = vec![self.expect_ident()?];
        while matches!(self.peek(), Some(Token::Comma)) {
            self.advance(); // Consome a vírgula
            vars.push(self.expect_ident()?);
        }
        Ok(Stmt::new(
            StmtKind::Store(StoreStmt { value, vars }),
            sp..self.pos,
        ))
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        let sp = self.pos;
        self.advance(); // RETURN
        let expr = match self.peek() {
            Some(Token::Newline) | Some(Token::Semicolon) | None => None,
            _ => Some(self.parse_expr()?),
        };
        Ok(Stmt::new(StmtKind::Return(expr), sp..self.pos))
    }

    /// Statement that starts with an identifier: call or assignment.
    fn parse_ident_stmt(&mut self, sp: usize) -> Result<Stmt, ParseError> {
        // Peek ahead: if ident followed by `(` → call statement
        // if ident followed by `:=` or `=` → assignment
        // if ident followed by `[` → array element assignment
        let name = match self.advance() {
            Some(Token::Ident(s)) => s,
            _ => unreachable!(),
        };

        match self.peek().cloned() {
            // Function call used as statement: Name( ... )
            Some(Token::LParen) => {
                let call = self.parse_call_args(name, sp)?;
                Ok(Stmt::new(StmtKind::Call(call), sp..self.pos))
            }
            // Assignment: x := expr
            Some(Token::Assign) => {
                self.advance();
                let value = self.parse_expr()?;
                let target = Expr::ident(name, sp..sp + 1);
                Ok(Stmt::new(StmtKind::Assign(AssignStmt { target, value }), sp..self.pos))
            }
            // Legacy assignment: x = expr (only valid when `x` is already known var)
            Some(Token::Eq) => {
                self.advance();
                let value = self.parse_expr()?;
                let target = Expr::ident(name, sp..sp + 1);
                Ok(Stmt::new(StmtKind::Assign(AssignStmt { target, value }), sp..self.pos))
            }
            // Array element assignment: a[i] := expr
            Some(Token::LBracket) => {
                self.advance();
                let idx = self.parse_expr()?;
                self.expect(&Token::RBracket)?;
                self.expect(&Token::Assign)?;
                let value = self.parse_expr()?;
                let arr = Expr::ident(&name, sp..sp + 1);
                let target = Expr {
                    kind: ExprKind::Index(Box::new(arr), Box::new(idx)),
                    span: sp..self.pos,
                };
                Ok(Stmt::new(StmtKind::Assign(AssignStmt { target, value }), sp..self.pos))
            }
            // Array element assignment with BracketString: a[ i ] := expr
            Some(Token::BracketString(_)) => {
                let s = match self.peek() {
                    Some(Token::BracketString(s)) => s.clone(),
                    _ => unreachable!(),
                };
                self.advance();
                let s = self.balance_bracket_content(s);
                let normalized = crate::lexer::normalize(s.trim());
                // Split by commas to support multidimensional arrays: a[i,j] => nested Index calls
                let indices_str: Vec<&str> = normalized.split(',').collect();
                let mut indices = Vec::new();
                for idx_str in indices_str {
                    let sub_tokens: Vec<Token> = crate::lexer::tokenize(idx_str.trim())
                        .into_iter()
                        .filter_map(|(t, _)| t.ok())
                        .filter(|t| !matches!(t, Token::Newline))
                        .collect();
                    let mut sub = Parser::new(sub_tokens);
                    let idx = sub.parse_expr().map_err(|e| {
                        ParseError::Other(format!("invalid array index [{}]: {}", idx_str.trim(), e))
                    })?;
                    indices.push(idx);
                }
                // Apply indices left-to-right: a[i,j] := expr => Index(Index(a, i), j) := expr
                let mut arr = Expr::ident(&name, sp..sp + 1);
                for idx in indices {
                    arr = Expr {
                        kind: ExprKind::Index(Box::new(arr), Box::new(idx)),
                        span: sp..self.pos,
                    };
                }
                self.expect(&Token::Assign)?;
                let value = self.parse_expr()?;
                Ok(Stmt::new(StmtKind::Assign(AssignStmt { target: arr, value }), sp..self.pos))
            }
            // OOP field assignment: obj:field := expr
            Some(Token::Colon) => {
                self.advance();
                let field = self.expect_ident()?;
                self.expect(&Token::Assign)?;
                let value = self.parse_expr()?;
                let obj = Expr::ident(&name, sp..sp + 1);
                let target = Expr {
                    kind: ExprKind::Field(Box::new(obj), field),
                    span: sp..self.pos,
                };
                Ok(Stmt::new(StmtKind::Assign(AssignStmt { target, value }), sp..self.pos))
            }
            // Compound assignment: x += e  →  x := x + e  (desugar at parse time)
            Some(Token::PlusAssign)
            | Some(Token::MinusAssign)
            | Some(Token::StarAssign)
            | Some(Token::SlashAssign) => {
                let op = match self.peek() {
                    Some(Token::PlusAssign)  => BinOp::Add,
                    Some(Token::MinusAssign) => BinOp::Sub,
                    Some(Token::StarAssign)  => BinOp::Mul,
                    Some(Token::SlashAssign) => BinOp::Div,
                    _ => unreachable!(),
                };
                self.advance();
                let rhs = self.parse_expr()?;
                let target = Expr::ident(&name, sp..sp + 1);
                let lhs    = Expr::ident(&name, sp..sp + 1);
                let value  = Expr {
                    kind: ExprKind::BinOp(op, Box::new(lhs), Box::new(rhs)),
                    span: sp..self.pos,
                };
                Ok(Stmt::new(StmtKind::Assign(AssignStmt { target, value }), sp..self.pos))
            }
            // Post-increment: var++
            Some(Token::Increment) => {
                self.advance();
                let expr = Expr::ident(&name, sp..sp + 1);
                let expr_with_op = Expr {
                    kind: ExprKind::UnOp(UnOp::PostIncrement, Box::new(expr)),
                    span: sp..self.pos,
                };
                Ok(Stmt::new(StmtKind::Call(CallExpr { 
                    callee: "__EXPR_STMT__".to_string(),
                    args: vec![expr_with_op],
                    span: sp..self.pos,
                }), sp..self.pos))
            }
            // Post-decrement: var--
            Some(Token::Decrement) => {
                self.advance();
                let expr = Expr::ident(&name, sp..sp + 1);
                let expr_with_op = Expr {
                    kind: ExprKind::UnOp(UnOp::PostDecrement, Box::new(expr)),
                    span: sp..self.pos,
                };
                Ok(Stmt::new(StmtKind::Call(CallExpr { 
                    callee: "__EXPR_STMT__".to_string(),
                    args: vec![expr_with_op],
                    span: sp..self.pos,
                }), sp..self.pos))
            }
            other => Err(ParseError::Unexpected {
                got: format!("{other:?}"),
                expected: ":=, =, (, [ or :".into(),
                pos: self.pos,
            }),
        }
    }

    // -----------------------------------------------------------------------
    // Expressions — Pratt-style precedence climbing
    // -----------------------------------------------------------------------

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        loop {
            let is_or = match self.peek() {
                Some(Token::Or) => true,
                Some(Token::Ident(s)) => s == "OR" || s == ".OR.",
                _ => false,
            };
            if !is_or {
                break;
            }
            let sp = self.pos;
            self.advance();
            let right = self.parse_and()?;
            left = Expr {
                kind: ExprKind::BinOp(BinOp::Or, Box::new(left), Box::new(right)),
                span: sp..self.pos,
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_not()?;
        loop {
            let is_and = match self.peek() {
                Some(Token::And) => true,
                Some(Token::Ident(s)) => s == "AND" || s == ".AND.",
                _ => false,
            };
            if !is_and {
                break;
            }
            let sp = self.pos;
            self.advance();
            let right = self.parse_not()?;
            left = Expr {
                kind: ExprKind::BinOp(BinOp::And, Box::new(left), Box::new(right)),
                span: sp..self.pos,
            };
        }
        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expr, ParseError> {
        let sp = self.pos;
        let is_not = match self.peek() {
            Some(Token::Bang) => true,
            Some(Token::Not) => true,
            Some(Token::Ident(s)) => s == "NOT" || s == ".NOT.",
            _ => false,
        };
        if is_not {
            self.advance();
            let expr = self.parse_not()?;
            return Ok(Expr {
                kind: ExprKind::UnOp(UnOp::Not, Box::new(expr)),
                span: sp..self.pos,
            });
        }
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_additive()?;
        loop {
            let sp = self.pos;
            let op = match self.peek() {
                Some(Token::StrictEq)  => BinOp::StrictEq,
                Some(Token::NotEq) | Some(Token::NotEq2) => BinOp::NotEq,
                Some(Token::Lt)        => BinOp::Lt,
                Some(Token::Lte)       => BinOp::Lte,
                Some(Token::Gt)        => BinOp::Gt,
                Some(Token::Gte)       => BinOp::Gte,
                Some(Token::Eq)        => BinOp::Eq,
                Some(Token::InStr)     => BinOp::InStr,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr {
                kind: ExprKind::BinOp(op, Box::new(left), Box::new(right)),
                span: sp..self.pos,
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let sp = self.pos;
            let op = match self.peek() {
                Some(Token::Plus)  => BinOp::Add,
                Some(Token::Minus) => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr {
                kind: ExprKind::BinOp(op, Box::new(left), Box::new(right)),
                span: sp..self.pos,
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        loop {
            let sp = self.pos;
            let op = match self.peek() {
                Some(Token::Star)    => BinOp::Mul,
                Some(Token::Slash)   => BinOp::Div,
                Some(Token::Percent) => BinOp::Mod,
                Some(Token::Caret)   => BinOp::Pow,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr {
                kind: ExprKind::BinOp(op, Box::new(left), Box::new(right)),
                span: sp..self.pos,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        let sp = self.pos;
        match self.peek().cloned() {
            Some(Token::Minus) => {
                self.advance();
                let e = self.parse_primary()?;
                Ok(Expr {
                    kind: ExprKind::UnOp(UnOp::Neg, Box::new(e)),
                    span: sp..self.pos,
                })
            }
            Some(Token::Increment) => {
                self.advance();
                let e = self.parse_primary()?;
                Ok(Expr {
                    kind: ExprKind::UnOp(UnOp::PreIncrement, Box::new(e)),
                    span: sp..self.pos,
                })
            }
            Some(Token::Decrement) => {
                self.advance();
                let e = self.parse_primary()?;
                Ok(Expr {
                    kind: ExprKind::UnOp(UnOp::PreDecrement, Box::new(e)),
                    span: sp..self.pos,
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let sp = self.pos;
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek().cloned() {
                // array index: expr[idx] or expr[idx1, idx2, ...]  — BracketString("idx") follows an expression.
                // Re-normalize the content (uppercase) then sub-parse as expression(s).
                Some(Token::BracketString(s)) => {
                    let s = s.clone();
                    self.advance();
                    let s = self.balance_bracket_content(s);
                    let normalized = crate::lexer::normalize(s.trim());
                    // Split by commas to support multidimensional arrays: a[i,j,k] => nested Index calls
                    let indices_str: Vec<&str> = normalized.split(',').collect();
                    let mut indices = Vec::new();
                    for idx_str in indices_str {
                        let sub_tokens: Vec<Token> = crate::lexer::tokenize(idx_str.trim())
                            .into_iter()
                            .filter_map(|(t, _)| t.ok())
                            .filter(|t| !matches!(t, Token::Newline))
                            .collect();
                        let mut sub = Parser::new(sub_tokens);
                        let idx = sub.parse_expr().map_err(|e| {
                            ParseError::Other(format!("invalid array index [{}]: {}", idx_str.trim(), e))
                        })?;
                        indices.push(idx);
                    }
                    // Apply indices left-to-right: a[i,j] => Index(Index(a, i), j)
                    for idx in indices {
                        expr = Expr {
                            kind: ExprKind::Index(Box::new(expr), Box::new(idx)),
                            span: sp..self.pos,
                        };
                    }
                }
                // Kept for error-recovery on unclosed brackets
                Some(Token::LBracket) => {
                    self.advance();
                    let idx = self.parse_expr()?;
                    self.expect(&Token::RBracket)?;
                    expr = Expr {
                        kind: ExprKind::Index(Box::new(expr), Box::new(idx)),
                        span: sp..self.pos,
                    };
                }
                // alias/memvar operator: expr->field
                // m->varname forces MEMVAR lookup; ALIAS->field is DBF area access.
                Some(Token::Alias) => {
                    self.advance();
                    let field = self.expect_ident()?;
                    let kind = if let ExprKind::Ident(ref alias) = expr.kind {
                        if alias.to_ascii_uppercase() == "M" {
                            ExprKind::Macro(field)
                        } else {
                            ExprKind::Field(Box::new(expr), field)
                        }
                    } else {
                        ExprKind::Field(Box::new(expr), field)
                    };
                    expr = Expr { kind, span: sp..self.pos };
                }
                // field access: expr:field or method call: expr:method(...)
                Some(Token::Colon) => {
                    self.advance();
                    let field = self.expect_ident()?;
                    if matches!(self.peek(), Some(Token::LParen)) {
                        // method call → represent as Call with obj:method naming
                        let call = self.parse_call_args(
                            format!("__OBJ_CALL__{field}"),
                            sp,
                        )?;
                        expr = Expr { kind: ExprKind::Call(call), span: sp..self.pos };
                    } else {
                        expr = Expr {
                            kind: ExprKind::Field(Box::new(expr), field),
                            span: sp..self.pos,
                        };
                    }
                }
                Some(Token::Increment) => {
                    self.advance();
                    expr = Expr {
                        kind: ExprKind::UnOp(UnOp::PostIncrement, Box::new(expr)),
                        span: sp..self.pos,
                    };
                }
                Some(Token::Decrement) => {
                    self.advance();
                    expr = Expr {
                        kind: ExprKind::UnOp(UnOp::PostDecrement, Box::new(expr)),
                        span: sp..self.pos,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let sp = self.pos;
        match self.peek().cloned() {
            Some(Token::Nil) => {
                self.advance();
                Ok(Expr::nil(sp..self.pos))
            }
            Some(Token::True) => {
                self.advance();
                Ok(Expr { kind: ExprKind::Bool(true), span: sp..self.pos })
            }
            Some(Token::False) => {
                self.advance();
                Ok(Expr { kind: ExprKind::Bool(false), span: sp..self.pos })
            }
            Some(Token::IntLit(n)) => {
                self.advance();
                Ok(Expr::int(n, sp..self.pos))
        }
            Some(Token::FloatLit(f)) => {
                self.advance();
                Ok(Expr { kind: ExprKind::Float(f), span: sp..self.pos })
            }
            Some(Token::StringLit(s)) | Some(Token::StringLitSingle(s)) => {
                self.advance();
                Ok(Expr::string(s, sp..self.pos))
            }
            // Bracket string `[text]` — lexer emits BracketString with raw inner content.
            Some(Token::BracketString(s)) => {
                let s = s.clone();            
                self.advance();
                Ok(Expr::string(s, sp..self.pos))
            }
            // Array literal: { e1, e2, ... }
            Some(Token::LBrace) => {
                self.advance();
                let mut elems = Vec::new();
                while !matches!(self.peek(), Some(Token::RBrace) | None) {
                    elems.push(self.parse_expr()?);
                    if matches!(self.peek(), Some(Token::Comma)) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(&Token::RBrace)?;
                Ok(Expr { kind: ExprKind::ArrayLit(elems), span: sp..self.pos })
            }
            // Grouped expression
            Some(Token::LParen) => {
                self.advance();
                let e = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(e)
            }
            // Macro substitution: &varName
            Some(Token::Macro) => {
                self.advance();
                let name = self.expect_ident()?;
                Ok(Expr { kind: ExprKind::Macro(name), span: sp..self.pos })
            }
            // Identifier: variable or function call
            Some(Token::Ident(name)) => {
                self.advance();
                if matches!(self.peek(), Some(Token::LParen)) {
                    // Inline IIF check
                    if name == "IIF" || name == "IF" {
                        return self.parse_iif(sp);
                    }
                    let call = self.parse_call_args(name, sp)?;
                    Ok(Expr { kind: ExprKind::Call(call), span: sp..self.pos })
                } else {
                    Ok(Expr::ident(name, sp..self.pos))
                }
            }
            Some(other) => Err(ParseError::Unexpected {
                got: format!("{other:?}"),
                expected: "expression".into(),
                pos: sp,
            }),
            None => Err(ParseError::Eof("expression".into())),
        }
    }

    fn parse_call_args(&mut self, callee: String, sp: usize) -> Result<CallExpr, ParseError> {
        self.expect(&Token::LParen)?;
        self.skip_newlines();
        let mut args = Vec::new();
        while !matches!(self.peek(), Some(Token::RParen) | None) {
            // NIL argument: bare comma → NIL
            if matches!(self.peek(), Some(Token::Comma)) {
                args.push(Expr::nil(self.pos..self.pos));
            } else {
                args.push(self.parse_expr()?);
            }
            self.skip_newlines();
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
                self.skip_newlines();
            } else {
                break;
            }
        }
        self.expect(&Token::RParen)?;
        Ok(CallExpr { callee, args, span: sp..self.pos })
    }

    fn parse_iif(&mut self, sp: usize) -> Result<Expr, ParseError> {
        self.expect(&Token::LParen)?;
        self.skip_newlines();
        let cond = self.parse_expr()?;
        self.skip_newlines();
        self.expect(&Token::Comma)?;
        self.skip_newlines();
        let then = self.parse_expr()?;
        self.skip_newlines();
        self.expect(&Token::Comma)?;
        self.skip_newlines();
        let else_ = self.parse_expr()?;
        self.skip_newlines();
        self.expect(&Token::RParen)?;
        Ok(Expr {
            kind: ExprKind::Iif(Box::new(cond), Box::new(then), Box::new(else_)),
            span: sp..self.pos,
        })
    }
}

// ---------------------------------------------------------------------------
// Public convenience: parse from a token stream
// ---------------------------------------------------------------------------

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    Parser::new(tokens).parse_program()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{normalize, tokenize};

    fn tokens(src: &str) -> Vec<Token> {
        let norm = normalize(src);
        tokenize(&norm)
            .into_iter()
            .filter_map(|(t, _)| t.ok())
            .collect()
    }

    #[test]
    fn test_parse_empty_proc() {
        let prog = parse(tokens("PROCEDURE Main()\nRETURN")).unwrap();
        assert_eq!(prog.units.len(), 1);
        match &prog.units[0] {
            TopLevel::Procedure(p) => assert_eq!(p.name, "MAIN"),
            _ => panic!("expected procedure"),
        }
    }

    #[test]
    fn test_parse_local_decl() {
        let src = "PROCEDURE Main()\nLOCAL x := 42\nRETURN";
        let prog = parse(tokens(src)).unwrap();
        match &prog.units[0] {
            TopLevel::Procedure(p) => {
                // body[0] = LOCAL decl, body[1] = RETURN
                assert!(p.body.len() >= 1);
                match &p.body[0].kind {
                    StmtKind::VarDecl(d) => {
                        assert_eq!(d.vars[0].name, "X");
                        assert!(matches!(
                            d.vars[0].init.as_ref().map(|e| &e.kind),
                            Some(ExprKind::Integer(42))
                        ));
                    }
                    _ => panic!("expected VarDecl"),
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_aadd_call() {
        let src = "PROCEDURE Main()\nLOCAL aTeste := {}\nAAdd( aTeste, 1 )\nRETURN";
        let prog = parse(tokens(src)).unwrap();
        match &prog.units[0] {
            TopLevel::Procedure(p) => {
                // body[0] = LOCAL, body[1] = AAdd call, body[2] = RETURN
                match &p.body[1].kind {
                    StmtKind::Call(c) => assert_eq!(c.callee, "AADD"),
                    _ => panic!("expected Call statement"),
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_for_loop() {
        let src = "PROCEDURE Main()\nFOR i := 1 TO 10\nNEXT\nRETURN";
        let prog = parse(tokens(src)).unwrap();
        match &prog.units[0] {
            TopLevel::Procedure(p) => {
                match &p.body[0].kind {
                    StmtKind::For(f) => {
                        assert_eq!(f.var, "I");
                        assert!(f.step.is_none());
                    }
                    _ => panic!("expected For"),
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_m_arrow_is_macro() {
        // m->varName deve produzir ExprKind::Macro("VARNAME")
        let src = "PROCEDURE Main()\n? m->nCounter\nRETURN";
        let prog = parse(tokens(src)).unwrap();
        match &prog.units[0] {
            TopLevel::Procedure(p) => match &p.body[0].kind {
                StmtKind::Print(e) => {
                    assert!(matches!(&e.kind, ExprKind::Macro(n) if n == "NCOUNTER"));
                }
                _ => panic!("expected Print"),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_alias_arrow_is_field() {
        // CUST->NAME deve produzir ExprKind::Field (DBF alias, não MEMVAR)
        let src = "PROCEDURE Main()\n? CUST->NAME\nRETURN";
        let prog = parse(tokens(src)).unwrap();
        match &prog.units[0] {
            TopLevel::Procedure(p) => match &p.body[0].kind {
                StmtKind::Print(e) => {
                    assert!(matches!(&e.kind, ExprKind::Field(_, f) if f == "NAME"));
                }
                _ => panic!("expected Print"),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_nested_array_index_read() {
        // aArray[aPosicao[1]] — índice interno deve ser resolvido primeiro.
        // O lexer emite BracketString(" APOSICAO[ 1 ") + RBracket (outer ]).
        // balance_bracket_content deve recompor " APOSICAO[ 1 ]" antes do sub-parse.
        let src = "PROCEDURE Main()\nLOCAL aArray, aPosicao\n? aArray[aPosicao[1]]\nRETURN";
        let prog = parse(tokens(src)).unwrap();
        match &prog.units[0] {
            TopLevel::Procedure(p) => match &p.body[1].kind {
                StmtKind::Print(e) => {
                    // Deve ser Index(Ident(AARRAY), Index(Ident(APOSICAO), Integer(1)))
                    match &e.kind {
                        ExprKind::Index(outer_arr, inner_idx) => {
                            assert!(matches!(&outer_arr.kind, ExprKind::Ident(n) if n == "AARRAY"));
                            assert!(matches!(&inner_idx.kind,
                                ExprKind::Index(arr, idx)
                                if matches!(&arr.kind, ExprKind::Ident(n) if n == "APOSICAO")
                                && matches!(&idx.kind, ExprKind::Integer(1))
                            ));
                        }
                        other => panic!("expected nested Index, got {:?}", other),
                    }
                }
                _ => panic!("expected Print"),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_nested_array_index_assign() {
        // aArray[aPosicao[1]] := val — lado esquerdo de atribuição com índice aninhado.
        let src = "PROCEDURE Main()\nLOCAL aArray, aPosicao\naArray[aPosicao[1]] := 42\nRETURN";
        let prog = parse(tokens(src)).unwrap();
        match &prog.units[0] {
            TopLevel::Procedure(p) => match &p.body[1].kind {
                StmtKind::Assign(a) => {
                    match &a.target.kind {
                        ExprKind::Index(outer_arr, inner_idx) => {
                            assert!(matches!(&outer_arr.kind, ExprKind::Ident(n) if n == "AARRAY"));
                            assert!(matches!(&inner_idx.kind,
                                ExprKind::Index(arr, idx)
                                if matches!(&arr.kind, ExprKind::Ident(n) if n == "APOSICAO")
                                && matches!(&idx.kind, ExprKind::Integer(1))
                            ));
                        }
                        other => panic!("expected nested Index on lhs, got {:?}", other),
                    }
                }
                _ => panic!("expected Assign"),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_compound_assign_desugars_to_binop() {
        // nVar += 5  →  Assign(nVar, BinOp(Add, nVar, 5))
        let src = "PROCEDURE Main()\nLOCAL nVar := 0\nnVar += 5\nRETURN";
        let prog = parse(tokens(src)).unwrap();
        match &prog.units[0] {
            TopLevel::Procedure(p) => match &p.body[1].kind {
                StmtKind::Assign(a) => {
                    assert!(matches!(&a.target.kind, ExprKind::Ident(n) if n == "NVAR"));
                    assert!(matches!(&a.value.kind,
                        ExprKind::BinOp(BinOp::Add, lhs, rhs)
                        if matches!(&lhs.kind, ExprKind::Ident(n) if n == "NVAR")
                        && matches!(&rhs.kind, ExprKind::Integer(5))
                    ));
                }
                _ => panic!("expected Assign"),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_compound_assign_all_ops() {
        // Cobre os 4 operadores; verifica apenas o BinOp gerado.
        let cases = [
            ("nVar -= 3", BinOp::Sub),
            ("nVar *= 2", BinOp::Mul),
            ("nVar /= 4", BinOp::Div),
        ];
        for (stmt, expected_op) in &cases {
            let src = format!("PROCEDURE Main()\nLOCAL nVar := 0\n{stmt}\nRETURN");
            let prog = parse(tokens(&src)).unwrap();
            match &prog.units[0] {
                TopLevel::Procedure(p) => match &p.body[1].kind {
                    StmtKind::Assign(a) => {
                        assert!(
                            matches!(&a.value.kind, ExprKind::BinOp(op, _, _) if op == expected_op),
                            "{stmt}: expected BinOp {:?}", expected_op
                        );
                    }
                    _ => panic!("{stmt}: expected Assign"),
                },
                _ => panic!(),
            }
        }
    }
}
