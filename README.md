# Shipwrecked — SWed

> *"In honor of Barry Rebell and Brian Russell — the architects of Clipper."*

**SWed** is a source-to-source transpiler that converts **Harbour / xBase `.prg` files** into idiomatic **Rust** code, bridging decades of legacy business software with modern memory safety and native performance.

---

## Architecture

```
.prg source
    │
    ▼
┌─────────────┐     logos crate
│   Lexer     │  ──────────────►  Vec<Token>
└─────────────┘
    │
    ▼
┌─────────────┐
│   Parser    │  recursive descent  ►  ast::Program
└─────────────┘
    │
    ▼
┌─────────────┐     hbdocs.json
│  Semantic   │  ──────────────►  Diagnostics
│  Analyzer   │
└─────────────┘
    │
    ▼
┌─────────────┐
│  Codegen    │  AST → Rust source string
└─────────────┘
    │
    ▼
 output.rs   +   swed_rt  (runtime crate)
```

## Workspace layout

```
shipwrecked/
├── Cargo.toml       ← workspace root
├── hbdocs.json      ← Harbour built-in function signatures
├── examples/
│
├── swed/            ← transpiler binary (Lexer → Parser → Semantic → Codegen)
├── swed_rt/         ← runtime linked by generated code (HbValue, builtins, DBF, PUBLIC vars)
├── swed_mkh/        ← symbol manifest (.mkh) analyser + test generator (swed_testgen binary)
│
├── swed_co/         ← core types and traits (HbType, SwedError, NativeFunction, …)
├── swed_bf/         ← Harbour built-in function implementations (Left, AllTrim, Date, …)
├── swed_db/         ← database / RDD layer (WorkArea, DbfHandler — migrated from swed_rt)
├── swed_io/         ← file I/O + encoding (CP1252 → UTF-8 via encoding_rs)
├── swed_kn/         ← knife tools: ErrorInterceptor, hex dump, patch suggestions
└── swed_ui/         ← TUI layer: Ratatui widgets, GetElement trait, @..SAY / @..GET / READ
```

### Crate dependency graph

```
swed_co  (no deps)
   ├── swed_rt
   │     ├── swed_bf
   │     ├── swed_db
   │     └── swed_kn
   ├── swed_io
   └── swed_ui
         └── swed_rt

swed_mkh  (standalone — analyses .prg, emits .mkh, generates tests)
swed      (binary — links swed_rt + swed_mkh for the full pipeline)
```

## Harbour → Rust mapping

| Harbour | Rust (generated) |
|---|---|
| `PROCEDURE Main()` | `fn main()` |
| `FUNCTION f()` | `fn f() -> HbValue` |
| `LOCAL x := v` | `let mut x = v;` |
| `STATIC x := v` | `thread_local! { static X: RefCell<HbValue> = ... }` |
| `AAdd(a, v)` | `a.hb_aadd(v)` |
| `LEN(x)` | `hb_len(x)` |
| `FOR i := 1 TO n` | `for i in hb_range(1, n, 1)` |
| `DO WHILE cond` | `while cond { ... }` |
| `IF / ELSEIF / ELSE` | `if / else if / else` |
| `? expr` | `println!("{}", expr)` |
| `NIL` | `HbValue::Nil` |
| `.T.` / `.F.` | `HbValue::Logical(true/false)` |
| `[string]` | `HbValue::String("string".into())` |
| `{ e1, e2 }` | `hb_array![e1, e2]` |
| `IIF(c, t, f)` | `if c { t } else { f }` |

## Usage

```bash
# Run demo (no args)
cargo run

# Transpile a file
cargo run -- examples/demo.prg hbdocs.json
# → writes examples/demo.rs

# Run all tests
cargo test --workspace
```

## Scope resolution

Variable precedence follows Harbour's runtime chain:

```
LOCAL  >  STATIC  >  PRIVATE (MEMVAR)  >  PUBLIC
```

The `semantic::Analyzer` resolves every identifier against this chain during
the analysis pass. Undeclared variables emit a `Warning` (not an error) and
are auto-declared as `PRIVATE`, matching Harbour's runtime behaviour.

## Extending hbdocs.json

Add a new entry to register a custom function for arity validation:

```json
{
  "name": "MYFUNCTION",
  "returns": "C",
  "is_procedure": false,
  "params": [
    { "name": "cInput", "hb_type": "C", "optional": false },
    { "name": "nFlag",  "hb_type": "N", "optional": true  }
  ]
}
```

## Roadmap

### Done
- [x] Lexer / Parser / Semantic / Codegen pipeline (~95%)
- [x] `HbValue` type system with NIL-safe arithmetic
- [x] `HbArray` (1-indexed) + `hb_array!` macro
- [x] PUBLIC variable store (`publics_var` singleton)
- [x] DBF layer (`WorkArea`, `DbfHandler`, `Row`) in `swed_rt`
- [x] Symbol manifest (`.mkh`) analyser + emitter — `swed_mkh`
- [x] Automated test generator — `swed_testgen` binary
- [x] Windows-1252 encoding support in `main.rs`
- [x] 167 workspace tests passing

### In progress
- [ ] `swed_co` — core types and traits (foundation for all new crates)
- [ ] `swed_bf` — Harbour built-ins extracted from `swed_rt` into their own crate
- [ ] `swed_db` — RDD layer migrated from `swed_rt`; `Rdd` trait for swappable drivers
- [ ] `swed_io` — encoding / file utilities extracted from `main.rs`
- [ ] `swed_kn` — `ErrorInterceptor` + hex dump + patch suggestions
- [ ] `swed_ui` — Ratatui widgets, `GetElement` trait, `@..SAY` / `@..GET` / `READ` loop

### Planned
- [ ] VS Code Extension — SWed as LSP pre-compiler (Go-to-Definition via `.mkh`)
- [ ] Full OOP: `CLASS` / `METHOD` / inheritance via traits
- [ ] Harbour macro expansion (`#define`, `#include`, `&varName`)
- [ ] `clap`-based CLI with `--output`, `--verbose`, `--check` flags
- [ ] `miette`-powered diagnostics (rustc-style error messages)
- [ ] Source maps (`.prg` line numbers in generated Rust output)

## License

MIT OR Apache-2.0
