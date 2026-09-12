---
title: g18.027 v0.4.0 public-surface freeze audit
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
tags: [coordination, handoff, worker, g18, g18.027, release, api, audit]
---

## What This Thread Was Doing

Execute [`g18.027`](../roadmaps/g18/027-v040-public-surface-freeze-audit.md)
after g18.026 merges. Produce the final classified `v0.3.0`→main public delta
that freezes `0.4.0` before candidate preparation.

## Why It Matters

Bovine Desktop needs the release quickly, but g18.006 must not discover an
unclassified breaking change after version preparation begins. This audit
separates additions, compatible behavior and deliberate pre-v1 migrations at
the exact final product head.

## Current State

Known breaking rows include the Slider-family presentation API replacement,
Markdown's safe HTML default and removed dead Slider fallback helpers. Editors,
renderers and new subpaths are expected additive; Tabs inactive fill is
expected behavioral. These are starting hypotheses, not an exhaustive result.

## Boundaries

Audit and document only. Compare immutable `v0.3.0` to exact accepted
post-g18.026 main. Do not repair product code or edit versions, locks,
changelog, release notes, workflows, Desktop, g18.006 or g18.009. Stop and
return a decision capsule for any unclassified break or necessary source fix.

## Important Context

Cover public core/Svelte, private packed React parity, Rust source/tag APIs,
component defaults, package entries, dependency/peer constraints and removed
public CSS variables. The resulting evidence must be reproducible and usable
directly by g18.006 release notes. No later public break may enter `0.4.0`.

## Suggested Next Move

Capture baseline/final identities, build the current declarations and packed
exports, then produce one row-level classified report before adding tooling.
Only add a focused audit helper when the existing build and diff evidence
cannot express a required class reliably.

## Completion Protocol

Open one non-draft audit PR from the queue branch. Prove exact-head identity,
complete classification, planted negative coverage where needed, docs QA and
`git diff --check`; report `ready_for_review`. Never merge, prepare the
candidate, tag, publish or mutate Desktop.
