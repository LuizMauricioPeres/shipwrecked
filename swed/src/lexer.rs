// swed/src/lexer.rs
// Lexer Layer — Harbour token recognition via `logos`
// Harbour is case-insensitive; callers must normalize input to UPPERCASE
// before lexing (see lexer::normalize).

use logos::Logos;
use std::collections::HashMap;

/// Extrai constantes #define do código fonte e retorna (mapa de constantes, código sem preprocessor)
fn extract_defines(src: &str) -> (HashMap<String, String>, String) {
    let mut defines = HashMap::new();
    let mut code_lines = Vec::new();

    for line in src.lines() {
        let trimmed = line.trim_start();
        // Harbour `*` comment: valid only when `*` is the first non-space char on the line.
        // Strip here so the lexer never sees `*` as a comment-start mid-expression.
        // Exception: `*/` is the closing delimiter of a C-style block comment — must be kept
        // so the logos regex can match and discard the full `/* ... */` span correctly.
        if trimmed.starts_with('*') && !trimmed.starts_with("*/") {
            code_lines.push("");
            continue;
        }
        if trimmed.starts_with("#define") {
            // Formato: #define NAME value [/* comment */]
            // Remove comentário primeiro
            let line_without_comment = if let Some(comment_pos) = trimmed.find("/*") {
                &trimmed[..comment_pos]
            } else {
                trimmed
            };

            let parts: Vec<&str> = line_without_comment
                .split_whitespace()
                .collect();
            if parts.len() >= 3 {
                let name = parts[1].to_uppercase();
                let value = parts[2..].join(" ");
                defines.insert(name, value);
            }
            code_lines.push(""); // manter linhas em branco para preservar numeração
        } else if !trimmed.starts_with('#') {
            // Manter linhas que não são preprocessor
            code_lines.push(line);
        } else {
            // Outras diretivas (#ifdef, etc.) — remover
            code_lines.push("");
        }
    }

    (defines, code_lines.join("\n"))
}

/// Remove linhas que começam com `;` (continuação de linha em Harbour)
/// ou remove o newline antes de `;` para permitir linha contínua.
/// Padrão Harbour: `;` no final permite quebra de linha.
fn handle_line_continuations(src: &str) -> String {
    let mut result = String::new();
    for ch in src.chars() {
        if ch == '\n' {
            // Lookahead: se a próxima linha começa com espaços e `;`, ou é só `;`,
            // é uma continuação — remover a newline
            // Para simplificar, vamos simplesmente remover `;` seguido de newline
            result.push(ch);
        } else {
            result.push(ch);
        }
    }
    // Remover `;\n` (com qualquer espaço antes)
    let result = result.replace("; \n", " ");
    let result = result.replace(";\n", " ");
    result
}

