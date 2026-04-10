# Roadmaps

Status: active
Updated: 2026-04-09

Roadmaps hold executable milestone work for Poodle.

## Rules

- active milestone files live in generation folders such as `g01/`
- file names use `NNN-slug.md` with numbering local to the generation
- references use roadmap IDs such as `g09.001`
- generation rollover is manual only
- backlog items belong in `backlog/`
- architecture belongs in `../architecture/`, not here

## Current Index State

- highest on-disk generation folder: `g10`
- current executable generation plan: `g10`
- `g09` is complete
- `g10` is active, `g10.003` complete, next milestone is Jetstream implementation

The canonical summary of roadmap status is `generation-index.md`.
If a generation README conflicts with that file, treat the index as the source of truth until the generation README is reconciled.

## Available Generations

- `g01` foundation, token system, contracts, primitive suite, workstation shells, and first Underlay bridge baseline
- `g02` advanced composites, docs/catalog depth, cleanup, and release baseline
- `g03` hardening, migration policy, parity automation, downstream adoption, validation, and extension support
- `g04` Underlay component parity, new component families, feature depth, and specialist media or editing surfaces
- `g05` GPUI foundation, spec crates, cross-runtime parity baseline, and demo-app alignment
- `g06` shared multi-renderer contract layer, typed token resolution, layout or event abstractions, and full component-surface expansion
- `g07` GPUI rendering build-out, adapter crate, primitive and composite rendering, workstation shell updates, and parity reporting
- `g08` consolidated GPUI production-quality and compliance program
- `g09` completed GPUI continuation and semantic sizing/density generation
- `g10` active Jetstream-focused and component-overhaul generation

## Working Rule

Use the top-level generation index to determine what is current, then open the relevant generation folder.
Do not assume the highest-numbered generation folder is the active one.

## Next Task

`g10.003` is complete. The next milestone is Jetstream component implementation.
