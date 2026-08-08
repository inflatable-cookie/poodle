# Overlay, Navigation, And Menu Primitive Tranche

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- expanded `@pug/svelte-primitives` with the overlay, navigation, and menu
  family: `Tabs`, `TabStrip`, `Tooltip`, `Popover`, `Dialog`, `Drawer`,
  `Menu`, and `ContextMenu`
- updated the primitive package readme and the baseline spec so the current
  Svelte package surface now covers the full documented foundation contract set
  instead of leaving the overlay and navigation layer implicit
- kept the batch contract-first and Svelte-native rather than pretending the
  package already has a real Bits substrate wired in

## Validation

- `bun run docs:build`
- `git diff --check`

## Risks

- this tranche completes the documented foundation surface, but the new overlay
  and menu primitives are still native Svelte wrappers rather than true
  Bits-backed implementations
- Bits' broader advanced families such as `Combobox`, `Command`, date or
  calendar controls, `Pagination`, `Table`, `Toggle`, `ToggleGroup`, and
  `Toolbar` still sit outside the current Pug contract catalogue

## Next Task

Define the next-generation primitive expansion beyond the current foundation
baseline, deciding which wider Bits families should become first-class Pug
contracts instead of remaining substrate-only possibilities.
