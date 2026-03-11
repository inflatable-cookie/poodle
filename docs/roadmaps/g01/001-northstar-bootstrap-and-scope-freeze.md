# g01.001 Northstar Bootstrap And Scope Freeze

Status: planned
Owner: Pug Core
Updated: 2026-03-11
Primary repos: `pug`

## Context

Pug is a new repo with no established implementation surface. It needs a clear
execution and documentation baseline before tokens, components, or downstream
bridges are allowed to proliferate.

## Problem

Without a deliberate bootstrap tranche, the repo will drift into framework-
specific decisions, partial package creation, and undocumented assumptions
about what belongs in Pug versus Underlay or downstream app repos.

## Goals

- [ ] establish the Northstar documentation surface as the control plane
- [ ] freeze the project scope and non-goals
- [ ] freeze the package-boundary model at a high level
- [ ] define the planning standard for future generations and milestones
- [ ] define what belongs inside Pug versus downstream extension repos

## Non-Goals

- [ ] no token implementation yet
- [ ] no component implementation yet
- [ ] no adoption work yet

## Execution Checklist

- [ ] finalize `docs/vision/`, `docs/architecture/`, `docs/roadmaps/`,
  `docs/logs/`, `docs/research/`, and `docs/specs/`
- [ ] document the generalized-library boundary and downstream extension
  boundary
- [ ] document the Underlay and Loophole posture clearly enough to constrain
  later work
- [ ] ensure every active section ends with a concrete next task
- [ ] confirm the roadmap surface is dense enough to support real execution

## Acceptance Criteria

- [ ] `docs/` is the unambiguous planning authority
- [ ] repo purpose and non-goals are explicit
- [ ] the architectural ownership split is explicit
- [ ] the roadmap surface is dense enough to guide real work

## Deliverables

- [ ] repo bootstrap docs
- [ ] generation map
- [ ] first active generation with milestone files

## Evidence Requirements

- [ ] clean docs surface with no template residue that obscures repo purpose
- [ ] roadmap generation structure in place

## Next Task

Open `g01.002` and define the canonical token schema that everything else will
depend on.
