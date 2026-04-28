//! `swed_testgen` — gerador de testes de regressão para fontes Harbour transpilados.
//!
//! # Uso
//!
//! ```text
//! swed_testgen <arquivo.prg> [--out-dir <diretório>]
//! ```
//!
//! Lê `<arquivo.prg>`, analisa os símbolos via `swed_mkh::analyse_file` e gera
//! `<out-dir>/<stem>_regressao.rs` com testes `#[cfg(test)]` prontos para `cargo test`.
//!
//! O diretório de saída padrão é `tests/` relativo ao `.prg`.

#![allow(missing_docs)]

use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        eprintln!("Uso: swed_testgen <arquivo.prg> [--out-dir <diretório>]");
        eprintln!();
        eprintln!("Gera <out-dir>/<stem>_regressao.rs com testes #[cfg(test)].");
        eprintln!("Padrão de out-dir: tests/ relativo ao .prg");
        process::exit(1);
    }

    let prg_path = PathBuf::from(&args[1]);

    if !prg_path.exists() {
        eprintln!("[swed_testgen] arquivo não encontrado: {}", prg_path.display());
        process::exit(1);
    }

    let out_dir = parse_out_dir(&args).unwrap_or_else(|| {
        prg_path
            .parent()
            .unwrap_or(Path::new("."))
            .join("tests")
    });

    let manifest = match swed_mkh::analyse_file(&prg_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("[swed_testgen] erro ao analisar {}: {e}", prg_path.display());
            process::exit(1);
        }
    };

    eprintln!(
        "[swed_testgen] {} — {} símbolos, {} usos",
        prg_path.display(),
        manifest.symbols.len(),
        manifest.usages.len()
    );

    match swed_mkh::generate_tests(&prg_path, &manifest, &out_dir) {
        Ok(out) => println!("[swed_testgen] gerado → {}", out.display()),
        Err(e) => {
            eprintln!("[swed_testgen] erro ao gerar testes: {e}");
            process::exit(1);
        }
    }
}

/// Extrai `--out-dir <valor>` dos args, se presente.
fn parse_out_dir(args: &[String]) -> Option<PathBuf> {
    args.windows(2)
        .find(|w| w[0] == "--out-dir")
        .map(|w| PathBuf::from(&w[1]))
}
