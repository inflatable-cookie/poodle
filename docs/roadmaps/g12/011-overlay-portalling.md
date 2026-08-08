# g12.011 — Overlay Portalling

**Status: complete.**

## Problem

Found while building the agent composer (`g12.010`): the `ModelPicker` panel was
not painting over everything on screen. The cause was not z-index. The picker sat
inside a scrolling pane, and a scroll container clips its descendants whatever
their `z-index` — the panel was being cut off, not covered.

The first fix measured placement against the nearest scrolling ancestor so the
panel at least opened into the roomier side. That is a mitigation, not a fix, and
it does not generalise:

- `overflow: hidden/auto/scroll` on any ancestor clips an absolutely positioned
  surface
- `transform`, `filter`, `contain` or `backdrop-filter` on any ancestor
  establishes a containing block, which traps even `position: fixed` — so the
  components that had already moved to fixed positioning (`Menu`, `Tooltip`,
  `HoverCard`, `IconButton`, `ListCard`) were only accidentally correct
- a stacking context anywhere above caps paint order against its siblings

All three are invisible from inside the component. Twenty-three components were
affected. `Dialog` was the only thing in the library that portalled.

## What Shipped

**Core (`@inflatable-cookie/poodle-headless`), `src/dom/anchor.ts`:**

- `resolvePortalTarget` — theme-root-aware mount point (explicit
  `[data-poodle-theme-root]`, else nearest `[data-theme]`, never `<html>`,
  falling back to `<body>`), promoted from the Svelte-only `portal` action
- `resolveClipRect` / `collectClipAncestors` / `intersectClip` /
  `isAnchorClipped` — the box an anchor is actually visible within, and whether
  it still is
- `observeAnchorMovement` — scroll (capture, passive) on every scrollable
  ancestor, window resize, `ResizeObserver` on anchor and surface
- `pointAnchor` / `AnchorTarget` — virtual anchors, for overlays positioned at a
  pointer rather than an element
- `layerContains` (in `dom/dismiss.ts`) — a dismiss layer whose parts are not one
  subtree

The geometry is pure and unit-tested (`packages/core/test/anchor.test.ts`); the
listener wiring is a thin DOM binding over it.

**Framework primitives:**

- `@inflatable-cookie/poodle-svelte` — `anchored` action: `use:anchored={{ anchor, placement,
  offset, matchWidth, minWidth, onPlacement }}`
- `@inflatable-cookie/poodle-react` — `<AnchoredSurface>`, same options, `tag="span"` for inline
  bubbles
- `@inflatable-cookie/poodle-styles` — `anchored-surface.css`: the shared `position: fixed` shell
  plus the `data-anchor-hidden` rule

**Migrated — 23 in all, both web frameworks:** Popover, Select, ModelPicker, RefSelect,
ThemeSelect, OrderBy, FilterBuilder, Menubar, ColorPicker, DatePicker,
DateRangePicker, DateTimePicker, DateTimeRangePicker, DateTimeZonePicker, Menu,
MenuSurface, ContextMenu, SplitButton, Tooltip, HoverCard, IconButton, ListCard,
Tabs (tab tooltips).

**Deleted, not replaced** — each of these existed only to work around the
containment the portal now removes:

- `selectMenuPlacement` (core) — Select's bespoke above/below + align-end rule
- `resolveClipRect` / `resolveSurfacePlacement` (ModelPicker's model module,
  both frameworks) — the g12.010 mitigation
- ContextMenu's and ListCard's hand-rolled viewport clamping, and their
  measure-then-reveal `visibility: hidden` passes
- Tooltip's second-pass position correction, which existed because a transformed
  ancestor shifted the fixed bubble
- SplitButton's `getScrollContainer` boundary maths — the boundary is now the
  viewport, so only the max-height cap remains
- per-component `resize` / `scroll` listener pairs in Menu, Tooltip, IconButton
  and SplitButton, now handled once by `observeAnchorMovement`
- every `position: absolute` + `top/left/bottom/right` block in the migrated
  stylesheets

## Consequences For Consumers

The rule is written up as `docs/contracts/002-anchored-overlays.md`, and the
affected component contracts point at it.

Two things change for anyone reading the DOM:

1. **The surface is not a descendant of the trigger.** Tests and host code reach
   it through the trigger's `aria-controls`, not a container query. The smoke
   and parity gates already looked at `container.parentNode`, so they were
   unaffected; the component-level tests were updated.
2. **A descendant selector that crosses the boundary stops matching.**
   `.poodle-x[data-density="compact"] .poodle-x__surface` no longer applies, so
   the surface carries its own `data-size` / `data-density` / `data-variant` and
   the rules are scoped to it. Fixed in `model-picker`, `ref-select`, `menubar`
   and `select`.

## Verification

- `effigy ci` green — 613 web tests (up 4: portalling and dismiss-layer
  assertions for ModelPicker and RefSelect in both frameworks), 203 core tests
  (up 9 for the new clip geometry, down 3 with `selectMenuPlacement`), 204
  contract-crate tests (unchanged — this is a web-layer change)
- `check:svelte` clean; React typechecked through the preview tsconfig
- `bun test/visual/overlay-portal-probe.ts` — 60/60 checks green across both
  frameworks. It wraps a specimen in a hostile ancestor (scrolling +
  transformed + `z-index: 0`, with a higher-z sibling beside it) and asserts,
  for six anchored overlays, that the surface portalled out, takes fixed
  positioning, fits the viewport, is the topmost painted element at its own
  centre (`elementFromPoint`), and hides when its anchor scrolls out of the
  pane. Confirmed non-vacuous: with the portal append disabled the same probe
  fails on `portalled`, `topmost` and `fits the viewport`
- cross-framework visual sweep green — the CSS deletions changed no pixels

## Not Done

- **Native targets.** GPUI and Jetstream render overlays through their own
  scene graphs and have no equivalent containment problem, so nothing changed
  there. If either grows a clipping viewport, the geometry in `dom/anchor.ts` is
  pure and ports directly.
- **Arrow/caret elements.** No component draws one today; if one does, the
  resolver would need to return the anchor-relative offset along the cross axis.
