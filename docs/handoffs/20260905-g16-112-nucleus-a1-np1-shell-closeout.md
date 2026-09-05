---
title: g16.112 Nucleus A1 NP-1 shell closeout handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: pr-open-awaiting-review
owner: Poodle Northstar orchestrator
created: 2026-09-05
updated: 2026-09-05
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.112]
---

## What This Thread Was Doing

Execute g16.112 NP-1 from the revision-17 dispatch: pair the seven shell rows
with the shared A1 Svelte/GPUI accessibility receipt harness.

## Why It Matters

This tranche moves the shell primitives from M1-only evidence toward the same
executed accessibility projection contract established by g16.111.

## Current State

- Dedicated worker branch: `worker/g16-112-nucleus-np1`.
- Base: `3dbabac3990fb5f3856305b7c8f971039b0a81be`.
- Added scenarios and Svelte snapshots for Icon, Text, Surface, Button,
  IconButton, AppHeader, and SplitView.
- Added native A1 proof entry points in
  `packages/gpui/preview/tests/headless/nucleus_a11y.rs`.
- Execution log: `docs/logs/2026-09/20260905-g16-112-nucleus-a1-np1-shell.md`.
- Svelte A1 check passed. Native headless check passed with 212 tests, five
  validated A1 receipts, and two recorded divergences under the NP-1 store.
- No sibling worktree dependencies.

## Boundaries

Keep the PR to NP-1 owned scenarios, snapshots, native A1 tests, execution log,
handoff, and append-only papercut evidence. Do not change component contracts,
Svelte components, backend behaviour, shared closeout surfaces, or windowed
selectors. Do not merge.

## Important Context

The canonical dispatch handoff is
`docs/handoffs/20260905-g16-112-nucleus-a1-np1-shell.md` on revision 17.
The A1 comparison law and receipt emission remain owned by g16.111; this lane
only supplies row scenarios and proofs.

## Suggested Next Move

Finish the managed `effigy regressions:native` run, record its exact result,
then run the receipt and ledger checks with the generated artifacts. Resolve
only concrete row-local mismatches; record any contract/backend divergence and
stop for Chatterbox if a repair is required.

## Completion Protocol

Before handoff, run the relevant headless selectors, `git diff --check`, commit
one focused PR, push the dedicated branch, and provide this absolute handoff
path to the orchestrator. The orchestrator owns independent clean exact-head
review and merge. Stop after push; do not merge or run windowed selectors.
