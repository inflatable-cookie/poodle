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
- `parseWorkspaceLayoutSnapshot`
- `serializeWorkspaceLayoutSnapshot`
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

## Current Downstream Adoption Proof

The current Loophole-facing foundation proof lives in:

- `packages/svelte/workstation/loophole-foundation-proof.json`

That proof records:

- which Pug packages Loophole may consume directly
- which workstation exports are approved as shared shell foundation
- which DAW surfaces remain downstream-owned
- which adoption frictions remain explicit

## Explicit Non-Goals

This package does not become a DAW widget kit.

It does not own:

- transport bars
- timelines
- mixer strips
- automation lanes
- plugin racks
- clip editors
- session rulers
- meter bridges

## Next Task

Use this package surface while following the workstation-oriented reference-app
lane, keeping generic shell foundation separate from downstream-owned DAW or
domain widgets and from unproven native-runtime parity claims.
