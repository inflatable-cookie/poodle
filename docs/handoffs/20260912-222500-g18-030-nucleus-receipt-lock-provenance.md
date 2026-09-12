---
title: g18.030 Nucleus receipt lock provenance
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-12
updated: 2026-09-12
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom's operator-directed g18.006 takeover authorized Poodle Chatterbox to resolve its plan-level release-gate blockers and continue the retained task through Northstar Queue."
queue:
  capability: complex
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, g18, g18.030, nucleus, provenance, release-blocker]
---

## What This Thread Was Doing

Execute [`g18.030`](../roadmaps/g18/030-nucleus-receipt-lock-provenance.md) as
the structural precursor for retained g18.006. Remove the receipt emitter's
hard-coded preview-lock hash and release versions, derive them from the actual
lockfile, and repin the current source-bound evidence once.

## Why It Matters

The `0.4.0` candidate must change `packages/gpui/preview/Cargo.lock`, but honest
M1/A1 emission currently requires a matching source edit that the closed
release-only candidate policy correctly refuses. Derivation removes that
recurring release conflict without widening candidate ownership.

## Current State

Dispatch from pushed main containing this handoff. Retained Queue task
`17ac3fee-de90-4b32-9672-1134770bb086`, workspace
`wks_85bb63e05d4b3f90`, and branch
`ns-17ac3fee-de90-4b32-9672-1134770bb086` are preserved clean at
`274757dc2b8f4f9800ee87211a8cc9daee16ff67`, with no PR or candidate commit.
Its worker and recovery coordinator proved the exact emitter conflict and
finished valid blocked callbacks.

## Boundaries

Follow g18.030 exactly. Do not change Cargo manifests or lockfiles, the merged
candidate-scope policy, versions, changelog, release notes, workflows, tags,
registries, Desktop, or the retained task. Use only headless selectors. The
worker never merges or resumes g18.006.

## Important Context

`nucleus_receipts.rs` is compiled only into the headless regression evidence
target and already has the `sha2` dev dependency available. Prefer a small
pure fail-closed Cargo-lock package parser over a new dependency. Preserve the
canonical five-package order and receipt schema. Because this source is itself
inside the Nucleus source identity, its accepted implementation requires one
complete current M1/A1, manifest, ledger and affected census repin.

## Suggested Next Move

Write fixture laws for current and planted-version lock content, changed-byte
hashing, and missing/duplicate/invalid package metadata. Then replace the
literals, freeze the implementation commit and emit the complete current
headless evidence exactly once.

## Completion Protocol

Open one non-draft PR after focused provenance laws and the complete current
evidence repin validate. Run the focused Rust tests, one required headless
Nucleus emission, receipt/ledger/census checks, docs lint and
`git diff --check`; do not run `qa`, a release gate, windowed selectors or CI
polling. Report `ready_for_review` immediately with the exact head, derived
current and planted `0.4.0` proof, receipt counts, one-source identity and PR
number. Queue owns exact-head CI, review and merge. No release mutation or
g18.006 continuation occurs in this worker.
