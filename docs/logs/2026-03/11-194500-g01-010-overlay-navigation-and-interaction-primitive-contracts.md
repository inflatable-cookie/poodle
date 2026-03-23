# 2026-03-11 g01.010 Overlay, Navigation, And Interaction Primitive Contracts

## Changed

- completed the `g01.010` foundation primitive tranche for overlays,
  navigation, and interaction surfaces
- added navigation contracts for:
  - `Tabs`
  - `TabStrip`
- added overlay and command-surface contracts for:
  - `Menu`
  - `ContextMenu`
  - `Tooltip`
  - `Popover`
  - `Dialog`
  - `Drawer`
- added the normative overlay behavior spec:
  - `docs/specs/004-overlay-focus-dismissal-and-layering-rules.md`
- updated the contract and roadmap indexes so the new primitive family is
  visible from the main docs surfaces
- closed `g01.010` in the active roadmap
- tightened the accessibility baseline so overlay invocation, focus trapping,
  dismissal, focus restoration, and layering stay explicit for both Svelte and
  GPUI
- cleaned up earlier contract references that were still pointing at
  `g01.010` as future work, especially around `Select` and
  `SegmentedControl`

## Downstream Alignment

- Aura's archived settings shell and sidebar tabs reinforced the need to keep
  `Tabs` separate from document-style `TabStrip`
- Aura's archived context-menu and command-palette state confirmed that
  invocation origin, dismissal, and restoration need contract-level ownership
  rather than ad hoc store behavior
- Spark's archived focus-state handling reinforced that overlay contracts must
  explicitly defend text focus, modal focus trapping, and command-surface
  restoration on the GPUI side
- the result is still generic Poodle surface, not Loophole-specific UI:
  workstation and product composites will build above these primitives in the
  next roadmap step

## Validation

- `bun packages/tokens/scripts/build-tokens.ts`
- `git diff --check`

## Remaining

- execute `g01.011` for the first product composites built above the primitive
  layer
- keep accessibility explicit as composite contracts begin composing overlays,
  tab systems, forms, and feedback surfaces together

## Next Task

Open `docs/roadmaps/g01/011-product-composites-and-information-architecture-baseline.md`
and author the first product composite tranche above the now-complete
foundation primitive surface.
