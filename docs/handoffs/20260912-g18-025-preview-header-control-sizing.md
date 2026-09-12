---
title: g18.025 Preview header control sizing
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
queue_approval: "Tom explicitly requested the preview header sizing fix now on 2026-09-12."
queue:
  dependsOn: []
  capability: general
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.025, preview, header, sizing]
---

## What This Thread Was Doing

Execute [`g18.025`](../roadmaps/g18/025-preview-header-control-sizing.md) as a
small paired-preview repair. Make Theme, Density, Size, Contrast, and Search use
one fixed `md` chrome size in both headers.

## Why It Matters

The current header mixes visible control heights and looks mechanically
misaligned. It also controls the specimen Size axis, so allowing that selection
to resize the header makes the control surface unstable.

## Current State

Integration main contains merged g18.023 and g18.024. The operator approved
this repair on 2026-09-12. It has no prerequisite. Retained g18.006 and g18.009
remain closed.

## Boundaries

Follow g18.025 exactly. Touch only the paired preview DisplayControls surfaces,
their focused styles/tests, and one execution log. Use Poodle's existing `md`
size machinery; do not force arbitrary heights or change reusable component
APIs. Do not edit generated shell semantics, release state, versions,
changelog, workflows, Desktop, or unrelated preview UI.

## Important Context

### UI Design Brief

The header is fixed `md` chrome. All five painted controls measure 36px and
share top/bottom edges beneath aligned labels. Selecting specimen `xs`–`xl`
changes catalogue content only; it never resizes the header. Preserve wrapping,
the flexible Search group, and all existing interactions. Mirror Svelte and
React and prove geometry in Chromium and WebKit.

## Suggested Next Move

Plant a paired browser measurement across every specimen Size choice, then bind
the complete DisplayControls subtree to fixed `md` sizing through existing
presentation APIs before adjusting any local alignment CSS.

## Completion Protocol

Open one non-draft PR from the queue-owned branch. Prove its exact head with
paired preview tests, Chromium/WebKit geometry and interaction checks, preview
builds, docs QA, and `git diff --check`; then report `ready_for_review`. Never
merge, resume g18.006/g18.009, release, publish, or mutate Desktop.
