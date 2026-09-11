---
title: g18.017 block Slider fixed inline presentation
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
queue_approval: "Tom rejected the pill-ended, value-dependent external block Slider text treatment on 2026-09-11 and said Go for it after approving rounded-square family corners and fixed split-colour in-track Slider text."
queue:
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.017, slider, block, visual]
---

## What This Thread Was Doing

Execute
[`g18.017`](../roadmaps/g18/017-block-slider-fixed-inline-presentation.md).
Give block Slider and RangeSlider rounded-square control corners, then keep a
single Slider's label/value fixed inside the track with split-colour crossover.

## Why It Matters

The current pill geometry misses the intended block shape. More seriously, the
single Slider assigns text to value-sized regions and ejects both strings to an
external fallback whenever either region becomes too narrow. The interface
therefore jumps as the thumb moves.

## Current State

Web uses `999px`; Rust rendering uses `radius.pill`. The shared TS/Rust fit law
is explicitly value-dependent and all-or-nothing. Svelte, React and GPUI mirror
that contract. g18.011 is held; g18.006 and g18.009 remain held. This task is
file-independent of the active editor repairs.

## Boundaries

Follow g18.017 exactly. Use `radius.control` for both block family capsules.
For single Slider only, pin label/value to logical edges, duplicate and clip the
text paint across the selected boundary, and keep exact value text in-track
under collision. Preserve interaction and accessibility semantics. Do not add
props, redesign RangeSlider text, change tokens broadly, release, or touch
Desktop.

## Important Context

The visible thumb remains circular and above pointer-inert text. The split paint
must work in LTR, RTL and forced colours. Narrow width suppresses the optional
visible label before the exact numeric value; it never creates an external
fallback. If existing GPUI clipping cannot express the preferred treatment,
use the operator-approved cross-runtime fallback: one combined in-track group
on the larger side of the thumb with a fixed midpoint tie rule. Record the
substrate evidence; never restore the external row.

## Suggested Next Move

Plant low/mid/high paired browser and GPUI layout evidence, then model the stable
whole-track text geometry before changing render structure or radius tokens.

## Completion Protocol

Open one non-draft PR from the queue-owned branch, prove the exact head with
focused TS/Rust/headless/mounted/browser/visual checks, paired preview builds,
contrast/accessibility checks, docs QA and `git diff --check`, then report
`ready_for_review`. Never merge, release g18.011, resume g18.006/g18.009,
publish, or mutate Desktop.
