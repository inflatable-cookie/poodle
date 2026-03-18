# g11.009 — Hosted External-Surface And Plugin-Editor Container Contract

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.008
Primary repos: `pug`

## Goals

- [x] add a generalized host container for external or embedded foreign
  surfaces
- [x] avoid folding plugin workflow semantics into Pug

## Execution Checklist

- [x] define hosted-surface container identity and title posture
- [x] define focus and active state
- [x] define embedded versus detached hosting relationship
- [x] define bounded host states:
  - [x] ready
  - [x] loading
  - [x] unavailable
  - [x] blocked
  - [x] degraded
- [x] explicitly keep product workflow and plugin management semantics
  downstream
- [x] produce contract file in `docs/contracts/workstation/` following the
  12-section template

## Deliverables

- `docs/contracts/workstation/hosted-surface.md` — 12-section contract for
  HostedSurface component

## Acceptance Criteria

- [x] a generalized hosted external-surface contract exists
- [x] the contract is usable for plugin/editor-like surfaces without naming a
  specific downstream workflow

## Next Task

Open `g11.010` and begin the first Svelte implementation batch.
