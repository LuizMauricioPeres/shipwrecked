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
swed/
├── Cargo.toml          ← workspace root + transpiler binary
├── hbdocs.json         ← Harbour built-in function signatures
├── examples/
│   └── demo.prg        ← sample Harbour source
├── src/
│   ├── main.rs         ← CLI entry point + demo pipeline
│   ├── ast.rs          ← AST node definitions
│   ├── lexer.rs        ← Token definitions (logos)
│   ├── parser.rs       ← Recursive-descent parser
│   ├── scope.rs        ← Variable scope resolution
│   ├── semantic.rs     ← Semantic analysis + diagnostics
│   ├── symbol_table.rs ← hbdocs.json loader + arity validation
│   ├── codegen.rs      ← AST → Rust source emitter
│   └── hb_array.rs     ← Legacy codegen helper (superseded by swed_rt)
└── swed_rt/
    ├── Cargo.toml
    └── src/
        ├── lib.rs       ← Public API + hb_array! macro
        ├── value.rs     ← HbValue enum + arithmetic operators
        ├── array.rs     ← HbArray (1-indexed, dynamic)
        └── builtins.rs  ← All Harbour built-in functions
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

- [ ] `swed_rt` DBF layer (`dbase` crate)
- [ ] Full OOP: `CLASS` / `METHOD` / inheritance via traits
- [ ] Harbour macro expansion (`#define`, `#include`, `&varName`)
- [ ] `clap`-based CLI with `--output`, `--verbose`, `--check` flags
- [ ] `miette`-powered diagnostics (rustc-style error messages)
- [ ] Source maps (`.prg` line numbers in Rust output comments)
- [ ] Parallel transpilation via `rayon`

## License

MIT OR Apache-2.0
