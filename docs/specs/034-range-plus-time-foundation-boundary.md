# 034 Range Plus Time Foundation Boundary

Status: active
Updated: 2026-03-12
Depends on: `030-date-selection-and-calendar-foundation-baseline.md`, `031-time-aware-date-foundation-boundary.md`

## Purpose

The date-only and time-aware single-value baselines are now explicit. This spec
defines the smallest range-plus-time family that still belongs in foundation
without silently promoting timezone, recurrence, or scheduling workflow
ownership.

## Foundation Range Plus Time Family

The current range-plus-time foundation family is:

- `DateTimeRangePicker`

This covers bounded start and end local values while keeping routing,
timezone, and domain scheduling semantics out of the primitive layer.

## Value Representation Rule

Public range-plus-time values must use contract-owned local-value structures:

- `{ start: { date: string | null; time: string | null }, end: { date: string | null; time: string | null } }`

Public APIs must not require `Date` instances, timestamps, or timezone-aware
objects.

## Ownership Boundary Rule

The following belong in foundation:

- bounded local start/end date plus time entry
- range-calendar plus paired time-field composition

The following do not belong in foundation yet:

- timezone-aware date-time range entry
- recurrence editors
- transport, booking, or publishing schedule workflows
- availability grids
- preset-driven relative scheduling flows

## Accessibility Rule

Range-plus-time primitives must preserve:

- reachable range-calendar interaction
- reachable paired time-entry controls
- visible partial and complete range states
- trigger-to-popup relationship and dismissal semantics

## Composition Rule

`DateTimeRangePicker` is the current generalized range-plus-time primitive.

Higher layers may wrap it for product workflows, but they should not replace
its local-value contract with timestamps or timezone objects when local range
semantics are what the workflow actually owns.

## Current Risk

This baseline intentionally stops short of timezone and recurrence semantics.
That keeps the contract honest, but it means richer scheduling workflows still
need future composite or specialized foundation decisions.

## Evidence

- `docs/contracts/foundation/date-time-range-picker.md`
- `docs/specs/031-time-aware-date-foundation-boundary.md`
- `packages/svelte/primitives/README.md`

## Next Task

Keep the range-plus-time baseline stable while deciding whether timezone-aware
ranges belong in higher layers or whether widening the date family further
would be justified.
