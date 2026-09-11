---
title: g18.021 CodeEditor token-bound syntax presentation
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
queue_approval: "Tom explicitly approved dispatch on 2026-09-11."
queue:
  dependsOn: []
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.021, code-editor, codemirror, syntax]
---

## What This Thread Was Doing

Execute [`g18.021`](../roadmaps/g18/021-code-editor-token-bound-syntax-presentation.md)
as the bounded repair for g18.011 finding F1. Add real Poodle-token syntax
presentation to both CodeEditor engines without changing the consumer-owned
language registry.

## Why It Matters

The acceptance sweep proved that full-mode TypeScript and JSON are visually
identical to plain text in both frameworks and browsers. Poodle loads the real
grammar but never installs CodeMirror highlighting, so the public syntax claim
is false and the 0.4.0 release remains blocked.

## Current State

Main is clean at `a639b1b78`. g18.012 is merged and its registry/loading/cost
proof is accepted. The original g18.011 task is blocked with its worker stopped,
callback accepted, no PR, and its remaining surfaces unswept. Preserve that
task and thread; once this repair is dispatched, add it as an in-place g18.011
dependency.

## Boundaries

Follow g18.021 exactly. Install one private CodeMirror `HighlightStyle` using
Poodle semantic CSS variables in the paired engines, active only for a full
non-plain language. Keep grammars consumer-owned and keep all engine types and
style configuration private. Do not repair the React preview hash follow-up,
resume the sweep, or touch release/Desktop state in this worker.

## Important Context

Test rendered syntax, not merely parser activation or extension presence. The
proof must use real TypeScript and JSON loaders, computed styles in Chromium
and WebKit, at least two Poodle themes, live full/plain and language switching,
and installed-package cost inspection. CSS variables must update without an
editor remount. A direct `@lezer/highlight` dependency is permitted only for
the tags imported by the internal style.

## Suggested Next Move

Plant the paired real-grammar failure in the existing language-registry probe,
then add the smallest private token style to one engine and mirror it exactly
to the other before running the focused board.

## Completion Protocol

Open one non-draft PR from the queue-owned branch, prove the exact head with
paired component and Chromium/WebKit syntax checks, live theme/mode/language
switching, installed-package cost checks, package/preview builds, docs QA and
`git diff --check`, then report `ready_for_review`. Never merge, retry or
replace g18.011, resume g18.006/g18.009, release, publish, or mutate Desktop.
