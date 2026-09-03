---
title: g16.069 Nucleus AppHeader M1 worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-034145-g16-069-nucleus-app-header-receipt.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, app-header, receipt]
---

## What This Thread Will Do

Implement only `g16.069` from
`docs/roadmaps/g16/069-nucleus-app-header-m1.md`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Worker branch: `feature/g16-069-nucleus-app-header-receipt`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-069-nucleus-app-header-receipt`
- Required sibling worktree links: none
- Card: `docs/roadmaps/g16/069-nucleus-app-header-m1.md`
- Integration ownership: orchestrator owns receipt-source merge ordering and
  all g16 front-door closeout edits
- Worker class: day-to-day; one bounded shell-header fixture and receipt with no
  public API or architecture decision
- Frontier-worker justification: none
- PR URL: pending; never merge

## Boundaries

In scope: one production AppHeader mounted fixture over the already-proven Icon
and Text dependencies, exact metadata/structure assertions, one M1 receipt,
exact evidence refresh, and a focused repair only when a biting mounted
counterexample requires it.

Out of scope: Nucleus source/data, pixels/screenshots, A1/V1/V2, public APIs,
web behavior, responsive-web proof, native window dragging, Jetstream,
releases/workflows, and local windowed/native-visual selectors.

## Important Context

Read AGENTS, the card, g16.062, g16.067, g16.068, the Nucleus manifest/ledger
and receipt emitter, plus AppHeader/Icon/Text contracts and renderers before
editing. Headless metadata and mounted layout are M1, not decoded pixels or AT
proof. The actual Nucleus shell explains why the component is in the cohort,
but no Nucleus fixture or app-specific value enters Poodle.

## Completion Protocol

1. Use only production renderers for AppHeader, Icon, and Text. A raw layout
   container may group slot children; it cannot stand in for a component.
2. Prove default/custom identity and absent/present center structure, exact
   size/density/token metadata, mounted containment, and root inertness.
3. Dispatch harmless test-platform pointer input without inventing behavior.
4. Emit the AppHeader receipt only after all assertions. Commit runtime/test
   changes before falsification, run the native receipt selector at that exact
   commit, then refresh all receipts, manifest, ledger, card, and one log.
5. Run the card boards; never run windowed or native-visual selectors.
6. Push one PR and return URL, exact head/runtime source SHA, receipt,
   falsifications, validation, and limits. Never merge or edit shared front
   doors.
