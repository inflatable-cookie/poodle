---
title: g16.067 Nucleus Icon and IconButton M1 worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-025000-g16-067-nucleus-icon-receipts.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, icon, receipt]
---

## What This Thread Will Do

Implement only `g16.067` from
`docs/roadmaps/g16/067-nucleus-icon-icon-button-m1.md`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Worker branch: `feature/g16-067-nucleus-icon-receipts`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-067-nucleus-icon-receipts`
- Required sibling worktree links: none
- Card: `docs/roadmaps/g16/067-nucleus-icon-icon-button-m1.md`
- Integration ownership: orchestrator owns receipt-source merge ordering and
  all g16 front-door closeout edits
- Worker class: day-to-day; bounded mounted fixture and evidence work with no
  new architecture or public API
- Frontier-worker justification: none
- PR URL: pending; never merge

## Boundaries

In scope: production-path mounted proofs and `M1` receipts for Icon and
IconButton, IconProvider as setup, exact source/lock receipt refresh, focused
repairs only if a biting counterexample requires them.

Out of scope: Nucleus source, A1/V1/V2, visual judgment, public APIs, web
behavior, Jetstream, releases/workflows, and local windowed/native-visual
selectors.

## Important Context

Read AGENTS, the card, `g16.062`, the Nucleus programme/manifest/ledger,
Icon/IconButton contracts, receipt emitter, and existing mounted IconButton
test before editing. Commit the coherent runtime/test change before any
falsification plant. Emit receipts only from the real native regression run at
that exact source commit; then add the evidence/log commit.

## Completion Protocol

1. Add one named production-path mounted Icon proof. Keep IconProvider a
   non-rendered setup prerequisite.
2. Retain and strengthen the existing IconButton mounted scenario only as
   needed to meet the card; do not invent another interaction machine.
3. Add receipt emission to the executed tests, run the real selector, and
   refresh the manifest, Button receipt, new receipts, and ledger from the
   exact committed source.
4. Falsify the oracle rows, restore from a committed baseline, and record the
   exact failures without widening scope.
5. Run the card's focused and broad headless boards. Never run a windowed or
   native-visual selector.
6. Push one PR and return its URL, exact head/runtime source SHA, receipts,
   falsifications, validation, and limits. Never merge or edit shared front
   doors.
