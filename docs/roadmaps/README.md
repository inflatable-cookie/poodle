# Roadmaps

Status: active
Updated: 2026-03-15

Roadmaps hold executable milestone work for Pug.

## Rules

- active milestone files live in generation folders such as `g01/`
- file names use `NNN-slug.md` with numbering local to the generation
- references use roadmap IDs such as `g01.007`
- generation rollover is manual only
- backlog items belong in `backlog/`
- architecture belongs in `../architecture/`, not here

## Current Generation

- Active generation: none (between generations)
- Most recently completed generation: `g05`
- Next generation: `g06` (planned)

## Generation Map

- `g01` foundation, token system, contracts, primitive suite, workstation
  shells, and first Underlay bridge baseline
- `g02` advanced composites, docs/catalog depth, cleanup, and release baseline
- `g03` hardening, migration policy, parity automation, first downstream
  adoption tranches, validation, and mature extension support
- `g04` Underlay component parity, new component families (dialog patterns,
  file/media input, list interactions, code/color, navigation cards, editing),
  feature depth for existing components, and specialist media/editing surfaces
- `g05` GPUI foundation, spec crates, cross-runtime parity baseline, demo-app
  alignment, and Svelte demo rebuild
- `g06` shared multi-renderer contract layer — crate restructuring, typed
  token resolution, layout/event/style abstractions, adapter traits, and spec
  expansion to full 124-component surface (planned)
- `g07` GPUI rendering build-out — adapter crate, primitive and composite
  rendering, demo-app parity, downstream proof, published docs (planned)
- `g08` Jetstream rendering build-out — adapter crate, token bridge, component
  rendering in game engine context, integration demo scene (planned)

## Planning Standard

Pug should follow the same general planning posture used in the denser Chorus
and Jetstream generations:

- each generation should carry a substantive milestone run, not a handful of
  broad placeholders
- milestones should be narrow enough to sequence real work but broad enough to
  matter as a batch
- foundations should land before depth, adoption, and hardening work
- closeout milestones should be explicit instead of implied

Pug has more limited scope than Loophole, so its generations can be somewhat
smaller than a 20-30 milestone tranche. They should still be dense enough to
express a real program rather than three or four umbrella headings.

## Index

- `generation-index.md`
- `g01/README.md`
- `g02/README.md`
- `g03/README.md`
- `g04/README.md`
- `g05/README.md`
- `g06/README.md`
- `g07/README.md`
- `g08/README.md`
- `backlog/README.md`

## Next Task

Execute `g06` in order, beginning with `g06.001` multi-renderer architecture
audit. `g07` (GPUI build-out) and `g08` (Jetstream build-out) follow after
`g06` closeout and can run in parallel. Jetstream g04.016 (UI Rendering
Infrastructure) should be completed before `g08` begins.
