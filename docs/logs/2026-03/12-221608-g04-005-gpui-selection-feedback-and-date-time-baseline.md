---
title: g04.005 gpui selection feedback and date-time baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, rust, primitives]
---

## Summary

Completed `g04.005` by widening `pug-gpui-primitives` into the first GPUI
selection, compact feedback, and date-time primitive baseline.

## What changed

- added the normative baseline `docs/specs/052-gpui-selection-feedback-and-date-time-primitives-baseline.md`
- completed `docs/roadmaps/g04/005-gpui-selection-value-feedback-and-date-time-primitives.md`
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
- added shared GPUI primitive types for mixed-state selection, grouped options,
  compact feedback tones, week-start policy, and date/time object values
- pinned popup-owned value posture and date/time object semantics inside the
  Rust crate so later composites inherit the same public values as Svelte
- added crate tests for selection state, slider or progress normalization,
  compact feedback roles, and date/time value posture
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI
  selection or feedback or date baseline artifact is machine-checked
- rolled the package and roadmap surfaces forward to `g04.006`

## Validation

- `cargo fmt --manifest-path packages/gpui/primitives/Cargo.toml`
- `cargo check --manifest-path packages/gpui/primitives/Cargo.toml`
- `cargo test --manifest-path packages/gpui/primitives/Cargo.toml`
- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.005` is now explicit. Pug has a much broader GPUI primitive baseline for
selection, compact status, and date/time value controls, which materially
reduces the remaining “Svelte-only by default” surface before composite and
workstation parity.

## Next

Open `g04.006` and implement the GPUI overlay, disclosure, navigation, and
menu primitive tranche on top of the widened GPUI primitive crate.
