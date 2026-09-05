---
title: g16.077 Nucleus TextInput M1 worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-104800-g16-077-nucleus-text-input-receipt.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, text-input, receipt]
---

## What This Thread Will Do

Implement only `g16.077` from
`docs/roadmaps/g16/077-nucleus-text-input-m1.md`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Worker branch: `feature/g16-077-nucleus-text-input-receipt`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-077-nucleus-text-input-receipt`
- Required sibling worktree links: none
- Card: `docs/roadmaps/g16/077-nucleus-text-input-m1.md`
- Integration ownership: orchestrator owns receipt-source merge ordering and
  all g16 front-door closeout edits
- Worker class: day-to-day; one retained mounted TextInput fixture and receipt,
  with focused native repair only when a mounted counterexample bites
- Frontier-worker justification: none
- PR URL: pending; never merge

## Boundaries

In scope: strengthen the retained TextInput fixture; real TextInput
`IntoElement` adapter; controlled value/selection/focus rebuilds; paired field
identity; mounted pointer/keyboard/edit/selection/submit/cancel/blur/clear;
disabled/read-only/placeholder/max-length rules; exact semantic and style
metadata; one M1 receipt; exact cohort refresh. A focused shared Rust/backend/
GPUI repair is allowed only when the committed mounted proof demonstrates the
missing behavior.

Out of scope: Nucleus source/data, web changes, public APIs, multiline, slug
lifecycle, app validation policy, browser-only behavior, A1 accessibility tree,
V1/V2 pixels, OS IME claims, Jetstream, releases/workflows, and local windowed/
native-visual selectors.

## Important Context

Read AGENTS, the card, g16.007, g16.062, the Nucleus manifest/ledger and receipt
emitter, the TextInput contract, spec, edit machine, renderer, adapter, backend,
and retained mounted/backend tests before editing. Preserve g16.007's bounded
claims. The named fixture already proves substantial mounted behavior but is
not yet real adapter or terminal receipt evidence.

## Completion Protocol

1. Keep the exact manifest test name. Mount
   `node_compat::TextInput::from_spec(...).into_element()` through the element
   HeadlessDriver factory; do not stop at a rendered Node.
2. Commit the expanded proof/counterexample before repair. Use caller-owned ids
   and host-owned value/selection/focus. Drive every behavioral claim through
   mounted production input.
3. Prove field isolation, controlled rebuilds, edit/selection/command ordering,
   disabled/read-only/placeholder/scalar-limit behavior, exact metadata, and
   silent teardown. Keep A1, V1, multiline, and OS IME claims excluded.
4. Commit runtime changes before falsification. Plant representative oracle
   counterexamples, record exact failures, restore from committed source, and
   rerun.
5. Emit only after terminal assertions. Run the receipt selector at the exact
   runtime commit, then refresh every receipt, manifest, ledger, card, and log.
6. Run the card boards. Never run windowed or native-visual selectors.
7. Push one PR and return URL, exact head/runtime SHA, receipt,
   falsifications, validation, and limits. Never merge or edit shared front
   doors.
