# Roadmaps

Status: reference
Updated: 2026-08-11

Roadmaps record Poodle's executable milestone work. `g13` is active.

## Current State

- [Generation index](generation-index.md) is the canonical status summary.
- `g13` is the Rust-authored component and scene IR program. Its bounded pilot
  (`001`–`008`) is complete and recorded **revise**: the IR is kept as one
  source for cross-runtime vocabulary with drift gating, the behavioural
  ambition is dropped, and broad component migration (`009`–`016`) is closed.
- The former single-card g13–g15 audio generations are consolidated into
  `g12.025`–`027`. `g12` is closed.
- `g09`, `g10`, `g11`, and `g12` are complete.
- The GPUI node-backend migration is complete and the duplicate GPUI component
  tier has been removed.
- The current docs gate has inherited inventory drift; `g13.001` must repair
  it before IR schema implementation begins.
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
- `g12` — React parity, verification depth, native hardening, package
  consolidation, and the complete audio-family delivery
- `g13` — Rust-authored component/scene IR pilot and gated cross-runtime rollout

## Rules

- Active milestone files live in generation folders such as `g13/`.
- File names use `NNN-slug.md`; roadmap IDs use forms such as `g13.001`.
- Backlog items belong in `backlog/`.
- Architecture belongs in `../architecture/`, not here.
- Generation rollover is manual. Close, pause, supersede, or rehome every live
  roadmap before opening the next generation.
- Purge stale strict-planning artifacts from `../specs/` during rollover.

## Reading Roadmaps

Start with the [generation index](generation-index.md), then open a generation
README and the relevant card. Prefer current architecture, contracts, specs,
and implementation when a historical card describes an older package shape.
