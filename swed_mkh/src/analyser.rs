//! Analisador de fontes Harbour (.prg) — dois passes: definições e usos.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::types::{Manifest, ParamDef, Symbol, SymbolKind, Usage, Visibility};

/// Analisa um arquivo `.prg` e retorna seu `Manifest`.
///
/// Lê bytes brutos (encoding CP850/Win1252 preservado) e calcula MD5 sobre eles.
///
/// # Errors
/// Retorna `Err(String)` se o arquivo não puder ser lido.
pub fn analyse_file(path: &Path) -> Result<Manifest, String> {
    let bytes = fs::read(path)
        .map_err(|e| format!("cannot read {}: {}", path.display(), e))?;

    let digest = format!("{:x}", md5::compute(&bytes));

    // Identificadores são ASCII puro; bytes >0x7F viram U+FFFD sem afetar o parsing.
    let source = String::from_utf8_lossy(&bytes).into_owned();

    let mut parser = Parser::new(&source);
    parser.run();

    Ok(Manifest {
        source_path: path.to_string_lossy().into_owned(),
        md5: digest,
        symbols: parser.symbols,
        usages: parser.usages,
    })
}

// ── Parser ────────────────────────────────────────────────────────────────────

struct Parser {
    lines: Vec<String>,
    symbols: Vec<Symbol>,
    usages: Vec<Usage>,
    cond_stack: Vec<CondState>,
    current_class: Option<String>,
    current_scope: String,
    defined: HashSet<String>,
}

#[derive(Clone)]
struct CondState;

impl Parser {
    fn new(source: &str) -> Self {
        Self {
            lines: source.lines().map(str::to_string).collect(),
            symbols: Vec::new(),
            usages: Vec::new(),
            cond_stack: Vec::new(),
            current_class: None,
            current_scope: String::from("GLOBAL"),
            defined: HashSet::new(),
        }
    }

    fn in_cond(&self) -> bool {
        !self.cond_stack.is_empty()
    }

    fn run(&mut self) {
        self.pass_definitions();
        self.pass_usages();
    }

    // ── Pass 1: definições ────────────────────────────────────────────────────

    fn pass_definitions(&mut self) {
        self.cond_stack.clear();
        self.current_class = None;
        self.current_scope = String::from("GLOBAL");

        for line_idx in 0..self.lines.len() {
            let lineno = line_idx + 1;
            let raw = self.lines[line_idx].clone();

            // `trimmed` preserva o case original — usado para extração de params.
            // `upper` normaliza para detecção de keywords e nomes de símbolos.
            let trimmed = strip_comment(raw.trim()).to_string();
            let upper_owned = trimmed.to_ascii_uppercase();
            let upper = upper_owned.trim();

            // ── Pré-processador ──────────────────────────────────────────────
            if upper.starts_with("#IFDEF") || upper.starts_with("#IFNDEF") {
                self.cond_stack.push(CondState);
                continue;
            }
            if upper.starts_with("#ELSE") || upper.starts_with("#ENDIF") {
                if upper.starts_with("#ENDIF") {
                    self.cond_stack.pop();
                }
                continue;
            }

            let in_cond = self.in_cond();

            // ── CLASS ────────────────────────────────────────────────────────
            if let Some(name) = parse_keyword(upper, "CLASS") {
                let sym = Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Class,
                    scope: String::from("GLOBAL"),
                    line: lineno,
                    attributes: vec![],
                    conditional: in_cond,
                    hb_type: None,
                    params: vec![],
                };
                self.current_class = Some(name);
                self.current_scope = sym.name.clone();
                self.push_symbol(sym);
                continue;
            }

            // ── ENDCLASS ─────────────────────────────────────────────────────
            if upper.starts_with("ENDCLASS") || upper == "END CLASS" {
                self.current_class = None;
                self.current_scope = String::from("GLOBAL");
                continue;
            }

            // ── FUNCTION ─────────────────────────────────────────────────────
            // `upper` detecta keyword e normaliza o nome; `trimmed` preserva
            // case dos params para que a Hungarian filter funcione corretamente.
            if let Some((name, hb_type, params)) = parse_callable(upper, &trimmed, "FUNCTION") {
                let scope = self.current_class.clone()
                    .unwrap_or_else(|| String::from("GLOBAL"));
                let sym = Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Function,
                    scope,
                    line: lineno,
                    attributes: vec![],
                    conditional: in_cond,
                    hb_type,
                    params,
                };
                self.current_scope = name;
                self.push_symbol(sym);
                continue;
            }

