# Roadmaps

Status: reference
Updated: 2026-08-26

Roadmaps record Poodle's executable milestone work. `g15` completed the full
v0.2.x release programme: the roster and parity baseline, failed v0.2.0
publication, v0.2.1 recovery, crates.io-GPUI correction, v0.2.2 publication,
and adoption by all 16 authoritative consumers. `g16` begins with a current
active-cohort parity evidence ledger before any new conformance runway is
chosen.

## Current State

- [Generation index](generation-index.md) is the canonical status summary.
- `g16.001` is complete and operator-reviewed in PR #75. It repairs stale
  parity reporting and produces one component-level evidence ledger. `g16.002`
  closed — partial outcome: mounted GPUI behaviour for Checkbox, Switch, and
  SegmentedControl. PR #77 closed `g16.003` RadioGroup native identity and
  mounted evidence. `g16.004` closed ToggleGroup resulting-selection, single-mode
  roving focus, and instance-scoped native identity. `g16.005` closed Slider
  axis, keyboard, callback, and mounted parity in PR #79; ledger 34 → 35
  mounted, 140 → 139 missing. `g16.006` closed Tabs drag, keyboard, and mounted
  parity in PR #80; ledger 35 → 36 mounted, 139 → 138 missing. PR #81 closed
  `g16.007` core TextInput controlled editing and one honest mounted claim;
  ledger 36 → 37 mounted, 138 → 137 missing. The checkpoint compiled ready
  `g16.008` to repair generic native text routing before another evidence cell
  moves. No broader conformance programme is implied.
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
  it as `v0.2.1` in run `32658293188`. The broken Git tag was later retracted
  after v0.2.2 replaced its fork-sourced GPUI graph; npm retains 0.2.1 for
  install stability while `latest` is 0.2.2. React remains source-only. The
  corrected v0.2.2 candidate then restored crates.io GPUI and all 16
  authoritative consumers moved to the public boundary. The operator removed
  Loophole Legacy, so its cancelled card is historical evidence rather than a
  release obligation. The generation is complete through `g15.079`.
  Jetstream backend admission remains deferred.
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
- `g15` — completed v0.2.x release and adoption programme; corrected v0.2.2
  published from candidate `d5607def`, with all 16 authoritative consumers
  adopted and React retained as source-only
- `g16` — active evidence-recovery checkpoint; `001` complete and
  operator-reviewed; `002` closed — partial outcome; `003` merged in PR #77;
  `004` merged in PR #78; `005` merged in PR #79; `006` merged in PR #80;
  `007` merged in PR #81; `008` ready

## Rules

- Active milestone files live in generation folders such as `g16/`.
- File names use `NNN-slug.md`; roadmap IDs use forms such as `g16.001`.
- A roadmap file is a complete worker handoff. Do not add a batch-card
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
