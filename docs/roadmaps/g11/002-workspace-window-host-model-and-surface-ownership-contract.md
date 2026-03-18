# g11.002 — Workspace Window Host Model And Surface-Ownership Contract

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.001
Primary repos: `pug`

## Goals

- [x] define a generalized workspace window host model
- [x] make surface ownership and movement explicit without introducing
  app-specific window policy

## Execution Checklist

- [x] define the workstation vocabulary for:
  - [x] workspace window
  - [x] active surface
  - [x] detached surface
  - [x] surface ordering
  - [x] window-local focus
- [x] specify how surfaces move between windows
- [x] specify the minimum snapshot/state required to persist window-to-surface
  relationships
- [x] define what the host model does not own:
  - [x] app toolbar behavior
  - [x] project identity
  - [x] product-level menus and commands
- [x] produce contract file in `docs/contracts/workstation/` following the
  12-section template

## Deliverables

- `docs/contracts/workstation/workspace-window.md` — 12-section contract for
  WorkspaceWindow component

## Acceptance Criteria

- [x] one documented window-host contract exists for Svelte and GPUI
- [x] surface identity and movement rules are explicit
- [x] the contract is clearly generalized rather than Loophole-shaped

## Next Task

Open `g11.003` and expand the region grammar and layout snapshot.
