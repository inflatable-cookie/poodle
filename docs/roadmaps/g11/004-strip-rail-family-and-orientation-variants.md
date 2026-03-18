# g11.004 — Strip Rail Family And Orientation Variants

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.003
Primary repos: `pug`

## Goals

- [x] add generalized strip rails for all four edges
- [x] support compact and collapsed strip posture without app-local hacks

## Execution Checklist

- [x] define strip rail component API for top, bottom, left, and right edges
- [x] specify supported states:
  - [x] idle
  - [x] active
  - [x] compact
  - [x] collapsed
- [x] define orientation-aware spacing, borders, icon sizing, and labels
- [x] define interaction affordances that remain generalized
- [x] produce contract file in `docs/contracts/workstation/` following the
  12-section template

## Deliverables

- `docs/contracts/workstation/strip-rail.md` — 12-section contract for
  StripRail component

## Acceptance Criteria

- [x] strip rail contract supports all four orientations
- [x] compact and collapsed variants are explicitly covered
- [x] the component family is reusable outside any single downstream app

## Next Task

Open `g11.005` and add resize, divider, and collapse affordances.
