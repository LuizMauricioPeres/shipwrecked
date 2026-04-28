//! Emitter: serializa um `Manifest` para o formato `.mkh` ou stdout.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::types::{Manifest, Symbol, SymbolKind, Usage, Visibility};

/// Grava o arquivo `.mkh` em `<dir_do_prg>/cache_maker/<stem>.mkh`.
///
/// # Errors
/// Retorna `Err(String)` se não conseguir criar o diretório ou escrever o arquivo.
pub fn write_mkh(prg_path: &Path, manifest: &Manifest) -> Result<PathBuf, String> {
    let parent = prg_path
        .parent()
        .ok_or_else(|| format!("no parent dir for {}", prg_path.display()))?;

    let cache_dir = parent.join("cache_maker");
    fs::create_dir_all(&cache_dir)
        .map_err(|e| format!("mkdir {}: {}", cache_dir.display(), e))?;

    let stem = prg_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| String::from("invalid filename"))?;

    let out_path = cache_dir.join(format!("{stem}.mkh"));
    fs::write(&out_path, render_mkh(manifest))
        .map_err(|e| format!("write {}: {}", out_path.display(), e))?;

    Ok(out_path)
}

/// Renderiza o conteúdo `.mkh` como `String`.
fn render_mkh(m: &Manifest) -> String {
    let mut buf = String::with_capacity(4096);

    buf.push_str("; ============================================================\n");
    buf.push_str("; swed_mkh — símbolo manifesto (.mkh)\n");
    buf.push_str("; ============================================================\n");
    buf.push_str(&format!("; SOURCE  : {}\n", m.source_path));
    buf.push_str(&format!("; MD5     : {}\n", m.md5));
    buf.push_str(&format!("; SYMBOLS : {}\n", m.symbols.len()));
    let grouped = group_usages(&m.usages);
    buf.push_str(&format!("; USAGES  : {} ({} distintos)\n", m.usages.len(), grouped.len()));
    buf.push_str("; ------------------------------------------------------------\n");
    buf.push_str("; FORMAT  : [SYMBOL] -> [TIPO] -> Nome | Escopo | Linha | Params | Atributos\n");
    buf.push_str("; ------------------------------------------------------------\n\n");

    buf.push_str("[DEFINITIONS]\n");
    for sym in &m.symbols {
        buf.push_str(&format_symbol(sym));
        buf.push('\n');
    }

    buf.push_str("\n[USAGES]\n");
    for (name, coords) in &grouped {
        buf.push_str(&format_usage_grouped(name, coords));
        buf.push('\n');
    }

    buf
}

/// Renderização legível para `--verbose` / stdout.
pub fn render_stdout(m: &Manifest) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== {} (md5: {})\n", m.source_path, m.md5));
    out.push_str(&format!("  Symbols  : {}\n", m.symbols.len()));
    out.push_str(&format!("  Usages   : {}\n", m.usages.len()));
    for sym in &m.symbols {
        out.push_str(&format!("  {}\n", format_symbol(sym)));
    }
    for (name, coords) in group_usages(&m.usages) {
        out.push_str(&format!("  {}\n", format_usage_grouped(&name, &coords)));
    }
    out
}

// ── Formatadores ──────────────────────────────────────────────────────────────

fn format_symbol(sym: &Symbol) -> String {
    let tipo = kind_str(&sym.kind);

    // Mostra o tipo Rust inferido via Hungarian após o nome: `NCALC:f64`
    let name_str = match &sym.hb_type {
        Some(t) => format!("{}:{}", sym.name, t.to_rust_type()),
        None    => sym.name.clone(),
    };

    let params_str = if sym.params.is_empty() {
        String::from("-")
    } else {
        sym.params
            .iter()
            .map(|p| match &p.hb_type {
                Some(t) => format!("{}:{}", p.original_name, t.to_rust_type()),
                None    => format!("{}:HbValue", p.original_name),
            })
            .collect::<Vec<_>>()
            .join(",")
    };

    let mut attrs: Vec<String> = sym.attributes.clone();
    if sym.conditional {
        attrs.push(String::from("CONDITIONAL"));
    }
    if let SymbolKind::ClassVar { visibility } = &sym.kind {
        attrs.push(vis_str(visibility).to_string());
    }
    let attrs_str = if attrs.is_empty() {
        String::from("-")
    } else {
        attrs.join(",")
    };

    format!(
        "[SYMBOL] -> [{}] -> {} | {} | {} | {} | {}",
        tipo, name_str, sym.scope, sym.line, params_str, attrs_str
    )
}

