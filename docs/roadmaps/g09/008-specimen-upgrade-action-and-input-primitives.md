# g09.008 — Specimen Upgrade: Action and Input Primitives

Status: complete
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.003
Primary repos: `pug`

## Goals

- [ ] replace all mockup action and input specimens with real Pug component
  usage
- [ ] add specimens for components with `simple_specimen()` placeholders

## Execution Checklist

- [ ] rewrite `split_button.rs` to compose `PugButton` + `PugSeparator` +
  `PugIconButton` with proper variant resolution (currently partially done —
  verify all tokens resolve correctly)
- [ ] rewrite `number_entry.rs` to use `PugNumberEntry` with min/max/step
  demonstration and disabled state
- [ ] rewrite `pin_input.rs` to use `PugPinInput` showing 4-digit and 6-digit
  variants with masked mode
- [ ] rewrite `toolbar.rs` to use `PugToolbar` with action items, separator,
  and alignment variants
- [ ] replace `editable-label` simple_specimen placeholder with real
  `PugEditableLabel` showing display and edit modes
- [ ] rewrite `time_field.rs` to use `PugTimeField` with populated value,
  placeholder, disabled, and validation states
- [ ] rewrite `file_upload.rs` to use `PugFileUpload` showing drop zone,
  file type filters, and progress state
- [ ] rewrite `color_picker.rs` to use `PugColorPicker` showing swatch grid
  and selected value display
- [ ] update `mod.rs` slug routing for any renamed or new files
- [ ] verify all specimen slugs render without panic

## Acceptance Criteria

- [ ] zero hand-built mockup code remains in action/input specimen files
- [ ] every interactive specimen responds to click/input events
- [ ] `editable-label` slug renders a real specimen (not simple_specimen)
- [ ] `cargo check` passes for the preview crate

## Next Task

Open `g09.009` and upgrade selection and feedback specimens.
