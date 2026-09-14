---
title: g18.036 Slider-role drag boundary
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar Chatterbox
created: 2026-09-14
updated: 2026-09-14
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom asked on 2026-09-14 to assess the INTERACTIVE_SELECTOR role=slider papercut and promote/queue it when the canonical Poodle runway was ready."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: false
tags: [coordination, handoff, worker, g18, g18.036, drag-drop, slider]
---

## What This Thread Was Doing

Execute [`g18.036`](../roadmaps/g18/036-slider-role-drag-boundary.md) as the
next bounded finding from Poodle's post-0.4.0 consumer sweep.

## Why It Matters

Poodle's shared drag controller correctly ignores native controls and
`role="button"` descendants of whole-row sources, but it omits
`role="slider"`. Loophole therefore needs a consumer-side no-drag boundary so
its Fader cannot accidentally begin a panel drag. This is a reusable Poodle
interaction defect, not Loophole application policy.

## Current State

Main is clean and synchronized at the planning commit carrying this handoff.
`INTERACTIVE_SELECTOR` is the proven seam. Spec 069 already requires
interactive descendants to win over a whole-row drag source, architecture 011
keeps continuous slider/fader gestures outside payload drag sessions, and
Fader/Knob already expose `role=slider`. Queue inspection found no unfinished
Poodle or Loophole lane.

## Boundaries

Change only the core selector, its focused DOM regression, and spec 069's exact
inventory. Protect the role host and its nested descendants while proving
ordinary source chrome still drags. Do not edit Loophole, remove its workaround,
inventory other ARIA roles, add a public option, touch Svelte/React component
implementations, change native/GPUI behavior, edit workflows, or perform a
release mutation.

## Important Context

The desired behavior is settled: a slider owns its continuous pointer gesture.
Use the existing `closest(...)` interactive-host check rather than adding a
component-specific Fader or Knob exception. Preserve
`data-poodle-no-drag` for custom non-semantic surfaces. The task's UI brief is
a behavior-only refinement with no visible change.

Validation budget: run the focused
`bun test test/headless-dom/drag-drop-controller.test.ts` during implementation,
then one final `effigy ci:web` and `git diff --check`. Do not add overlapping
`test:components`, `docs:check`, or `qa` boards after a green `ci:web`.

## Suggested Next Move

Plant the role-host and nested-child counterexamples first, including a plain
chrome control in the same fixture, then make the one selector correction.

## Completion Protocol

Push one clean non-draft PR from the Queue workspace. Report the exact focused
test and final web-board results as `ready_for_review`, then stop. Queue owns
independent exact-head review, CI observation, merge, and hook closeout. Leave
Loophole and npm release state untouched.
