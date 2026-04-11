# 030 Date Selection And Calendar Foundation Baseline

Status: active
Updated: 2026-03-12
Depends on: `004-overlay-focus-dismissal-and-layering-rules.md`, `028-primitive-baseline-and-bits-aligned-surface.md`, `029-advanced-primitive-promotion-and-substrate-mapping.md`

## Purpose

The wider primitive catalogue now needs a date-selection baseline that is still
general enough for foundation ownership. This spec defines which date surfaces
belong in foundation now, how they represent values, and where the boundary
stops before scheduling or domain workflows start.

## Foundation Date Family

The current foundation date-selection family is:

- `Calendar`
- `RangeCalendar`
- `DatePicker`
- `DateRangePicker`

These are the minimum generalized date-selection surfaces most apps need
without implying full scheduling, time, or recurrence ownership.

## Value Representation Rule

Public foundation date values must use contract-owned date-only structures:

- single-date value: ISO `YYYY-MM-DD`
- range value: `{ start: string | null; end: string | null }`

Public APIs must not require downstream callers to pass platform `Date`
instances, timezone-specific objects, or substrate-owned date wrappers.

## Ownership Boundary Rule

The following belong in foundation:

- visible month calendars for single-date selection
- visible month calendars for bounded range selection
- trigger-based date value controls that wrap those calendars

The following do not belong in foundation yet:

- date plus time or timezone entry
- recurrence editors
- booking, scheduling, and availability workflows
- relative-date preset orchestration such as "last 7 days" bars
- domain-specific reporting or transport semantics

## Accessibility Rule

Date-selection primitives must preserve:

- visible month labeling
- keyboard movement across day grids
- selected day or range semantics
- trigger-to-popup accessibility relationships for picker variants

Native platform date controls may be used internally later, but they must not
erase the documented semantics above.

## Composition Rule

`Calendar` and `RangeCalendar` are inline building blocks.

`DatePicker` and `DateRangePicker` are value controls built on those calendars
plus overlay ownership.

Composite or workstation layers may wrap these primitives, but they should not
invent a parallel date value contract when the baseline family is sufficient.

## Current Risk

The current tranche is still Svelte-native and single-month oriented. That is
acceptable for baseline ownership and first implementation, but richer
multi-month posture, preset helpers, and more advanced locale treatment still
need deliberate follow-up rather than incidental growth.

## Evidence

- `docs/specs/029-advanced-primitive-promotion-and-substrate-mapping.md`
- `docs/contracts/components/calendar.md`
- `docs/contracts/components/range-calendar.md`
- `docs/contracts/components/date-picker.md`
- `docs/contracts/components/date-range-picker.md`
- `packages/svelte/primitives/README.md`

## Next Task

Keep the date-only baseline stable while deciding whether timezone-aware entry,
range-plus-time selection, or relative-date preset helpers should become
foundation surfaces or remain composite-owned workflows.
