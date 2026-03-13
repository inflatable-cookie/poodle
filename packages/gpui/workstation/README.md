# Pug GPUI Workstation

Contract-backed GPUI workstation baseline for Pug.

## Public Surface

- crate: `pug-gpui-workstation`
- current workstation shell, command, and layout tranche:
  - `AppHeaderSpec`
  - `ProjectHeaderSpec`
  - `PanelSurfaceSpec`
  - `PanelHeaderSpec`
  - `PanelTabsSpec`
  - `SurfaceTabsSpec`
  - `DockRegionSpec`
  - `SplitViewSpec`
  - `CommandPaletteShellSpec`
  - `CommandPaletteSpec`
  - `ActionDiscoveryPanelSpec`
  - `ShellStatusBarSpec`
  - `WorkspaceShellSpec`
- persistence helpers:
  - `serialize_workspace_layout_snapshot`
  - `parse_workspace_layout_snapshot`
- shared support types:
  - `ActionDiscoverySection`
  - `CommandActionItem`
  - `DiscoveryState`
  - `DockEdge`
  - `DockRegionSnapshot`
  - `PanelTabItem`
  - `SplitOrientation`
  - `SurfaceTabItem`
  - `WorkspaceLayoutSnapshot`
  - `WorkspaceShellState`

## Current Posture

- this crate begins the GPUI workstation layer with the `g04.009` shell,
  command-discovery, and layout-orchestration baseline
- `g04.010` now makes the native accessibility, focus, keyboard, and
  assistive-technology posture for the command and workstation shell surface
  explicit in `packages/gpui/native-accessibility-proof.json`
- it intentionally freezes generic workstation shell semantics before the repo
  contains full native GPUI docking, drag-region, or window-management depth
- later GPUI accessibility and downstream proof tranches should build on these
  shell semantics rather than inventing app-local workstation models

## Dependency Rule

- `pug-gpui-workstation` depends on `pug-gpui-primitives` for tabs, overlays,
  and shell semantics, and on `pug-gpui-composites` for broader shared
  browse/detail surfaces where needed
- it resolves shell chrome, spacing, and elevation from `pug-gpui-tokens`
- shell semantics should stay aligned to documented workstation contracts

## Non-Goals

- this crate does not yet prove mounted GPUI workstation rendering parity
- this crate does not yet prove native drag regions, menu ownership, multi-window
  movement, or full dock/panel transfer mechanics
- this crate does not treat shell-layout specs as proof that native
  accessibility or downstream adoption is complete

## Next Task

Use this first GPUI workstation baseline and the explicit native accessibility
proof posture while executing `g04.011`, hardening the cross-runtime parity
report, intentional delta register, and acceptance-harness expansion.
