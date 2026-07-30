# Roadmap Generation Index

## Active Execution Track

- `g12`
  - Status: active, strict-paused
  - Range: `001` to `016` on disk
  - Notes: React full parity (`001`–`008`) is complete — all 132 components
    ported to `@poodle/react` against a single shared stylesheet, with a
    full per-component preview gallery mirroring the Svelte preview's slugs.
    Web verification and native hardening through `g12.015` are complete.
    `g12.016` completed the public DockRegion drag extension and exact preview
    artifact checkpoint requested by Longhorn. No implementation card is ready.

- `g11`
  - Status: completed
  - Range: `001` to `008`
  - Notes: Svelte modernization and audited consumer rollout, plus the
    headless-core dual-layer program — framework-free state-machine core,
    interface-stable Svelte adapter layer, appearance-recipe overrides and
    full recipe-hook coverage, Rust machine mirror, and the multi-framework
    adapter pilot that seeded g12. Master spec:
    `docs/specs/062-headless-core-and-dual-layer-strategy.md`. Recorded
    residual debt stays in `g11.004` (extraction register) and `g11.006`
    (domain-math port, Jetstream adoption).

- `g10`
  - Status: completed
  - Range: `001` to `021`
  - Notes: Jetstream feasibility, Svelte overhaul closeout, unified component
    packaging, GPUI parity recovery, token fidelity, contract sync, spec struct
    coverage, and GPUI accessibility baseline are complete. `g10.012` is now
    closed as historical runtime-truth documentation, not the live queue.

## Completed Foundations

- `g09`
  - Status: completed
  - Range: `001` to `009`
  - Notes: architecture unification, GPUI continuation, semantic sizing or
    density rollout, and the original cross-runtime verification tranche are
    complete enough that `g09` no longer acts as the live queue

- `g01`
  - Status: completed
  - Range: `001` to `014`
  - Notes: repository bootstrap, token model, contract system, primitive suite, workstation shells, Underlay bridge, and first parity baseline

- `g02`
  - Status: completed
  - Range: `001` to `016`
  - Notes: advanced composites, product and workstation depth, docs and preview cleanup, API cleanup, packaging, and release baseline

- `g03`
  - Status: completed
  - Range: `001` to `014`
  - Notes: migration policy, parity automation, docs publishing, downstream adoption, ecosystem validation, change control, and extension support

- `g04`
  - Status: completed
  - Range: `001` to `018`
  - Notes: Underlay component parity, new component families, feature depth, and specialist editing or media surfaces

- `g05`
  - Status: completed
  - Range: `001` to `014`
  - Notes: GPUI foundation, spec crates, cross-runtime parity baseline, and demo alignment

- `g06`
  - Status: completed
  - Range: `001` to `015`
  - Notes: shared multi-renderer contract layer, crate restructuring, typed token resolution, layout and event abstractions, style descriptors, adapter traits, and full component-surface expansion

- `g07`
  - Status: completed
  - Range: `001` to `015`
  - Notes: GPUI rendering build-out, adapter crate, theme integration, primitive and composite rendering, workstation shell updates, and cross-runtime parity reporting

- `g08`
  - Status: completed
  - Range: `001` to `011`
  - Notes: consolidated GPUI production-quality, contract-compliance, specimen, accessibility, and visual-parity work

## Working Rule

When roadmap files disagree:

1. treat this index as the top-level source of truth
2. treat `docs/roadmaps/README.md` as the entrypoint
3. treat `g12` as the active generation until it is explicitly closed

## Rollover policy

Create a new generation only when maintainers explicitly decide the sequencing
baseline needs a real reset.

Generations should be substantial. As a healthy default, expect something
closer to 20 to 40 roadmap files before rollover is worth discussing. Treat
that as a judgment guardrail, not an automatic counter.

Rollover is a closeout event, not a convenience move. Before opening the next
generation:

- close, pause, supersede, or rehome every roadmap in the current generation
- refresh the roadmap front doors so the old generation is visibly closed
- purge stale generation-specific strict-planning artifacts from the active
  `docs/specs/` tree

If that cleanup has not happened, stay in the current generation and finish the
closeout there first.

## Next Task

Choose and promote the next bounded Poodle card. Do not infer one from the
candidate list in `g12/README.md`.
