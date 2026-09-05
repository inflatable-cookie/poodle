---
title: g16.101 Tree item accessible name worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-04
updated: 2026-09-04
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260904-170000-g16-101-tree-item-accessible-name.md
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.101]
---

## What This Thread Was Doing

Dispatch the Chatterbox-approved g16.101 Tree accessible-name repair.
The card and current dispatch manifest are authoritative.

## Current State

- **Repository:** `inflatable-cookie/poodle`
- **Planning base:** `fbf930285160ebd8da659b09b8b4ae26e4b106cd`; promoted manifest revision 6 is an ancestor of origin/main
- **Planning checkout:** clean; `HEAD == origin/main`
- **Worker branch:** `feature/g16-101-tree-item-accessible-name`
- **Active card:** `docs/roadmaps/g16/101-tree-item-accessible-name.md`
- **Allowed runway:** g16.101 only
- **Dispatch topology:** concurrent with g16.100, g16.102, g16.096, and g16.097; no path overlap
- **Required sibling links:** none
- **Review oracle:** every row in the g16.101 card
- **Capability:** capable coding model, medium reasoning
- **Reserved coordinator paths:** `docs/roadmaps/g16/README.md`, `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`

## Boundaries

Set explicit `aria-label` names from visible node/loading/placeholder text in
Svelte and React, add the single contract accessibility line, and prove named
rows and rename updates. Keep the flat windowed hierarchy; do not touch drag,
selection, keyboard behavior, or GPUI. Do not merge.

## Required Validation

Run focused Svelte/React tests, `effigy test:a11y`, `effigy ci:web`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Stop for a
consumer `aria-labelledby` conflict or rename-input name collision.

## Completion Protocol

Verify the launcher worktree and pushed handoff before broad reads. Read
`AGENTS.md`, the g16 front door, the card, and canonical refs. Push a reviewable
PR with the exact head and evidence; the coordinator owns review, merge, and
reserved closeout.
