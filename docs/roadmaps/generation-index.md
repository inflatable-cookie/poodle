# Roadmap Generation Index

Updated: 2026-08-16

## Active Track

- `g15`
  - Status: active — `g15.001`–`g15.002` complete; `g15.003` in progress;
    `g15.004` ready for parallel dispatch
  - Posture: release-first v0.2.0 baseline
  - Range: `001` to `014`
  - Verdict: `g15.001`–`g15.002` accepted — the generation is a measured
    release baseline, not a parity architecture
  - Evidence: the v0.2.0 denominator is every public Svelte component
    export, not a representative subset. React mirror coverage, a certified
    GPUI subset, and deferred Jetstream are recorded separately; one runtime
    does not borrow another runtime's pass.
  - Next: continue `g15.003` and dispatch `g15.004` in parallel.

## Latest Completed Track

- `g14`
  - Status: complete (`g14.022`)
  - Range: `001` to `026`
  - Verdict: `g14.008` **rejected** architecture 009/spec 066 as Poodle's
    standing component-conformance mechanism.
  - Evidence: 22,746 source LOC against 472 LOC replaced; HistoryCenter absent
    from the comparator; 1,205 differences after correction; manual component
    registries and incomplete authority consumption.
  - Retained result: component/backend fixes, focused regression claims,
    headless GPUI execution, and the human-centred specimen boundary.
  - Disposition: `009`–`014` are retired. `021` preserved the evidence and
    removed the rejected plane. `017` and `020` are superseded execution
    plans; their approved web references stand and native completion
    recompiles under the g15 runway. `026` is carried forward into g15 with
    its human-centred rubric intact. `022` closed the generation
    (`docs/logs/2026-08/16-g14-022-generation-closeout.md`).

- `g13`
  - Status: completed
  - Range: `001` to `020` plus execution batch cards
  - Verdict: **revise**, followed by retirement/unwind of the component IR.
  - Durable evidence: Svelte remains the reference; the web and native pairs
    keep one substrate each; behaviour codegen failed its cost/replacement
    test; specimen structure still needs a smaller coordination method.
  - Historical spec: `docs/specs/063-rust-authored-component-and-scene-ir.md`.

## Completed Foundations

- `g12` — React parity, verification depth, native hardening, package
  consolidation, complete audio family (`001`–`027`)
- `g11` — Svelte modernization, framework-free web machines, appearance
  consolidation, audio controls (`001`–`022`)
- `g10` — Jetstream feasibility and GPUI production hardening (`001`–`020`)
- `g09` — native package consolidation, typed size/density migration,
  cross-runtime parity (`001`–`020`)
- `g01`–`g08` — historical foundation and parity generations

## Archive Policy

Completed generations stay in `docs/roadmaps/gNN/`. Superseded or abandoned
runways move to `docs/roadmaps/archive/` when they would otherwise obscure the
active sequence. Historical execution logs remain in `docs/logs/`.
