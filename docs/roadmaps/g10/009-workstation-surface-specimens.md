# g10.009 — Workstation Surface Specimens

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.008
Primary repos: `pug`

## Goals

- [ ] create per-component specimens for all workstation surfaces
- [ ] demonstrate how game applications compose Pug surfaces into window
  layout

## Execution Checklist

- [ ] create `workspace_shell.rs` — WorkspaceShell with header, sidebar,
  content, and status bar
- [ ] create `app_header.rs` — AppHeader with logo, navigation, and action
  slots
- [ ] create `project_header.rs` — ProjectHeader with project name, branch
  badge, and actions
- [ ] create `command_palette.rs` — CommandPalette with search, category
  grouping, and keyboard navigation
- [ ] create `command_palette_shell.rs` — CommandPaletteShell managing
  open/close activation
- [ ] create `panel_header.rs` — PanelHeader with title, collapse/close
  controls
- [ ] create `panel_surface.rs` — PanelSurface with header integration and
  content scroll
- [ ] create `panel_tabs.rs` — PanelTabs for switching panel views
- [ ] create `dock_region.rs` — DockRegion with resize and snap positions
- [ ] create `split_view.rs` — SplitView with resizable split orientation
- [ ] create `shell_status_bar.rs` — ShellStatusBar with left/center/right
  slots
- [ ] create `surface_tabs.rs` — SurfaceTabs for major app surfaces
- [ ] create `action_discovery.rs` — ActionDiscoveryPanel with categorized
  action list and shortcuts
- [ ] register all modules and wire slug routing
- [ ] verify all 13 specimens render without panic

## Acceptance Criteria

- [ ] all 13 workstation specimens render in the preview app
- [ ] WorkspaceShell demonstrates full shell composition
- [ ] CommandPalette supports keyboard navigation
- [ ] `cargo check` passes

## Next Task

Open `g10.010` and implement display controls.
