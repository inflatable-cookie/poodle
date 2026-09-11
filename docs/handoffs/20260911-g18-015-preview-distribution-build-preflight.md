---
title: g18.015 preview distribution build preflight
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-11
updated: 2026-09-11
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom reproduced the post-PR stale-distribution preview failure on 2026-09-11 and said Go for it after the exact startup-order diagnosis and paired-selector repair proposal."
queue:
  capability: general
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.015, effigy, preview, distribution]
---

## What This Thread Was Doing

Execute [`g18.015`](../roadmaps/g18/015-preview-distribution-build-preflight.md).
Make both public web preview selectors rebuild the required package
distributions before starting Vite.

## Why It Matters

The preview packages consume ignored built distributions. PR #242 correctly
added a core export in source, but Svelte Vite started first and evaluated a
framework build against stale core output, producing a browser missing-export
syntax error. A later build fixed the file but not the already failed startup.

## Current State

`core:build`, `svelte:package`, and `react:package` already encode the required
build order. `svelte:preview` and `react:preview` currently invoke only their
low-level Vite run tasks. The current core distribution now serves the export;
the durable defect is unsafe startup ordering. g18.011 remains Queue-held.

## Boundaries

Follow g18.015 exactly. Compose each public preview selector through its
existing package task before Vite, fail closed on build failure, and prove the
served module graph from isolated missing/stale distributions. Do not mutate
live ignored output during tests, add watcher architecture, edit workflows,
change package APIs, release, or touch Desktop.

## Important Context

TOML inspection alone is insufficient. Plant the exact class of named-export
mismatch, prove preflight repairs it before the listener is ready, request the
served Svelte and React module graphs, and terminate every test-owned process.
Keep raw run tasks available as explicitly low-level surfaces.

## Suggested Next Move

Build an isolated stale-distribution fixture and failing startup regression,
then make the smallest Effigy composition change using the existing package
builders before adding live HTTP/process-cleanup proof.

## Completion Protocol

Open one non-draft PR from the queue-owned branch, prove the exact head with
focused task/distribution tests, both preview builds, docs QA, and
`git diff --check`, then report `ready_for_review`. Never merge, release
g18.011, resume g18.006/g18.009, edit workflows, publish, or mutate Desktop.
