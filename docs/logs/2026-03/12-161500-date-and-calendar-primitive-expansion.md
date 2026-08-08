# Date And Calendar Primitive Expansion

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- widened the primitive catalogue with the generalized date-selection baseline:
  `Calendar`, `RangeCalendar`, `DatePicker`, and `DateRangePicker`
- added shared ISO-date helpers and exported the new primitives from
  `@pug/svelte-primitives`
- documented the date-selection ownership line in
  `docs/specs/030-date-selection-and-calendar-foundation-baseline.md`
- added foundation seed contracts for each new date family surface

## Validation

- `bun -e 'import { readFileSync } from "node:fs"; import { compile } from "svelte/compiler"; ...'` in `packages/svelte/primitives`
- `bun run docs:build`
- `git diff --check`

## Risks

- these controls are still Svelte-native wrappers, not true Bits-backed date
  primitives
- the baseline is date-only and single-month oriented; time-aware and richer
  preset or scheduling workflows still need explicit ownership decisions
- workspace-wide `svelte-check` is currently noisy from unrelated existing repo
  errors, so validation here used targeted compile coverage for the new files

## Next Task

Decide whether the next widened primitive tranche should cover time-aware date
controls or stay focused on non-date substrate families such as navigation
menu, hover-card, or rating ownership.
