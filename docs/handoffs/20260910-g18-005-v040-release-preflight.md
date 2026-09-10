---
title: g18.005 v0.4.0 release preflight
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
queue_approval: "Operator said 'do the preflight stuff too' while directing g18.004 to run alongside g18.003; Chatterbox bounded this lane to changelog/parser repair and read-only release inspection, with no release mutation."
queue:
  capability: general
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.005, release, preflight, changelog]
---

## What This Thread Was Doing

Execute [`g18.005`](../roadmaps/g18/005-v040-release-preflight.md) from the
exact pushed planning commit. Remove the known changelog/parser obstacle and
prove Poodle's release status and plan are inspectable before the final 0.4.0
candidate exists.

## Why It Matters

Desktop is waiting for a published Poodle editor entry, but the release lane
must wait for rich text and Tabs. Parser repair and release-state inventory are
independent and can finish now, leaving the final g18.006 lane smaller and less
risky without publishing against moving source.

## Current State

Live npm latest is 0.3.0 for core and Svelte. g18.003 is reviewing as PR #237;
g18.004 is an approved concurrent Queue task. A prior read-only `effigy release
status --check-gates` stopped on existing `CHANGELOG.md` grammar errors. No
version, tag, workflow, registry, or Desktop mutation is authorized.

## Boundaries

Follow the task exactly. Preserve all historical changelog meaning while using
Effigy's supported Keep a Changelog grammar. Fix repository content, not
Effigy, CI, or release policy. Do not edit product code, manifests, locks,
dependencies, workflows, 0.4.0 release notes, g18.003/g18.004 paths, or Desktop.
Do not run a release workflow, create a tag, publish, or claim final candidate
certification.

## Important Context

Read the task, working rules, packaging/versioning and release-channel specs,
release manifest, release operations, workflow, changelog, current tags, and
Effigy release protocol before editing. `0.4.0` is provisional until g18.006
classifies the complete post-component delta. Read-only registry and forge
inspection is allowed; every mutating release action is excluded.

## Suggested Next Move

Capture the parser errors, inventory each historical changelog version/date/
link/entry, then normalize headings and prose without semantic loss. Re-run
only safe release status/plan inspection and record the candidate-time gates
that remain deferred.

## Completion Protocol

Prove the semantic changelog inventory is preserved, ordinary Effigy release
status/plan parsing succeeds, the package set is unchanged, tags and npm
versions remain unchanged, docs QA passes, and `git diff --check` is clean.
Write one concise g18.005 execution log. Commit, push, open one non-draft PR
from the queue-owned branch, and report `ready_for_review` with its number,
exact head, parser proof, unchanged release state, and deferred g18.006 gates.
Leave the tracked tree clean. Never merge, version, tag, dispatch a workflow,
publish, or mutate Desktop.
