---
title: g16.068 Nucleus Text and Surface M1 worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-031500-g16-068-nucleus-text-surface-receipts.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, text, surface, receipt]
---

## What This Thread Will Do

Implement only `g16.068` from
`docs/roadmaps/g16/068-nucleus-text-surface-m1.md`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Worker branch: `feature/g16-068-nucleus-text-surface-receipts`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-068-nucleus-text-surface-receipts`
- Required sibling worktree links: none
- Card: `docs/roadmaps/g16/068-nucleus-text-surface-m1.md`
- Integration ownership: orchestrator owns receipt-source merge ordering and
  all g16 front-door closeout edits
- Worker class: day-to-day; bounded styled-only mounted fixture and evidence
  work with no public API or architecture decision
- Frontier-worker justification: none
- PR URL: pending; never merge

## Boundaries

In scope: one production-path Text-in-Surface mounted composite, separate M1
receipts, exact source/lock refresh, and focused repair only when a biting
counterexample requires it.

Out of scope: pixels/screenshots, A1/V1/V2, Nucleus source, public APIs, web
behavior, Jetstream, releases/workflows, and local windowed/native-visual
selectors.

## Important Context

Read AGENTS, the card, `g16.062`, `g16.067`, the Nucleus manifest/ledger and
receipt emitter, plus Text and Surface contracts/renderers before editing.
The g16.067 review narrowed headless Icon claims; apply the same discipline:
Node metadata and mounted layout are M1, not decoded pixels or AT proof.

## Completion Protocol

1. Build the fixture only through `poodle_render::text` and
   `poodle_render::surface`; retain exact component-specific assertions.
2. Dispatch harmless test-platform pointer input without inventing behavior
   for styled-only primitives. Prove both remain outside the focus chain.
3. Place both receipt emissions at the terminal boundary after all assertions.
4. Commit the runtime/test change before falsification. Run the real native
   selector at that exact commit, then refresh all receipts, manifest, ledger,
   card, and one log in an evidence commit.
5. Run the card boards; never run windowed or native-visual selectors.
6. Push one PR and return its URL, exact head/runtime source SHA, receipts,
   falsifications, validation, and limits. Never merge or edit shared front
   doors.
