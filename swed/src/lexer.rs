// swed/src/lexer.rs
// Lexer Layer — Harbour token recognition via `logos`
// Harbour is case-insensitive; callers must normalize input to UPPERCASE
// before lexing (see lexer::normalize).

use logos::Logos;

/// Normalizes Harbour source to uppercase before lexing.
/// Preserves string literals as-is.
pub fn normalize(src: &str) -> String {
    // Strip preprocessor directive lines before case-folding.
    // Lines whose first non-space char is `#` are Harbour/xBase directives
    // (#ifdef, #ifndef, #else, #endif, #define, #include, #undef, #command, etc.).
    // We strip the marker lines and keep the code inside all branches — a proper
    // preprocessor can be added later; for now this avoids parse errors on real files.
    let stripped: String = src
        .lines()
        .map(|line| if line.trim_start().starts_with('#') { "" } else { line })
        .collect::<Vec<_>>()
        .join("\n");

    let mut out = String::with_capacity(stripped.len());
    let mut in_string = false;
    let mut string_delim = ' ';
    let mut in_bracket = false; // [text] string literal — preserve case

    for ch in stripped.chars() {
        if in_string {
            out.push(ch);
            if ch == string_delim { in_string = false; }
        } else if in_bracket {
            out.push(ch); // preserve original case inside [...]
            if ch == ']' { in_bracket = false; }
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
    // Harbour `*` comment: only valid when `*` is the first non-space char on a line.
    // We handle this by giving the regex higher priority than the `*` operator token.
    #[regex(r"\*[^\n]*", logos::skip, priority = 3)]
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
}
