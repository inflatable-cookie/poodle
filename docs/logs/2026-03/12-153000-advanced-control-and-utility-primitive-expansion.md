# Advanced Control And Utility Primitive Expansion

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- widened the primitive contract catalogue beyond the original foundation
  baseline with `Toggle`, `ToggleGroup`, `Toolbar`, `Meter`, `PinInput`, and
  `Combobox`
- added the first implementation tranche for those generalized utility and
  advanced-input controls to `@pug/svelte-primitives`
- documented the promotion policy in
  `docs/specs/029-advanced-primitive-promotion-and-substrate-mapping.md` so
  wider substrate families are mapped deliberately into foundation, composite,
  and workstation ownership

## Validation

- `bun run docs:build`
- `git diff --check`

## Risks

- these new controls are still Svelte-native wrappers rather than real
  Bits-backed implementations
- date or calendar, rating, command, and data-oriented substrate families still
  need explicit ownership decisions before they should enter the contract
  catalogue

## Next Task

Choose the next wider primitive tranche deliberately, with date or calendar
primitives the most likely next candidate if they are meant to be foundation
surfaces rather than composite-owned workflows.
