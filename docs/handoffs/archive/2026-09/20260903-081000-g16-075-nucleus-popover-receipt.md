---
title: g16.075 Nucleus Popover M1 worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-081000-g16-075-nucleus-popover-receipt.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, popover, receipt]
---

## What This Thread Will Do

Implement only `g16.075` from
`docs/roadmaps/g16/075-nucleus-popover-m1.md`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Worker branch: `feature/g16-075-nucleus-popover-receipt`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-075-nucleus-popover-receipt`
- Required sibling worktree links: none
- Card: `docs/roadmaps/g16/075-nucleus-popover-m1.md`
- Integration ownership: orchestrator owns receipt-source merge ordering and
  all g16 front-door closeout edits
- Worker class: day-to-day; one retained mounted overlay fixture and receipt,
  with focused native repair only when a mounted counterexample bites
- Frontier-worker justification: none
- PR URL: pending; never merge

## Boundaries

In scope: strengthen the retained nested Popover fixture; real Popover
`IntoElement` adapter; controlled outer/inner and sibling instances; exact
trigger/surface/token/geometry metadata; mounted toggle, Escape, outside,
disabled, focus-entry, focus-restoration, and rebuild behavior; one M1 receipt;
exact cohort refresh. A focused shared Rust/backend/GPUI repair is allowed only
when the committed mounted proof demonstrates the missing behavior.

Out of scope: Nucleus source/data, web changes, public APIs, A1 accessibility
tree, browser portal or collision parity, V1/V2 pixels, Jetstream,
releases/workflows, and local windowed/native-visual selectors.

## Important Context

Read AGENTS, the card, g16.062, g16.068, g16.074, the Nucleus manifest/ledger
and receipt emitter, architecture 002, plus the Popover and Surface contracts,
machine, renderer, adapter, floating-overlay path, dismiss stack, focus queue,
and retained mounted regression before editing. The current test proves only
that nested deferred painting does not panic; it is not yet adapter, lifecycle,
identity, focus, or receipt evidence. Keep app labels generic.

## Completion Protocol

1. Keep the exact manifest test name. Mount
   `node_compat::Popover::from_spec(...).into_element()` through the element
   HeadlessDriver factory; do not stop at a rendered Node.
2. Commit the expanded proof/counterexample before repair. Use caller-scoped
   identities and host-owned open state. Drive all behavioral claims through
   mounted pointer/key input.
3. Prove nested inner-first dismissal, sibling isolation, disabled inertia,
   exact focus strategies/restoration, placement/offset geometry, and
   contract-owned surface metadata. Keep A1/V1 claims excluded.
4. Commit runtime changes before falsification. Plant representative oracle
   counterexamples, record exact failures, restore from committed source, and
   rerun.
5. Emit only after terminal assertions. Run the native receipt selector at the
   exact runtime commit, then refresh every receipt, manifest, ledger, card,
   and one log.
6. Run the card boards. Never run windowed or native-visual selectors.
7. Push one PR and return URL, exact head/runtime source SHA, receipt,
   falsifications, validation, and limits. Never merge or edit shared front
   doors.
