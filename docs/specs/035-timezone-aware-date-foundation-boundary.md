# 035 Timezone Aware Date Foundation Boundary

Status: active
Updated: 2026-03-12
Depends on: `031-time-aware-date-foundation-boundary.md`, `034-range-plus-time-foundation-boundary.md`

## Purpose

The local-value date family is now broad enough that the remaining ambiguity is
timezone awareness. This spec defines the smallest timezone-conscious
foundation family that still avoids turning primitives into full scheduling or
conversion workflows.

## Foundation Timezone Aware Family

The current timezone-aware family is:

- `TimeZoneSelect`
- `DateTimeZonePicker`

These cover named timezone selection and explicit local date-time plus timezone
entry without claiming offset math, recurrence, or transport semantics.

## Value Representation Rule

Public timezone-aware values must use contract-owned structures:

- timezone-only value: timezone identifier string
- zoned date-time value: `{ date: string | null; time: string | null; timeZone: string | null }`

Public APIs must not require timestamps, UTC conversions, or platform
date-time objects.

## Ownership Boundary Rule

The following belong in foundation:

- named timezone selection
- local date plus time plus timezone entry

The following do not belong in foundation yet:

- timezone-aware date-time ranges
- conversion or normalization workflows
- recurrence and scheduling systems
- transport, booking, or publishing policy logic
- derived display formatting beyond the selected value summary

## Accessibility Rule

Timezone-aware primitives must preserve:

- reachable timezone selection
- visible partial and complete value states
- trigger-to-popup relationship for picker variants
- predictable dismissal and continued editing posture

## Composition Rule

`TimeZoneSelect` is the standalone timezone-value primitive.

`DateTimeZonePicker` composes `Calendar`, `TimeField`, and
`TimeZoneSelect` under one contract-owned value.

Higher layers may wrap these primitives, but they should not replace the value
contract with timestamps when explicit local-plus-timezone meaning is what the
workflow owns.

## Current Risk

This tranche still avoids timezone-aware ranges and conversion semantics. That
is intentional. It keeps foundation honest, but it means richer scheduling or
publishing workflows still need explicit later ownership decisions.

## Evidence

- `docs/contracts/foundation/time-zone-select.md`
- `docs/contracts/foundation/date-time-zone-picker.md`
- `packages/svelte/primitives/README.md`

## Next Task

Decide whether timezone-aware ranges should remain outside foundation, or
whether the date family is now broad enough that widening further would be
churn rather than progress.
