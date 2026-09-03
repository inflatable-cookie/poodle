---
title: g16.074 Nucleus Dialog M1 worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-074500-g16-074-nucleus-dialog-receipt.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, dialog, receipt]
---

## What This Thread Will Do

Implement only `g16.074` from
`docs/roadmaps/g16/074-nucleus-dialog-m1.md`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Worker branch: `feature/g16-074-nucleus-dialog-receipt`
- Worker worktree: launcher-provided Paseo worktree; planned slug
  `g16-074-nucleus-dialog-receipt`
- Required sibling worktree links: none
- Card: `docs/roadmaps/g16/074-nucleus-dialog-m1.md`
- Integration ownership: orchestrator owns receipt-source merge ordering and
  all g16 front-door closeout edits
- Worker class: day-to-day; one mounted modal fixture and receipt, with focused
  native repair only when the mounted counterexample bites
- Frontier-worker justification: none
- PR URL: pending; never merge

## Boundaries

In scope: one production Dialog fixture; generic title/description/body/actions;
production Surface and Button composition; exact modal tokens/layout; mounted
close, backdrop, Escape, cancel, confirm, refusal, and controlled host rebuild;
one M1 receipt; exact evidence refresh. A focused shared Rust/backend/GPUI
compatibility repair is allowed only when the mounted proof demonstrates the
missing behavior.

Out of scope: Nucleus source/data, web behavior, public APIs, A1 focus-trap or
accessibility-tree authority, V1/V2 pixels, browser body-scroll policy, nested
overlay design, AlertDialog/FormDialog, Jetstream, releases/workflows, and
local windowed/native-visual selectors.

## Important Context

Read AGENTS, the card, g16.062, g16.068, g16.073, the Nucleus manifest/ledger
and receipt emitter, plus the Dialog, Surface, and Button contracts/specs and
renderers before editing. Nucleus uses small titled dialogs with a close button
and explicit actions. Keep labels/data generic. Current native Dialog has a
custom GPUI backdrop adapter; test Escape and backdrop policy independently
rather than assuming the web contract reaches it. Do not infer A1 focus
semantics from M1 mounted input.

## Completion Protocol

1. Add the exact named fixture and update only the Dialog manifest row's test.
2. Commit the initial proof/counterexample before repair. Build through the
   production renderers and dispatch every behavioral claim through the
   mounted test platform.
3. Preserve host-owned open state: accepted close rebuilds without Dialog;
   refusal keeps it mounted. Keep dismissal policy axes independent.
4. Commit final runtime/test changes before falsification. Plant representative
   review-oracle counterexamples, record observed failures, restore from the
   committed source, and rerun.
5. Emit the receipt only after all assertions. Run the native receipt selector
   at the exact runtime commit, then refresh all receipts, manifest, ledger,
   card, and one log.
6. Run the card boards; never run windowed or native-visual selectors.
7. Push one PR and return URL, exact head/runtime source SHA, receipt,
   falsifications, validation, and limits. Never merge or edit shared front
   doors.
