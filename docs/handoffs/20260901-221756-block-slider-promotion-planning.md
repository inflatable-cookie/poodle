---
title: Block Slider and RangeSlider promotion planning
kind: northstar-handoff
handoff_mode: planning-delegate
planning_mode: conversational-discovery
dispatch_authority: orchestrator
promotion_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-221756-block-slider-promotion-planning.md
base_required: pushed-main
tags: [planning, slider, range-slider, appearance, promotion]
---

## Objective

Resolve only the remaining public-contract choices for the already approved
additive block appearance on Slider and RangeSlider. Produce one operator-
reviewed decision packet. Do not implement, promote a roadmap card, or repeat
questions the operator already answered.

## Settled Decisions — Do Not Re-Ask

- The appearance is additive. Current standard and embedded defaults remain.
- Visible label/value content is separate from accessible names/value text.
- Inline content appears only when it fits; one stable fallback readout owns
  the narrow case.
- RangeSlider targets the nearest thumb at pointer-down and holds that thumb
  for the gesture.
- The visible thumb may be small; the effective target remains measurable at
  Poodle's adopted minimum.
- Full vertical admission waits for real native RangeSlider geometry.
- Motion policy is already authoritative in architecture 012 and g16.034.
- Jetstream remains deferred. No default migration is in scope.

## Decisions Still Open

Settle these as one coherent contract, using the dossier recommendation where
the operator has no contrary preference:

1. Exact public appearance name and whether it is one shared Slider/
   RangeSlider field or a component-specific variant.
2. Visible label/value formatter inputs, deterministic fit threshold, and
   stable fallback-readout placement.
3. PageUp/PageDown, RTL direction, exact overlap tie, and cancellation/lost-
   capture commit law.
4. Forced-colour roles, effective-target evidence, and native per-thumb
   min/max/value-text prerequisites.
5. Confirm invalid/read-only states stay wrapper-owned rather than entering
   this appearance.

Ask compact question groups only when repository evidence cannot resolve a
choice. Never ask again whether the appearance is additive, whether labels are
separate from ARIA, how nearest-thumb ownership works, or whether vertical is
admitted now.

## Authority And Evidence

- `docs/triage/20260901-125758-post-motion-research-queue.md`
- `docs/research/value-tracks/block-slider-visual-direction.md`
- `docs/architecture/012-semantic-motion-policy.md`
- `docs/contracts/components/slider.md`
- `docs/contracts/components/range-slider.md`
- `docs/contracts/components/size-and-density.md`
- current Svelte, React, shared Rust, renderer, and GPUI Slider/RangeSlider
  implementations and focused evidence

The queue note and dossier are evidence, not execution authority. Current
contracts remain authoritative until later promotion.

## Write Scope

Create exactly one packet:

`docs/triage/20260901-221756-block-slider-promotion-decision.md`

Do not edit code, contracts, architecture, roadmaps, specs, other triage,
research dossiers, front doors, or `PAPERCUTS.md`. Do not merge.

## Worker Shape

- Worker class: planning delegate. Frontier planning is justified by the
  cross-runtime public API, interaction, accessibility, and evidence choices;
  implementation remains ordinary day-to-day work after the packet is
  promoted.
- Ready-frontier shape: independent of g16.036 Tree authority and the DesEngs
  research lanes.
- Serial edge: implementation cannot launch until this packet is accepted,
  promoted into contracts/card, and assigned a same-repository merge order.

## Completion

The packet must separate settled decisions, new operator decisions,
recommendations, alternatives, explicit non-goals, required oracle rows,
proposed canonical destinations, and any genuinely unresolved question. Run
`effigy docs:lint` and `git diff --check origin/main...HEAD`; confirm one-file
scope; commit, push, and open a PR. The orchestrator owns review, merge, and
promotion.

