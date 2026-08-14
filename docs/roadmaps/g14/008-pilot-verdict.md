# g14.008 — Pilot Verdict

Status: planned
Depends on: `g14.001`–`g14.007`, `g14.023`

## Outcome

Record **adopt**, **revise**, or **reject** from measured evidence. Do not
open corpus rollout by momentum.

## Evidence

- full mechanism LOC and per-component marginal cost
- duplicate declarations and executable fixtures/vectors removed
- curated specimen cost and usefulness assessed separately from case coverage
- defects existing gates missed and the new system caught
- schema/action/assertion growth by profile
- active-cohort execution time and reliability
- estimated Jetstream admission cost and any GPUI leakage in the node boundary
- primitive/backend gaps fixed and still missing
- component-specific branches in generic tooling; target zero
- standing gate integration and planted-failure results
- worker/operator friction and snapshot review quality

## Verdict Rules

Adopt only if one pipeline covers all six profiles across Svelte, React, and
GPUI, required active-runtime absence remains red, generic tooling stays
component-neutral, the Rust boundary remains backend-neutral, and ongoing cost
is lower than the duplication removed. Revise only with a bounded correction
and a named second verdict. Reject removes pilot machinery while retaining
component fixes and evidence.

## Acceptance

- Verdict and evidence live in one roadmap artifact.
- Architecture/spec status changes match the verdict.
- Every experimental surface has a disposition.
- Exactly one next task is ready; blocked rollout never opens implicitly.

## Validation

Run `effigy conformance:complete` and the complete headless conformance board,
docs gates, `git diff --check`, and verify all figures from source rather than
worker summaries. No foreground conformance selector is permitted.
