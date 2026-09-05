---
title: g16.098 cold-checkout web board repair worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-04
updated: 2026-09-04
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260904-150000-g16-098-cold-checkout-web-board-repair.md
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.098]
---

## What This Thread Was Doing

This is the coordinator's dispatch of the Chatterbox-approved g16.098 repair
lane. Read the current card and manifest; do not infer additional work.

## Why It Matters

Fresh runners fail three React preview suites because the project misses the
workspace alias and web package builds run after component tests. This blocks
the automatic web board, g16.096, and release recertification.

## Current State

- **Repository:** `inflatable-cookie/poodle`
- **Planning base:** `3dbd1cabd1ca504d9744e948a0b4079b0b865eea`
- **Planning checkout:** clean; `HEAD == origin/main`; promoted commit is an ancestor
- **Worker branch:** `feature/g16-098-cold-checkout-web-board-repair`
- **Active card:** `docs/roadmaps/g16/098-cold-checkout-web-board-repair.md`
- **Allowed runway:** g16.098 only
- **Dispatch topology:** concurrent with g16.095 revision; serial predecessor of g16.096 and g16.097
- **Required sibling links:** none
- **Owned paths:** `vitest.config.ts`; the `ci:web` sequence in `tasks/effigy.tasks.toml`; one cold-path proof under `test/` or `scripts/web-distribution/`; one execution log; append-only `PAPERCUTS.md`
- **Reserved coordinator paths:** `docs/roadmaps/g16/README.md`, `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`
- **Review oracle:** all rows in the g16.098 card, including planted alias failure, cold and warm board passes, and unchanged scope
- **Capability:** capable coding model, medium reasoning

## Boundaries

Add the React preview workspace alias, move both web package builds before
`test:components` while preserving the install test after builds, and commit a
bite-sized cold-checkout proof. Do not change workflows, package exports,
components, release surfaces, or run release/windowed/native-visual selectors.
Do not merge the PR.

## Required Validation

Run the card's cold detached-worktree proof, `effigy ci:web` on cold and warm
paths, `effigy docs:check`, and `git diff --check origin/main...HEAD`. Record
the pre-fix failure and post-fix evidence in the log. Stop for any failure
outside alias/order scope and report it to the coordinator.

## Completion Protocol

Verify the launcher worktree and pushed handoff before broad reads. Read
`AGENTS.md`, the g16 front door, the card, and canonical refs. Falsify every
Review Oracle row, push the branch, and open a reviewable PR. The coordinator
owns exact-head review, merge, and reserved closeout.
