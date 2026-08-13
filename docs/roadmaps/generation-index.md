# Roadmap Generation Index

Updated: 2026-08-11

## Active Track

- `g13`
  - Status: active
  - Range: `001` to `016`
  - Governing spec: `docs/specs/063-rust-authored-component-and-scene-ir.md`
  - Notes: Rust-authored component and scene IR. The bounded pilot `001`–`008`
    is complete and `g13.008` recorded **revise** on 2026-08-13
    (`docs/roadmaps/g13/pilot-verdict-evidence.md`). The IR is narrowed to
    cross-runtime vocabulary with drift gating; the behavioural ambition is
    dropped. Cards `009`–`016` are **closed, not deferred** — they describe a
    generative migration the verdict declines.
  - Next: `g13.017` narrow the IR to the vocabulary scope; the runway ends at
    `g13.020`, consolidate and reassess.

## Latest Completed Track

- `g12`
  - Status: completed
  - Range: `001` to `027`
  - Notes: React parity, cross-web visual verification, native contract and
    accessibility hardening, GPUI node-backend convergence, public package and
    icon boundaries, the complete audio family across all four runtimes, and
    embedded Slider/RangeSlider correction.
  - Consolidation: the audio cards briefly filed as `g13.001`, `g14.001`, and
    `g15.001` now live at `g12.025`–`027`. Those single-card generations did
    not represent sequencing resets and are retired.
  - Closeout: no live card. Historical validation exceptions remain recorded
    in their owning cards and logs. Current docs-inventory drift is rehomed to
    the `g13.001` baseline repair.

## Completed Foundations

- `g11` — Svelte modernization, framework-free web machines, appearance
  recipes, Rust machine mirrors, and the React adapter pilot (`001`–`008`)
- `g10` — Jetstream feasibility, unified package shape, GPUI production
  hardening, contract sync, and accessibility baseline (`001`–`021`)
- `g09` — native package consolidation and semantic sizing/density rollout
  (`001`–`009`)
- `g08` — GPUI production-quality, contract-compliance, specimen,
  accessibility, and visual-parity program (`001`–`011`)
- `g07` — GPUI renderer build-out, adapter expansion, and downstream proof
  (`001`–`015`)
- `g06` — shared renderer contracts, typed tokens, layout/events, style IR,
  and adapter traits (`001`–`015`)
- `g05` — GPUI foundation, spec crates, parity baseline, and demo alignment
  (`001`–`014`)
- `g04` — Underlay parity, specialist component families, editing, and media
  surfaces (`001`–`018`)
- `g03` — hardening, migration, parity automation, adoption, and ecosystem
  validation (`001`–`014`)
- `g02` — advanced composites, documentation depth, packaging, and release
  baseline (`001`–`016`)
- `g01` — repository foundation, tokens, contracts, primitives, workstation
  shells, and first Underlay bridge (`001`–`014`)

## Working Rule

When roadmap files disagree:

1. Treat this index as the top-level status source.
2. Treat `docs/roadmaps/README.md` as the front door.
3. Treat `g12` as closed and `g13` as the only active generation.
4. Treat `g13.009`–`016` as closed and non-executable. `g13.008` recorded
   **revise**, so the adopt gate they waited on will not open. The live runway
   is `g13.017`–`020`.

## Rollover Policy

Create a generation only when maintainers explicitly reset the sequencing
baseline. Generations should hold a substantial program; one follow-on card is
normally consolidated into the active generation.

Before rollover:

- close, pause, supersede, or rehome every live card
- refresh both roadmap front doors
- promote or retire generation-specific provisional specs
- leave one named first task in the new generation

## Current Program Posture

The implementation baseline remains the current dual path until the pilot
passes: shared TypeScript behavior/styles for Svelte and React; shared Rust
spec/render/node composition for GPUI and Jetstream. Spec 063 proposed a
Rust-authored declarative authority above both paths; after the `g13.008`
**revise** verdict it is narrowed to cross-runtime vocabulary with drift
gating. Architecture 001/006 were deliberately **not** amended — nothing is
promoted to stable while the model stays provisional at vocabulary scope.
