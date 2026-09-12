---
title: g18.024 Slider-family layout and vertical repair
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-12
updated: 2026-09-12
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom explicitly approved dispatch of the Slider updates on 2026-09-12, then added shared-size alignment, step-aware number formatting, and broken vertical block geometry to the same scope."
queue:
  dependsOn: []
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.024, slider, range-slider, layout, vertical]
---

## What This Thread Was Doing

Execute [`g18.024`](../roadmaps/g18/024-slider-family-layout-and-vertical-repair.md)
as the release-blocking post-g18.022 correction. Align block Slider and
RangeSlider to the shared control-size axis, remove accessibility-envelope
whitespace from layout, serialize default values sensibly, and repair vertical
block geometry across active runtimes.

## Why It Matters

The new block family is the correct public direction, but its private size
ladder does not align with other components, its 44px hit envelope creates
visible top/bottom whitespace, raw floating-point values leak into the UI, and
the vertical specimens are visibly clipped and unusable. These are baseline
component defects and block `0.4.0`.

## Current State

Integration main contains merged g18.022 at `02ab7f7ec` and its closeout at
`510b281ba`. The operator approved this repair on 2026-09-12. g18.023 is an
independent CodeEditor tokens lane and may run concurrently. Retained g18.006
and downstream g18.009 remain closed until both repairs merge.

## Boundaries

Follow g18.024 exactly. Repair Slider and RangeSlider together in Svelte,
React, shared Rust composition, and GPUI wherever the contract is affected.
Keep the block/embedded API, value machine, polarity, callbacks, accessibility,
and consumer formatter surface intact. Do not add aliases or a new public
formatter/size API. Do not edit g18.023, release state, versions, changelog,
workflows, Desktop, or unrelated components.

## Important Context

### UI Design Brief

The visible block capsule is the layout box and consumes Poodle's shared
24/28/36/44/52px control ladder. It aligns edge-for-edge beside same-size
Button, Input, and Select controls. The ≥44×44 effective handle target remains
measurable and interactive but does not add row height, padding, or margin;
density cannot mask the chosen size.

Default value text is a short step-aware decimal: round the normalized snapped
value to the precision implied by `min` and a finite positive `step`, trim
insignificant zeroes, normalize negative zero, and never expose binary tails.
The existing custom formatter always wins.

Vertical block controls use native-axis geometry. Host height is explicit;
shared size controls rail width and internal metrics. Text stays upright and
inside: Slider value top/label center; RangeSlider upper top/label center/lower
bottom. Track, fill, handles, focus, hit targets, clipping, pointer coordinates,
and text share the same vertical frame. Prove both web frameworks in Chromium
and WebKit and prove shared-Rust/GPUI render state without a windowed selector.

## Suggested Next Move

Plant measured failures against merged main for the five shared heights, mixed
control alignment, layout-neutral hit rectangles, fractional default strings,
and the current vertical specimens. Fix the shared value/visual law first,
then mirror layout changes across renderers before broad validation.

## Completion Protocol

Open one non-draft PR from the queue-owned branch. Prove its exact head with
focused core/component/native tests, paired Chromium/WebKit geometry and
interaction checks, headless GPUI render-state proof, package/preview builds,
accessibility checks, docs QA, and `git diff --check`; then report
`ready_for_review`. Never merge, resume g18.006/g18.009, release, publish, or
mutate Desktop.
