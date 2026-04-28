# swed_ui — TUI / Ratatui

User interface layer for SWed. Translates Harbour screen commands (`@..SAY`, `@..GET`, `READ`) into interactive Ratatui widgets. Depends on `swed_co` and `swed_rt`.

## GetElement trait

The core abstraction for interactive GET fields:

```rust
pub trait GetElement {
    fn render(&self, frame: &mut Frame, area: Rect);
    fn handle_key(&mut self, key: KeyEvent) -> bool; // true = key consumed
    fn value(&self) -> HbValue;
}
```

## Field implementations

| Harbour command | swed_ui widget | Behaviour |
|---|---|---|
| `@r,c GET cVar PICTURE "@!"` | `CharInput` | Text field with max length and Picture mask |
| `@r,c GET nVar PICTURE "999.99"` | `NumericInput` | Numeric with decimal precision guard |
| `@r,c GET dVar` | `DateInput` | DD/MM/YYYY mask with calendar validation |
| `@r,c GET lVar` | `LogicalToggle` | `.T.`/`.F.`, toggled by Y / N / Space |
| `@r,c SAY cStr` | `translate_say()` | Static `Paragraph` widget (no interaction) |

## @..SAY translation

```rust
use ratatui::{style::Style, text::Span, widgets::Paragraph};

pub fn translate_say(content: &str, style: Style) -> Paragraph<'_> {
    Paragraph::new(Span::styled(content, style))
}
```

## READ loop

`AppState::run()` drives the crossterm event loop:

- `Tab` / `Enter` — advance cursor to the next field
- `Shift+Tab` — move cursor to the previous field
- `ESC` — abort; returns `None`
- `F10` / `Ctrl+W` — confirm; returns `Some(Vec<HbValue>)`

Each field receives `handle_key` until it returns `false` (key not consumed), then the loop advances the cursor.

## Source layout

```
swed_ui/src/
├── lib.rs
├── app_state.rs       ← READ loop + field cursor
├── say.rs             ← @..SAY → Paragraph
├── traits/
│   └── get_element.rs ← GetElement trait
└── widgets/
    ├── char_input.rs
    ├── numeric_input.rs
    ├── date_input.rs
    └── logical_toggle.rs
```
