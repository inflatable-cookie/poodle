---
title: g18.010 CodeEditor editing focus treatment
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
queue_approval: "Tom confirmed on 2026-09-11 that CodeEditor should not retain or gain a focus ring when typing begins and directed the 0.4.0 candidate lane to pause while UX issues are corrected."
queue:
  capability: general
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.010, code-editor, focus, ux]
---

## What This Thread Was Doing

Execute [`g18.010`](../roadmaps/g18/010-code-editor-editing-focus-treatment.md)
from the exact pushed planning commit. Correct CodeEditor’s outer focus
treatment in both web wrappers before the 0.4.0 candidate resumes.

## Why It Matters

The merged editor paints a large navigation ring once ordinary typing changes
the document modality to keyboard. A code editor already has caret and
selection cues; retaining the outer ring during active editing is noisy and
misstates Poodle’s keyboard-focus intent.

## Current State

Svelte and React share a CSS rule keyed to the live document modality plus
`:focus-within`. The global tracker correctly classifies every non-modifier
keydown as keyboard, but that global fact cannot distinguish Tab entry from
typing inside the editor. g18.006 has been told to pause with its workspace and
work preserved; queued g18.009 is held.

## Boundaries

Follow g18.010 exactly. Use component-local entry presentation, keep document
modality truthful, preserve Tab/Shift+Tab navigation indication, and dismiss
only on real editing intent. Cover both frameworks and IME/clipboard/history
routes. Do not add a public prop, redesign global modality, touch other
components, or mutate release/Desktop state.

## Important Context

Pointer entry must remain ring-free. Keyboard navigation entry still needs the
shared outer treatment. Typing, deletion, composition, paste, cut, undo, and
redo dismiss it; navigation-only keys do not. Leaving and re-entering by
keyboard restores it. Internal controls retain their own focus-visible cues.

## Suggested Next Move

Plant the paired browser failure against the real public CodeEditor first.
Prove the document modality remains keyboard while a local editor state removes
the ring, then bind re-entry and IME before broadening the implementation.

## Completion Protocol

Run focused Svelte/React component and browser interaction tests, paired
preview builds, relevant accessibility checks, docs QA, and
`git diff --check`. Open one non-draft PR and report `ready_for_review` with
the exact head and focus-origin proof. Leave the tree clean. Never merge,
release, alter g18.006/g18.009, or mutate Desktop.
