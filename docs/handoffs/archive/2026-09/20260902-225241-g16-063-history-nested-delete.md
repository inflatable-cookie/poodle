---
title: g16.063 HistoryCenter nested deletion parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-225241-g16-063-history-nested-delete.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, history-center, parity]
---

## What This Thread Was Doing

Implement only `g16.063` from
`docs/roadmaps/g16/063-history-center-nested-deletion-parity.md`.

## Why It Matters

The TypeScript deletion path updates only a root map while Rust already uses a
recursive helper. Neither runtime has the same nested counterexample, so the
pair can disagree invisibly.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning base commit: `4ffa31345bc94f82c22d64d83e64b3af2613cfe3`
- Worker branch: `fix/g16-063-history-nested-delete`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-063-history-nested-delete`
- Required sibling worktree links: none
- Programme/card: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md` /
  `docs/roadmaps/g16/063-history-center-nested-deletion-parity.md`
- Dispatch topology: parallel with `g16.062`, `g16.064`, and `g16.065`
- Parallel safety: owns HistoryCenter core/headless semantic files only
- Integration ownership: orchestrator owns shared front doors and merge order
- Model capability profile: ordinary/day-to-day paired-runtime implementation
- Frontier-worker justification: none
- PR URL: pending; do not merge

## Boundaries

In scope: one shared nested semantic vector, TypeScript recursive replacement,
Rust proof, focused contract wording if required, card/log evidence.

Out of scope: public props, web-shell UI, rejection meanings, persistence,
Nucleus, lab, release, workflows, Jetstream, and ledger claims.

## Important Context

Read `AGENTS.md`, the programme, card, HistoryCenter contract, both machines,
and current root-level tests. Preserve unrelated branches and exact effect
count/order. Diagnosis is part of the lane; do not assume the proposal's line
numbers remain current.

## Suggested Next Move

Run worker preflight, commit the paired nested counterexample, and show the
TypeScript failure before changing replacement logic.

## Completion Protocol

1. Reuse only the clean launcher worktree and tracked handoff.
2. Prove nested removal, sibling retention, root behavior, and exact
   delete/reload effects in both runtimes. Restore all plants from a committed
   point.
3. Run focused core/headless HistoryCenter tests, paired contract checks,
   `effigy ci:web`, `effigy ci:rust`, `effigy docs:check`, and diff checks.
4. Push one PR and report exact head, falsifications, validation, and limits.
   Never merge or run windowed/release selectors.

- Closeout refs: card and one September log; front doors stay orchestrator-owned.
