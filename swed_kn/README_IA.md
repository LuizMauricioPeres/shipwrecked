CRATE: swed_kn v0.2.0
TYPE: library (lib)
ROLE: Knife Tools — runtime diagnostics, hex dump, patch suggestions, JSON error log
STATUS: partial/stub — hex dump present; ErrorInterceptor impl + patch engine pending

DEPS:
  swed_co — ErrorInterceptor trait, SwedError, SeverityLevel
  swed_rt — HbValue (inspected in hex dump and patch heuristics)

DESIGN: Observer pattern via ErrorInterceptor trait (swed_co).
        Attaches at runtime without modifying swed_rt or generated code.
        Append-only JSON log for post-mortem; hex dump to stderr for live debugging.
        Optional: linked only when user opts in (dev/debug builds).

SOURCE_FILES:
  lib.rs              — public API; re-exports KnifeInterceptor
  traits/             — Inspector trait (custom diagnostic backends)
  [PENDING]:
    interceptor.rs    — KnifeInterceptor impl ErrorInterceptor
    hex_dump.rs       — HbValue → formatted hex rows
    patch.rs          — patch suggestion engine
    logger.rs         — append-only JSON writer to swed_kn.log

TRAIT_IMPL (interceptor.rs — PENDING):
  KnifeInterceptor:
    fn on_critical(&self, err:&SwedError, val:&HbValue)
      1. call hex_dump::dump(val) → stderr
      2. call patch::suggest(val, err) → &str patch description
      3. call logger::append(err, val, patch) → swed_kn.log

HEX_DUMP_FORMAT (hex_dump.rs — PENDING):
  16 bytes per row; offset | hex pairs | ASCII sidebar
  example:
    0000: 3f f0 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ?...............
  HbValue discriminant shown as header:
    [HbValue::Float] 8 bytes

PATCH_HEURISTICS (patch.rs — PENDING):
  Nil where Integer expected   → HbValue::Integer(0)
  Nil where String expected    → HbValue::String("".into())
  Nil where Logical expected   → HbValue::Logical(false)
  Float overflow               → clamp to f64::MAX
  String→Numeric coercion fail → HbValue::Integer(0)
  Division by zero             → HbValue::Nil  (Harbour semantics)

LOG_FORMAT (logger.rs — PENDING):
  swed_kn.log — newline-delimited JSON
  {
    "timestamp": "2026-05-04T12:00:00Z",
    "severity": "Critical",
    "message": "...",
    "hb_type": "Float",
    "hex_dump": "3f f0 ...",
    "patch": "Clamp to f64::MAX"
  }

TRAIT (traits/inspector.rs — stub):
  Inspector:
    fn inspect(&self, err:&SwedError, val:&HbValue) -> InspectorReport
    fn name(&self) -> &'static str
  Allows custom diagnostic backends (e.g. Sentry, OpenTelemetry) to plug in

INVARIANTS:
  - on_critical must NEVER panic (it is called inside arithmetic ops)
  - on_critical must NEVER mutate HbValue or SwedError (shared refs)
  - log file open is lazy; fallback to stderr if file unwritable
  - hex dump output is always valid UTF-8

PENDING (this crate):
  interceptor.rs — KnifeInterceptor impl                     (priority: M)
  hex_dump.rs    — HbValue → hex rows                        (priority: M)
  patch.rs       — heuristic patch suggestions               (priority: M)
  logger.rs      — JSON log writer                           (priority: L)
  Arc<AtomicBool> bulkhead (swed_kn safety roadmap)          (priority: L)
    prevents runaway interceptor from blocking hot path

INTEGRATES_WITH:
  swed_co: implements ErrorInterceptor trait
  swed_rt: HbValue received in on_critical for inspection
  swed (binary): KnifeInterceptor registered at startup in debug builds
