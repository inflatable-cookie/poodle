---
title: g16.100 DockRegion showTabs portable worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-04
updated: 2026-09-04
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260904-170000-g16-100-dock-region-show-tabs-portable.md
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.100]
---

## What This Thread Was Doing

Dispatch the Chatterbox-approved g16.100 portable DockRegion capability lane.
The card and current dispatch manifest are authoritative.

## Current State

- **Repository:** `inflatable-cookie/poodle`
- **Planning base:** `fbf930285160ebd8da659b09b8b4ae26e4b106cd`; promoted manifest revision 6 is an ancestor of origin/main
- **Planning checkout:** clean; `HEAD == origin/main`
- **Worker branch:** `feature/g16-100-dock-region-show-tabs-portable`
- **Active card:** `docs/roadmaps/g16/100-dock-region-show-tabs-portable.md`
- **Allowed runway:** g16.100 only
- **Dispatch topology:** concurrent with g16.101, g16.102, g16.096, and g16.097; no path overlap
- **Required sibling links:** none
- **Review oracle:** every row in the g16.100 card
- **Capability:** capable coding model, medium reasoning
- **Reserved coordinator paths:** `docs/roadmaps/g16/README.md`, `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`

## Boundaries

Promote `showTabs` into the portable contract, add the Rust spec/render field,
port React behavior, add the approved GPUI specimen, remove its baseline entry,
and re-kind only the Tree and OrderBy callback entries as framework idiom with
the card's exact reason. Do not change Tabs, tab drag, sizing, or unrelated
Jetstream behavior. Do not merge.

## Required Validation

Run the card's docs, web, contract, render, drift, and diff checks. Prove
`show_tabs=false` emits no tabs while retaining the collapse toggle, React
parity, single contract placement, zero needs-decision baseline entries, and
the re-added svelteOnly ratchet counterexample.

## Completion Protocol

Verify the launcher worktree and pushed handoff before broad reads. Read
`AGENTS.md`, the g16 front door, the card, and canonical refs. Push a reviewable
PR with the exact head and evidence; the coordinator owns review, merge, and
reserved closeout.
