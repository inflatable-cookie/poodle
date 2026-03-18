# g11.007 — Window-Aware Surface Tabs And Panel-Tab Orchestration

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.006
Primary repos: `pug`

## Goals

- [x] make surface tabs and panel tabs coexist cleanly inside the window-host
  model
- [x] prevent tab semantics from staying local-ordering-only

## Execution Checklist

- [x] define surface-tab semantics tied to workspace window ownership
- [x] define panel-tab semantics tied to dock-local panel switching
- [x] define ordering, active, and close/move behaviors
- [x] specify what tab systems deliberately do not own:
  - [x] product navigation
  - [x] command routing
  - [x] project semantics
- [x] update the existing tab contracts in `docs/contracts/workstation/`
  following the 12-section template

## Deliverables

- `docs/contracts/workstation/surface-tabs.md` — updated with `windowId` prop
  and window-aware identity semantics
- `docs/contracts/workstation/panel-tabs.md` — updated with `dockId` prop
  and dock-local context semantics

## Acceptance Criteria

- [x] surface tabs and panel tabs have distinct documented roles
- [x] the tab model is window-aware
- [x] no renderer-specific or downstream-specific divergence is required

## Next Task

Open `g11.008` and define stronger panel variants.
