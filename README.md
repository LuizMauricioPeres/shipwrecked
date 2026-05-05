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
┌─────────────┐     hbdocs.json (343 entries)
│  Semantic   │  ──────────────►  Diagnostics
│  Analyzer   │
└─────────────┘
    │
    ▼
┌─────────────┐     (planned — swed_nm crate)
│  swed_nm    │  AST rewrite pass:
│  Normalizer │    • BuiltinNameResolver  — chr/array/setcolor → hb_chr/hb_array/hb_setcolor
│             │    • IndexAssignNorm      — a[i] := v → hb_set_val(i, v)        fixes E0070
│             │    • ChainedIndexAssign   — a[i][j] := v → hb_set_nested(i,j,v) fixes E0070
│             │    • IndexTypeAnnotation  — inject :HbValue to fix E0282
│             │    • IncrDecrNorm         — x++ / x-- in rvalue context
│             │    • BuiltinImportInject  — generate use swed_bf::{...} in _module.rs
└─────────────┘
    │
    ▼
┌─────────────┐
│  Codegen    │  AST → Rust source string
└─────────────┘
    │
    ▼
 <name>.rs  +  <name>_module.rs   (dual-file output — in progress)
```

## Workspace layout

```
shipwrecked/
├── Cargo.toml       ← workspace root
├── hbdocs.json      ← Harbour built-in function signatures (343 entries)
├── examples/
│
├── swed/            ← transpiler binary (Lexer → Parser → Semantic → Codegen)
├── swed_rt/         ← runtime linked by generated code (HbValue, builtins, PUBLIC vars)
├── swed_mkh/        ← symbol manifest (.mkh) analyser + test generator (swed_testgen binary)
│
├── swed_co/         ← core types and traits (HbType, SwedError, NativeFunction, …)
├── swed_bf/         ← Harbour built-in function implementations (Str, Date, Pad*, Chr, …)
├── swed_nm/         ← [PLANNED] Semantic Normalizer — AST rewrite pass before codegen
├── swed_db/         ← database / RDD layer (WorkArea, DbfHandler, Row, field_get/set)
├── swed_io/         ← file I/O + encoding (CP1252 → UTF-8 via encoding_rs)
├── swed_kn/         ← knife tools: ErrorInterceptor, hex dump, patch suggestions (stub)
└── swed_ui/         ← TUI layer: Ratatui widgets, GetElement trait, @..SAY / @..GET / READ
```

### Crate dependency graph

```
swed_co  (no deps)
   ├── swed_rt
   │     ├── swed_bf
   │     ├── swed_db
   │     └── swed_kn  (dev / optional)
   ├── swed_io
   └── swed_ui
         └── swed_rt

