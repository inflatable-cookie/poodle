# 031 Time Aware Date Foundation Boundary

Status: active
Updated: 2026-03-12
Depends on: `030-date-selection-and-calendar-foundation-baseline.md`, `029-advanced-primitive-promotion-and-substrate-mapping.md`

## Purpose

The date-only baseline is now explicit. This spec defines the smallest
time-aware date family that still belongs in foundation without silently
promoting scheduling or timezone workflows into the primitive layer.

## Foundation Time Aware Family

The current time-aware foundation family is:

- `TimeField`
- `DateTimePicker`

These cover generalized wall-clock entry and combined date-plus-time values
without implying recurrence, transport, calendar apps, or availability logic.

## Value Representation Rule

Public time-aware values must use contract-owned local-value structures:

- time-only value: `HH:MM`
- combined date and time value: `{ date: string | null; time: string | null }`

Public APIs must not require `Date` instances, timezone offsets, or substrate
date-time wrappers.

## Ownership Boundary Rule

The following belong in foundation:

- standalone local time value entry
- popup-owned combined date and time value entry

The following do not belong in foundation yet:

- timezone selectors
- combined timestamp or UTC normalization contracts
- recurrence editors
- date-time range scheduling
- preset-driven relative time workflows
- booking, transport, or domain scheduling semantics

## Composition Rule

`TimeField` is the standalone local time primitive.

`DateTimePicker` composes `Calendar` and `TimeField` under one value contract.

Higher layers may wrap them, but they should not replace these value contracts
with runtime-specific timestamp objects when local wall-clock values are what
the workflow actually owns.

## Accessibility Rule

Time-aware date primitives must preserve:

- accessible trigger and popup relationships
- reachable date-grid and time-entry controls
- visible partial and complete value states
- keyboard dismissal and continued editing posture

## Current Risk

The current tranche intentionally avoids timezone semantics. Range-plus-time is
now covered by a dedicated follow-on baseline, but richer scheduling or
publishing workflows still need explicit composite or future foundation
decisions rather than incidental growth.

## Evidence

- `docs/contracts/foundation/time-field.md`
- `docs/contracts/foundation/date-time-picker.md`
- `docs/contracts/foundation/date-time-range-picker.md`
- `docs/specs/030-date-selection-and-calendar-foundation-baseline.md`
- `docs/specs/034-range-plus-time-foundation-boundary.md`
- `packages/svelte/primitives/README.md`

## Next Task

Decide whether timezone-aware entry should remain composite-owned, or whether a
small timezone-conscious foundation tranche is generalized enough to promote
next.
