---
title: g18.013 RichTextEditor toolbar controls
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
queue_approval: "Tom required proper RichTextEditor buttons on 2026-09-11 and selected MarkdownEditor as the intended Poodle reference."
queue:
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.013, rich-text, toolbar, ux]
---

## What This Thread Was Doing

Execute [`g18.013`](../roadmaps/g18/013-rich-text-editor-toolbar-controls.md).
Replace the link-like RichTextEditor command row with proper grouped Poodle
controls in both Svelte and React before the acceptance sweep.

## Why It Matters

The commands work, but the specimen exposes them as an unstructured sentence
of labels. Users cannot scan command groups, hit areas, active state, or
disabled context confidently. Poodle already has the intended toolbar language
in MarkdownEditor and should reuse it consistently.

## Current State

Both wrappers render native text buttons from the same command snapshot and
custom rich-text CSS. Active, available, roving-focus, and execution behavior
exist and must survive. g18.011 is Queue-held until this repair and g18.010
close. g18.006 remains blocked; g18.009 remains held.

## Boundaries

Follow g18.013 exactly. Reuse IconButton/MarkdownEditor control language,
centralize command presentation, and preserve the existing public toolbar and
engine contracts. Do not add arbitrary toolbar slots, new commands, schema
changes, release mutations, or Desktop work.

## Important Context

This is more than adding borders around text. Use recognizable icons or concise
conventional glyphs, logical groups, compact responsive clusters, tooltips,
accessible names, pressed/disabled state, and proper Button chrome for the link
editor actions. Prove Svelte/React parity and keyboard/selection continuity.

## Suggested Next Move

Plant paired specimen/browser assertions against the current link-like row,
then define one shared command-presentation map and compose it through existing
Poodle controls before changing layout CSS.

## Completion Protocol

Open one non-draft PR from the queue-owned branch, prove the exact head with
focused component/browser/accessibility/visual checks, both preview builds,
docs QA, and `git diff --check`, then report `ready_for_review`. Never merge,
release g18.011, resume g18.006/g18.009, publish, or mutate Desktop.
