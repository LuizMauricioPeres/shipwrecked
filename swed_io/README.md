# swed_io — I/O and Encoding

File system utilities for SWed. Handles source reading with encoding detection, config parsing, and output writing. Depends on `swed_co` (for `SwedError`).

## Functions

| Function | Description |
|---|---|
| `read_prg(path) -> Result<String, SwedError>` | Reads `.prg` source; auto-detects CP1252 vs UTF-8 |
| `read_config(path) -> Result<Config, SwedError>` | Parses `swed.json` or INI-style SET commands |
| `write_output(path, content) -> Result<(), SwedError>` | Writes transpiled `.rs` to disk (creates parent dirs) |

## Encoding strategy

Legacy Harbour source files often use **Windows-1252** (CP1252). Detection order:

1. Read raw bytes (`fs::read`)
2. Try UTF-8 (`str::from_utf8`) — if valid, return as-is
3. On failure → decode via `encoding_rs::WINDOWS_1252`
4. Emit a `SeverityLevel::Notice` diagnostic if fallback was triggered

```rust
use swed_io::read_prg;

let source = read_prg("legacy.prg")?;
// Always yields valid UTF-8, regardless of original encoding
```

## Config (SET commands)

SWed reads `swed.json` to mirror common Harbour `SET` defaults:

```json
{
  "date_format": "DD/MM/YYYY",
  "century": true,
  "decimal": 2,
  "output_dir": "./output"
}
```

## Source layout

```
swed_io/src/
├── lib.rs
├── reader.rs       ← read_prg with CP1252 fallback (encoding_rs)
├── writer.rs       ← write_output
├── config.rs       ← Config struct + JSON/INI parser
└── traits/
    └── encoder.rs  ← Encoder trait for custom encoding adapters
```
