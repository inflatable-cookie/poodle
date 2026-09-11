---
title: g18.018 RichTextEditor controlled-echo selection preservation
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
queue_approval: "Tom reproduced RichTextEditor moving the caret to the document end after every character on 2026-09-11 and said Go for it after the controlled-echo setContent diagnosis."
queue:
  dependsOn:
    - e3a0e8cb-287c-447d-90c3-c226bca3d763
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.018, rich-text, selection, controlled]
---

## What This Thread Was Doing

Execute
[`g18.018`](../roadmaps/g18/018-rich-text-controlled-echo-selection.md) after
g18.013 closes. Make an accepted controlled echo a true no-op so typing and
selection remain where the user put them.

## Why It Matters

Both public specimens immediately echo each `onChange` document through
`value`. The wrappers classify the fresh JSON object as a host replacement and
the engine calls TipTap `setContent`, moving the caret to the document end after
every character. Ordinary middle-document editing is unusable.

## Current State

The defect is source-proven in both wrappers and mirrored engines. Existing
host-echo tests bind content and callback count but not selection, history,
focus, scroll or IME. g18.013 owns overlapping toolbar shell files, so this task
is dependency-queued behind it. g18.011, g18.006 and g18.009 remain held.

## Boundaries

Follow g18.018 exactly. Treat an echo matching the live emitted ProseMirror JSON
as acceptance without replacing editor state. Preserve genuinely different host
replacement and prior-value rejection. Do not expose engine transactions or
selection, redesign the toolbar/images, add public props, release, or touch
Desktop.

## Important Context

Object identity is not semantic controlled-state identity. Bind same-object and
deep-cloned echoes, repeated typing in the middle of a multi-block document,
non-collapsed replacement, paste, IME, undo/redo, focus and scroll. Bind delayed
stale host values separately so the repair cannot weaken host authority.

## Suggested Next Move

After rebasing on merged g18.013, plant the paired mounted caret failures and a
replacement-path spy before changing wrapper or engine bookkeeping.

## Completion Protocol

Open one non-draft PR from the queue-owned branch, prove the exact head with
focused paired component/specimen/browser checks, both package and preview
builds, accessibility checks, docs QA and `git diff --check`, then report
`ready_for_review`. Never merge, release g18.011, resume g18.006/g18.009,
publish, or mutate Desktop.
