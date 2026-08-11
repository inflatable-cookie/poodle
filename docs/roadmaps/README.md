# Roadmaps

Status: reference
Updated: 2026-08-11

Roadmaps record Poodle's executable milestone work. `g15` is complete.

## Current State

- [Generation index](generation-index.md) is the canonical status summary.
- `g15` corrected the Slider/RangeSlider embedded-control boundary and migrated
  ModMatrixGrid onto it.
- `g14` closed the audio family with Keyboard, Waveform Display, and Mod Matrix
  Grid across every in-flight backend.
- `g09`, `g10`, `g11`, `g12`, `g13`, `g14`, and `g15` are complete.
- The GPUI node-backend migration is complete and the duplicate GPUI component
  tier has been removed.
- Web, native, accessibility, visual-parity, and package-consumer gates are
  green.
- Release automation is tracked separately from roadmap implementation work.

Historical generation READMEs and cards preserve the decisions, evidence, and
next-step language that was current when each batch closed. They are not the
operator guide for the present package shape.

## Generations

- `g01` — foundations, tokens, contracts, primitives, and the first Underlay bridge
- `g02` — advanced composites, documentation depth, cleanup, and release baseline
- `g03` — hardening, migration, parity automation, and downstream adoption
- `g04` — component parity, specialist families, media, and editing surfaces
- `g05` — GPUI foundation and cross-runtime parity baseline
- `g06` — shared renderer contracts, typed tokens, layout, and events
- `g07` — GPUI rendering build-out and adapter expansion
- `g08` — GPUI production-quality and compliance programme
- `g09` — GPUI continuation and semantic sizing and density
- `g10` — Jetstream feasibility and GPUI production hardening
- `g11` — Svelte modernization and consumer rollout
- `g12` — React parity, verification depth, native hardening, and package consolidation
- `g13` — audio family parity across Svelte, React, GPUI, and Jetstream
- `g14` — audio family Phase 3 closeout
- `g15` — embedded Slider controls and ModMatrixGrid composition correction

## Rules

- Active milestone files live in generation folders such as `g12/`.
- File names use `NNN-slug.md`; roadmap IDs use forms such as `g12.019`.
- Backlog items belong in `backlog/`.
- Architecture belongs in `../architecture/`, not here.
- Generation rollover is manual. Close, pause, supersede, or rehome every live
  roadmap before opening the next generation.
- Purge stale strict-planning artifacts from `../specs/` during rollover.

## Reading Roadmaps

Start with the [generation index](generation-index.md), then open a generation
README and the relevant card. Prefer current architecture, contracts, specs,
and implementation when a historical card describes an older package shape.
