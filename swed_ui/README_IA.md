CRATE: swed_ui v0.2.0
TYPE: library (lib)
ROLE: TUI layer — translates Harbour @..SAY/@..GET/READ into Ratatui interactive widgets
STATUS: functional; AppState + 4 widget types + GetList implemented; codegen wired

DEPS:
  swed_co  — ModuleComponent lifecycle trait
  swed_rt  — HbValue (field values in/out)
  ratatui 0.30 — TUI rendering framework
  crossterm 0.29 — terminal event handling

SOURCE_FILES:
  lib.rs
  app_state.rs     — READ loop driver; field cursor management
  say.rs           — @..SAY → Paragraph widget (static labels)
  get_element.rs   — GetElement trait (polymorphism for all GET fields)
  get_list.rs      — GetList: ordered list of Box<dyn GetElement>
  read.rs          — READ command: drives crossterm event loop
  traits.rs        — re-export of GetElement; ModuleComponent impl for AppState
  widgets/
    char_input.rs
    numeric_input.rs
    date_input.rs
    logical_toggle.rs
  traits/          — trait defs dir (mirrors traits.rs; check actual layout)

TRAIT:
  GetElement (get_element.rs):
    fn render(&self, frame:&mut Frame, area:Rect)
    fn handle_key(&mut self, key:KeyEvent) -> bool   — true=consumed
    fn value(&self) -> HbValue
    fn is_dirty(&self) -> bool                       — field was modified
  Implemented by: CharInput, NumericInput, DateInput, LogicalToggle

TYPES:

  AppState — central READ coordinator
    widgets: Vec<Box<dyn GetElement>>
    cursor: usize                  — active field index
    confirmed: bool
    fn new(widgets:Vec<Box<dyn GetElement>>) -> AppState
    fn run(&mut self) -> Option<Vec<HbValue>>
      — returns Some(values) on F10/Ctrl+W; None on ESC
    impl ModuleComponent for AppState

  GetList — builder for AppState
    fn push(&mut self, el:Box<dyn GetElement>)
    fn into_app_state(self) -> AppState

  CharInput — @..GET cVar PICTURE "@!"
    value: String
    max_len: usize
    picture: Option<String>     — mask (future: validate against mask)

  NumericInput — @..GET nVar PICTURE "999.99"
    value: f64
    decimals: u8               — decimal precision guard

  DateInput — @..GET dVar
    value: HbValue::Date       — internal days-since-epoch
    mask: "DD/MM/YYYY"         — display only; validated on commit

  LogicalToggle — @..GET lVar
    value: bool
    keys: Y/N/Space toggle

SAY_TRANSLATION (say.rs):
  fn translate_say(content:&str, style:ratatui::style::Style) -> Paragraph<'_>
    → Paragraph::new(Span::styled(content, style))

KEYMAP (app_state.rs READ loop):
  Tab / Enter     → advance cursor to next field
  Shift+Tab       → move cursor to previous field
  ESC             → abort; return None
  F10 / Ctrl+W    → confirm; return Some(Vec<HbValue>)
  all others      → delegated to active widget's handle_key()
  field skipped if handle_key returns false (key not consumed)

CODEGEN_PATTERN (emitted by swed/codegen.rs):
  // @..SAY / @..GET / READ block
  {
      let mut __app = AppState::new(vec![
          Box::new(CharInput::new("", 20, None)),          // GET cVar
          Box::new(NumericInput::new(0.0, 2)),             // GET nVar
      ]);
      if let Some(vals) = __app.run() {
          c_var = vals[0].clone();
          n_var = vals[1].clone();
      }
  }

INVARIANTS:
  - AppState::run() is blocking; consumes crossterm raw mode
  - raw mode always restored on drop (even if handle_key panics)
  - GetElement::value() returns current field value at any time (before confirmation)
  - cursor wraps: Tab on last field → first field; Shift+Tab on first → last

PENDING (this crate):
  @..BOX widget (border drawing)                             (priority: M)
  ACHOICE — menu selection widget                           (priority: M)
  Browse/TBROWSE widget (tabular data navigation)           (priority: L)
  Picture mask validation in CharInput                      (priority: M)
  DateInput: calendar validation (day-in-month, leap year)  (priority: M)
  Mouse support (ratatui 0.30 MouseEvent)                   (priority: L)
  doc comments on swed_ui public items (3 warnings)         (priority: L)

INTEGRATES_WITH:
  swed_co: AppState implements ModuleComponent (on_init/on_shutdown)
  swed_rt: all field values are HbValue; GetElement::value() → HbValue
  swed (binary): codegen emits AppState::new(...).run() scoped blocks
