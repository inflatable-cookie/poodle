---
title: g18.008 web editor preview specimens
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-10
updated: 2026-09-10
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom confirmed on 2026-09-10 that both the TipTap and CodeMirror components obviously need specimen pages in both Svelte and React because that is baseline in this repository."
queue:
  capability: general
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.008, preview, specimens, editor]
---

## What This Thread Was Doing

Execute [`g18.008`](../roadmaps/g18/008-web-editor-preview-specimens.md) from
the exact pushed planning commit. Correct the missed web preview admission for
the editors merged by g18.002 and g18.003.

## Why It Matters

Poodle's catalogue is the human review surface for public components. The
merged CodeMirror and TipTap/ProseMirror APIs are packaged and tested but cannot
be found or exercised in either web preview. Web-only admission excludes native
parity work; it does not waive Svelte or React documentation.

## Current State

Main contains g18.002 merge `308fa52c5cd68d9c776f320c368e4fb0896e4d4d`
and g18.003 merge `fb0b73732b5c0a2a9361fddd75962eccd2710b0f`.
Neither preview registers CodeEditor, RichTextEditor, or RichTextRenderer. The
shared `webOnlyComponents` supplement already feeds both web galleries without
entering the portable/native catalogue.

## Boundaries

Follow g18.008 exactly. Add three distinct pages in each framework, use the
real public subpath exports, and keep examples representative. Images must be an
explicit optional rich-text configuration; tables are standard; embeds stay
absent. Do not change editor APIs, engine boundaries, native parity, release
state, or Desktop.

## Important Context

The prior execution logs called preview admission closeout work. That was the
planning defect this task corrects; do not preserve it as a reason to omit the
pages. `RichTextRenderer` is a separate public export and therefore gets its
own addressable specimen. Exhaustive behavior belongs in focused tests, not on
the Examples page.

## Suggested Next Move

Bind all three slugs through the shared web-only registry first. Then build the
paired specimens around shared representative documents and add browser proof
that the real editors update controlled host state in both runtimes.

## Completion Protocol

Prove matched navigation/search/direct routes, real public-entry mounting,
CodeEditor controlled edits, RichTextEditor controlled ProseMirror updates,
RichTextRenderer non-editability, images-off/on policy, unchanged native
denominator, both preview builds/typechecks, docs QA, and `git diff --check`.
Commit, push, open one non-draft PR, and report `ready_for_review` with its
number and exact head. Leave the tree clean. Never merge, release, mutate
Desktop, or add native editor work.
