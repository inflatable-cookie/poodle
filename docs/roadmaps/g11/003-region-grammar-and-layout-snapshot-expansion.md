# g11.003 — Region Grammar And Layout-Snapshot Expansion

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.002
Primary repos: `pug`

## Goals

- [x] expand the workstation region grammar to cover real multi-region shells
- [x] make layout snapshots strong enough for reuse across renderers

## Execution Checklist

- [x] define canonical region keys:
  - [x] `topStrip`
  - [x] `bottomStrip`
  - [x] `leftStrip`
  - [x] `rightStrip`
  - [x] `left`
  - [x] `right`
  - [x] `centerTop`
  - [x] `centerBottom`
- [x] add snapshot fields for:
  - [x] active panel
  - [x] tab placement
  - [x] collapsed state
  - [x] region visibility
  - [x] size ratios where appropriate
- [x] verify that the snapshot remains renderer-agnostic
- [x] produce or update contract files in `docs/contracts/workstation/`
  following the 12-section template

## Deliverables

- `docs/contracts/workstation/workspace-layout.md` — 12-section contract for
  the region grammar, layout snapshot types, and region visibility rules

## Acceptance Criteria

- [x] the full region grammar is documented and typed
- [x] layout snapshot shape covers collapse and tab-placement state
- [x] both Svelte and GPUI can consume the same snapshot semantics

## Next Task

Open `g11.004` and add the strip rail family.
