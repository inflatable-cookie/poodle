# g11.012 — Docs, Specimens, And Downstream Reference Adoption Proof

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.011
Primary repos: `pug`, `loophole`

## Goals

- [x] refresh docs and specimens for the new workstation surface
- [x] prove that at least one downstream workstation consumer benefits from the
  new substrate
- [x] establish Svelte-side evidence baseline before GPUI parity in `g12`

## Execution Checklist

- [x] update workstation docs and component references
- [x] verify all new workstation contracts in `docs/contracts/workstation/`
  follow the 12-section template
- [x] document any known gaps that will need addressing in `g12` (GPUI) and
  `g13` (Jetstream)

## Deliverables

### Contract Coverage

All 21 workstation contracts in `docs/contracts/workstation/` follow the
12-section template:

**Pre-existing (13):** workspace-shell, split-view, dock-region, panel-surface,
panel-header, panel-tabs, surface-tabs, app-header, project-header,
shell-status-bar, command-palette-shell, command-palette, action-discovery-panel

**New in g11 (6):** workspace-window, workspace-layout, strip-rail,
resize-handle, split-divider, collapse-affordance, hosted-surface

**Updated in g11 (4):** dock-region (emphasis, collapsed posture),
surface-tabs (windowId), panel-tabs (dockId), panel-surface (variant)

### Known Gaps for g12/g13

- GPUI implementation of all 6 new components
- GPUI adaptation of updated props (emphasis, variant, windowId, dockId)
- Jetstream feasibility assessment for workstation components
- Cross-runtime parity evidence

## Acceptance Criteria

- [x] docs and specimens are current for the new workstation surface
- [x] contract files exist for all new workstation components

## Next Task

Open `g11.013` and close the generation with a clear shared-versus-downstream
boundary record.
