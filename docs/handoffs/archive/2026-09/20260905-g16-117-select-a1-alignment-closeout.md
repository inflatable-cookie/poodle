---
title: g16.117 Select A1 alignment closeout
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-for-independent-review
base: 4b92c9c5edcbfec5745f280b54d25b7042413687
---

## What This Thread Was Doing

This worker repaired the five Select A1 divergences identified by g16.111,
using Svelte as the semantic reference and keeping the searchable path
unchanged.

## Why It Matters

Select is the first A1 accessibility contract repair in this tranche. An empty
paired receipt proves that the web and GPUI projections agree on role, naming,
state, relationships, and focus order.

## Current State

- The branch is rebased onto current `origin/main` `4b92c9c5e` (manifest revision 18).
- Contract, Svelte, React, GPUI render, and the Svelte A1 snapshot are changed
  only within `g16.117`.
- The A1 Select receipt is generated from reachable implementation commit
  `f1e7032c0` and has `accessibility.diff: []`; the evidence head is
  `be5a83f39`.
- The divergence store at
  `/Users/tom/.paseo/worktrees/1ugbsx1t/worker-g16-117-select-a1-alignment/docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/select`
  is deleted.
- Execution evidence is recorded in
  `/Users/tom/.paseo/worktrees/1ugbsx1t/worker-g16-117-select-a1-alignment/docs/logs/2026-09/20260905-g16-117-select-a1-alignment.md`.
- No PR has been merged; independent exact-head review is still required.
- The focused implementation plus evidence commits are ready to push from
  this worker branch.

## Boundaries

Keep this PR limited to the Select contract, Svelte/React parity, shared Rust
render semantics, A1 snapshot/receipt, divergence cleanup, and the two
closeout records. Do not run windowed selectors, merge, or expand into
searchable behavior or pointer behavior changes.

## Important Context

The governing card is
`/Users/tom/.paseo/worktrees/1ugbsx1t/worker-g16-117-select-a1-alignment/docs/roadmaps/g16/117-select-a1-alignment.md`.
The five decisions are: non-searchable trigger is a button; trigger value text
is omitted; the label names the listbox; options are not tab stops; and the
chevron is a non-focusable decorative pointer affordance. The A1 runner is the
headless `select_a1` test; its receipt is terminal evidence.

## Suggested Next Move

Check the pushed PR at its exact head. Re-run the focused A1 receipt and the
card's headless gates in the review workspace, then inspect the five semantic
decisions against the review oracle.

## Completion Protocol

The worker has committed the focused batch, refreshed the receipt and cohort
identity to the reachable implementation head and current lock digest, and
run the remaining headless checks. It will push one PR and stop.
The orchestrator owns independent exact-head review and merge. The next task
is review only; unresolved risk is limited to review findings or any required
headless gate failure.

Worker/PR flow: preserve this dedicated branch and do not merge locally.
Review lease: independent reviewer must use the pushed exact commit and a
clean checkout.