/// Normalizes Harbour source to uppercase before lexing.
/// Preserves string literals as-is.
/// Processa #define antes de normalizar.
pub fn normalize(src: &str) -> String {
    let (defines, mut stripped) = extract_defines(src);
    
    // Remover continuações de linha (`;` no final permite quebra de linha)
    stripped = handle_line_continuations(&stripped);

    let mut out = String::with_capacity(stripped.len());
    let mut in_string = false;
    let mut string_delim = ' ';
    let mut in_bracket = false; // [text] string literal — preserve case
    let mut current_ident = String::new();

    let is_ident_char = |ch: char| ch.is_alphanumeric() || ch == '_';

    let mut chars_iter = stripped.chars().peekable();
    while let Some(ch) = chars_iter.next() {
        if in_string {
            out.push(ch);
            if ch == string_delim { in_string = false; }
        } else if in_bracket {
            // Inside [...]: preserve original case for string literals like [Alice],
            // but still expand #define constants (they are UPPERCASE by convention).
            if is_ident_char(ch) {
                current_ident.push(ch); // accumulate with original case
                if !chars_iter.peek().map_or(false, |&next_ch| is_ident_char(next_ch)) {
                    let upper = current_ident.to_ascii_uppercase();
                    if let Some(value) = defines.get(upper.as_str()) {
                        out.push_str(value);
                    } else {
                        out.push_str(&current_ident); // preserve case for non-defines
                    }
                    current_ident.clear();
                }
            } else {
                out.push(ch);
                if ch == ']' { in_bracket = false; }
            }
        } else if is_ident_char(ch) {
            // Acumular caracteres de identificador
            current_ident.push(ch.to_ascii_uppercase());
            // Verificar se o próximo caractere é parte do identificador
            if !chars_iter.peek().map_or(false, |&next_ch| is_ident_char(next_ch)) {
                // Fim do identificador — substituir se for constante
                if let Some(value) = defines.get(current_ident.as_str()) {
                    out.push_str(value);
                } else {
                    out.push_str(&current_ident);
                }
                current_ident.clear();
            }
        } else {
            match ch {
                '"' | '\'' => {
                    in_string = true;
                    string_delim = ch;
                    out.push(ch);
                }
                '[' => {
                    in_bracket = true;
                    out.push('[');
                }
                _ => out.push(ch.to_ascii_uppercase()),
            }
        }
    }
    // Processar qualquer identificador restante
    if !current_ident.is_empty() {
        if let Some(value) = defines.get(current_ident.as_str()) {
            out.push_str(value);
        } else {
            out.push_str(&current_ident);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Token definitions
// ---------------------------------------------------------------------------

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r]+")] // skip horizontal whitespace
pub enum Token {
    // ------------------------------------------------------------------
    // Keywords (Harbour is case-insensitive; source is pre-normalized)
    // ------------------------------------------------------------------
    #[token("PROCEDURE")]
    Procedure,
    #[token("FUNCTION")]
    Function,
    #[token("RETURN")]
    Return,
    #[token("LOCAL")]
    Local,
    #[token("STATIC")]
    Static,
    #[token("MEMVAR")]
    Memvar,
    #[token("FIELD")]
    FieldKw,
    #[token("PUBLIC")]
    Public,
    #[token("PRIVATE")]
    Private,
    #[token("IF")]
    If,
    #[token("ELSE")]
    Else,
    #[token("ELSEIF")]
    ElseIf,
    #[token("ENDIF")]
    EndIf,
    #[token("DO")]
    Do,
    #[token("WHILE")]
    While,
    #[token("ENDDO")]
    EndDo,
    #[token("FOR")]
    For,
    #[token("NEXT")]
    Next,
    #[token("TO")]
    To,
    #[token("STEP")]
    Step,
    #[token("STORE")]
    Store,
    #[token("IN")]
    In,
    #[token("EXIT")]
    Exit,
    #[token("LOOP")]
    Loop,
    #[token("CLS")]
    Cls,
    #[token("CLASS")]
    Class,
    #[token("ENDCLASS")]
    EndClass,
    #[token("METHOD")]
    Method,
    #[token("DATA")]
    Data,
    #[token("NIL")]
    Nil,
    #[token(".T.")]
    True,
    #[token(".F.")]
    False,
    #[token(".AND.")]
    And,
    #[token(".OR.")]
    Or,
    #[token(".NOT.")]
    Not,

    // ------------------------------------------------------------------
    // Screen / UI command keywords
    // ------------------------------------------------------------------
    /// `SAY` — used in `@ row, col SAY expr`
    #[token("SAY")]
    Say,
    /// `GET` — used in `@ row, col GET varname [PICTURE "mask"]`
    #[token("GET")]
    Get,
    /// `READ` — triggers the interactive form loop
    #[token("READ")]
    Read,
    /// `PICTURE` — optional clause after GET
    #[token("PICTURE")]
    Picture,

    // ------------------------------------------------------------------
    // Operators — ORDER MATTERS: longer patterns first
    // ------------------------------------------------------------------
    #[token(":=")]
    Assign, // modern Harbour assignment
    #[token("==")]
    StrictEq,
    #[token("!=")]
    NotEq,
    #[token("<>")]
    NotEq2,
    #[token(">=")]
    Gte,
    #[token("<=")]
    Lte,
    #[token("->")]
    Alias, // DBF alias operator  e.g.  CUST->NAME
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    StarAssign,
    #[token("/=")]
    SlashAssign,
    #[token("=")]
    Eq, // legacy assignment OR comparison (context-sensitive)
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("^")]
    Caret,
    #[token("%")]
    Percent,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token("!")]
    Bang,
    /// `??` — QQOUT; longer match must precede `?`
    #[token("??")]
    QuestionQuestion,
    /// `?` — QOUT shorthand
    #[token("?")]
    Question,
    #[token("$")]
    InStr, // substring check operator
    #[token("@")]
    At,
    #[token("&")]
    Macro, // macro-substitution prefix

    // ------------------------------------------------------------------
    // Delimiters
    // ------------------------------------------------------------------
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace, // array literal start
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon, // OOP method/data accessor  obj:method()

    // ------------------------------------------------------------------
    // Context-sensitive tokens: `[` and `;`
    // ------------------------------------------------------------------

    /// `[` as the start of a bracket-string literal.
    /// Requires context from the parser (preceded by `=`, `(`, `[`, `{`).
    /// The lexer emits `LBracket`; the parser decides string vs array-index.
    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    /// `;` is both a line-continuation marker (when at end of line)
    /// and an inline statement separator.  The parser resolves the role.
    #[token(";")]
    Semicolon,

    // ------------------------------------------------------------------
    // Literals
    // ------------------------------------------------------------------

    /// `[text]` as a string literal — longest-match wins over `LBracket`.
    /// Content is captured with original case (normalize skips `[…]`).
    #[regex(r"\[[^\]]*\]", lex_bracket_string)]
    BracketString(String),

    #[regex(r#""[^"]*""#, lex_string)]
    StringLit(String),

    #[regex(r"'[^']*'", lex_string)]
    StringLitSingle(String),

    #[regex(r"\d+\.\d+", |lex| lex.slice().parse::<f64>().ok())]
    FloatLit(f64),

    #[regex(r"\d+", |lex| lex.slice().parse::<i64>().ok())]
    IntLit(i64),

    // ------------------------------------------------------------------
    // Identifiers & line endings
    // ------------------------------------------------------------------
    #[regex(r"[A-Z_][A-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    /// Physical end-of-line.  Acts as implicit statement terminator
    /// unless the previous token was `;` (continuation).
    #[token("\n")]
    Newline,

    // ------------------------------------------------------------------
    // Comments  (stripped at lex time)
    // ------------------------------------------------------------------
    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,

    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    BlockComment,
}

