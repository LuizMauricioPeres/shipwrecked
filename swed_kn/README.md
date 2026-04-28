# swed_kn — Knife Tools

Diagnostic and debugging utilities for SWed. Implements the `ErrorInterceptor` trait from `swed_co` to observe runtime errors without modifying `swed_rt` or generated code. Depends on `swed_co` and `swed_rt`.

## Behavior on Critical error

When a `SeverityLevel::Critical` error fires:

1. Intercept the `(&SwedError, &HbValue)` pair via `ErrorInterceptor::on_critical`
2. Emit a **hex dump** of the `HbValue` bytes to stderr (16 bytes per row, ASCII sidebar)
3. Inspect the `HbValue` discriminant and suggest a type-coercion patch
4. Append a structured JSON entry to `swed_kn.log` for post-mortem analysis

## Patch heuristics

| Detected condition | Suggested patch |
|---|---|
| `Nil` where `Integer` expected | `HbValue::Integer(0)` |
| `Nil` where `String` expected | `HbValue::String("".into())` |
| `Nil` where `Logical` expected | `HbValue::Logical(false)` |
| `Float` arithmetic overflow | Clamp to `f64::MAX` |
| String-to-Numeric coercion fail | `HbValue::Integer(0)` (Harbour default) |
| Division by zero | `HbValue::Nil` (Harbour runtime behaviour) |

## ErrorInterceptor trait (from swed_co)

```rust
pub trait ErrorInterceptor {
    fn on_critical(&self, err: &SwedError, val: &HbValue);
}
```

## Log format

```json
{
  "timestamp": "2026-04-28T14:32:00Z",
  "severity": "Critical",
  "message": "arithmetic overflow in hb_add",
  "hb_type": "Float",
  "hex_dump": "3f f0 00 00 00 00 00 00",
  "patch": "Clamp to f64::MAX"
}
```

## Source layout

```
swed_kn/src/
├── lib.rs
├── interceptor.rs  ← ErrorInterceptor impl
├── hex_dump.rs     ← HbValue → formatted hex dump
├── patch.rs        ← patch suggestion engine
├── logger.rs       ← append-only JSON logger to swed_kn.log
└── traits/
    └── inspector.rs ← Inspector trait for custom diagnostic backends
```
