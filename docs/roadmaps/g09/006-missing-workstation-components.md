# g09.006 — Missing Workstation Components

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.005
Primary repos: `pug`

## Goals

- [ ] implement first-class workstation surface components that compose
  primitives and composites into application-level layout shells
- [ ] these are the highest-level components and define the overall app
  structure

## Execution Checklist

- [ ] create `PugWorkspaceShell` — top-level app container with header,
  sidebar, content area, and status bar slots
- [ ] create `PugAppHeader` — application header bar with logo, navigation,
  user menu, and action slots
- [ ] create `PugCommandPalette` — searchable command list overlay with
  keyboard navigation and category grouping
- [ ] create `PugCommandPaletteShell` — wrapper managing open/close state
  and keyboard shortcut activation
- [ ] create `PugPanelHeader` — panel title bar with title, actions, and
  collapse/close controls
- [ ] create `PugPanelSurface` — panel content container with header
  integration and scroll management
- [ ] create `PugPanelTabs` — tab strip for switching between panel content
  views
- [ ] create `PugDockRegion` — dockable panel region with resize handles
  and snap positions
- [ ] create `PugSplitView` — resizable split layout with configurable
  orientation and min/max constraints
- [ ] create `PugShellStatusBar` — bottom status bar with left/center/right
  slots for status items
- [ ] create `PugSurfaceTabs` — top-level tab strip for switching between
  major app surfaces
- [ ] create `PugActionDiscoveryPanel` — categorized action list with search,
  keyboard shortcut display, and recent actions
- [ ] register all workstation components in `lib.rs`
- [ ] verify compilation with `cargo check`

## Acceptance Criteria

- [ ] all 12 workstation components compile and are exported
- [ ] WorkspaceShell correctly composes header, sidebar, content, and
  status bar children
- [ ] CommandPalette supports keyboard navigation (up/down/enter/escape)
- [ ] SplitView handles drag-to-resize with min/max constraints
- [ ] DockRegion supports snap-to-edge positioning
- [ ] `cargo check` passes with zero errors

## Next Task

Open `g09.007` and begin specimen upgrades for structural primitives.