fn lex_bracket_string(lex: &mut logos::Lexer<Token>) -> String {
    let s = lex.slice();
    s[1..s.len() - 1].to_string()
}

fn lex_string(lex: &mut logos::Lexer<Token>) -> String {
    let s = lex.slice();
    // Strip surrounding quotes
    s[1..s.len() - 1].to_string()
}

// ---------------------------------------------------------------------------
// Thin wrapper that emits (Token, Span) pairs
// ---------------------------------------------------------------------------

pub use logos::Span;

pub fn tokenize(src: &str) -> Vec<(Result<Token, ()>, Span)> {
    Token::lexer(src).spanned().collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_vs_eq() {
        let src = normalize("x := 1\ny = 2");
        let tokens: Vec<_> = tokenize(&src)
            .into_iter()
            .filter_map(|(t, _)| t.ok())
            .collect();
        assert!(tokens.contains(&Token::Assign));
        assert!(tokens.contains(&Token::Eq));
    }

    #[test]
    fn test_bracket_ambiguity_emits_bracket_string() {
        // logos longest-match: `[hello]` is always BracketString, never LBracket+Ident+RBracket.
        // Context (string vs array-index) is resolved in the parser (parse_postfix).
        let src = normalize("= [hello]");
        let tokens: Vec<_> = tokenize(&src)
            .into_iter()
            .filter_map(|(t, _)| t.ok())
            .collect();
        assert!(tokens.contains(&Token::BracketString("hello".into())));
        assert!(!tokens.contains(&Token::LBracket));
    }

    #[test]
    fn test_bracket_string_preserves_case() {
        let src = normalize("[Alice]");
        let tokens: Vec<_> = tokenize(&src)
            .into_iter()
            .filter_map(|(t, _)| t.ok())
            .collect();
        assert_eq!(tokens[0], Token::BracketString("Alice".into()));
     }
     
    #[test]
    fn test_case_insensitive_keyword() {
        let src = normalize("procedure main");
        let tokens: Vec<_> = tokenize(&src)
            .into_iter()
            .filter_map(|(t, _)| t.ok())
            .collect();
        assert_eq!(tokens[0], Token::Procedure);
    }

    #[test]
    fn test_star_mid_line_is_mul_not_comment() {
        // `*` mid-line must be multiplication, not a Harbour `*` comment.
        // Regression guard for the bug where `\*[^\n]*` consumed the rest of the line.
        let src = normalize("nValor := 1 * 2");
        let tokens: Vec<_> = tokenize(&src)
            .into_iter()
            .filter_map(|(t, _)| t.ok())
            .collect();
        assert!(tokens.contains(&Token::Star), "Star token missing: {:?}", tokens);
        assert!(tokens.contains(&Token::IntLit(2)), "IntLit(2) was eaten as comment: {:?}", tokens);
    }

    #[test]
    fn test_star_line_start_is_comment() {
        // `*` as the first non-space char on a line must be stripped as a comment.
        let src = normalize("* this is a harbour comment\nnValor := 1");
        let tokens: Vec<_> = tokenize(&src)
            .into_iter()
            .filter_map(|(t, _)| t.ok())
            .filter(|t| !matches!(t, Token::Newline))
            .collect();
        assert!(!tokens.contains(&Token::Star), "Star should have been stripped: {:?}", tokens);
        assert!(tokens.contains(&Token::Assign), "code after comment must survive: {:?}", tokens);
    }

    #[test]
    fn test_define_expanded_inside_bracket_index() {
        // #define PECA 1 must be expanded even when used as array index: aArr[PECA]
        let src = "#define PECA 1\naArr[PECA]";
        let norm = normalize(src);
        let tokens: Vec<_> = tokenize(&norm)
            .into_iter()
            .filter_map(|(t, _)| t.ok())
            .filter(|t| !matches!(t, Token::Newline))
            .collect();
        // The bracket content "1" must lex as IntLit(1), not Ident("PECA")
        assert!(
            tokens.iter().any(|t| matches!(t, Token::BracketString(s) if s.trim() == "1")),
            "define inside bracket-index was not expanded: {:?}", tokens
        );
    }

    #[test]
    fn test_define_inside_bracket_preserves_string_case() {
        // Non-define identifiers inside [string] must preserve original case.
        let src = "[Alice]";
        let norm = normalize(src);
        let tokens: Vec<_> = tokenize(&norm)
            .into_iter()
            .filter_map(|(t, _)| t.ok())
            .collect();
        assert_eq!(tokens[0], Token::BracketString("Alice".into()), "case not preserved: {:?}", tokens);
    }
}
