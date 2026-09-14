---
title: g18.038 Tabs cross-window bridge distribution repair
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-14
updated: 2026-09-14
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260914-120149-g18-038-tabs-cross-window-bridge-distribution-repair.md
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom granted blanket approval on 2026-09-14 for the promoted Poodle repair, bounded 0.4.2 release, and retained Longhorn continuation chain."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, g18, g18.038, tabs, svelte, distribution]
---

## What This Thread Was Doing

Execute [`g18.038`](../roadmaps/g18/038-tabs-cross-window-bridge-distribution-repair.md).
Restore the Svelte Tabs parent-to-item bridge forward that exists in `v0.3.0`
but was dropped from source and the published `0.4.1` bundle.

## Why It Matters

Longhorn's retained `g02.039` adoption cannot exercise public cross-window tab
transfer because host preparation never runs. Poodle owns the regression and
must fix the reusable package rather than ask Longhorn for a shim.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`; planning branch `main`.
- Planning base before this handoff: `a59b8df1bd76a7e79e45489b59636406769ae768`,
  pushed and equal to `origin/main` at dispatch preparation.
- Roadmap task: [`g18.038`](../roadmaps/g18/038-tabs-cross-window-bridge-distribution-repair.md),
  the sole approved ready lane.
- Worker branch and worktree are Queue-managed from the pushed handoff commit;
  accept the launcher-provided clean non-`main` worktree.
- Required sibling worktree links: none.
- Owned mutable paths: `packages/svelte/components/src/Tabs.svelte`,
  `packages/svelte/components/test/Tabs.test.ts`,
  `test/package-install/fixture/DockRegion.test.ts`, and
  `docs/contracts/components/tabs.md`.
- The orchestrator reserves task/handoff/front-door/lifecycle closeout.
- Model capability profile: automatic general implementation pool.
- Frontier-worker justification: none; the repair and oracle are settled and
  bounded.
- PR state: pending. Queue owns independent review, CI observation, merge, and
  closeout after the worker reports `ready_for_review`.

## Boundaries

Follow g18.038 exactly. Restore the existing prop forward and prove behavior in
source plus the source-free installed package. Do not change public API or
bridge lifecycle, add a shim/fallback, edit React/native/GPUI, or touch
Longhorn. Do not bump versions, edit workflows, tag, publish, or run a release.

The validation budget is the focused Svelte Tabs test during implementation,
then one final `effigy release:web-certificate`, the Impeccable detector over
the changed Tabs UI target, and `git diff --check`. Do not stack `ci:web`,
`qa`, `docs:check`, or another release board around the certificate. Never run
a `*-windowed` selector.

## Important Context

The public prop and TabsItem registration already exist. The defect is the
parent `<TabsItem>` construction omitting `{crossWindowSourceBridge}`. A source
test alone is insufficient: the installed-package fixture must cross the real
pointer pre-drag threshold and observe `prepare` exactly once from the packed
archive. Preserve pinned tabs, local reorder, selection, focus, keyboard, and
native drag behavior.

`0.4.1` is already public and immutable. This task lands only the source fix;
the orchestrator owns the separately approved `0.4.2` release after merge.
Longhorn task `750ac957-830d-4224-a289-3770c5c3d582`, workspace
`wks_c6ea0088e9f696b9`, branch
`ns-750ac957-830d-4224-a289-3770c5c3d582`, and retained worker stay intact.

### UI Design Brief

- Canonical owner: g18.038.
- Classification: interaction refinement with no visual change.
- Affected user: a host initiating cross-window transfer from a Svelte tab.
- Current behavior: pointer pre-drag remains idle because TabsItem never gets
  the supplied host bridge.
- Target behavior: crossing the existing threshold calls host `prepare` once
  for the selected tab; all local Tabs behavior is unchanged.
- Governing pattern: the existing Tabs bridge contract and `v0.3.0` forwarding
  path.
- Scenario oracle: source and packed-consumer pointer gestures each observe one
  prepare call with the correct tab identity.
- Stop if this needs a new API, semantic redesign, fallback, or visual change.

## Suggested Next Move

Run the worker preflight, read g18.038 and the Tabs contract, then plant the
source and installed-package regressions before restoring the one prop forward.

## Completion Protocol

Use the launcher-provided clean dedicated worktree. Before broad reads, verify
root, branch, clean status, worktree registration, `HEAD == origin/main`, the
recorded planning base is an ancestor, and this committed handoff matches its
tracked blob. Read `AGENTS.md`, the task, the Tabs contract, the Effigy skill,
and the Northstar UI Build route; use Impeccable for the UI refinement.

Complete the task's falsification table, update its execution evidence, run
only the declared validation budget, and run `git diff --check`. Push one clean
non-draft PR against `main`, link the task and evidence, then report
`ready_for_review` without polling GitHub. Do not merge. Queue owns independent
exact-head review, required checks, merge, integration reconciliation, and
canonical closeout. After merge, the orchestrator proceeds to the separately
approved `0.4.2` release lane and returns exact registry evidence to Longhorn.
