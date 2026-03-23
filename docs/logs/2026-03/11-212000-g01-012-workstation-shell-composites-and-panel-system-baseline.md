# 2026-03-11 g01.012 Workstation-Shell Composites And Panel System Baseline

## Changed

- completed the `g01.012` workstation-shell tranche above the foundation and
  product-composite layers
- added the workstation index at:
  - `docs/contracts/workstation/README.md`
- added workstation contracts for:
  - `AppHeader`
  - `ProjectHeader`
  - `PanelHeader`
  - `PanelTabs`
  - `SurfaceTabs`
  - `DockRegion`
  - `SplitView`
  - `WorkspaceShell`
  - `CommandPaletteShell`
- kept the previously seeded `PanelSurface` as the base shell surface and
  connected the rest of the panel-system layer around it
- added the normative workstation/panel-system baseline:
  - `docs/specs/006-workstation-shell-and-panel-system-rules.md`
- updated contract/spec/roadmap indexes so the workstation layer is visible from
  the main docs surfaces
- closed `g01.012` in the active roadmap

## Downstream Alignment

- Aura's app shell, project toolbar, surface tabs, dock-region tabs, collapse
  posture, and workspace-window structure directly informed the split between
  `AppHeader`, `ProjectHeader`, `SurfaceTabs`, `DockRegion`, and
  `WorkspaceShell`
- Spark's root and dock rendering reinforced that GPUI needs explicit contracts
  for shell-region hierarchy, panel tabs, collapse semantics, command-palette
  focus restoration, and keyboard-reachable split dividers
- the resulting layer stays generic enough for multiple workstation-style apps
  while keeping DAW-specific widgets outside Poodle core

## Accessibility

- made shell accessibility explicit instead of implied:
  - named-region hierarchy
  - panel and surface tab semantics
  - collapse-state exposure
  - keyboard-resizable split dividers
  - deterministic focus restoration after collapse, close, and modal utility
    overlays
- kept GPUI accessibility as a first-class requirement rather than a native
  follow-up

## Validation

- `bun packages/tokens/scripts/build-tokens.ts`
- `git diff --check`

## Remaining

- execute `g01.013` for the Underlay bridge and token-ingestion baseline
- keep Underlay-facing integration behind bridge/adaptor ownership rather than
  exposing Poodle contracts directly to app code

## Next Task

Open `docs/roadmaps/g01/013-underlay-bridge-and-token-ingestion-baseline.md`
and define the Underlay bridge against the now-explicit token, primitive,
product-composite, and workstation-shell layers.
