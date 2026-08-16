# Roadmaps

Status: reference
Updated: 2026-08-16

Roadmaps record Poodle's executable milestone work. `g15` is the active
v0.2.0 release-baseline generation.

## Current State

- [Generation index](generation-index.md) is the canonical status summary.
- `g14` tested executable component conformance across Svelte, React, and
  GPUI. `g14.008` rejected the mechanism after its cost and coverage audit;
  `g14.021` preserved the useful fixes and removed the failed authority;
  `g14.022` completed the closeout. The generation is complete.
- `g15` is the release-first v0.2.0 runway. The release-baseline roster
  inventory is complete; `g15.002`, the first focused-evidence tranche, is
  ready for dispatch. Jetstream remains deferred.
- `g13` is complete. Its Rust-authored component IR pilot recorded **revise**,
  then retired and unwound component generation. It remains evidence for g14.
- The first g14 machine-pinning/scene runway was reset after five merged
  batches. Its history is archived in
  [the false-start record](archive/2026-08-14-g14-machine-pinning-false-start.md).
- `g09`–`g12` are complete. Earlier generations remain historical evidence.
- Release automation is tracked separately from roadmap implementation work.

## Generations

- `g01` — foundations, tokens, contracts, primitives, and first Underlay bridge
- `g02` — advanced composites, documentation depth, cleanup, release baseline
- `g03` — hardening, migration, parity automation, downstream adoption
- `g04` — component parity, specialist families, media, editing surfaces
- `g05` — GPUI foundation and cross-runtime parity baseline
- `g06` — shared renderer contracts, typed tokens, layout, events
- `g07` — GPUI rendering build-out and adapter expansion
- `g08` — GPUI production quality and compliance
- `g09` — native package consolidation and semantic sizing/density
- `g10` — Jetstream feasibility and GPUI production hardening
- `g11` — Svelte modernization and shared web machinery
- `g12` — React parity, verification depth, native hardening, audio family
- `g13` — retired Rust-authored component/scene IR pilot
- `g14` — rejected executable-conformance pilot; generation complete
- `g15` — active v0.2.0 release baseline; `g15.002` ready

## Rules

- Active milestone files live in generation folders such as `g14/`.
- File names use `NNN-slug.md`; roadmap IDs use forms such as `g14.001`.
- A g14 roadmap file is a complete worker handoff. Do not add a batch-card
  layer.
- Backlog items belong in `backlog/`.
- Architecture belongs in `../architecture/`, not here.
- Generation rollover is manual. Close, pause, supersede, or rehome every live
  roadmap before opening the next generation.
- Purge stale strict-planning artifacts from `../specs/` during rollover.

## Reading Roadmaps

Start with the [generation index](generation-index.md), then open the active
generation README and relevant milestone. Prefer current architecture,
contracts, specs, and implementation when historical cards describe an older
package shape.
