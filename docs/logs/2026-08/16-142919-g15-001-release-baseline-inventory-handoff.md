---
title: g15.001 release-baseline inventory handoff
status: active
owner: Poodle core
updated: 2026-08-16
tags: [coordination, handoff, g15, release]
---

## What This Thread Was Doing

The orchestrator closed g14 after rejecting and removing its executable
conformance architecture, then opened g15 as the release-first runway for
Poodle v0.2.0. This handoff starts the first g15 worker: measure the complete
public Svelte roster, record exact evidence and gaps across the active
runtimes, and compile the rest of the generation from those findings.

## Why It Matters

Longhorn and most projects under `~/Dev/projects` now depend on Poodle. The
release cannot wait for another speculative cross-runtime architecture, but it
also cannot claim support from a representative sample. g15.001 freezes the
real denominator and converts unknown release risk into an owned, executable
runway.

## Current State

- Done so far: PR23 merged as `d02e9410`; g14 is complete; the rejected corpus,
  comparator, portable-interface plane, and generated Rust declarations are
  gone. Component contracts, owner-local tests, the headless GPUI regression
  platform, and human-centred specimen boundary remain.
- Still open: execute g15.001 in full. Produce the frozen roster, per-surface
  evidence, release-gap register, August log, and measured family-tranche
  roadmap cards. Do not implement any gap in this worktree.
- Active spec lane: none. This is release inventory and roadmap compilation,
  not architecture or provisional-spec work.
- Canonical refs:
  - `/Users/tom/Dev/projects/poodle/docs/contracts/001-working-rules.md`
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/README.md`
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/001-release-baseline-roster-inventory.md`
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/conformance-estate.md`
  - `/Users/tom/Dev/projects/poodle/docs/logs/2026-08/16-g14-022-generation-closeout.md`
- Remaining continuation envelope: g15.001 only. The card may compile the
  measured follow-on roadmap files and list them in the g15 README, but must
  not execute or dispatch them.
- Lane budget / pause signal: one documentation/evidence PR. Stop after the
  inventory, gap register, measured runway, validation, and handback.
- Key files:
  - `/Users/tom/Dev/projects/poodle/packages/svelte/components/src/index.ts`
  - `/Users/tom/Dev/projects/poodle/packages/svelte/components/package.json`
  - `/Users/tom/Dev/projects/poodle/packages/react/components/src/index.ts`
  - `/Users/tom/Dev/projects/poodle/packages/contracts/components/src`
  - `/Users/tom/Dev/projects/poodle/packages/render/src`
  - `/Users/tom/Dev/projects/poodle/packages/gpui/preview/src`
  - `/Users/tom/Dev/projects/poodle/packages/svelte/preview/src`

## Boundaries

- Execute only
  `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/001-release-baseline-roster-inventory.md`.
- Start from `d02e9410` or a later clean `main` containing PR23.
- Do not change component or runtime source, public APIs, tests, package
  exports, curated specimens, generated artifacts, task configuration,
  `.github/workflows/`, Jetstream, or any downstream repository.
- Downstream inspection is read-only contextual evidence. A component is not a
  release failure merely because no consumer use was found.
- Do not design a replacement interface authority, shared corpus, normalized
  comparator, universal specimen language, or completion gate.
- Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
  any Jetstream selector. Use only the headless Effigy surfaces named by the
  card.
- Follow repo constraints from
  [AGENTS.md](/Users/tom/Dev/projects/poodle/AGENTS.md).

## Important Context

- Planning lineage: g13's Rust-authored component IR was revised then unwound;
  g14's executable conformance plane was rejected after a cost and coverage
  audit, then removed. g15 must not smuggle either mechanism back under new
  names.
- Spec-to-canonical relationship: component contracts and working rules govern
  semantics. The Svelte implementation is the reference when runtimes drift.
  Curated specimens teach humans; they are not exhaustive test matrices or
  parity snapshots.
- The one-card runway is deliberate. g15.001 is the denominator and planning
  pass. It must compile the rest of the visible runway from measured gaps,
  normally covering these outcome classes in evidence-led order:
  1. Svelte release blockers: missing contract, export, implementation,
     specimen, focused evidence, or packed-consumer reachability across the
     full public roster.
  2. React mirror gaps that can close through the existing shared web CSS and
     framework-free behaviour substrate.
  3. Shared Rust composition and GPUI gaps, grouped into bounded component
     families. Include the carried Licence and model-connection requirements;
     prioritize primitives and dependencies before composites.
  4. The carried human-centred specimen catalogue audit, kept separate from
     exhaustive conformance evidence.
  5. A later bounded, primitive-first visual-conformance lane using retained
     headless capture infrastructure. It is a diagnostic aid, not a new
     component authority or release prerequisite invented by this card.
  6. Final v0.2.0 package, documentation, and release certification after all
     Svelte-denominator blockers are closed.
- Do not force one roadmap file per component. Compile coherent family
  tranches with explicit dependencies, acceptance, writable scope, headless
  validation, and stop conditions. The inventory decides how many are needed.
- A `complete` posture requires exact component-local evidence. Aggregate
  selectors may be recorded as board health, but they do not prove an
  individual component or permit one runtime to borrow another's pass.
- Svelte v0.2.0 certification and full active-cohort parity are different
  claims. The release denominator is the full Svelte roster; missing React,
  Rust, or GPUI surfaces remain explicit parity gaps, and experimental package
  labels remain honest.
- Jetstream is program-deferred. Do not inspect it to manufacture per-component
  completion or require a sibling checkout.
- `effigy qa` most recently passed every lane except a pre-existing `bun audit`
  failure for the nanoid advisory reached through the React preview's Vite
  dependency. Run the board and report the current result; do not assume the
  advisory is still present, bypass it, or expand this docs-only card to fix it.
- `effigy doctor` has known generated-in-src, god-file, stale-suppression, and
  comment-ratio findings. Record the current baseline without fixing it.

## Suggested Next Move

Read the complete g15.001 card and its governing refs before collecting data.
Derive the denominator mechanically from component-valued exports in the
Svelte index, then reconcile packed reachability. Build one row per component
with exact paths or test names for every surface; use `missing` rather than
inference. Only after the table and gap register are complete should you group
the measured gaps into ordered family-tranche roadmap cards and update the g15
README runway.

## Completion Protocol

1. Add `docs/roadmaps/g15/release-baseline-roster.md` with the frozen
   denominator, method, count, and exact per-surface evidence.
2. Add `docs/roadmaps/g15/release-gap-register.md` with every incomplete
   surface, disposition, and live owner. Owners may be the measured roadmap
   card IDs created in this PR.
3. Add the bounded follow-on roadmap cards required by the measured register
   and list them in dependency order in `docs/roadmaps/g15/README.md`. Leave
   them blocked pending orchestrator review; do not dispatch or implement
   them.
4. Do not change roadmap status lines or `docs/roadmaps/dispatch.md`; those are
   orchestrator-owned.
5. Write one August g15.001 batch log recording the inventory method,
   denominator, key counts, uncertainties, generated runway, and exact
   validation results.
6. Run the card's headless Effigy validation. Never run windowed/native-visual
   or Jetstream selectors. Record known baseline failures honestly rather than
   fixing unrelated debt.
7. Run `git diff --check`, commit and push the worktree branch, and open or
   update one PR. Do not merge it.
8. Return the PR number, denominator count, gap counts by surface/runtime, the
   proposed first executable tranche, validation evidence, and unresolved
   judgment calls to the orchestrator.
