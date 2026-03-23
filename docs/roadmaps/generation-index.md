# Roadmap Generation Index

## Active Execution Track

- `g08`
  - Status: active
  - Range: `001` to `009`
  - Notes: consolidated GPUI production-quality, contract-compliance, specimen, accessibility, and visual-parity work

- `g09`
  - Status: planned
  - Range: `001` to `008`
  - Notes: consolidated Jetstream production-quality, feasibility, specimen, and parity work

## Completed Foundations

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

## Preserved Historical Planning

- `g10`
  - Status: preserved reference
  - Notes: historical Jetstream production-quality plan from before the roadmap consolidation

- `g11`
  - Status: preserved reference
  - Notes: historical GPUI contract-compliance plan from before the roadmap consolidation

Use `g10` and `g11` for historical context only unless their useful decisions are intentionally re-imported into the active `g08` or `g09` track.

## What Was Consolidated

The previous later-generation plan was reduced to the current `g08` and `g09` execution track after an audit found inflated completion claims and stale sequencing assumptions in the old `g08` through `g13` material.

Useful architectural decisions and constraint notes from that older plan are preserved in `docs/roadmaps/archive/g08-g11-reference-notes.md`.

## Working Rule

When roadmap files disagree:

1. treat this index as the top-level source of truth
2. treat `docs/roadmaps/README.md` as the entrypoint
3. treat `g10` and `g11` as historical context unless explicitly reactivated

## Next Task

Execute `g08.001`, then continue reconciling any generation README that still reads as active execution despite being superseded by the consolidated `g08` or `g09` track.
