# 052 GPUI Selection, Feedback, And Date-Time Primitives Baseline

Status: active
Updated: 2026-03-12
Depends on: `051-gpui-action-text-entry-and-field-primitives-baseline.md`

## Purpose

Freeze the GPUI primitive tranche that closes the biggest remaining
foundation-level Svelte-only gap. This baseline adds selection controls,
compact feedback primitives, and the contract-backed date or time family to
`poodle-gpui-primitives` so later GPUI composites are not forced to invent their
own value and range semantics.

## Package Rule

The `g04.005` tranche extends `poodle-gpui-primitives` with:

- `CheckboxSpec`
- `RadioGroupSpec`
- `SwitchSpec`
- `SelectSpec`
- `SegmentedControlSpec`
- `SliderSpec`
- `ProgressSpec`
- `BadgeSpec`
- `StatusIndicatorSpec`
- `CalendarSpec`
- `RangeCalendarSpec`
- `DatePickerSpec`
- `DateRangePickerSpec`
- `TimeFieldSpec`
- `DateTimePickerSpec`
- `DateTimeRangePickerSpec`

These exports remain part of the same preview-channel public-intent Rust crate
as the earlier structural and form-foundation tranches.

## Contract Coverage Rule

The crate must stay aligned to the existing foundation contracts for:

- `checkbox`
- `radio-group`
- `switch`
- `select`
- `segmented-control`
- `slider`
- `progress`
- `badge`
- `status-indicator`
- `calendar`
- `range-calendar`
- `date-picker`
- `date-range-picker`
- `time-field`
- `date-time-picker`
- `date-time-range-picker`

## Value And Range Rule

This baseline freezes the shared value semantics that later GPUI composites must
reuse:

- boolean and mixed-state selection
- single-choice grouped selection
- popup-owned selected-value controls
- range values using `{ start, end }` string objects
- combined local date and time values using `{ date, time }`
- compact non-interactive feedback surfaces

Later composites should consume these value models rather than introducing
runtime-local substitutes.

## Runtime Honesty Rule

This tranche remains spec-first and honest about current depth:

- value, range, and open-state posture are explicit
- token-backed visual roles are explicit
- mounted overlay behavior, roving-focus plumbing, and full native accessibility
  proof still belong to later `g04` milestones

The repo may expose these primitives as contract-backed GPUI specs before every
one of them is rendered by a fully mounted native control implementation.

## Token Rule

Selection, feedback, and date-time primitives must continue resolving from
`poodle-gpui-tokens` for at least:

- accent and status color roles
- control-size and surface roles
- focus treatment roles
- range and progress emphasis roles
- muted and emphasized inline feedback roles

## Date-Time Rule

GPUI public date and time primitives must preserve the same contract-owned
value posture as Svelte:

- ISO date strings stay Poodle-owned
- local `HH:MM` time strings stay Poodle-owned
- combined picker values stay object-based rather than timestamp-based
- range pickers stay object-based rather than pair-of-native-date-object based

## Seed Evidence

- `packages/gpui/selection-feedback-date-baseline.json`
- `packages/gpui/primitives/README.md`
- `packages/gpui/primitives/src/lib.rs`
- `packages/gpui/primitives/src/checkbox.rs`
- `packages/gpui/primitives/src/radio_group.rs`
- `packages/gpui/primitives/src/switch.rs`
- `packages/gpui/primitives/src/select.rs`
- `packages/gpui/primitives/src/segmented_control.rs`
- `packages/gpui/primitives/src/slider.rs`
- `packages/gpui/primitives/src/progress.rs`
- `packages/gpui/primitives/src/badge.rs`
- `packages/gpui/primitives/src/status_indicator.rs`
- `packages/gpui/primitives/src/calendar.rs`
- `packages/gpui/primitives/src/range_calendar.rs`
- `packages/gpui/primitives/src/date_picker.rs`
- `packages/gpui/primitives/src/date_range_picker.rs`
- `packages/gpui/primitives/src/time_field.rs`
- `packages/gpui/primitives/src/date_time_picker.rs`
- `packages/gpui/primitives/src/date_time_range_picker.rs`
