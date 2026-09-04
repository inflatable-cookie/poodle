---
title: g16.099 React prop port tranche worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-04
updated: 2026-09-04
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260904-160000-g16-099-react-prop-port-tranche.md
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.099]
---

## What This Thread Was Doing

This handoff dispatches the Chatterbox-promoted g16.099 React parity tranche.
The current manifest and card are the authority; do not infer extra ports.

## Current State

- **Repository:** `inflatable-cookie/poodle`
- **Planning base:** `bed764f8452689ff18e513461401e744ae5d7f7e` (manifest revision 4 is an ancestor of origin/main)
- **Planning checkout:** clean; `HEAD == origin/main`
- **Worker branch:** `feature/g16-099-react-prop-port-tranche`
- **Active card:** `docs/roadmaps/g16/099-react-prop-port-tranche.md`
- **Allowed runway:** g16.099 only
- **Dispatch topology:** concurrent with g16.098; no path overlap
- **Required sibling links:** none
- **Owned paths:** the five named React shells and their tests, removals only from the merged g16.095 `BASELINE`, this lane's log, and append-only `PAPERCUTS.md`
- **Reserved coordinator paths:** `docs/roadmaps/g16/README.md`, `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`
- **Review oracle:** every row in the g16.099 card
- **Capability:** capable coding model, medium reasoning

## Boundaries

Port exactly the pending entries: Button `formenctype`, `formmethod`, and
`style`; Calendar `today`; SplitView `divider`; AppHeader `element` in the
card-approved React form; and DockRegion `showCollapseToggle`. Remove only
the corresponding cleared baseline entries. Leave framework-idiom and
needs-decision entries untouched. Do not edit Svelte, contracts beyond the
single AppHeader runtime note if required, workflows, release surfaces, or
merge the PR.

## Required Validation

Run the focused React tests, `effigy docs:react-prop-drift`, `effigy ci:web`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Plant a
reverted prop and prove the gate fails; prove the baseline shrinks and defaults
match. Record the execution log and open a reviewable PR.

## Completion Protocol

Verify the launcher worktree and pushed handoff before broad reads. Read
`AGENTS.md`, the g16 front door, the card, and canonical refs. Falsify every
Review Oracle row, push the branch, and report the exact head. The coordinator
owns exact-head review, merge, and reserved closeout.
