# Roadmaps

Status: active
Updated: 2026-04-13

Roadmaps hold executable milestone work for Poodle.

## Rules

- active milestone files live in generation folders such as `g01/`
- file names use `NNN-slug.md` with numbering local to the generation
- references use roadmap IDs such as `g09.001`
- generation rollover is manual only
- treat generations as substantial sequencing eras, not one-or-two-file
  buckets; as a healthy default, expect roughly 20 to 40 roadmap files in one
  generation before rollover is even worth discussing
- treat rollover as full generation closeout, not a convenience reset: close,
  supersede, or rehome every roadmap in the current generation first, then
  purge stale strict-planning artifacts from the active `docs/specs/` tree
- backlog items belong in `backlog/`
- architecture belongs in `../architecture/`, not here

## Current Index State

- highest on-disk generation folder: `g10`
- current executable generation plan: `g10`
- `g09` is complete
- `g10` is active; on-disk milestones `g10.005` through `g10.011` are **complete**
  (GPUI preview/parity tranche and shared button spec work); **`g10.012` is the
  open GPUI production-hardening / delta-closure track**

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

## Rollover guardrail

Do not open `gNN+1` while the current generation still has live roadmap files
or stale strict-planning debris in the active specs tree.

Before rollover:

- every roadmap in the closing generation must be explicitly closed, paused,
  superseded, or moved to backlog
- the roadmap front doors must agree that the old generation is no longer the
  live queue
- stale strict-planning artifacts for that generation must be purged from the
  active `docs/specs/` tree

## Next Task

Open `g10/README.md`. Living GPUI deferred work: `g10/012-gpui-runtime-truth-and-deferred-work-closure.md`.
Do not extend `g08/delta-register.md` (deprecated stub).
