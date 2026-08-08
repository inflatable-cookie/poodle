# Time Aware Date Primitive Expansion

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- widened the primitive catalogue with the smallest generalized time-aware date
  tranche: `TimeField` and `DateTimePicker`
- kept the public value contract local and Pug-owned with `HH:MM` time strings
  and `{ date, time }` datetime objects instead of `Date` instances
- documented the ownership boundary in
  `docs/specs/031-time-aware-date-foundation-boundary.md`
- added foundation seed contracts for the new time-aware surfaces

## Validation

- targeted Svelte compilation of the new primitive files
- `bun run docs:build`
- `git diff --check`

## Risks

- these controls are still Svelte-native wrappers, not true Bits-backed date or
  time primitives
- timezone-aware, recurrence, and range-plus-time workflows are still outside
  the foundation baseline and need explicit ownership decisions later

## Next Task

Choose the next family-sized primitive batch deliberately, with navigation
menu, hover-card, rating, or timezone-aware date ownership now the clearest
remaining candidates.
