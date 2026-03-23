# g07.010 — GPUI Workstation Shell and Layout Updates

Status: Completed
Updated: 2026-03-14

## Objective

Implement `RenderComponent` for all 13 workstation shell and layout specs in
the GPUI adapter, migrating them to the new adapter pattern from g06.

## Deliverables

### RenderComponent implementations (render_workstation.rs)

| Spec | Element ID | Notes |
|------|-----------|-------|
| ActionDiscoveryPanelSpec | `action-discovery-panel` | Grouped action sections |
| AppHeaderSpec | `app-header` | Application title bar |
| CommandPaletteSpec | `command-palette` | Filtered action list |
| CommandPaletteShellSpec | `command-palette-shell` | Palette overlay container |
| DockRegionSpec | `dock-region` | Collapsible dock with tabs |
| PanelHeaderSpec | `panel-header` | Panel title with actions |
| PanelSurfaceSpec | `panel-surface` | Panel content container |
| PanelTabsSpec | `panel-tabs` | Reorderable panel tab strip |
| ProjectHeaderSpec | `project-header` | Project name and status |
| ShellStatusBarSpec | `shell-status-bar` | Bottom status indicators |
| SplitViewSpec | `split-view` | Resizable split layout |
| SurfaceTabsSpec | `surface-tabs` | Top-level surface tab strip |
| WorkspaceShellSpec | `workspace-shell` | Root workspace container |

### Test coverage

- 13 tests verifying spec_type propagation through render pipeline
- DockRegionSpec tested with DockEdge and SplitViewSpec with SplitOrientation

### Module registration

- `render_workstation` module added to lib.rs
- SUPPORTED_WORKSTATION populated with all 13 workstation spec names

## Verification

```
cargo test — 124 → 137 tests passing (13 new)
cargo check — clean compilation, no warnings
```

## Full Adapter Coverage

With this milestone, the GPUI adapter covers all 118 Flint specs:
- 64 primitive specs (g07.002–006)
- 41 composite specs (g07.007–009)
- 13 workstation specs (g07.010)
