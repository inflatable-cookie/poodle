# Roadmaps

Status: reference
Updated: 2026-09-06

Roadmaps record Poodle's executable milestone work. `g15` completed the full
v0.2.x release programme: the roster and parity baseline, failed v0.2.0
publication, v0.2.1 recovery, crates.io-GPUI correction, v0.2.2 publication,
and adoption by all 16 authoritative consumers. `g16` used a current
active-cohort parity evidence ledger to close bounded semantic and mounted
behavior gaps, published v0.3.0, and took Nucleus to M1/A1 29/29. `g17`
finishes the Nucleus switch evidence.

## Current State

- [Generation index](generation-index.md) is the canonical status summary;
  `dispatch.md` is the live frontier; `g17/README.md` indexes the active
  cards and programmes.
- `g17` is active (opened 2026-09-06): Nucleus V1 visual receipts are held
  on the lab's first cohort bundle; V2, M2, A2, and the switch decision are
  planned. `g16` (evidence recovery, v0.3.0, Nucleus M1/A1), `g15` (release
  and adoption programme) and `g14` (rejected conformance pilot) are
  complete; earlier generations are historical.
- Narrative history moved to
  `../logs/2026-09/20260905-front-door-compaction.md` on 2026-09-05.

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
- `g16` — completed evidence-led continuation generation (2026-08-25 to
  2026-09-06): drag-and-drop, motion policy, compiled web distribution,
  v0.3.0 published and adopted by all 15 consumers, Linux PR/main boards,
  Nucleus M1 and A1 29/29, visual lab opened
- `g17` — active Nucleus switch path: V1/V2/M2 receipts, A2 via the
  upstream AccessKit route, switch decision on receipts

## Rules

- Active milestone files live in generation folders such as `g17/`.
- File names use `NNN-slug.md`; roadmap IDs use forms such as `g16.001`.
- A roadmap file is a complete worker handoff. Do not add a batch-card
  layer.
- Backlog items belong in `backlog/`.
- Architecture belongs in `../architecture/`, not here.
- Generation rollover is manual. Close, pause, supersede, or rehome every live
  roadmap before opening the next generation.
- Purge stale strict-planning artifacts from `../specs/` during rollover.
- `dispatch.md` is the canonical dispatch manifest. Chatterbox is its only
  writer; the coordinator launches exactly the ready frontier it lists.
- Consumer defect intake is a recurring read-only sweep of sibling consumer
  repositories' `PAPERCUTS.md` files for Poodle-attributed friction, owned by
  Chatterbox and run on a low-cost model. Each finding enters ordinary triage
  and promotion; the sweep itself promotes nothing and dispatches no worker.

## Reading Roadmaps

Start with the [generation index](generation-index.md), then open the active
generation README and relevant milestone. Prefer current architecture,
contracts, specs, and implementation when historical cards describe an older
package shape.
