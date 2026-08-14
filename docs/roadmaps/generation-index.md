# Roadmap Generation Index

Updated: 2026-08-14

## Active Track

- `g14`
  - Status: active
  - Range: `001` to `024`
  - Governing architecture:
    `docs/architecture/009-cross-runtime-component-conformance.md`
  - Governing spec: `docs/specs/066-executable-component-conformance.md`
  - Goal: no silent portable drift across Svelte, React, and Rust through
    GPUI. One portable interface and case/specimen corpus; two implementation
    substrates; three executed observations; strict active-cohort completion.
    Jetstream remains a deferred backend over the same Rust boundary.
  - Runway: `001`–`004` are accepted; `005`–`007` prove the remaining component
    profiles; `008` records adopt/revise/reject. `009`–`014` remain blocked
    until adopt. `015`–`017` stage and complete the licence surface;
    `018`–`020` do the same for model connections; `021` cleans the estate,
    `022` closes the generation, interposed `023` replaces foreground GPUI
    conformance before profile `005`, and independent `024` adds the batched
    AudioMeter web rendering tier.
  - Next: dispatch `g14.023`. `g14.019` is complete; the model-connection web
    reference is approved. `g14.024` is ready to run in a separate
    web-performance worktree.

## Latest Completed Track

- `g13`
  - Status: completed
  - Range: `001` to `020` plus execution batch cards
  - Verdict: **revise**, followed by retirement/unwind of the component IR.
  - Durable evidence: Svelte remains the reference; the web and native pairs
    keep one substrate each; behaviour codegen failed its cost/replacement
    test; specimen structure should still be authored once.
  - Historical spec: `docs/specs/063-rust-authored-component-and-scene-ir.md`.

## Completed Foundations

- `g12` — React parity, verification depth, native hardening, package
  consolidation, complete audio family (`001`–`027`)
- `g11` — Svelte modernization, framework-free web machines, appearance
  recipes, Rust mirrors, React adapter pilot (`001`–`008`)
- `g10` — Jetstream feasibility and GPUI production hardening (`001`–`021`)
- `g09` — native package consolidation and sizing/density (`001`–`009`)
- `g08` — GPUI production quality and compliance (`001`–`011`)
- `g07` — GPUI renderer build-out and adapter expansion (`001`–`015`)
- `g06` — renderer contracts, typed tokens, layout/events, style IR (`001`–`015`)
- `g05` — GPUI foundation and parity baseline (`001`–`014`)
- `g04` — Underlay parity, specialist families, editing/media (`001`–`018`)
- `g03` — hardening, migration, parity automation, adoption (`001`–`014`)
- `g02` — composites, documentation, packaging, release baseline (`001`–`016`)
- `g01` — repository foundations, tokens, primitives, shells (`001`–`014`)

## Working Rule

When roadmap files disagree:

1. Treat this index as the top-level status source.
2. Treat `docs/roadmaps/README.md` as the front door.
3. Treat `g13` and the first g14 runway as historical evidence.
4. Treat the redesigned `g14/README.md` as the only executable runway.

## Rollover Policy

Create a generation only when maintainers explicitly reset sequencing. Before
rollover: close or rehome every live card, refresh both front doors, promote or
retire provisional specs, and name one first task.

## Current Program Posture

The implementation baseline remains pair-wise: shared TypeScript behaviour and
styles for Svelte/React; shared Rust spec/render/node composition for native
backends. g14 executes that plane through GPUI while keeping the boundary
neutral for later Jetstream admission. It shares interface declarations,
cases, specimen structure, and observations; it does not compile behaviour or
impose one renderer model on web and native.
