---
title: g16.076 Nucleus Select M1 worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-091200-g16-076-nucleus-select-receipt.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, select, receipt]
---

## What This Thread Will Do

Implement only `g16.076` from
`docs/roadmaps/g16/076-nucleus-select-m1.md`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Worker branch: `feature/g16-076-nucleus-select-receipt`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-076-nucleus-select-receipt`
- Required sibling worktree links: none
- Card: `docs/roadmaps/g16/076-nucleus-select-m1.md`
- Integration ownership: orchestrator owns receipt-source merge ordering and
  all g16 front-door closeout edits
- Worker class: day-to-day; one retained mounted Select fixture and receipt,
  with focused native repair only when a mounted counterexample bites
- Frontier-worker justification: none
- PR URL: pending; never merge

## Boundaries

In scope: strengthen the retained two-instance Select fixture; real Select
`IntoElement` adapter; controlled instance state/rebuilds; exact structure,
semantics, token and bounded geometry metadata; mounted pointer, keyboard,
search, selection, clear, dismissal, focus restoration, disablement, stale-
highlight, and instance-isolation behavior; one M1 receipt; exact cohort
refresh. A focused shared Rust/backend/GPUI repair is allowed only when the
committed mounted proof demonstrates the missing behavior.

Out of scope: Nucleus source/data, web changes, public APIs, browser native-
select/portal/collision behavior, A1 accessibility tree, V1/V2 pixels,
Jetstream, releases/workflows, and local windowed/native-visual selectors.

## Important Context

Read AGENTS, the card, g16.062, g16.073, g16.075, the Nucleus manifest/ledger
and receipt emitter, architecture 002, plus Select/Popover contracts, Select
spec/machine/renderer/adapter, overlay/dismiss/focus paths, and both retained
Select mounted regressions before editing. The current two-instance fixture is
renderer-mounted and has no receipt; convert it to real adapter evidence rather
than replacing its behavioral coverage. Keep all labels and values generic.

## Completion Protocol

1. Keep the exact manifest test name. Mount
   `node_compat::Select::from_spec(...).into_element()` through the element
   HeadlessDriver factory; do not stop at a rendered Node.
2. Commit the expanded proof/counterexample before repair. Use caller-scoped
   identities and host-owned state. Drive all behavior through mounted input.
3. Prove paired-instance isolation, controlled rebuilds, disabled and stale-
   highlight inertia, search/caret/freeform behavior, dismissal/restoration,
   exact production structure/tokens, and bounded/overflow geometry. Keep A1
   and V1 excluded.
4. Commit runtime changes before falsification. Plant representative oracle
   counterexamples, record exact failures, restore from committed source, and
   rerun.
5. Emit only after terminal assertions. Run the receipt selector at the exact
   runtime commit, then refresh every receipt, manifest, ledger, card, and one
   log.
6. Run the card boards. Never run windowed or native-visual selectors.
7. Push one PR and return URL, exact head/runtime source SHA, receipt,
   falsifications, validation, and limits. Never merge or edit shared front
   doors.
