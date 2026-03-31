# 2026-03-11 g01.011 Product Composites And Information Architecture Baseline

## Changed

- completed the `g01.011` product-composite tranche above the now-live
  foundation primitives
- added the composite index at:
  - `docs/contracts/composites/README.md`
- added information-display composites for:
  - `Card`
  - `PageHeader`
  - `Breadcrumbs`
  - `DetailItem`
  - `DetailSection`
  - `DetailShell`
  - `EmptyState`
- added browse-shell composites for:
  - `FilterToolbar`
  - `ListShell`
  - `GridShell`
- added the normative composition/IA baseline:
  - `docs/specs/005-product-composite-composition-and-information-architecture-rules.md`
- updated contract/spec/roadmap indexes so the composite layer is visible from
  the main docs surfaces
- closed `g01.011` in the active roadmap

## Downstream Alignment

- Aura's settings sections and generated settings surfaces reinforced the need
  for generic `DetailItem`, `DetailSection`, `DetailShell`, and `PageHeader`
  contracts rather than app-specific settings widgets in Poodle core
- Aura's command and browse surfaces reinforced the need for a reusable
  `FilterToolbar` plus `ListShell` and `GridShell` framing that keeps result
  logic app-owned
- the resulting composite layer stays usable for Underlay-style product apps
  first, while leaving workstation shell chrome and dock behavior for `g01.012`

## Accessibility

- kept heading hierarchy, region labeling, label/value semantics, empty-state
  text requirements, and state-change focus continuity explicit across the whole
  tranche
- kept GPUI accessibility obligations direct:
  - named-region and grouped-content mapping
  - breadcrumb current-location exposure
  - detail row label/value semantics
  - focus continuity when list/detail shells swap between ready, empty, loading,
    and error states

## Validation

- `bun packages/tokens/scripts/build-tokens.ts`
- `git diff --check`

## Remaining

- execute `g01.012` for workstation-shell composites and panel-system baseline
- keep product composites and workstation shell contracts separate so Poodle does
  not drift back toward app-specific DAW surfaces

## Next Task

Open `docs/roadmaps/g01/012-workstation-shell-composites-and-panel-system-baseline.md`
and author the workstation-shell composite batch above `PanelSurface`,
`TabStrip`, menus, dialogs, and related primitives.
