# Timezone Aware Date Foundation Expansion

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- widened the date family with `TimeZoneSelect` and `ZonedDateTimePicker`
- kept the public value contract string- and object-based rather than leaking
  timestamps or conversion semantics into foundation
- documented the timezone-aware boundary in
  `docs/specs/035-timezone-aware-date-foundation-boundary.md`
- added foundation seed contracts for the new timezone-aware surfaces

## Validation

- targeted Svelte compilation of the new primitive files
- `bun run docs:build`
- `git diff --check`

## Risks

- these controls are still Svelte-native wrappers rather than true Bits-backed
  implementations
- timezone-aware ranges, conversion workflows, and scheduling systems still
  need deliberate ownership decisions later

## Next Task

Assess for churn and decide whether the date family should now freeze, with the
next expansion more likely needing to revisit command or data ownership rather
than keep widening date primitives.
