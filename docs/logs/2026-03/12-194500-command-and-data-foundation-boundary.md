# Command And Data Foundation Boundary

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- widened the primitive surface with the low-level data primitives `Table` and
  `Pagination`
- kept command discovery and richer data interaction explicitly outside
  foundation
- documented the ownership split in
  `docs/specs/036-command-and-data-foundation-boundary.md`

## Validation

- targeted Svelte compilation of the new primitive files
- `bun run docs:build`
- `git diff --check`

## Risks

- these controls are still Svelte-native wrappers rather than true Bits-backed
  implementations
- command discovery, sorting, selection, and richer browse-state ownership
  remain intentionally above foundation and still need continued discipline

## Next Task

Assess for churn before widening further. The next meaningful batch should
probably tighten parity and ownership discipline around the now-large surface
instead of continuing to add primitives blindly.
