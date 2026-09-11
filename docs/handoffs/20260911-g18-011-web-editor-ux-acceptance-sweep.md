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
queue_approval: "Tom directed on 2026-09-11 that release work remain last, expanded the sweep to MarkdownRenderer, and approved in-place Queue dependency correction."
queue:
  dependsOn:
    - e3a0e8cb-287c-447d-90c3-c226bca3d763
    - 5b42e240-5e63-4bdd-8035-7f2285776c6e
    - be813655-c1eb-4b57-af4b-974d2f8c84de
    - 6bb46544-966a-4af0-af7c-e53846ad0407
    - eac944cd-2ee0-4810-bd60-0976e3270e56
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.011, editor, ux, qa]
---

## What This Thread Was Doing

Execute [`g18.011`](../roadmaps/g18/011-web-editor-ux-acceptance-sweep.md) after
its five Queue prerequisites close. Sweep CodeEditor, RichTextEditor,
RichTextRenderer, and MarkdownRenderer across their real Svelte and React
previews before g18.012 and release work.

## Why It Matters

The new specimen pages already exposed one release-blocking focus defect. Green
component suites did not substitute for ordinary human-facing use. The 0.4.0
candidate must not freeze all four editor surfaces until their real interaction
and presentation have been reviewed systematically.

## Current State

g18.006 is blocked with its original workspace and clean pre-candidate state
preserved. g18.009 is dependency-queued behind it. g18.010's focus correction is merged; g18.013
owns the RichTextEditor toolbar repair; g18.014 owns the inert Image Policy
specimen; g18.015 owns safe preview distribution startup; g18.016 owns live
CodeEditor line-number configuration; g18.017 owns the block Slider visual
repair; g18.018 owns controlled-echo selection; g18.019 owns MarkdownRenderer.
This task has those five open tasks as Queue prerequisites and no manual hold.
Existing editor pages come from g18.008; g18.019 adds the renderer pair.

## Boundaries

Follow g18.011 exactly. Exercise real public entries and publish reproducible,
severity-ranked evidence. Do not implement product fixes, settle the modular
language API, edit release state, or mutate Desktop. A blocking finding stops
the sweep and returns to Chatterbox with exact reproduction.

## Important Context

DOM presence and green builds are insufficient. Compare Svelte and React for
real editing, controlled state, keyboard/pointer behavior, accessibility,
configuration, themes and constrained layout. Both renderers are distinct
surfaces. Measure CodeMirror language loading honestly across runtime load,
emitted chunks and installed dependencies. The operator has already decided
that the language boundary must be extensible; collect the baseline needed by
the bounded follow-up, but do not redesign it in this sweep.

## Suggested Next Move

Build both previews from the accepted prerequisite merges, establish paired
routes and fixtures, then run the ordered CodeEditor, RichTextEditor,
RichTextRenderer and MarkdownRenderer journeys before expanding test
instrumentation.

## Completion Protocol

Open one evidence/test PR only if the sweep completes without an unresolved
blocking finding; otherwise report `blocked` with exact route, action, expected,
observed, framework, screenshot/log and severity. Run focused browser checks,
both preview builds, relevant accessibility checks, docs QA and
`git diff --check`. Never merge fixes, resume release tasks, publish, or mutate
Desktop. Operator acceptance remains required after Chatterbox disposition.
