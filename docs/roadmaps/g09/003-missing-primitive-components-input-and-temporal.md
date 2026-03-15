# g09.003 — Missing Primitive Components: Input and Temporal

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.002
Primary repos: `pug`

## Goals

- [ ] implement first-class `Pug*` component structs for all input and temporal
  primitives missing from GPUI
- [ ] ensure interactive components support callbacks and state management

## Execution Checklist

- [ ] create `PugNumberEntry` component backed by `NumberEntrySpec` —
  numeric input with increment/decrement buttons, min/max/step validation
- [ ] create `PugPinInput` component backed by `PinInputSpec` —
  fixed-length digit entry with per-cell focus and masked mode
- [ ] create `PugToolbar` component backed by `ToolbarSpec` —
  horizontal action bar with separator support and alignment
- [ ] create `PugRangeSlider` component backed by `RangeSliderSpec` —
  dual-thumb slider for min/max range selection
- [ ] create `PugDurationInput` component backed by `DurationInputSpec` —
  hours/minutes/seconds entry with formatted display
- [ ] create `PugTimeAgo` component backed by `TimeAgoSpec` —
  relative time display ("2 hours ago", "yesterday")
- [ ] create `PugTimeZoneSelect` component backed by `TimeZoneSelectSpec` —
  searchable timezone picker dropdown
- [ ] create `PugZonedDateTimePicker` component backed by
  `ZonedDateTimePickerSpec` — datetime picker with timezone awareness
- [ ] create `PugFileUpload` component backed by `FileUploadSpec` —
  drop zone with file type filtering, progress, and preview
- [ ] create `PugColorPicker` component backed by `ColorPickerSpec` —
  color selection with swatch grid and custom input
- [ ] register all new modules in `lib.rs` with `mod` and `pub use`
- [ ] verify all new components compile with `cargo check`

## Acceptance Criteria

- [ ] all listed components have `Pug*` structs implementing `IntoElement`
- [ ] interactive components (NumberEntry, PinInput, RangeSlider, ColorPicker)
  support `on_change` callbacks
- [ ] temporal components (DurationInput, TimeAgo, TimeZoneSelect) correctly
  format display values
- [ ] `cargo check` passes with zero errors for `pug_gpui_components`

## Next Task

Open `g09.004` and implement missing form and data composites.
