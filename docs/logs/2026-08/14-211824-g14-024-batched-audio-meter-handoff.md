---
title: Batched AudioMeter web surface handoff
status: active
owner: Poodle core
updated: 2026-08-14
tags: [coordination, handoff, audio, performance]
---

## What This Thread Was Doing

The orchestrator assessed Loophole's high-count console-meter requirement
against Poodle's existing AudioMeter math, web shells, native parity rules, and
test estate. It converted the raw proposal into a contract-first Canvas2D
delivery lane and wrote one complete worker roadmap.

## Why It Matters

Standalone AudioMeter is correct for isolated channel strips and plugin UIs,
but 100+ segment-DOM meters make rendering and update overhead scale with
component count. The new tier keeps one semantic meter contract while batching
state and paint for web consoles. It also proves the draw-pass seam before
Poodle considers a broader WebGL/custom-draw runtime.

## Current State

- Done so far: architecture 008, the AudioMeter contract, and approved spec 068
  now fix ownership, API vocabulary, numeric parity, accessibility, browser
  proof, Canvas2D-first scope, and performance acceptance.
- Still open: all implementation, tests, specimens, browser probes, allocation
  evidence, performance measurement, package exports, and the implementation
  log.
- Active spec lane: none; spec 068 is approved for execution.
- Canonical refs:
  - `/Users/tom/Dev/projects/poodle/docs/architecture/006-headless-core-and-machine-model.md`
  - `/Users/tom/Dev/projects/poodle/docs/architecture/008-audio-control-family.md`
  - `/Users/tom/Dev/projects/poodle/docs/specs/068-batched-audio-meter-surface.md`
  - `/Users/tom/Dev/projects/poodle/docs/contracts/components/audio-meter.md`
- Remaining continuation envelope: exactly g14.024 in one dedicated worktree
  and PR.
- Lane budget / pause signal: finish the roadmap, open the PR, then stop for
  orchestrator review. Do not start a WebGL2 follow-up.
- Key files:
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/024-batched-audio-meter-web-surface.md`
  - `/Users/tom/Dev/projects/poodle/packages/core/src/audio/meter.ts`
  - `/Users/tom/Dev/projects/poodle/packages/svelte/components/src/AudioMeter.svelte`
  - `/Users/tom/Dev/projects/poodle/packages/react/components/src/AudioMeter.tsx`

## Boundaries

- Stay within the writable scope and stop conditions in g14.024.
- Do not add WebGL2, native components, conformance cases, Loophole types,
  gain-reduction batching, plugin asset machinery, or release workflow changes.
- Preserve standalone AudioMeter defaults, markup, behavior, and accessibility.
- All browser checks are headless. Never run a `*-windowed` selector.
- Follow repo constraints from
  `/Users/tom/Dev/projects/poodle/AGENTS.md`.

## Important Context

- Planning lineage: architecture 008 introduced the shared VisualState and
  meter-ballistics seam; g12.025 completed four-runtime AudioMeter parity;
  g14.024 adds only the web paint strategy needed at console scale.
- Spec-to-canonical relationship: spec 068 is the execution contract;
  architecture 008 owns the stable package/runtime boundary; the AudioMeter
  contract owns observable component behavior.
- Decisions and preferences: Canvas2D ships first; the operator explicitly
  chose it over WebGL2-plus-fallback. The 128-meter result decides whether a
  later GPU card is warranted.
- Corrections to the raw proposal: hot frames use numeric registered slots,
  not string channel IDs in Float32; the feed retains `meanSquare` and explicit
  `durationMs`; typed state uses appropriate precision rather than forcing
  timestamps and flags into Float32.
- Open tensions: exact RMS needs a fixed preallocated ring with an honest feed
  limit; surface animation must share pure laws without changing standalone;
  WebKit geometry and palette behavior may expose assumptions hidden by
  Chromium; the hardware-specific `<2 ms` target is review evidence, not a
  general CI threshold.

## Suggested Next Move

Open
`/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/024-batched-audio-meter-web-surface.md`
in the fresh worktree. Start with Batch A and make the standalone-vs-bus golden
trace fail before adding DOM or canvas code. Continue through the card only
while its fixed decisions and stop conditions hold.

## Completion Protocol

1. Complete every accepted g14.024 checkbox or stop with exact evidence.
2. Add one August implementation log with API, allocation proof, Chromium and
   WebKit matrix, performance environment/results, and residual risk.
3. Run the card's headless Effigy validation and packed-install proof.
4. Open one PR from the dedicated worktree. Do not edit roadmap status,
   generation status, dispatch state, or release workflows.
5. Report the PR URL, commits, failed/waived checks, measured p95, and whether
   Canvas2D met the Loophole requirement.
6. Stop. The orchestrator reviews and decides whether any WebGL2 card exists.
