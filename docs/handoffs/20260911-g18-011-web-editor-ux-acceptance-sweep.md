---
title: g18.011 web editor UX acceptance sweep
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
queue_approval: "Tom directed on 2026-09-11 that g18.006 remain held until a testing sweep across all three new editor components is complete."
queue:
  dependsOn:
    - d5ece513-5f28-4bfe-9132-12a70cf7a89f
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.011, editor, ux, qa]
---

## What This Thread Was Doing

Execute [`g18.011`](../roadmaps/g18/011-web-editor-ux-acceptance-sweep.md) after
g18.010 and g18.013 close and Chatterbox releases this task's Queue hold. Sweep
CodeEditor, RichTextEditor, and RichTextRenderer across their real Svelte and
React previews before release work resumes.

## Why It Matters

The new specimen pages already exposed one release-blocking focus defect. Green
component suites did not substitute for ordinary human-facing use. The 0.4.0
candidate must not freeze all three editor surfaces until their real interaction
and presentation have been reviewed systematically.

## Current State

g18.006 is blocked with its original workspace and clean pre-candidate state
preserved. g18.009 is Queue-held. g18.010 owns the focus correction; g18.013
owns the known RichTextEditor toolbar repair. This task is explicitly held
until both merge. All three editor pages exist in both previews from g18.008.

## Boundaries

Follow g18.011 exactly. Exercise real public entries and publish reproducible,
severity-ranked evidence. Do not implement product fixes, settle the modular
language API, edit release state, or mutate Desktop. A blocking finding stops
the sweep and returns to Chatterbox with exact reproduction.

## Important Context

DOM presence and green builds are insufficient. Compare Svelte and React for
real editing, controlled state, keyboard/pointer behavior, accessibility,
configuration, themes and constrained layout. RichTextRenderer is a distinct
surface. Measure CodeMirror language loading honestly across runtime load,
emitted chunks and installed dependencies. The operator has already decided
that the language boundary must be extensible; collect the baseline needed by
the bounded follow-up, but do not redesign it in this sweep.

## Suggested Next Move

Build both previews from the accepted g18.010 merge, establish paired routes
and fixtures, then run the ordered CodeEditor, RichTextEditor and
RichTextRenderer journeys before expanding test instrumentation.

## Completion Protocol

Open one evidence/test PR only if the sweep completes without an unresolved
blocking finding; otherwise report `blocked` with exact route, action, expected,
observed, framework, screenshot/log and severity. Run focused browser checks,
both preview builds, relevant accessibility checks, docs QA and
`git diff --check`. Never merge fixes, resume release tasks, publish, or mutate
Desktop. Operator acceptance remains required after Chatterbox disposition.
