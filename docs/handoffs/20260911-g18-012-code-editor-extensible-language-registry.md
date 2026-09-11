---
title: g18.012 CodeEditor extensible language registry
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
queue_approval: "Tom confirmed on 2026-09-11 that Poodle cannot support every language directly and CodeEditor language support must be extensible."
queue:
  dependsOn: []
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.012, code-editor, codemirror, packaging]
---

## What This Thread Was Doing

Execute [`g18.012`](../roadmaps/g18/012-code-editor-extensible-language-registry.md)
before the g18.011 acceptance sweep. Replace the fixed CodeEditor language
catalogue with consumer-selected lazy language providers in Svelte and React.

## Why It Matters

Poodle cannot enumerate every language a consumer may need. The current switch
also makes every admitted grammar a Poodle package dependency even though
runtime loading is dynamic. The release boundary must let each product own its
language set and cost without turning CodeEditor into a general CodeMirror
escape hatch.

## Current State

The current contract exposes a closed `CodeEditorLanguage` union. Both web
engines contain matching switches with literal `@codemirror/lang-*` imports,
and both component manifests declare the fixed grammar set. This task owns its
before/after install, bundle and runtime baseline. It may run beside the active
rich-text chain. g18.011 waits for both; g18.006 remains paused at the final
candidate gate and g18.009 is dependency-queued behind it.

## Boundaries

Follow g18.012 exactly. Plain text stays built in. Publish one narrow,
substrate-explicit adapter that constructs Poodle’s opaque registry from typed
lazy CodeMirror language loaders. Do not expose arbitrary editor extensions,
ship an exhaustive catalogue, retain silent fallbacks, release packages, or
mutate Desktop.

## Important Context

“Dynamic import” alone is not the acceptance claim: selected grammars must be
consumer-installed, unselected grammars absent from the isolated install and
bundle graph, and only the active provider invoked. Svelte and React must share
language ids, registry semantics, controlled switching, and refusal behavior.
This is a deliberate pre-1.0 contract correction, not a compatibility exercise.

## Suggested Next Move

Capture the closed union/package/runtime baseline and plant isolated-consumer
tests against it. Then define the smallest shared opaque registry plus explicit
CodeMirror loader adapter before changing either wrapper.

## Completion Protocol

Open one non-draft PR from the queue-owned branch, prove the exact head with
focused package/component/browser checks, both preview builds, declaration and
export checks, docs QA, and `git diff --check`, then report `ready_for_review`.
Never merge, release, publish, resume g18.006/g18.009, or edit Desktop. Its
accepted merge becomes a prerequisite of g18.011.