fn format_usage_grouped(name: &str, coords: &[(usize, usize)]) -> String {
    let locs: Vec<String> = coords
        .iter()
        .map(|(l, c)| format!("[Linha:{l}, Coluna:{c}]"))
        .collect();
    format!("[+] {} | {{ {} }}", name, locs.join(", "))
}

fn group_usages(usages: &[Usage]) -> BTreeMap<String, Vec<(usize, usize)>> {
    let mut map: BTreeMap<String, Vec<(usize, usize)>> = BTreeMap::new();
    for u in usages {
        map.entry(u.name.clone()).or_default().push((u.line, u.col));
    }
    map
}

fn kind_str(k: &SymbolKind) -> &'static str {
    match k {
        SymbolKind::Function => "FUNCTION",
        SymbolKind::Procedure => "PROCEDURE",
        SymbolKind::Method => "METHOD",
        SymbolKind::Public => "PUBLIC",
        SymbolKind::Static => "STATIC",
        SymbolKind::Memvar => "MEMVAR",
        SymbolKind::ClassVar { .. } => "VAR",
        SymbolKind::Access => "ACCESS",
        SymbolKind::Assign => "ASSIGN",
        SymbolKind::Class => "CLASS",
    }
}

fn vis_str(v: &Visibility) -> &'static str {
    match v {
        Visibility::Exported => "EXPORTED",
        Visibility::Hidden => "HIDDEN",
        Visibility::Protected => "PROTECTED",
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ParamDef, Symbol};

    fn sym(name: &str, kind: SymbolKind) -> Symbol {
        Symbol {
            name: name.into(),
            kind,
            scope: "GLOBAL".into(),
            line: 1,
            attributes: vec![],
            conditional: false,
            hb_type: None,
            params: vec![],
        }
    }

    #[test]
    fn kind_str_all() {
        assert_eq!(kind_str(&SymbolKind::Function), "FUNCTION");
        assert_eq!(kind_str(&SymbolKind::Procedure), "PROCEDURE");
        assert_eq!(kind_str(&SymbolKind::Method), "METHOD");
        assert_eq!(kind_str(&SymbolKind::Class), "CLASS");
        assert_eq!(kind_str(&SymbolKind::ClassVar { visibility: Visibility::Exported }), "VAR");
    }

    #[test]
    fn format_symbol_no_params() {
        let s = sym("MYFUNC", SymbolKind::Function);
        let out = format_symbol(&s);
        assert!(out.contains("[FUNCTION]"));
        assert!(out.contains("MYFUNC"));
        assert!(out.contains("| - |")); // params vazio
    }

    #[test]
    fn format_symbol_with_params() {
        let mut s = sym("CALC", SymbolKind::Function);
        // camelCase → Hungarian detectado → tipo nativo Rust no .mkh
        s.params = vec![ParamDef::new("nValor"), ParamDef::new("cNome")];
        let out = format_symbol(&s);
        assert!(out.contains("nValor:f64")); // n → f64 (coringa numérico)
        assert!(out.contains("cNome:String"));
    }

    #[test]
    fn format_symbol_conditional() {
        let mut s = sym("DBG", SymbolKind::Memvar);
        s.conditional = true;
        assert!(format_symbol(&s).contains("CONDITIONAL"));
    }

    #[test]
    fn format_symbol_with_hb_type() {
        use crate::types::HbType;
        let mut s = sym("NEMPRESA", SymbolKind::Public);
        s.hb_type = Some(HbType::Float);
        // deve aparecer como NEMPRESA:f64
        assert!(format_symbol(&s).contains("NEMPRESA:f64"));
    }

    #[test]
    fn group_usages_dedup() {
        let usages = vec![
            Usage { name: "FOO".into(), line: 1, col: 1 },
            Usage { name: "FOO".into(), line: 5, col: 3 },
            Usage { name: "BAR".into(), line: 2, col: 1 },
        ];
        let g = group_usages(&usages);
        assert_eq!(g["FOO"].len(), 2);
        assert_eq!(g["BAR"].len(), 1);
    }

    #[test]
    fn render_mkh_header_contains_source() {
        let m = Manifest {
            source_path: "foo.prg".into(),
            md5: "deadbeef".into(),
            symbols: vec![],
            usages: vec![],
        };
        let out = render_mkh(&m);
        assert!(out.contains("SOURCE  : foo.prg"));
        assert!(out.contains("MD5     : deadbeef"));
        assert!(out.contains("[DEFINITIONS]"));
        assert!(out.contains("[USAGES]"));
    }
}
