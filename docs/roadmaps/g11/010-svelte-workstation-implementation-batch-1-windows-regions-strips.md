# g11.010 — Svelte Workstation Implementation Batch 1: Windows, Regions, Strips

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.009
Primary repos: `pug`

## Goals

- [x] implement the first half of the new workstation substrate in Svelte
- [x] prove the contract on the renderer that is currently furthest ahead

## Execution Checklist

- [x] implement workspace window host primitives in Svelte
- [x] implement expanded region snapshot support
- [x] implement strip rail components and variants
- [x] implement resize handles, split dividers, and collapse affordances

## Deliverables

### New Components

| Component | File | Description |
|-----------|------|-------------|
| WorkspaceWindow | `WorkspaceWindow.svelte` | Window host with surface ownership, slot-based shell composition |
| StripRail | `StripRail.svelte` | Four-edge strip with icon/mixed modes, grouped items, badges |
| ResizeHandle | `ResizeHandle.svelte` | Standalone resize primitive with drag and keyboard support |
| CollapseAffordance | `CollapseAffordance.svelte` | Directional collapse/expand toggle button |
| SplitDivider | `SplitDivider.svelte` | Divider composing ResizeHandle with optional collapse buttons |

### Type Expansions

- `types.ts` updated with v2 snapshot types, new component types
  (`WindowSurface`, `StripItem`, `StripEdge`, `StripMode`,
  `HostedSurfaceState`, `PanelVariant`, `DockEmphasis`,
  `DockCollapsedPosture`, `CollapseDirection`, `StripRegionSnapshot`,
  `CenterRegionSnapshot`)
- `WorkspaceLayoutSnapshot` expanded with `regions` and `splitRatios` (v2)
  while maintaining v1 backwards compatibility
- `index.ts` updated with all new component and type exports

## Acceptance Criteria

- [x] the new window, region, strip, and resize surface exists in Svelte
- [x] no renderer-local semantic drift is introduced

## Next Task

Open `g11.011` and complete the Svelte workstation implementation.
