---
title: g18.016 CodeEditor live line-number reconfiguration
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
queue_approval: "Tom reproduced the inert CodeEditor Line Numbers specimen toggle on 2026-09-11 and said Go for it after the exact missing-live-reconfiguration diagnosis."
queue:
  capability: general
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.016, code-editor, line-numbers]
---

## What This Thread Was Doing

Execute
[`g18.016`](../roadmaps/g18/016-code-editor-live-line-number-reconfiguration.md).
Make `CodeEditor.lineNumbers` reconfigure the mounted Svelte and React editors
instead of applying only at initial construction.

## Why It Matters

Both Configuration specimens correctly change their host state and button
label, but the shared engine shape never reconfigures the CodeMirror line-number
extension. The visible gutter therefore contradicts the public prop and the
specimen control.

## Current State

Fresh mounts with line numbers off work. Both engine copies conditionally add
`lineNumbers()` to the initial extension array, but neither has a dedicated
compartment or update branch. g18.011 is Queue-held; g18.006 and g18.009 remain
held. This repair can run alongside g18.013 and g18.015.

## Boundaries

Follow g18.016 exactly. Reconfigure a dedicated line-number compartment on the
existing `EditorView`, preserve all editor state, and prove the real paired
specimen controls. Do not remount, change public props, start language-registry
work, edit workflows, release, or touch Desktop.

## Important Context

The regression must cover both directions, initial false, rapid controlled
updates, stable editor identity, value, selection, focus, undo history and
active diagnostics. Keep the two engine files behaviorally identical apart from
their existing framework header.

## Suggested Next Move

Plant the true→false→true failure through each public wrapper, then add one
line-number compartment beside the existing language, behavior, read-only,
diagnostic, wrap and tab-size compartments.

## Completion Protocol

Open one non-draft PR from the queue-owned branch, prove the exact head with
focused paired component/specimen/browser checks, both package and preview
builds, accessibility checks, docs QA, and `git diff --check`, then report
`ready_for_review`. Never merge, release g18.011, resume g18.006/g18.009,
publish, or mutate Desktop.
