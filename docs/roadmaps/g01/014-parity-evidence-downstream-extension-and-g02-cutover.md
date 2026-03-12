# g01.014 Parity Evidence, Downstream Extension, And g02 Cutover

Status: completed
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.005, g01.006, g01.007, g01.008, g01.009, g01.010, g01.011, g01.012, g01.013
Primary repos: `pug`, downstream consumers

## Context

`g01` should not close with only component ideas. It needs explicit rules for
how parity is evidenced and how downstream repos extend Pug without bloating
it.

## Goals

- [x] define parity evidence expectations for Svelte and GPUI
- [x] define documented-delta rules
- [x] define downstream extension rules for Loophole and future apps
- [x] define what qualifies Pug to move into broader composite and adoption work

## Execution Checklist

- [x] define the minimum parity proof required for shared components
- [x] define how intentional framework deltas are documented and approved
- [x] define the downstream extension contract for app-specific systems
- [x] summarize the stable `g01` surface that `g02` can build on

## Acceptance Criteria

- [x] parity evidence rules are explicit
- [x] downstream extension contract is explicit
- [x] `g02` starts from a stable enough baseline instead of reopening `g01`
  questions

## Deliverables

- [x] parity evidence baseline
- [x] downstream extension contract
- [x] `g01` closeout summary

## Next Task

Open `g02.001` and begin the richer composite suite with forms and validation
system depth.
