# g05.005 GPUI Selection, Value, Feedback, And Date-Time Primitives

Status: completed
Owner: Flint Core
Updated: 2026-03-12
Depends on: g05.001, g05.002, g05.003, g05.004
Primary repos: `flint`

## Goals

- [x] implement the GPUI selection, value, feedback, and date-time primitives
- [x] keep the larger primitive catalogue from remaining Svelte-only by default

## Execution Checklist

- [x] implement checkbox, radio, switch, segmented control, select-like,
  slider-like, progress, badge, status, and related feedback primitives in GPUI
- [x] implement the date and time families that remain foundation-safe and
  already contract-backed
- [x] document which selection or date behaviors need explicit GPUI deltas
- [x] verify the GPUI primitive layer is broad enough to support composite parity

## Acceptance Criteria

- [x] GPUI selection and value primitive posture is explicit
- [x] GPUI feedback and date-time primitive posture is explicit

## Completed Work

- added the normative baseline `docs/specs/052-gpui-selection-feedback-and-date-time-primitives-baseline.md`
- added the machine-readable artifact `packages/gpui/selection-feedback-date-baseline.json`
- expanded `packages/gpui/primitives` with:
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
- added shared GPUI primitive types for mixed-state selection, grouped options, orientation, status tone, week start, and date or time object values
- froze popup-owned value posture and date/time object semantics inside the Rust crate so later composites inherit the same contract-owned public values as Svelte
- added crate tests for selection state, slider/progress normalization, compact feedback roles, and single/range date/time value posture
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI selection/feedback/date baseline artifact is machine-checked
- updated package and roadmap surfaces so the repo now points at `g05.006`

## Next Task

Open `g05.006` and implement the GPUI overlay, disclosure, navigation, and
menu primitive tranche.
