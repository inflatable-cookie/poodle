---
title: g18.023 CodeEditor dual syntax palettes
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: awaiting-operator-approval
owner: Poodle Northstar orchestrator
created: 2026-09-11
updated: 2026-09-11
base_required: pushed-main
queue_dispatch: northstar-queue
queue:
  dependsOn: []
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.023, code-editor, tokens, syntax]
---

## What This Thread Was Doing

Execute [`g18.023`](../roadmaps/g18/023-code-editor-dual-syntax-palettes.md) as
the release-blocking correction to g18.021. Replace the accent/status-based
CodeEditor mapping with designed dark and light syntax palettes and sparse
Poodle-theme overrides.

## Why It Matters

The current engine technically paints several token groups, but the examples
read as accent and white. Its tests prove equality with general UI tokens rather
than a useful syntax hierarchy. That misses the purpose of highlighting and is
not acceptable for `0.4.0`.

## Current State

Integration main contains merged g18.021 and approved g18.022 planning. The
g18.011 evidence PR is in its own Queue revision for browser/assertion gaps; do
not edit it. g18.022 owns Slider-family paths and can run independently.
Retained g18.006 and g18.009 remain closed. This task has no queue prerequisite
and may run alongside those two active lanes.

## Boundaries

Follow g18.023 exactly. Add dark/light primitive syntax ramps, semantic syntax
roles, light-theme references and sparse named-theme overrides; regenerate all
token artifacts; map the paired private CodeMirror engines to those roles; and
replace token-identity tests with real palette, contrast, and live-theme proof.
Keep languages consumer-owned, CodeMirror private, plain modes unhighlighted,
and diagnostics intact. Do not touch g18.011, Slider work, public CodeEditor
props, release state, versions, changelog, workflows, Desktop, or native editor
surfaces.

## Important Context

### UI Design Brief

Use Precision Workbench Operate mode. Build one independently tuned dark syntax
palette and one independently tuned light palette. The code field must not be
dominated by the product accent: comments and punctuation recede; ordinary
identifiers remain calm; keywords, strings, literals, types, callables, and
properties/attributes use a restrained but clearly multi-hue hierarchy. Every
role must meet AA text contrast against the actual panel. Poodle themes inherit
the appropriate base and override individual semantic roles only when needed.
Switching themes restyles the mounted editor without remount or grammar reload.
Invalid syntax retains a non-colour cue. Test representative TypeScript and
JSON across both frameworks, Chromium/WebKit, all named themes, selection,
active line, search, diagnostics, and forced colours.

Use token-source references rather than duplicated colour literals: semantic
`color.syntax.*` roles reference dark primitives by default; light theme modes
reference the light primitives; named themes may override one role at a time.
Do not expose CodeMirror palette objects or language-specific styling.

## Suggested Next Move

Plant a perceptual regression that fails the current accent/status mapping,
then design and contrast-check the two primitive ramps before changing the
engine mapping. Regenerate token artifacts once the semantic role set is fixed.

## Completion Protocol

Open one non-draft PR from the queue-owned branch. Prove its exact head with
token generation/audits, paired component and Chromium/WebKit computed-style
checks, all-theme contrast, live-theme/no-remount behavior, plain-mode cost,
package/preview builds, installed-package audit, docs QA, and
`git diff --check`; then report `ready_for_review`. Never merge, edit or restart
g18.011/g18.022, resume g18.006/g18.009, release, publish, or mutate Desktop.
