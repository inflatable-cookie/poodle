---
title: g16.060 Tabs controlled-panel focus worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-160500-g16-060-tabs-controlled-focus.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, tabs, focus, Papercuts]
---

## Assignment

Implement only `g16.060` from
`docs/roadmaps/g16/060-tabs-controlled-panel-focus-transfer.md`. Add the paired
web Tabs controlled-value focus policy. Do not edit Figmatic.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Required base: pushed current `origin/main`
- Worker class: day-to-day implementation. The API and focus law are frozen;
  this is bounded paired-shell lifecycle work, not frontier architecture.
- Public seam: `focusOnValueChange="preserve" | "selected-tab"`, default
  `"preserve"`
- Consumer evidence:
  https://github.com/inflatable-cookie/figmatic/pull/69#issuecomment-5514814268
- Integration ownership: orchestrator owns exact-head review, merge, roadmap
  closeout, and the receipt returned to Figmatic

## Boundary

Implement every card acceptance row. Capture whether focus was inside the
outgoing selected panel before a controlled value change can unmount it; after
render, focus only the latest newly selected enabled tab. Preserve default
behaviour and every existing Tabs subsystem.

Do not add panel initial-focus callbacks, IconButton autofocus, exported focus
handles, consumer selectors, Figmatic code, Rust/GPUI/Jetstream changes,
versions, release files, or workflows.

## Required Proof

- Paired Svelte and React lifecycle tests for eligible transfer, default
  inertness, outside focus, already-tab focus, missing/disabled destination,
  supersession, and teardown.
- One mounted consumer-shaped async controlled change from Components to Tree
  with focus in an outgoing interactive descendant.
- Plant the pre-fix/no-transfer behaviour from a committed proof, observe the
  named failure, restore, and rerun green.
- Run focused Tabs tests, public type/export proof as needed, drift checks,
  `effigy ci:web`, `effigy docs:check`, and range diff check. Never run
  windowed or release selectors.

## Completion Protocol

1. Use the launcher-provided clean non-main worktree. Do not create another.
2. Read the card, Tabs contract, both web implementations, and focused tests.
3. Work in one coherent behaviour-and-proof batch; avoid unrelated cleanup.
4. Update the card and one September execution log with truthful receipts.
5. Push one branch and open one PR against `main`. Never merge.
6. Return PR URL, exact head, public API/imports, falsification, validation,
   and Figmatic link/build guidance.

## Next Move

Trace controlled-value reconciliation and owned panel/tab refs in both shells,
then write the paired failing lifecycle tests before changing the focus path.

