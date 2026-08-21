# Roadmaps

Status: reference
Updated: 2026-08-21

Roadmaps record Poodle's executable milestone work. `g15` is the active
v0.2.0 release-baseline generation.

## Current State

- [Generation index](generation-index.md) is the canonical status summary.
- `g14` tested executable component conformance across Svelte, React, and
  GPUI. `g14.008` rejected the mechanism after its cost and coverage audit;
  `g14.021` preserved the useful fixes and removed the failed authority;
  `g14.022` completed the closeout. The generation is complete.
- `g15` is the release-first v0.2.0 runway. The 175-component Svelte and React
  implementation/evidence rosters, measured native declaration/specimen
  baseline, defect-led specimen curation, native specimen probe, and four of
  six screen-clear reviews are complete. `g15.041` is in flight to close the
  Popover blocker returned by `g15.032`; `g15.033` is the last page-review
  child. The remaining release path is explicit: Stepper and native
  presentation blockers (`042`–`043`), offscreen primitive conformance
  (`044`–`047` under parent `012`), packed roster and release automation
  (`048`–`049`), one exact candidate (`050`), then operator gate `013`.
  Jetstream remains deferred.
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
- `g15` — active v0.2.0 release baseline; `g15.001`–`g15.010`,
  `g15.014`–`g15.017`, `g15.019`–`g15.026`, `g15.028`–`g15.031`, and
  `g15.034`–`g15.040` complete; `g15.041` in flight; `g15.042` and `g15.044`
  ready as independent lanes; range recompiled through `g15.050`; `g15.013`
  final operator gate

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
