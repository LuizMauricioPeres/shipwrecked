// swed/src/main.rs
// Shipwrecked (SWed) — Harbour .prg → Rust transpiler
// "In honor of Barry Rebell and Brian Russell — the architects of Clipper."

mod ast;
mod codegen;
mod hb_array;
mod lexer;
mod parser;
mod scope;
mod semantic;
mod symbol_table;

use std::{env, fs, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { run_demo(); return; }

    let prg_path  = PathBuf::from(&args[1]);
    let docs_path = args.get(2).map(PathBuf::from);

    let source = match fs::read_to_string(&prg_path) {
        Ok(s) => s,
        Err(e) => { eprintln!("[swed] error: {e}"); std::process::exit(1); }
    };
    let sym = match docs_path {
        Some(ref p) => symbol_table::SymbolTable::load_hbdocs(p).unwrap_or_else(|e| {
            eprintln!("[swed] warning: {e}; using built-ins");
            symbol_table::SymbolTable::with_builtins()
        }),
        None => symbol_table::SymbolTable::with_builtins(),
    };
    let rust_src = transpile(&source, &sym);
    let out_path = prg_path.with_extension("rs");
    fs::write(&out_path, &rust_src).expect("could not write output");
    println!("[swed] written → {out_path:?}");
}

fn transpile(source: &str, sym: &symbol_table::SymbolTable) -> String {
    let normalized = lexer::normalize(source);
    let token_stream: Vec<lexer::Token> = lexer::tokenize(&normalized)
        .into_iter().filter_map(|(t, _)| t.ok()).collect();
    let program = match parser::parse(token_stream) {
        Ok(p) => p,
        Err(e) => { eprintln!("[parse error] {e}"); std::process::exit(1); }
    };
    let mut analyzer = semantic::Analyzer::new(sym);
    analyzer.analyze(&program);
    analyzer.print_diagnostics("<input>");
    if analyzer.has_errors() { eprintln!("[swed] aborting."); std::process::exit(1); }
    codegen::generate(&program)
}

fn run_demo() {
    let harbour_src = r#"
PROCEDURE Main()
   LOCAL aTeste := {}
   LOCAL i      := 0

   AAdd( aTeste, [Primeiro] )
   AAdd( aTeste, [Segundo]  )
   AAdd( aTeste, [Terceiro] )

   FOR i := 1 TO LEN( aTeste ) STEP 1
      ? aTeste[i]
   NEXT

   IF LEN( aTeste ) > 2
      ? [Lista completa!]
   ELSE
      ? [Lista curta.]
   ENDIF

RETURN
"#;

    println!("╔═══════════════════════════════════════════════════╗");
    println!("║       SWed — Full Pipeline Demo                   ║");
    println!("╚═══════════════════════════════════════════════════╝\n");
    println!("── Harbour source ──────────────────────────────────");
    println!("{harbour_src}");

    let norm = lexer::normalize(harbour_src);
    let tokens: Vec<lexer::Token> = lexer::tokenize(&norm)
        .into_iter().filter_map(|(t, _)| t.ok()).collect();
    println!("── Tokens: {} ──────────────────────────────────────", tokens.len());

    let program = match parser::parse(tokens) {
        Ok(p) => { println!("── Parse: OK ({} units)", p.units.len()); p }
        Err(e) => { eprintln!("Parse error: {e}"); return; }
    };

    let sym = symbol_table::SymbolTable::with_builtins();
    let mut analyzer = semantic::Analyzer::new(&sym);
    analyzer.analyze(&program);

    let warns = analyzer.diagnostics.iter().filter(|d| d.level == semantic::DiagLevel::Warning).count();
    let errs  = analyzer.diagnostics.iter().filter(|d| d.level == semantic::DiagLevel::Error).count();
    println!("── Semantic: {} error(s), {} warning(s) ────────────", errs, warns);
    for d in &analyzer.diagnostics {
        println!("   [{:?}] {}", d.level, d.message);
    }

    let rust_out = codegen::generate(&program);
    println!("\n── Generated Rust ──────────────────────────────────");
    println!("{rust_out}");
    println!("⚓  Fair winds, Barry & Brian.");
}
