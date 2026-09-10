---
title: g18.003 TipTap/ProseMirror rich-text editor
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
queue_approval: "Operator said 'Go for it' after Chatterbox promoted g18.003 as the sole approved ready task and requested Northstar Queue execution."
queue:
  capability: complex
  skipPRReview: false
  notifyOriginOnCloseout: true
  completionNotificationAgentIds:
    - 5317069e-201f-4dea-94f3-af8f0b9faff2
tags: [coordination, handoff, worker, g18, g18.003, rich-text, tiptap, prosemirror, svelte, react]
---

## What This Thread Was Doing

Execute [`g18.003`](../roadmaps/g18/003-tiptap-prosemirror-rich-text-editor.md)
from the exact pushed planning commit. Ship paired Svelte and React
`RichTextEditor` and `RichTextRenderer` surfaces over pinned TipTap 3 and
ProseMirror packages.

## Why It Matters

Bovine Desktop will need reusable rich-text document editing without owning a
second schema or exposing a third-party engine. Poodle should supply the
generalized web seam: ProseMirror document authority, project-level curated
configuration, standard tables, optional images, and a matching read-only
renderer. Native parity is a later problem.

## Current State

Planning `main` is pushed at the commit containing this handoff. `g18.003` is
the sole approved ready frontier. The operator confirmed rich-text documents,
ProseMirror authority, TipTap 3, curated Poodle feature modules, tables in the
standard profile, project-selectable images, deferred embeds, and a matching
renderer. No released package contains this surface.

## Boundaries

Follow the task and component contract exactly. ProseMirror JSON and configured
schema semantics stay authoritative; do not invent a Poodle block schema.
TipTap and ProseMirror runtime objects stay private. Admit only Poodle's
curated feature and toolbar identifiers. Images are opt-in through the bounded
async `requestImage` seam. Unknown features and invalid documents fail closed.

Do not add arbitrary extensions or public engine handles, HTML or Markdown
conversion, uploads or storage policy, embeds, mentions, custom consumer nodes,
collaboration, comments, tracked changes, GPUI/shared-Rust/Jetstream work,
Desktop changes, a release, workflow edits, or windowed runs. Do not merge the
PR. Dedicated `./rich-text` entries must keep engine weight out of roots,
`./markdown`, and `./editor`.

## Important Context

Read the task, RichTextEditor contract, working rules, system shape, compiled
web distribution contract, package manifests, shared TypeScript component
machinery, CodeEditor package-isolation precedent, and Effigy inventory before
editing. Use the smallest maintained MIT TipTap 3 and ProseMirror package set.
Both wrappers and the renderer must share one validator, feature registry,
configured schema, types, and token-owned styles while remaining SSR-safe.

## Suggested Next Move

Plant the engine-leak, invalid-document partial-output, image-disabled,
async-image stale/cancel/unmount, renderer-equivalence, table keyboard escape,
SSR lifecycle, and root-light negative cases first. Then pin and license-audit
the minimal engine graph, prove server imports, and build the shared
TypeScript core before either framework shell.

## Completion Protocol

Use Effigy inventory and focused validation. Complete the task's adversarial
oracle across both frameworks: controlled no-echo behavior, live schema
reconfiguration, formatting and tables, optional images, async insertion
races, paste safety, accessibility, keyboard escape, large-document refusal,
editor/renderer equivalence, SSR cleanup, declarations, packed consumers, and
archive/bundle isolation. Record licenses, generated docs and evidence, one
execution log, and one Desktop adoption-request handoff.

Run focused rich-text selectors, package/type/distribution checks, relevant
component and accessibility checks, docs checks, and `git diff --check`.
Commit, push, open one non-draft PR from the queue-owned branch, and report
`ready_for_review` with its number, exact head, validation, package boundaries,
and remaining staged-admission limits. Leave the tracked tree clean. The plugin
owns independent review, merge, canonical closeout, and handoff archival. No
release, Desktop adoption, native implementation, or successor auto-starts.
