# g13.001 — Jetstream Workstation Feasibility Audit Against g11 Contracts

Status: planned
Owner: Pug Core
Updated: 2026-03-17
Depends on: g12.007
Primary repos: `pug`

## Goals

- [ ] classify every g11 workstation contract against Jetstream rendering
  constraints
- [ ] identify components that need native adaptation versus intentional
  exclusion

## Execution Checklist

- [ ] review each workstation contract from `docs/contracts/workstation/`
- [ ] classify each component as:
  - [ ] fully supported
  - [ ] supported with documented native adaptation
  - [ ] intentionally excluded (with rationale)
- [ ] review Jetstream rendering constraints (retained-mode UiTree, flexbox-only
  layout, solid colors, no gradients, no transforms, limited text rendering)
- [ ] produce a prioritized implementation plan for 002–003

## Acceptance Criteria

- [ ] every g11 workstation contract has an explicit Jetstream feasibility
  classification
- [ ] adaptations and exclusions have documented rationale
- [ ] implementation work is scoped for 002 and 003

## Next Task

Open `g13.002` and begin Jetstream workstation implementation batch 1.
