---
title: g18.033 GPUI FormDialog render convergence
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: merged
owner: Poodle Northstar orchestrator
created: 2026-09-13
updated: 2026-09-13
merged_pr: 268
merged_commit: 71788758102854d665d2be35b93a9ec4aa5ee3d9
review_comment: 5653871766
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom explicitly rejected the g18.032 closeout on 2026-09-13 because the GPUI hang remains and required the underlying problem to be fixed before moving on."
queue:
  capability: complex
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, g18, g18.033, gpui, form-dialog, validation]
---

## What This Thread Was Doing

Execute [`g18.033`](../roadmaps/g18/033-gpui-form-dialog-render-convergence.md).
Repair the renderer/layout non-termination exposed by g18.032, then finish the
full-board proof that g18.032 could not complete.

## Why It Matters

The timeout machinery now prevents another hours-long silent process, but the
required GPUI specimen leaf still cannot finish. Calling the infrastructure
task complete while leaving its blocking leaf red confused containment with a
fix. Poodle needs both: bounded execution and a healthy complete board.

## Current State

Main contains g18.032 and is clean at the planning commit carrying this
handoff. The final board passed 54/68 units before the five-minute child bound
killed `probe:gpui-specimens`. An exact shard reaches `form-dialog` and never
returns from its initial draw. A process sample points into GPUI flex layout
and node-backend state patches. The draw hangs before `settle` or the probe's
body timer can execute. The submitting example is present on the production
specimen route; the earlier claim that its spinner is causal is not yet proof.

## Boundaries

Follow g18.033 exactly. You may change the focused GPUI preview specimen/probe,
shared Rust FormDialog/Button/Spinner composition, and the causal GPUI
node-backend implementation plus focused tests. Do not alter public component
semantics without stopping for Chatterbox. Do not remove or special-case the
route, submitting state, Sizes/Densities interactions, any canonical route, or
the 175 denominator. Do not raise timeouts, ignore tests, invoke windowed
selectors, touch npm/release state, or change web implementations.

The validation budget is strict. Compile the focused binary once. Diagnostic
route subprocesses are capped at thirty seconds and must name progress; use no
full board while diagnosing. After the causal regression and repair, run the
exact route, the complete specimen selector once, and one final `qa:board`.
Do not stack `ci:rust`, `qa`, or equivalent broad selectors around it.

## Important Context

Read g18.032's execution-log section “Root cause of the specimen hang” and the
g18.033 acceptance table before editing. The first draw itself is stuck, so a
timer checked after draw cannot solve this. The existing
`production_loading_routes_commit_before_starting_full_mode_loops` law shows
the intended loading lifecycle for ordinary Spinner/Skeleton routes; preserve
that behavior if animation scheduling is implicated.

Build a regression that cannot hang the parent test indefinitely. Prefer a
bounded child-process exact-route law or a lower-level convergence assertion
that reproduces the causal invalidation loop. A green timeout wrapper alone is
not a repair. The final full board must be wholly green; reporting the named
timeout is no longer an accepted completion outcome.

## Suggested Next Move

Build the preview test binary without execution, then run only the exact
`form-dialog` route in a thirty-second child. Reduce the mounted node tree or
instrument backend phases until the operation that never converges is proven.
Plant the bounded regression before changing behavior.

## Completion Protocol

Push one clean non-draft PR from the Queue branch. Record exact focused-route,
175-route selector and single final-board timings in one execution log. Run
`git diff --check`. Report `ready_for_review` without polling GitHub; Queue
owns CI, independent review, merge and closeout. A timeout, skipped route,
partial board or removed submitting example is `blocked`, not completion.
