---
title: g16.070 Nucleus SplitView M1 worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-041500-g16-070-nucleus-split-view-receipt.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, split-view, receipt]
---

## What This Thread Will Do

Implement only `g16.070` from
`docs/roadmaps/g16/070-nucleus-split-view-m1.md`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Worker branch: `feature/g16-070-nucleus-split-view-receipt`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-070-nucleus-split-view-receipt`
- Required sibling worktree links: none
- Card: `docs/roadmaps/g16/070-nucleus-split-view-m1.md`
- Integration ownership: orchestrator owns receipt-source merge ordering and
  all g16 front-door closeout edits
- Worker class: day-to-day; one bounded existing-fixture strengthening and
  receipt with no public API or architecture decision
- Frontier-worker justification: none
- PR URL: pending; never merge

## Boundaries

In scope: strengthen the exact existing two-SplitView mounted regression; prove
production structure, independent divider identity, real keyboard resize
dispatch, one production CollapseToggle/Icon path, one M1 receipt, and exact
evidence refresh. Make a focused repair only when a biting mounted
counterexample requires it.

Out of scope: Nucleus source/data, pixels/screenshots, A1/V1/V2, public APIs,
web behavior, ratio synthesis from guessed extent, wholesale SplitView parity
repair, Jetstream, releases/workflows, and local windowed/native-visual
selectors.

## Important Context

Read AGENTS, the card, g16.062, g16.067, g16.069, the Nucleus manifest/ledger
and receipt emitter, plus SplitView/ResizeHandle/CollapseToggle/Icon contracts
and renderers before editing. Preserve the manifest's exact test name. The
native callback reports `ResizePhase` plus axis pixel delta because the host
owns rendered extent; this receipt must not relabel that documented boundary as
ratio parity. Headless metadata and mounted layout are M1, not decoded pixels
or AT proof.

## Completion Protocol

1. Strengthen the existing two-instance regression; do not create a duplicate
   SplitView fixture unless an unavoidable harness limitation is documented.
2. Use production renderers for SplitView and its CollapseToggle/Icon seam.
3. Dispatch resize and collapse input through the mounted test platform. Prove
   exact handler isolation and that direct invocation cannot satisfy the test.
4. Emit the SplitView receipt only after all assertions. Commit runtime/test
   changes before falsification, run the native receipt selector at that exact
   commit, then refresh all receipts, manifest, ledger, card, and one log.
5. Run the card boards; never run windowed or native-visual selectors.
6. Push one PR and return URL, exact head/runtime source SHA, receipt,
   falsifications, validation, and limits. Never merge or edit shared front
   doors.