            // ── PROCEDURE ────────────────────────────────────────────────────
            if let Some((name, hb_type, params)) = parse_callable(upper, &trimmed, "PROCEDURE") {
                let scope = self.current_class.clone()
                    .unwrap_or_else(|| String::from("GLOBAL"));
                let sym = Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Procedure,
                    scope,
                    line: lineno,
                    attributes: vec![],
                    conditional: in_cond,
                    hb_type,
                    params,
                };
                self.current_scope = name;
                self.push_symbol(sym);
                continue;
            }

            // ── METHOD ───────────────────────────────────────────────────────
            if let Some((raw_name, hb_type, params)) = parse_method(upper, &trimmed) {
                let (scope, name) = if let Some(colon) = raw_name.find(':') {
                    (raw_name[..colon].to_string(), raw_name[colon + 1..].to_string())
                } else {
                    (
                        self.current_class.clone()
                            .unwrap_or_else(|| self.current_scope.clone()),
                        raw_name,
                    )
                };
                self.push_symbol(Symbol {
                    name,
                    kind: SymbolKind::Method,
                    scope,
                    line: lineno,
                    attributes: vec![],
                    conditional: in_cond,
                    hb_type,
                    params,
                });
                continue;
            }

            // ── PUBLIC ───────────────────────────────────────────────────────
            if upper.starts_with("PUBLIC ") || upper == "PUBLIC" {
                let scope = self.current_scope.clone();
                for (vname, hb_type) in parse_varlist(&trimmed) {
                    self.push_symbol(Symbol {
                        name: vname,
                        kind: SymbolKind::Public,
                        scope: scope.clone(),
                        line: lineno,
                        attributes: vec![],
                        conditional: in_cond,
                        hb_type,
                        params: vec![],
                    });
                }
                continue;
            }

            // ── MEMVAR ───────────────────────────────────────────────────────
            if upper.starts_with("MEMVAR ") || upper == "MEMVAR" {
                let scope = self.current_scope.clone();
                for (vname, hb_type) in parse_varlist(&trimmed) {
                    self.push_symbol(Symbol {
                        name: vname,
                        kind: SymbolKind::Memvar,
                        scope: scope.clone(),
                        line: lineno,
                        attributes: vec![],
                        conditional: in_cond,
                        hb_type,
                        params: vec![],
                    });
                }
                continue;
            }

            // ── STATIC ───────────────────────────────────────────────────────
            if upper.starts_with("STATIC ") || upper == "STATIC" {
                let scope = self.current_scope.clone();
                for (vname, hb_type) in parse_varlist(&trimmed) {
                    self.push_symbol(Symbol {
                        name: vname,
                        kind: SymbolKind::Static,
                        scope: scope.clone(),
                        line: lineno,
                        attributes: vec![],
                        conditional: in_cond,
                        hb_type,
                        params: vec![],
                    });
                }
                continue;
            }

            // ── Membros de classe: VAR / ACCESS / ASSIGN ─────────────────────
            if self.current_class.is_some() {
                if let Some((vname, vis)) = parse_class_var(upper) {
                    let scope = self.current_class.clone().unwrap();
                    self.push_symbol(Symbol {
                        name: vname,
                        kind: SymbolKind::ClassVar { visibility: vis },
                        scope,
                        line: lineno,
                        attributes: vec![],
                        conditional: in_cond,
                        hb_type: None,
                        params: vec![],
                    });
                    continue;
                }
                if let Some(name) = parse_keyword(upper, "ACCESS") {
                    let scope = self.current_class.clone().unwrap();
                    self.push_symbol(Symbol {
                        name,
                        kind: SymbolKind::Access,
                        scope,
                        line: lineno,
                        attributes: vec![],
                        conditional: in_cond,
                        hb_type: None,
                        params: vec![],
                    });
                    continue;
                }
                if let Some(name) = parse_keyword(upper, "ASSIGN") {
                    let scope = self.current_class.clone().unwrap();
                    self.push_symbol(Symbol {
                        name,
                        kind: SymbolKind::Assign,
                        scope,
                        line: lineno,
                        attributes: vec![],
                        conditional: in_cond,
                        hb_type: None,
                        params: vec![],
                    });
                    continue;
                }
            }
        }
    }

    fn push_symbol(&mut self, sym: Symbol) {
        self.defined.insert(sym.name.to_ascii_uppercase());
        self.symbols.push(sym);
    }

    // ── Pass 2: usos externos ────────────────────────────────────────────────

    fn pass_usages(&mut self) {
        for line_idx in 0..self.lines.len() {
            let lineno = line_idx + 1;
            let raw = self.lines[line_idx].clone();
            let trimmed = strip_comment(raw.trim()).to_string();
            for u in collect_calls(&trimmed, lineno) {
                if !self.defined.contains(&u.name) && !is_keyword(&u.name) {
                    self.usages.push(u);
                }
            }
        }

        self.usages.sort_by(|a, b| a.line.cmp(&b.line).then(a.col.cmp(&b.col)));
        self.usages.dedup_by(|a, b| a.name == b.name && a.line == b.line && a.col == b.col);
    }
}

