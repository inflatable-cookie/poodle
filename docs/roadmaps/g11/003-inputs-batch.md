# g11.003 Inputs Batch

Status: planned
Owner: Flint Core
Depends on: contract audit

## Components

text_input, text_area, search_field, number_entry, pin_input, duration_input,
time_field, color_picker, file_upload, editable_label, combobox

## Structural Issues

- [ ] `editable_label` — GPUI component exists but contract name is
      `editable-label.md`; verify Rust spec `EditableLabelSpec` matches contract
- [ ] `combobox` — verify Rust spec `ComboboxSpec` has all props from contract
- [ ] `color_picker` — verify Rust spec `ColorPickerSpec` has all props from contract

## Per-Component Compliance

For each component:
- [ ] text_input — audit against `docs/contracts/foundation/text-input.md`
- [ ] text_area — audit against `docs/contracts/foundation/text-area.md`
- [ ] search_field — audit against `docs/contracts/foundation/search-field.md`
- [ ] number_entry — audit against `docs/contracts/foundation/number-entry.md`
- [ ] pin_input — audit against `docs/contracts/foundation/pin-input.md`
- [ ] duration_input — audit against `docs/contracts/foundation/duration-input.md`
- [ ] time_field — audit against `docs/contracts/foundation/time-field.md`
- [ ] color_picker — audit against `docs/contracts/foundation/color-picker.md`
- [ ] file_upload — audit against `docs/contracts/foundation/file-upload.md`
- [ ] editable_label — audit against `docs/contracts/foundation/editable-label.md`
- [ ] combobox — audit against `docs/contracts/foundation/combobox.md`

## Checklist (per component)

1. Read contract
2. Read Svelte implementation
3. Diff GPUI implementation against contract
4. Fix: anatomy, props, tokens, states, accessibility, visual output
5. Verify specimen
