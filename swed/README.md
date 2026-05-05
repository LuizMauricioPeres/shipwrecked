# swed — Transpiler

Source-to-source transpiler binary: Harbour / xBase `.prg` → idiomatic Rust. This is the pipeline crate; it owns the compiler stages and the CLI entry point.

## Pipeline

```
.prg source
    │
    ▼
┌──────────┐  logos crate
│  Lexer   │ ──────────────► Vec<Token>
└──────────┘
    │
    ▼
┌──────────┐
│  Parser  │  recursive descent  ►  ast::Program  (types from swed_co)
└──────────┘
    │
    ▼
┌──────────────────┐
│  swed_nm         │  AST rewrite (in-place &mut Program):
│  ::normalize()   │    1. BuiltinNameResolver — CHR→HB_CHR, MAX→HB_MAX, …
│                  │    2. IndexAssignNorm     — a[i]:=v → hb_set_val(a,i,v)
│                  │    3. IncrDecrNorm        — x++/x-- (stub)
└──────────────────┘
    │
    ▼
┌────────────────┐  hbdocs.json
│ Semantic /     │ ──────────────► Vec<Diagnostic>
│ Symbol Table   │
└────────────────┘
    │
    ▼
┌──────────┐
│ Codegen  │  AST → Rust source string
└──────────┘
    │
    ▼
 output.rs  +  swed_rt  (runtime crate linked by generated code)
```

## Source layout

```
swed/src/
├── main.rs         ← CLI entry point; reads .prg + hbdocs.json, writes .rs
├── ast.rs          ← thin re-export: `pub use swed_co::ast::*`
├── lexer.rs        ← Token definitions (logos derive)
├── parser.rs       ← Recursive-descent parser
├── scope.rs        ← Variable scope resolution (LOCAL > STATIC > PRIVATE > PUBLIC)
├── semantic.rs     ← Semantic analysis + diagnostics
├── symbol_table.rs ← hbdocs.json loader + arity validation
├── codegen.rs      ← AST → Rust source emitter
└── hb_array.rs     ← Array codegen helpers
```

## Usage

```bash
# Transpile a file
cargo run -- examples/demo.prg hbdocs.json
# → writes examples/demo.rs

# Run all workspace tests
cargo test --workspace
```

## Harbour → Rust mapping (key constructs)

| Harbour | Generated Rust |
|---|---|
| `PROCEDURE Main()` | `fn main()` |
| `FUNCTION f(x)` | `fn f(x: HbValue) -> HbValue` |
| `LOCAL x := v` | `let mut x = v;` |
| `STATIC x` | `thread_local! { static X: RefCell<HbValue> = ... }` |
| `PUBLIC x` | `public_store().write().unwrap().set("X", ...)` |
| `FOR i := 1 TO n` | `for i in hb_range(1, n, 1)` |
| `DO WHILE cond` | `while cond { ... }` |
| `AAdd(a, v)` | `a.hb_aadd(v)` |
| `a[i] := v` | `a.hb_set_val(i, v)` ← via `swed_nm::IndexAssignNorm` |
| `a[i][j] := v` | `a.hb_set_nested(i, j, v)` ← via `swed_nm` |
| `Chr(219)` | `hb_chr(...)` ← via `swed_nm::BuiltinNameResolver` |
| `? expr` | `println!("{}", expr)` |
| `NIL` | `HbValue::Nil` |
| `.T.` / `.F.` | `HbValue::Logical(true/false)` |
| `[string]` | `HbValue::String("string".into())` |
| `{ e1, e2 }` | `hb_array![e1, e2]` |

## Scope resolution

Variable precedence follows Harbour's runtime chain:

```
LOCAL  >  STATIC  >  PRIVATE (MEMVAR)  >  PUBLIC
```

Undeclared variables emit a `Warning` and are auto-declared as `PRIVATE`, matching Harbour's runtime behaviour.