// ── Collector de chamadas ─────────────────────────────────────────────────────

fn collect_calls(line: &str, lineno: usize) -> Vec<Usage> {
    let mut out = Vec::new();
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        if bytes[i] == b'"' || bytes[i] == b'\'' {
            let q = bytes[i];
            i += 1;
            while i < len && bytes[i] != q { i += 1; }
            i += 1;
            continue;
        }
        if is_ident_start(bytes[i]) {
            let start = i;
            while i < len && is_ident_cont(bytes[i]) { i += 1; }
            let ident = &line[start..i];
            if line[i..].trim_start().starts_with('(') {
                out.push(Usage {
                    name: ident.to_ascii_uppercase(),
                    line: lineno,
                    col: start + 1,
                });
            }
            continue;
        }
        i += 1;
    }
    out
}

// ── Helpers de parsing ────────────────────────────────────────────────────────

fn strip_comment(s: &str) -> &str {
    if let Some(idx) = s.find("//") { return &s[..idx]; }
    if let Some(idx) = s.find("/*") { return &s[..idx]; }
    s
}

/// Extrai nome (UPPERCASE), tipo Hungarian do nome e parâmetros de `KEYWORD name(p1, p2)`.
///
/// - `upper`: linha em UPPERCASE — detecção de keyword e armazenamento do nome.
/// - `original`: linha com case original — extração de params e detecção de Hungarian
///   no **nome do callable** (ex.: `nCalcImposto` → `HbType::Float`) antes de normalizar.
///
/// Retorna `(uppercase_name, hb_type, params)`.
fn parse_callable(
    upper: &str,
    original: &str,
    kw: &str,
) -> Option<(String, Option<crate::types::HbType>, Vec<ParamDef>)> {
    let prefix = format!("{} ", kw);
    if !upper.starts_with(&prefix) {
        return None;
    }

    let rest_upper = upper[prefix.len()..].trim();
    let rest_orig  = original.get(prefix.len()..).unwrap_or("").trim();

    let (name, hb_type, params) = if let Some(paren_u) = rest_upper.find('(') {
        let name = rest_upper[..paren_u].trim().to_string();

        // Usa o paren encontrado no original (ASCII-safe: case change não altera byte count).
        let (hb_type, params) = if let Some(paren_o) = rest_orig.find('(') {
            let orig_fn_name = rest_orig[..paren_o].trim();
            let hb_type = crate::types::HbType::from_hungarian(orig_fn_name);
            let inner = rest_orig[paren_o + 1..].trim();
            let close = inner.find(')').map_or(inner, |i| &inner[..i]);
            (hb_type, parse_param_list(close))
        } else {
            (None, vec![])
        };

        (name, hb_type, params)
    } else {
        let name: String = rest_upper
            .chars()
            .take_while(|&c| c != ' ' && c != '\t')
            .collect();
        let orig_fn_name: String = rest_orig
            .chars()
            .take_while(|&c| c != ' ' && c != '\t')
            .collect();
        let hb_type = crate::types::HbType::from_hungarian(&orig_fn_name);
        (name, hb_type, vec![])
    };

    if name.is_empty() { None } else { Some((name, hb_type, params)) }
}

/// Extrai nome, tipo Hungarian e params de `METHOD [ClassName:]MethodName[(params)]`.
fn parse_method(
    upper: &str,
    original: &str,
) -> Option<(String, Option<crate::types::HbType>, Vec<ParamDef>)> {
    if !upper.starts_with("METHOD ") {
        return None;
    }
    let upper_rest    = format!("_ {}", &upper["METHOD ".len()..]);
    let original_rest = format!("_ {}", &original.get("METHOD ".len()..).unwrap_or(""));
    parse_callable(&upper_rest, &original_rest, "_").or_else(|| {
        let rest = upper["METHOD ".len()..].trim();
        let raw: String = rest.chars().take_while(|&c| c != '(' && c != ' ').collect();
        if raw.is_empty() { None } else { Some((raw, None, vec![])) }
    })
}

