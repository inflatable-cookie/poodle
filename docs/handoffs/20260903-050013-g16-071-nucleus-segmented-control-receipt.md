---
title: g16.071 Nucleus SegmentedControl M1 worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
handoff: single-file-path-only
status: ready
owner: Poodle Northstar orchestrator
created: 2026-09-03
updated: 2026-09-03
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-050013-g16-071-nucleus-segmented-control-receipt.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, segmented-control, receipt]
---

## What This Thread Will Do

Implement only `g16.071` from
`docs/roadmaps/g16/071-nucleus-segmented-control-m1.md`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Worker branch: `feature/g16-071-nucleus-segmented-control-receipt`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-071-nucleus-segmented-control-receipt`
- Required sibling worktree links: none
- Card: `docs/roadmaps/g16/071-nucleus-segmented-control-m1.md`
- Integration ownership: orchestrator owns receipt-source merge ordering and
  all g16 front-door closeout edits
- Worker class: day-to-day; strengthen one existing mounted regression and add
  one receipt without public API or architecture decisions
- Frontier-worker justification: none
- PR URL: pending; never merge

## Boundaries

In scope: strengthen the exact existing SegmentedControl mounted regression;
prove production track/segment metadata, exclusive and roving semantics, real
pointer/keyboard input, controlled rebuild, disabled inertia, two-instance
identity, one M1 receipt, and exact evidence refresh. Make a focused repair
only when a biting mounted counterexample requires it.

Out of scope: Nucleus source/data, icons or icon-only variants, pixels,
screenshots, A1/V1/V2, public APIs, web hidden-radio behavior, Jetstream,
releases/workflows, and local windowed/native-visual selectors.

## Important Context

Read AGENTS, the card, g16.062, g16.070, the Nucleus manifest/ledger and receipt
emitter, plus the SegmentedControl contract/spec/renderer before editing.
Preserve the manifest's exact test name. The cohort use is a plain labelled
three-option controlled filter; do not copy Nucleus fixture data into Poodle.
Headless Node semantics are M1 metadata, not accessibility-tree A1 proof.

## Completion Protocol

1. Strengthen the existing regression; do not create a duplicate fixture.
2. Use the production renderer and mounted test-platform input for every
   behavioral claim. Keep direct-callback tests below the M1 evidence boundary.
3. Pin exact contract-owned metadata and prove two-instance focus and callback
   isolation.
4. Emit the receipt only after all assertions. Commit runtime/test changes
   before falsification, run the native receipt selector at that exact commit,
   then refresh all receipts, manifest, ledger, card, and one log.
5. Run the card boards; never run windowed or native-visual selectors.
6. Push one PR and return URL, exact head/runtime source SHA, receipt,
   falsifications, validation, and limits. Never merge or edit shared front
   doors.
