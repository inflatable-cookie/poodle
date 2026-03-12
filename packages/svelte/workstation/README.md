# Pug Svelte Workstation

First Svelte workstation implementation surface for contract-backed shell and
command-discovery components.

## Public Surface

- `AppHeader`
- `ProjectHeader`
- `ShellStatusBar`
- `WorkspaceShell`
- `PanelTabs`
- `SurfaceTabs`
- `PanelHeader`
- `PanelSurface`
- `SplitView`
- `DockRegion`
- `CommandPalette`
- `ActionDiscoveryPanel`
- root import: `@pug/svelte-workstation`
- type-only import: `@pug/svelte-workstation/types`

## Stability Notes

- public entry points are the package root and `./types`
- layout persistence helpers stay public because the shell contracts already
  require host-owned snapshot round-tripping
- desktop-specific drag regions, native menus, and multi-window orchestration
  remain downstream concerns
- GPUI workstation parity is still an explicit adoption blocker rather than an
  implied capability

## Next Task

Use this package surface while executing `g02.016`, confirming which
workstation entry points are stable enough to carry into the first
downstream-adoption generation.