swed_mkh  (standalone — analyses .prg, emits .mkh, generates tests)
swed      (binary — links swed_rt + swed_mkh + swed_db + swed_ui for the full pipeline)
```

## Harbour → Rust mapping

| Harbour | Rust (generated) |
|---|---|
| `PROCEDURE Main()` | `fn main()` |
| `FUNCTION f()` | `fn f() -> HbValue` |
| `LOCAL x := v` | `let mut x = v;` |
| `STATIC x := v` | `thread_local! { static X: RefCell<HbValue> = ... }` |
| `PUBLIC nEmp` | `public_store().write().unwrap().set("N_EMP", HbValue::Nil)` |
| `AAdd(a, v)` | `a.hb_aadd(v)` |
| `LEN(x)` | `x.hb_len_as_i64()` |
| `a[i]` (read) | `a.hb_get_val(HbValue::Integer(i))` |
| `a[i] := v` (write) | `a.hb_set_val(HbValue::Integer(i), v)` |
| `a[i][j] := v` | `a.hb_set_nested(i, j, v)` |
| `Chr(n)` | `hb_chr(n)` ← via swed_nm BuiltinNameResolver |
| `Array(n)` | `hb_array(n)` |
| `FOR i := 1 TO n` | `for i in hb_range(1, n, 1)` |
| `DO WHILE cond` | `while cond { ... }` |
| `IF / ELSEIF / ELSE` | `if / else if / else` |
| `? expr` | `println!("{}", expr)` |
| `NIL` | `HbValue::Nil` |
| `.T.` / `.F.` | `HbValue::Logical(true/false)` |
| `[string]` | `HbValue::String("string".into())` |
| `{ e1, e2 }` | `hb_array![e1, e2]` |
| `IIF(c, t, f)` | `if c { t } else { f }` |
| `FIELD->NAME` | `field_get("NAME")` / `field_set("NAME", val)` |
| `@ r, c SAY lbl GET var PICTURE "p"` `READ` | scoped `AppState::new(widgets).run()` block |

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
- [x] `Index<&HbValue>` on `HbValue` — zero-clone array subscript in generated code
- [x] PUBLIC variable store (`publics_var` singleton)
- [x] `swed_co` — core types and traits (`HbType`, `SwedError`, `NativeFunction`, …)
- [x] `swed_bf` — Harbour built-ins (`hb_str`, `hb_strzero`, `hb_ntos`, `hb_chr`, `hb_date`, `hb_type`, `hb_pad*`, …)
- [x] `swed_db` — RDD layer (`WorkArea`, `DbfHandler`, `Row`, `field_get/set/alias`)
- [x] `swed_io` — Windows-1252 encoding support
- [x] `swed_ui` — Ratatui TUI: `AppState`, `GetElement` trait, `CharInput`, `NumericInput`, `DateInput`, `LogicalToggle`
- [x] Codegen for `@..SAY` / `@..GET` / `READ` — grouped into scoped `AppState` blocks
- [x] Symbol manifest (`.mkh`) analyser + emitter — `swed_mkh`
- [x] Automated test generator — `swed_testgen` binary
- [x] 245 workspace tests passing

### In progress

- [ ] `swed_kn` — `ErrorInterceptor` implementation, hex dump, patch suggestions (currently stub)
- [ ] `hb_eq` / `hb_exact_eq` — fuzzy string comparison matching Harbour's `SET EXACT OFF` semantics
- [ ] `AddAssign` / `SubAssign` / `MulAssign` / `DivAssign` on `HbValue` via `mem::replace`
- [ ] Dual-file codegen — `<name>.rs` (logic) + `<name>_module.rs` (prelude with `pub use` for `swed_bf` functions detected in AST)

### Planned

#### swed_nm — Semantic Normalizer (create crate)

AST rewrite pass between Semantic and Codegen. Priorities derived from resta1.rs errors:

| Rule | Input | Output | Fixes |
|------|-------|--------|-------|
| `BuiltinNameResolver` | `chr(n)` `array(n)` `setcolor()` | `hb_chr(n)` `hb_array(n)` `hb_setcolor()` | E0425 |
| `IndexAssignNorm` | `x.hb_get_val(i) = v` | `x.hb_set_val(i, v)` | E0070 |
| `ChainedIndexAssignNorm` | `x.hb_get_val(i).hb_get_val(j) = v` | `x.hb_set_nested(i, j, v)` | E0070 (12+ sites) |
| `IndexTypeAnnotation` | ambiguous `hb_get_val(x)` | `hb_get_val(x as HbValue)` | E0282 |
| `IncrDecrNorm` | `x++` / `x--` in rvalue | `{ let old = x.clone(); x += 1; old }` | operator crimes |
| `BuiltinImportInjector` | used built-ins in AST | `use swed_bf::{hb_chr, …}` in `_module.rs` | E0425 |
| `AliasResolver` | `Trim` `Type` | `hb_rtrim` `hb_valtype` | name mismatch |
| `TuiCallDetector` | calls to screen functions | marks PRG as `requires: swed_ui` | dependency inference |

#### swed_bf — Pending functions (from hbdocs.json survey)

- **P2 String:** `hb_left`, `hb_right`, `hb_strtran`, `hb_hardcr`, `hb_valtostr`, `hb_transform`
- **P3 Array:** `hb_atail`, `hb_adel`, `hb_ains`, `hb_afill`, `hb_aclone`, `hb_acopy`, `hb_aeval`, `hb_asort`
- **P4 Numeric:** `hb_mod`, `hb_sqrt`, `hb_exp`, `hb_log`, `hb_word`
- **P5 Date/Time:** `hb_dtoc`, `hb_ctod`, `hb_cmonth`, `hb_cdow`, `hb_dow`, `hb_time`, `hb_seconds`, `hb_secs`, `hb_elaptime`
- **P6 Type:** `hb_isalpha`, `hb_isdigit`, `hb_islower`, `hb_isupper`, `hb_isaffirm`, `hb_isnegative`, `hb_eval`, `hb_pcount`
- **P7 System:** `hb_os`, `hb_version`, `hb_curdir`

#### Other

- [ ] `impl Into<HbValue>` on `swed_bf` function signatures — allows generated code to pass literals without `.into()`
- [ ] VS Code Extension — SWed as LSP pre-compiler (Go-to-Definition via `.mkh`)
- [ ] Full OOP: `CLASS` / `METHOD` / inheritance via traits
- [ ] Harbour macro expansion (`#define`, `#include`, `&varName`)
- [ ] `clap`-based CLI with `--output`, `--verbose`, `--check` flags
- [ ] `miette`-powered diagnostics (rustc-style error messages)
- [ ] Source maps (`.prg` line numbers in generated Rust output)

## License

MIT OR Apache-2.0
