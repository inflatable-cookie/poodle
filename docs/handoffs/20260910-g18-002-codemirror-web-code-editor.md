---
title: g18.002 CodeMirror web CodeEditor
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
queue_approval: "Operator said 'Go for it' after Chatterbox promoted g18.002 as the sole approved ready task and stated that this exact phrase would authorize queue dispatch."
queue:
  capability: complex
  skipPRReview: false
  notifyOriginOnCloseout: true
  completionNotificationAgentIds:
    - 5317069e-201f-4dea-94f3-af8f0b9faff2
tags: [coordination, handoff, worker, g18, g18.002, code-editor, codemirror, svelte, react]
---

## What This Thread Was Doing

Execute [`g18.002`](../roadmaps/g18/002-codemirror-web-code-editor.md) from the
exact pushed planning commit. Ship one controlled Poodle `CodeEditor` for
Svelte and React over pinned CodeMirror 6 packages, plus the accepted
`TabItem.pinned` behavior across Tabs' normal active cohort.

## Why It Matters

Bovine Desktop needs a serious reusable source editor without owning or
exposing a third-party engine. Poodle already owns generalized editing and tab
surfaces. This task provides the smallest reusable seam: full TypeScript/web
support now, engine-neutral public semantics, isolated package weight, and no
false claim that a GPUI CodeEditor exists.

## Current State

Planning `main` is pushed at the commit containing this handoff. `g18.002` is
the sole approved frontier and its CodeMirror 6, paired Svelte/React, first-
class TypeScript syntax, UTF-16 edit, package-isolation, and staged-admission
decisions are canonical. No released package contains the editor. The existing
Tabs contract already defines `pinned`; its implementation remains normal
cross-runtime parity work.

## Boundaries

Follow the task and component contracts exactly. CodeMirror stays private.
Keep file identity, loading, drafts, revisions, Markdown body/full-file
projection, diagnostics policy, saves, review, and recovery in Desktop. Do not
add arbitrary extensions, a raw editor handle, completion, LSP, hover, code
actions, diff editing, or a silent language fallback.

Do not implement a GPUI/shared-Rust CodeEditor, create native placeholders or
receipts, edit Desktop, publish or tag a release, change workflows, run a
windowed selector, accept a new parity delta, or merge the PR. Svelte syntax is
not in the admitted language set unless Chatterbox separately approves an
audited package and contract change. Tabs remains subject to the full active-
cohort rule.

## Important Context

Read the task, CodeEditor and Tabs contracts, working rules, system shape,
compiled web distribution contract, package manifests, existing shared
TypeScript component machinery, focus and drag/reorder substrates, and Effigy
inventory before editing. Prefer direct minimal `@codemirror/*` packages over a
framework wrapper. Both frameworks must use the same engine-independent types
and transaction translation. Dedicated `./editor` entries must keep CodeMirror
and language packages out of root-only consumers.

## Suggested Next Move

Plant the engine-leak, root-import, prior-value multi-range edit, real
TypeScript-mode, unsupported-language, controlled no-echo, and SSR lifecycle
negative cases first. Then pin the smallest CodeMirror package set and build
the shared TypeScript core before either framework shell.

## Completion Protocol

Use Effigy inventory and focused validation. Complete the task's adversarial
oracle, including packed consumers, both web wrappers, the 2 MiB bound,
accessibility and keyboard escape, and Tabs pinned behavior across core,
Svelte, React, Rust/render, and mounted GPUI. Record licenses, generated docs
and evidence, one execution log, and one Desktop adoption-request handoff.

Run focused editor and Tabs selectors, package/type/distribution checks,
relevant headless GPUI Tabs regressions, docs checks, and `git diff --check`.
Commit, push, open one non-draft PR from the queue-owned branch, and report
`ready_for_review` with its number, exact head, validation, package boundaries,
and remaining staged-admission limits. Leave the tracked tree clean. The plugin
owns independent review, merge, canonical closeout, and handoff archival. No
release, Desktop adoption, GPUI CodeEditor, or successor auto-starts.
