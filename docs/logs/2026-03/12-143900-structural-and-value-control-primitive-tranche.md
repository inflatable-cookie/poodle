# Structural And Value-Control Primitive Tranche

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- expanded `@pug/svelte-primitives` with the structural layer:
  `Box`, `Stack`, `Inline`, `Grid`, `Spacer`, `Surface`, `Separator`, and
  `ScrollShell`
- added the remaining core value-control tranche:
  `SegmentedControl`, `Slider`, `RangeSlider`, `TriStateSwitch`,
  `NumberEntry`, and `EditableLabel`
- updated the primitive package readme and the baseline spec so the current
  Svelte package surface now reflects both the composition base and the core
  standard-control family rather than only the original forms slice

## Validation

- `bun run docs:build`
- `git diff --check`

## Risks

- the overlay, navigation, and menu families are still missing and will want
  real Bits-backed wrappers rather than shallow native stand-ins
- the new slider/range-slider implementations establish the package surface,
  but their interaction polish should still be reviewed in live preview and
  later parity work

## Next Task

Implement the Bits-backed overlay, navigation, and menu primitive families as
the next coherent tranche, especially `Tabs`, `TabStrip`, `Tooltip`,
`Popover`, `Dialog`, `Drawer`, `Menu`, and `ContextMenu`.
