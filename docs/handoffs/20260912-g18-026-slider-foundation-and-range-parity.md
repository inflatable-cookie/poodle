---
title: g18.026 Slider foundation and RangeSlider parity
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
tags: [coordination, handoff, worker, g18, g18.026, slider, range-slider, foundation]
---

## What This Thread Was Doing

Execute [`g18.026`](../roadmaps/g18/026-slider-foundation-and-range-parity.md)
as the final Slider-family product repair before `0.4.0`. Keep the two public
components separate while extracting one private rendering foundation and
bringing RangeSlider up to the accepted Slider behavior.

## Why It Matters

The current duplicated web renderers let several post-g18.024 Slider fixes miss
RangeSlider. Bovine Desktop is waiting on the release, so the family needs one
maintainable seam now rather than another manual copy.

## Current State

Main contains completed g18.022–g18.025 plus direct accepted Slider/header
corrections through the planning base. Core already shares Slider-family math
and preserves separate one-thumb/two-thumb machines. The duplication is mainly
Svelte/React structure, pointer presentation and 375/464-line style surfaces.
Retained g18.006 is blocked and must remain untouched.

## Boundaries

Follow g18.026 exactly. Preserve separate public APIs, value types, callbacks,
focus models and ARIA. Share only private mechanics and presentation. Stop on
any required public change. Do not edit versions, locks, changelog, release
notes, workflows, Desktop, g18.027 or retained Queue task state. Never run a
windowed selector without separate approval.

## Important Context

### UI Design Brief

Slider and RangeSlider are one visual family, not one semantic control. Use the
same size ladder, axis geometry, capsule, fill, handle, focus, hit-target and
text-paint machinery. RangeSlider composes the handle twice and retains its
ordered-pair machine and two focus stops. Prove block/embedded,
horizontal/vertical, unipolar/bipolar, extrema, uneven step, equality, overlap,
RTL, disabled, both themes and forced colours in both web frameworks. Existing
contracts settle the experience; do not redesign it.

## Suggested Next Move

Plant failures for every accepted Slider behavior that RangeSlider currently
misses. Then extract the smallest private foundation under those tests before
changing the RangeSlider shells and styles.

## Completion Protocol

Open one non-draft PR from the queue branch. Prove the exact head with focused
core/component/native checks, paired Chromium/WebKit geometry and interaction,
accessibility, package and preview builds, docs QA and `git diff --check`.
Report `ready_for_review`; never merge or resume release work.
