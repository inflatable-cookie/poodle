# g10.004 — Action and Input Primitive Specimens

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.003
Primary repos: `pug`

## Goals

- [ ] create per-component specimens for all action and input primitives

## Execution Checklist

- [ ] create `button.rs` — Button showing Primary, Secondary, Ghost, Danger
  variants in sm/md/lg sizes with disabled state
- [ ] create `icon_button.rs` — IconButton with variant and size combinations
- [ ] create `split_button.rs` — SplitButton composing Button + Separator +
  IconButton dropdown trigger
- [ ] create `field.rs` — Field with label, input child, description, and
  error message
- [ ] create `text_input.rs` — TextInput with placeholder, filled, disabled,
  and validation error states
- [ ] create `text_area.rs` — TextArea with multi-line content and resize
- [ ] create `search_field.rs` — SearchField with search icon and clear button
- [ ] create `form_actions.rs` — FormActions with primary/secondary button
  alignment
- [ ] create `time_field.rs` — TimeField with value, placeholder, and
  disabled states
- [ ] create `editable_label.rs` — EditableLabel showing display and edit modes
- [ ] create `number_entry.rs` — NumberEntry with increment/decrement and
  min/max bounds
- [ ] create `pin_input.rs` — PinInput with 4-digit and 6-digit variants,
  masked mode
- [ ] create `toolbar.rs` — Toolbar with action items, separator, and
  alignment
- [ ] register all modules and wire slug routing
- [ ] verify all 13 specimens render without panic

## Acceptance Criteria

- [ ] all 13 action/input specimens render in the preview app
- [ ] button specimens show correct variant colors from theme
- [ ] input specimens show correct border and focus states
- [ ] `cargo check` passes

## Next Task

Open `g10.005` and build selection and feedback specimens.
