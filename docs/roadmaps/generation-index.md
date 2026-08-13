# Roadmap Generation Index

Updated: 2026-08-14

## Active Track

- `g14`
  - Status: active
  - Range: `001` to `018`
  - Governing architecture:
    `docs/architecture/009-cross-runtime-component-conformance.md`
  - Governing spec: `docs/specs/066-executable-component-conformance.md`
  - Goal: no silent portable drift across Svelte, React, GPUI, and Jetstream.
    One portable interface and case/specimen corpus; two implementation
    substrates; four executed observations; strict completion.
  - Runway: `001`–`007` prove increasing component profiles; `008` records
    adopt/revise/reject. `009`–`014` remain blocked until adopt. `015` is a
    parallel licence web-reference intake; `016` completes it through the
    adopted conformance path; `017`–`018` clean up and close.
  - Next: `g14.001` conformance kernel and Button proof plus independent
    `g14.015` licence web reference in separate worktrees.

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
styles for Svelte/React; shared Rust spec/render/node composition for
GPUI/Jetstream. g14 adds an executable conformance plane above those pairs. It
shares interface declarations, cases, specimen structure, and observations;
it does not compile behaviour or impose one renderer model on web and native.
