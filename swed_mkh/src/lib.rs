//! `swed_mkh` — Analisador de símbolos Harbour, gerador de manifestos `.mkh`
//! e gerador de testes de regressão Rust.
//!
//! # Uso rápido
//! ```no_run
//! use swed_mkh::{analyse_file, write_mkh, generate_tests};
//! use std::path::Path;
//!
//! let prg = Path::new("programa.prg");
//! let manifest = analyse_file(prg).unwrap();
//!
//! // Grava o manifesto .mkh
//! let mkh = write_mkh(prg, &manifest).unwrap();
//! println!("Manifesto gerado em {}", mkh.display());
//!
//! // Gera tests/programa_regressao.rs
//! let test_file = generate_tests(prg, &manifest, Path::new("tests")).unwrap();
//! println!("Testes gerados em {}", test_file.display());
//! ```

pub mod analyser;
pub mod emitter;
pub mod testgen;
pub mod types;

pub use analyser::analyse_file;
pub use emitter::{render_stdout, write_mkh};
pub use testgen::generate_tests;
pub use types::{HbType, Manifest, ParamDef, Symbol, SymbolKind, Usage, Visibility};