/// Lista de params separados por vírgula → `Vec<ParamDef>` com case original.
fn parse_param_list(raw: &str) -> Vec<ParamDef> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            // Descarta eventual prefixo de tipo explícito (ex.: `LOCAL nX`) — pega o último token
            let name = s.split_whitespace().last().unwrap_or(s);
            ParamDef::new(name)
        })
        .collect()
}

/// `KEYWORD name` → `Some("NAME")` (nome em UPPERCASE, sem params).
fn parse_keyword(upper: &str, kw: &str) -> Option<String> {
    let prefix = format!("{} ", kw);
    if upper.starts_with(&prefix) {
        let rest = upper[prefix.len()..].trim();
        let name: String = rest
            .chars()
            .take_while(|&c| c != '(' && c != ' ' && c != '\t' && c != ':')
            .collect();
        if name.is_empty() { None } else { Some(name) }
    } else {
        None
    }
}

/// Lista de variáveis Harbour separadas por vírgula após keyword (`PUBLIC x, y`).
///
/// Retorna `(nome_uppercase, hb_type)` — o tipo é detectado **antes** de normalizar
/// para UPPERCASE, preservando a Hungarian notation original (`nEmpresa` → Float).
fn parse_varlist(line: &str) -> Vec<(String, Option<crate::types::HbType>)> {
    let rest = line.splitn(2, ' ').nth(1).unwrap_or("").trim();
    rest.split(',')
        .filter_map(|s| {
            let base = s.split(":=").next().unwrap_or(s).trim();
            let base = base.split('[').next().unwrap_or(base).trim();
            let upper = base.to_ascii_uppercase();
            if upper.is_empty() || !upper.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return None;
            }
            // Hungarian detection no nome original (antes de UPPERCASE)
            let hb_type = crate::types::HbType::from_hungarian(base);
            Some((upper, hb_type))
        })
        .collect()
}

/// `VAR name [EXPORTED|HIDDEN|PROTECTED]` dentro de corpo de classe.
fn parse_class_var(upper: &str) -> Option<(String, Visibility)> {
    if !upper.starts_with("VAR ") {
        return None;
    }
    let rest = upper["VAR ".len()..].trim();
    let name: String = rest.chars().take_while(|&c| c != ' ' && c != '\t').collect();
    if name.is_empty() {
        return None;
    }
    let vis = if rest.contains("HIDDEN") {
        Visibility::Hidden
    } else if rest.contains("PROTECTED") {
        Visibility::Protected
    } else {
        Visibility::Exported
    };
    Some((name, vis))
}

