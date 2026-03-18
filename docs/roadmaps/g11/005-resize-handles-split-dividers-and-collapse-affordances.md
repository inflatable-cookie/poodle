# g11.005 — Resize Handles, Split Dividers, And Collapse Affordances

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.004
Primary repos: `pug`

## Goals

- [x] add the interaction primitives needed for workstation layout adjustment
- [x] remove the need for renderer-local ad hoc resize and collapse behavior

## Execution Checklist

- [x] define resize handle primitives for horizontal and vertical splits
- [x] define split divider posture and hit-target semantics
- [x] define collapse/expand affordances for docks and regions
- [x] specify snapshot changes produced by resize and collapse actions
- [x] verify keyboard/focus implications are documented
- [x] produce contract files in `docs/contracts/workstation/` following the
  12-section template

## Deliverables

- `docs/contracts/workstation/resize-handle.md` — standalone resize primitive
- `docs/contracts/workstation/split-divider.md` — divider with resize and
  optional collapse buttons
- `docs/contracts/workstation/collapse-affordance.md` — standalone
  collapse/expand trigger

## Acceptance Criteria

- [x] resize, divider, and collapse semantics are documented in one place
- [x] layout mutation semantics are explicit and renderer-agnostic
- [x] downstream consumers no longer need to invent basic workstation layout
  affordances from scratch

## Next Task

Open `g11.006` and deepen dock behavior.
