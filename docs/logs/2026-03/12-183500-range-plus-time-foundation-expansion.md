# Range Plus Time Foundation Expansion

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- widened the time-aware date family with `DateTimeRangePicker`
- kept the public value contract local and nested rather than leaking
  timestamps or timezone objects into the primitive surface
- documented the boundary in
  `docs/specs/034-range-plus-time-foundation-boundary.md`
- added a foundation seed contract for the new range-plus-time surface

## Validation

- targeted Svelte compilation of the new primitive files
- `bun run docs:build`
- `git diff --check`

## Risks

- this control is still a Svelte-native wrapper rather than a true Bits-backed
  implementation
- timezone-aware range semantics and domain scheduling workflows still need an
  explicit ownership decision later

## Next Task

Choose the next bounded tranche deliberately, with timezone-aware date entry
now the clearest remaining date-family ownership question.
