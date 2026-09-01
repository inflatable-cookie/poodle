---
title: Icon morph native feasibility spike
kind: northstar-handoff
handoff_mode: worker-pr-loop
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-221757-icon-morph-native-feasibility.md
base_required: pushed-main
tags: [research, icon, morph, gpui, feasibility]
---

## Objective

Run the bounded disposable GPUI 0.2.2 feasibility spike already approved by
the post-motion queue. Determine whether Poodle can interpolate curated icon
pair geometry on native without admitting arbitrary raw SVG or creating a
second icon system. Produce one durable dossier; commit no spike code.

## Settled Boundary — Do Not Re-Ask

- Feasibility comes before any public `IconMorph` contract or API.
- The likely public boundary is a curated semantic pair registry; arbitrary
  path input is rejected.
- Existing `Icon` stays static. Static endpoint swap or cross-fade is the safe
  fallback.
- MotionPolicy full/reduced/frozen is authoritative from g16.034.
- No Morphicons runtime dependency, copied geometry, automatic animation on
  icon-name changes, or Jetstream admission.

## Required Spike

Use disposable, untracked work only. Test the smallest representative curated
pair geometry needed to answer:

- whether GPUI 0.2.2 can construct and update dynamic paths per frame;
- frame pacing and retained-tree invalidation cost;
- compatible command/point normalization and what pair constraints are
  required;
- fill, stroke, colour, reduced/frozen endpoint, interruption, reversal,
  replacement, and teardown behaviour;
- whether the current Poodle icon registry can own provenance and pair
  identity without storing consumer paths; and
- what deterministic headless evidence is possible without claiming visual
  equivalence from structural tests.

Record negative results and static/cross-fade fallback honestly. Do not leave
tracked source, generated assets, samples, or dependencies behind.

## Authority And Evidence

- `docs/triage/20260901-125758-post-motion-research-queue.md`
- `docs/research/value-tracks/icon-morphing.md`
- `docs/architecture/012-semantic-motion-policy.md`
- current icon contracts, registry/audit, shared Rust node vocabulary, GPUI
  adapter, and GPUI 0.2.2 dependency source

Use pinned primary sources for any external technical or licensing claim.
Mutable product pages must be labeled mutable. Do not copy Morphicons data.

## Write Scope

Create exactly one file:

`docs/research/value-tracks/icon-morphing-native-feasibility.md`

Disposable spike work may live only in an ignored research area inside the
dedicated worktree and must be removed before handoff. Do not edit product
code, dependencies, contracts, architecture, specs, roadmaps, existing
dossiers, front doors, or `PAPERCUTS.md`. Do not merge.

## Worker Shape

- Worker class: mechanical research. Long inspection/spike/evidence work,
  suitable for the configured Luna grind profile; no frontier-worker
  justification applies.
- Ready-frontier shape: independent of g16.036, block-slider planning, and
  DesEngs dossier review.
- Serial edge: any contract or implementation card waits for this dossier and
  an operator decision on the fallback/native admission result.

## Validation And Completion

Run only narrow compile/tests needed for the disposable spike plus
`effigy docs:lint` and `git diff --check origin/main...HEAD`. Confirm the final
diff is the single dossier and the worktree has no disposable residue. Commit,
push, open a PR, and report exact head, evidence limits, recommendation, and
next decision. The orchestrator owns exact-head review and merge.