fn is_ident_start(b: u8) -> bool { b.is_ascii_alphabetic() || b == b'_' }
fn is_ident_cont(b: u8)  -> bool { b.is_ascii_alphanumeric() || b == b'_' }

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "IF" | "ELSE" | "ELSEIF" | "ENDIF" | "FOR" | "NEXT" | "WHILE"
            | "ENDWHILE" | "DO" | "RETURN" | "LOCAL" | "STATIC" | "PUBLIC"
            | "PRIVATE" | "MEMVAR" | "FIELD" | "FUNCTION" | "PROCEDURE"
            | "CLASS" | "METHOD" | "DATA" | "VAR" | "ACCESS" | "ASSIGN"
            | "ENDCLASS" | "BEGIN" | "END" | "SWITCH" | "CASE" | "OTHERWISE"
            | "ENDSWITCH" | "EXIT" | "LOOP" | "BREAK" | "TRY" | "CATCH"
            | "FINALLY" | "ENDTRY" | "WITH" | "OBJECT" | "ENDWITH"
            | "NIL" | "SELF" | "SUPER" | "TRUE" | "FALSE" | "AND" | "OR"
            | "NOT" | "IN" | "REPLACE" | "APPEND" | "BLANK" | "USE"
            | "SELECT" | "DBEVAL" | "SEEK" | "LOCATE" | "SKIP" | "GO"
            | "GOTO" | "STORE" | "COPY" | "CLOSE" | "COMMIT" | "ROLLBACK"
    )
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::HbType;

    #[test]
    fn callable_no_params() {
        let (name, hb_type, params) = parse_callable(
            "FUNCTION HELLOWORLD",
            "FUNCTION HelloWorld",
            "FUNCTION",
        ).unwrap();
        assert_eq!(name, "HELLOWORLD");
        assert_eq!(hb_type, None); // "HelloWorld": 'H' é maiúsculo → sem Hungarian
        assert!(params.is_empty());
    }

    #[test]
    fn callable_hungarian_fn_name() {
        // "nCalcImposto": 'n' minúsculo + 'C' maiúsculo → Float (antes de virar NCALCIMPOSTO)
        let (name, hb_type, params) = parse_callable(
            "FUNCTION NCALCIMPOSTO(NBASE, NALIQ)",
            "FUNCTION nCalcImposto(nBase, nAliq)",
            "FUNCTION",
        ).unwrap();
        assert_eq!(name, "NCALCIMPOSTO");
        assert_eq!(hb_type, Some(HbType::Float));
        assert_eq!(params[0].hb_type, Some(HbType::Float));
        assert_eq!(params[1].hb_type, Some(HbType::Float));
    }

    #[test]
    fn callable_params_preserve_case() {
        let (name, _hb_type, params) = parse_callable(
            "FUNCTION CALC(NA, NB)",
            "FUNCTION Calc(nA, nB)",
            "FUNCTION",
        ).unwrap();
        assert_eq!(name, "CALC");
        assert_eq!(params[0].original_name,   "nA");
        assert_eq!(params[0].normalized_name, "NA");
        assert_eq!(params[1].original_name,   "nB");
    }

    #[test]
    fn callable_params_infer_types() {
        let (_, _hb_type, params) = parse_callable(
            "PROCEDURE INIT(CNAME, NAGE, LACTIVE)",
            "PROCEDURE Init(cNome, nIdade, lAtivo)",
            "PROCEDURE",
        ).unwrap();
        assert_eq!(params[0].hb_type, Some(HbType::Character));
        assert_eq!(params[1].hb_type, Some(HbType::Float));
        assert_eq!(params[2].hb_type, Some(HbType::Logical));
    }

    #[test]
    fn callable_allcaps_params_no_type() {
        let (_, _hb_type, params) = parse_callable(
            "FUNCTION FOO(NVAL, CNAME)",
            "FUNCTION Foo(NVAL, CNAME)",
            "FUNCTION",
        ).unwrap();
        assert_eq!(params[0].hb_type, None);
        assert_eq!(params[1].hb_type, None);
    }

    #[test]
    fn varlist_simple() {
        // Nomes sem prefixo Hungarian → tipo None
        let r = parse_varlist("PUBLIC X, Y, Z");
        let names: Vec<&str> = r.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names, vec!["X", "Y", "Z"]);
        assert!(r.iter().all(|(_, t)| t.is_none()));
    }

    #[test]
    fn varlist_with_init() {
        // nCounter: 'n'+'C' → Float; cName: 'c'+'N' → Character (detectado antes do UPPERCASE)
        let r = parse_varlist("PUBLIC nCounter := 0, cName := ''");
        assert_eq!(r[0].0, "NCOUNTER");
        assert_eq!(r[0].1, Some(HbType::Float));
        assert_eq!(r[1].0, "CNAME");
        assert_eq!(r[1].1, Some(HbType::Character));
    }

    #[test]
    fn varlist_hungarian_types() {
        let r = parse_varlist("PUBLIC nEmpresa, cUsuario, lAtivo, dNasc");
        assert_eq!(r[0].1, Some(HbType::Float));     // n → Float
        assert_eq!(r[1].1, Some(HbType::Character)); // c → Character
        assert_eq!(r[2].1, Some(HbType::Logical));   // l → Logical
        assert_eq!(r[3].1, Some(HbType::Date));       // d → Date
    }

    #[test]
    fn class_var_hidden() {
        let (name, vis) = parse_class_var("VAR MYVAR HIDDEN").unwrap();
        assert_eq!(name, "MYVAR");
        assert_eq!(vis, Visibility::Hidden);
    }

    #[test]
    fn strip_comment_cpp() {
        assert_eq!(strip_comment("code // comment"), "code ");
    }

    #[test]
    fn collect_calls_simple() {
        assert_eq!(collect_calls("FetchUser()", 1)[0].name, "FETCHUSER");
    }

    #[test]
    fn collect_calls_skip_strings() {
        let calls = collect_calls(r#"? "FakeCall()""#, 1);
        assert!(calls.iter().all(|c| c.name != "FAKECALL"));
    }
}
