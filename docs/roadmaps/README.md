# Roadmaps

Status: reference
Updated: 2026-09-02

Roadmaps record Poodle's executable milestone work. `g15` completed the full
v0.2.x release programme: the roster and parity baseline, failed v0.2.0
publication, v0.2.1 recovery, crates.io-GPUI correction, v0.2.2 publication,
and adoption by all 16 authoritative consumers. `g16` uses a current
active-cohort parity evidence ledger to select bounded semantic and mounted
behavior repairs without inventing another conformance authority.

## Current State

- [Generation index](generation-index.md) is the canonical status summary;
  `dispatch.md` is the live frontier; `g16/README.md` indexes the cards and
  programmes.
- `g16` is active: v0.3.0 is published, its consumer adoption wave runs in
  the sibling repositories, and the Nucleus A1 accessibility tranches are in
  flight. `g15` (release and adoption programme) and `g14` (rejected
  conformance pilot) are complete; earlier generations are historical.
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
- `g16` — active evidence-led continuation generation. `001`–`044` are closed
  or research-complete, including Tree authority in PR #127. The canonical
  post-triage runway is compiled as `045`–`054`; six cards are ready and four
  preserve serial, ownership, or external gates. Dependable drag-and-drop remains a
  closed programme governed by architecture 011/spec 069.

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
