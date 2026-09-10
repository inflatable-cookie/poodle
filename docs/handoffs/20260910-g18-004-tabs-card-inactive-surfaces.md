---
title: g18.004 Tabs card inactive surfaces
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
queue_approval: "Operator said 'Dispatch that if it can run in parallel with 003'; Chatterbox verified the Tabs paths are independent of g18.003 PR #237 and approved concurrent execution."
queue:
  capability: general
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.004, tabs, svelte, react, gpui]
---

## What This Thread Was Doing

Execute [`g18.004`](../roadmaps/g18/004-tabs-card-inactive-surfaces.md) from the
exact pushed planning commit. Make every item in the `card` Tabs variant retain
a card-shaped fill across Svelte, React, shared Rust composition, and GPUI.

## Why It Matters

The selected card currently reads as a card while inactive peers float. The
operator wants all card items to retain the semantic surface fill, with the
existing `activeFill` axis continuing to own selected tint, solid, or none.

## Current State

g18.003 is independently reviewing as PR #237 from head
`59a744a3f6de046226d2c21d5d00ce887d32818a`. Its rich-text, package-manifest,
and distribution paths are distinct from this Tabs implementation. The
operator approved concurrent dispatch. Queue owns integration ordering and
closeout against current main.

## Boundaries

Follow the task exactly. Use `color.background.surface` for inactive card item
wrappers and no border. Selected tint and solid replace that base; selected
none retains it. Do not add a public prop, token, or variant, paint only the
inner label button, change Tabs interaction, touch rich-text/package-manifest
paths reserved to g18.003, run a windowed selector, or start a release.

Generated output must be Tabs-scoped. If a generator would rewrite g18.003
owned surfaces or require an unresolved merge, stop and report the exact path
collision rather than overwriting concurrent work.

## Important Context

Read the task, Tabs component contract, working rules, system shape, compiled
web distribution contract, existing appearance recipes, Rust Tabs spec/render
mapping, focused tests, and Effigy inventory before editing. Preserve disabled,
pinned, reorderable, closable, vertical, full-width, overflow, drag, focus, and
panel behavior. Pill and block must remain unchanged.

## Suggested Next Move

Plant paired computed-style and Rust node assertions first: inactive and active
card pairs, closable wrapper ownership, disabled state, tint/solid precedence,
and `activeFill="none"` across card versus pill/block. Then make the smallest
shared recipe and Rust mapping change.

## Completion Protocol

Run focused Tabs suites across core, Svelte, React, shared Rust, and headless
GPUI, plus relevant package/distribution/docs checks and `git diff --check`.
Refresh only Tabs-scoped generated docs/evidence and the strictly required
receipt source-commit repin; make no new visual claim. Commit, push, open one
non-draft PR from the queue-owned branch, and report `ready_for_review` with PR
number, exact head, tests, semantic mapping, and any concurrent-path conflict.
Leave the tracked tree clean. Never merge, capture a window, tag, or publish.
