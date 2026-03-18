# g11.011 — Svelte Workstation Implementation Batch 2: Docks, Tabs, Panels, And Hosted Surfaces

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.010
Primary repos: `pug`

## Goals

- [x] complete the Svelte workstation implementation for the deeper shell
  surface
- [x] make the new Svelte side a stable reference before full downstream proof

## Execution Checklist

- [x] implement deeper dock behavior
- [x] implement window-aware surface tabs and panel tabs
- [x] implement panel variants
- [x] implement hosted external-surface containers
- [x] extend specimens and docs examples to cover all new workstation pieces

## Deliverables

### New Component

| Component | File | Description |
|-----------|------|-------------|
| HostedSurface | `HostedSurface.svelte` | External content host with identity, 5 states, detach/reload/close |

### Updated Components

| Component | Changes |
|-----------|---------|
| DockRegion | Added `emphasis` (standard/quiet/strong) and `collapsedPosture` (hidden/icon-strip) props |
| PanelSurface | Added `variant` (utility/standard/focused) prop with distinct visual treatments |

### Total Workstation Surface

18 Svelte components (12 existing + 6 new: WorkspaceWindow, StripRail,
ResizeHandle, CollapseAffordance, SplitDivider, HostedSurface)

## Acceptance Criteria

- [x] Svelte workstation support covers the full `g11` contract
- [x] docs/specimens show the new substrate working together

## Next Task

Open `g11.012` and refresh docs, specimens, and downstream adoption proof.
