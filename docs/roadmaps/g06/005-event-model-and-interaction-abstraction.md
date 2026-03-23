# g06.005 — Event Model and Interaction Abstraction

Status: Completed
Updated: 2026-03-14

## Objective

Define renderer-agnostic semantic events that map to both GPUI's event
subscription model and Jetstream's `UiEvent` enum.

## Deliverables

New crate: `poodle-events` at `packages/contracts/events/`

### Semantic Event Variants

| Event | Emitted By | GPUI Source | Jetstream Source |
|-------|-----------|-------------|-----------------|
| `Activated` | Button, IconButton, MenuItem | `on_click` | `UiEvent::Clicked` |
| `ValueChanged` | TextInput, Slider, Select, Checkbox | `on_change` | `UiEvent::ValueChanged/TextChanged` |
| `FocusChanged` | Any focusable | `on_focus/on_blur` | `UiEvent::FocusGained/FocusLost` |
| `OpenChanged` | Dialog, Drawer, Popover, Accordion | State handlers | `ScreenStack` / widget state |
| `SelectionChanged` | Select, RadioGroup, Tabs | Selection handlers | `UiEvent::ValueChanged` |
| `Submitted` | FormShell, TextInput | Form submit | Button activation |
| `Cancelled` | Dialog, FormShell | Escape/cancel | Back navigation |
| `Hovered` | Any hoverable | Mouse enter/leave | Not supported (gamepad) |
| `DragChanged` | Slider, ReorderableList, SplitView | Drag handlers | `UiEvent::Drag*` |
| `KeyPressed` | Any with shortcuts | `on_key_down` | `UiEvent::KeyPressed` |
| `Navigate` | Focus system, lists | Arrow keys | Gamepad d-pad |
| `ScrollChanged` | ScrollShell, DataTable | Scroll handlers | `UiEvent::ScrollChanged` |

### Supporting Types

- `EventValue` — Text/Number/Bool/Multiple payload union
- `DragPhase` — Started/Moved/Ended
- `EventModifiers` — shift/ctrl/alt/meta flags
- `NavigateDirection` — Up/Down/Left/Right
- `ComponentEventProfile` — 9 profiles mapping component categories to their events

## Verification

- [x] `poodle-events` crate compiles with zero dependencies
- [x] 5 tests pass covering event payloads, value variants, modifiers, drag phases, profiles
