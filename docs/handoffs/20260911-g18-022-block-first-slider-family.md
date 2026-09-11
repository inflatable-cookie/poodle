---
title: g18.022 block-first Slider family
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
queue_approval: "Tom explicitly approved dispatch in the Poodle Chatterbox thread on 2026-09-11."
queue:
  dependsOn: []
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.022, slider, range-slider, gpui]
---

## What This Thread Was Doing

Execute [`g18.022`](../roadmaps/g18/022-block-first-slider-family.md) as the
breaking pre-v1 completion of the Slider family. Make block the default for
Slider and RangeSlider, retain only embedded as the alternate, finish vertical
block parity, and replace RangeSlider's moving/fallback text.

## Why It Matters

g18.017 fixed only horizontal Slider text and the family corner radius.
RangeSlider still moves or drops text according to thumb geometry, both block
controls reject vertical orientation, and the public API exposes two
overlapping switches whose old track presentation remains the default. These
gaps must close before `0.4.0` is cut.

## Current State

Planning is based on pushed main `4781446d4`. g18.017 and g18.021 are merged.
The existing g18.011 editor sweep is running independently in its retained
Queue task/workspace. Retained g18.006 release-candidate work and g18.009
publication remain closed. This task may execute beside g18.011 but neither
lane may mutate the other.

## Boundaries

Follow g18.022 exactly. Replace both public APIs with
`variant="block" | "embedded"`, default block, and remove `appearance`,
`standard`, `track`, and RangeSlider's combined visible-range formatter without
aliases. Block must support horizontal and vertical orientations; Slider block
must support unipolar and bipolar; RangeSlider must keep endpoint values and
its label at fixed scale-aligned
anchors. Preserve embedded behavior and all value, commit, focus, and
accessibility semantics. Do not touch editor files, g18.011 state, release
state, versions, changelog, workflows, Desktop, or unrelated controls.

## Important Context

### UI Design Brief

Use Precision Workbench Operate mode. Block is a rounded-square surface capsule
with accent-selected paint, surface remainder paint, and small circular
handles; embedded is the quiet dense track alternative. Horizontal
RangeSlider is lower value at logical start, label center, upper value at
logical end. Vertical Slider is value top, label center; vertical RangeSlider
is upper value top, label center, lower value bottom. Text stays upright and
spatially fixed.
Selected/remainder clipping may invert brightness at a fill crossover, but no
state may move, hide, or externalize text. Prove narrow fit, extrema, equality,
overlap, RTL, both polarities, every size, both themes, and forced colours.

The fixed anchors use the whole capsule. Required numeric text wins over the
optional label. Horizontal RangeSlider endpoints mirror with the existing RTL
scale. Native specs carry resolved strings. Delete the obsolete RangeSlider
fallback and old API branches rather than leaving unreachable compatibility
code.

## Suggested Next Move

Start with compile/parser failures for the new two-value variant API and layout
failures for fixed horizontal/vertical text. Consolidate the shared visual
state and layout law before mirroring the web and GPUI renderers, then update
the paired specimens.

## Completion Protocol

Open one non-draft PR from the queue-owned branch. Prove its exact head with
focused TypeScript/Rust/GPUI tests, paired Svelte/React real-browser specimens,
accessibility and forced-colour checks, embedded regression coverage, package
build/audit, docs QA, and `git diff --check`; then report `ready_for_review`.
Never merge, mutate or restart g18.011, resume g18.006/g18.009, release,
publish, edit Desktop, or run windowed selectors without separate approval.
