---
title: g18.032 fast validation and npm release
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: merged
owner: Poodle Northstar orchestrator
created: 2026-09-13
updated: 2026-09-13
merged_pr: 267
merged_commit: 1b1ee3cdfc3eae912e254d8f6b736c8db11d3d92
review_comment: 5653154132
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom explicitly directed Poodle Chatterbox on 2026-09-13 to fix the release process before any other work, then clarified that the complete suite including GPUI must also be trimmed, observable and fast."
queue:
  capability: complex
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, g18, g18.032, release, validation, npm, effigy]
---

## What This Thread Was Doing

Execute [`g18.032`](../roadmaps/g18/032-fast-validation-and-npm-release.md).
Replace the emergency release wrapper, release-specific candidate machinery
and opaque serial full board with the system fixed by spec 071.

## Why It Matters

The `0.4.0` release took roughly 36 hours of operator time. One hosted run sat
inside an unrelated aggregate gate for 5h45m without child output; successful
runs spent about half their time provisioning unused Cargo tooling and repeated
the package build/proof three times. Candidate preparation needed three
one-off policy/evidence precursors. The full board itself also needs visible,
bounded, deduplicated execution rather than merely being removed from npm.

## Current State

Dispatch from the clean pushed planning commit containing this handoff. Poodle
`0.4.0` is published; g18.006 and g18.009 are complete. The emergency workflow
still uses macOS, installs Rust/`cargo-deny`, repeats builds and leaves root
`effigy release gates` pointed at aggregate `qa`. `qa` serially nests web,
Rust, native/GPUI, package and policy work with duplicate leaves and no child
progress. Installed-package scope still carries active version/task policies.

## Boundaries

Follow g18.032 and spec 071. Workflow editing, one capped baseline full-board
run, one final full-board run and one non-publishing hosted candidate drill are
operator-authorized. Do not change package versions, changelog, release notes,
product source, Cargo inputs, native evidence, tags, registries or downstream
repos. Do not publish or invoke foreground/windowed selectors.

Keep `release.yml`: npm trusted publishing binds to that filename. Keep OIDC,
public-package and tag guards. Core and Svelte are the only publications;
React stays private but follows the web version.

## Important Context

Split diff admission from archive certification. Candidate mode builds and
source-free-tests one archive set, then uploads those bytes with identity.
Publish consumes them by run ID and verifies tag identity before
`npm publish <tarball>`; it never rebuilds.

Generic candidate admission derives versions from base/head. Delete active
release-specific branching; do not add a `g18.032` or `0.4.1` mode. Npm
candidates never touch Cargo or GPUI evidence.

For the full board, preserve the complete assertion inventory. Profile first,
then eliminate transitive repetition, reuse outputs and parallelize only safe
read-only groups. Every child must report start/end/elapsed time and have owned
process cleanup. A silent or over-budget run is a failure, not a reason to wait.

## Suggested Next Move

Plant automation, process-timeout and generic-candidate negatives first. Take
the single capped baseline. Then separate scope/archive proof, add release
authority, flatten the validation graph and rebuild the workflow.

## Completion Protocol

Use only the validation budget in g18.032. Open one non-draft PR from the Queue
branch and report without polling CI. Run exactly one non-publishing candidate
workflow drill on the exact pushed head after the workflow is addressable.
Record run ID, durations and artifact hashes. Cancel and report blocked if it
reaches ten minutes. Queue owns CI, independent review, merge and closeout.
Never tag, publish or mutate Desktop.
