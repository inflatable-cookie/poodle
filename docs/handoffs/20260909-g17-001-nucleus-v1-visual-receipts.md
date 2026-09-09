---
title: g17.001 Nucleus V1 visual receipts
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-09
updated: 2026-09-09
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Operator directed Chatterbox to use northstar-queue for dispatch, then authorized the 2026-09-09 flattened-task switchover to resume execution from its coherent approved frontier."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, g17, g17.001, nucleus, visual, receipts]
---

## What This Thread Is Doing

Execute [`g17.001`](../roadmaps/g17/001-nucleus-v1-visual-receipts.md) from
the exact pushed planning commit. Import the validated Poodle Lab cohort bundle
immutably, emit Nucleus V1 receipts for every covered row, and advance only
receipt-backed GPUI visual cells in the generated evidence ledger.

## Why It Matters

Poodle already has M1 and A1 for all 29 Nucleus rows. The completed Lab run is
the missing reusable visual evidence. V1 makes the renderer comparisons
traceable without treating 160 reported findings as accepted deltas or
silently changing comparison policy.

## Current State

Poodle `main` contains the flattened Northstar model. `g17.001` is the sole
approved frontier; `g17.002`–`g17.004` are complete. The Lab run
`2026-09-08T14-06-48` was produced from merged Lab commit
`f99465f048d7c5c58603b99ae51f3209e581848e` and closeout commit
`13ddc2fcbc0897a9f2ec78ee0dd061ce74c7f46d`: 174 captures, 116 comparisons,
two agreeing repeats, valid foreground proofs, and 160 findings. Current
Nucleus manifests, schemas, M1/A1 receipts, and ledger live under
`docs/evidence/nucleus/`.

## Boundaries

Follow the task exactly. Validate the imported bundle by directory hash and
validator version before trusting it. Extend the closed receipt contract for
V1 without weakening M1/A1 validation. Preserve every finding and move a
ledger cell only when a valid V1 receipt backs it.

Do not edit component behavior, pixels, scenarios, tolerances, Lab, Nucleus,
Longhorn, workflows, release surfaces, V2/M2/A2, or finding adjudication. Do
not invoke a windowed selector or merge the PR. Stop on any non-bijective
fixture/scenario mapping, failed byte validation, compatibility shim, or need
to decide whether a finding is acceptable.

## Important Context

Read the task, `docs/evidence/nucleus/README.md`, the receipt manifest and
schemas, `scripts/nucleus-parity-receipts.ts`,
`scripts/parity-evidence-ledger.ts`, and the Lab
`docs/contracts/004-receipt-import.md`. The g15.047 tolerance policy is fixed
historical authority. The bundle is evidence, not implementation input; keep
it immutable and cite its Lab source commit, run id, validator, directory hash,
capture count, and comparison count.

## Suggested Next Move

Locate the sanitized bundle at the named Lab closeout, run its validator before
copying anything, then plant tamper and unbacked-ledger negatives around the
smallest V1 schema/generator extension.

## Completion Protocol

Use Effigy inventory and focused planning. Add tests for tampered bundle bytes,
unknown/duplicate/mismatched fixture identities, missing traceability, an
unbacked `compared` ledger cell, retained findings, and M1/A1 non-regression.
Run the focused receipt and ledger selectors, docs checks, the relevant normal
headless QA, and `git diff --check`. Write one execution log with the exact
bundle and receipt evidence. Commit, push, open one PR from the queue-owned
branch, and report `ready_for_review` with its number and exact head. Leave the
tracked tree clean. The plugin owns independent review, merge, closeout, and
handoff archival.
