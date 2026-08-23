# Roadmaps

Status: reference
Updated: 2026-08-23

Roadmaps record Poodle's executable milestone work. `g15` completed the
v0.2.1 recovery release after `v0.2.0` stopped before publication. `g16` is
the active adoption generation, moving authoritative consumers onto the
published packages and matching Rust tag.

## Current State

- [Generation index](generation-index.md) is the canonical status summary.
- `g16` has frozen the 17-repository consumer estate and the exact `0.2.1`
  adoption policy. Longhorn, Underlay, and Soundcheck Library are ready as
  independent foundation lanes; application cards follow their results.
- `g14` tested executable component conformance across Svelte, React, and
  GPUI. `g14.008` rejected the mechanism after its cost and coverage audit;
  `g14.021` preserved the useful fixes and removed the failed authority;
  `g14.022` completed the closeout. The generation is complete.
- `g15` was the release-first runway. The 175-component Svelte and React
  implementation/evidence rosters, measured native declaration/specimen
  baseline, specimen curation and review, native specimen probe, packed roster,
  the first primitive fixture inventory, and truthful release automation are
  complete. PR #68 closed the exact Button comparison; PR #69 then closed its
  measured native focus-ring defect and Stepper keyboard-entry gap. PR #66
  closed `g15.049`; PR #67 closed the GPUI/Zed dependency-licence policy gap
  without admitting GPL code. Those lanes and the `0.2.0` candidate completed;
  its workflow then failed before publication. Card `054` produced green
  replacement candidate `3d914261`; completed gate `013` tagged and published
  it as `v0.2.1` in run `32658293188`. Core and Svelte are on npm `latest` at
  `0.2.1`; React remains source-only. The generation is complete.
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
- `g15` — completed v0.2.1 recovery baseline; core and Svelte published from
  candidate `3d914261`, with React retained as source-only
- `g16` — active published-consumer adoption; foundation lanes ready

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
