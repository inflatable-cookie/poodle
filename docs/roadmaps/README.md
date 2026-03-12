# Roadmaps

Status: active
Updated: 2026-03-11

Roadmaps hold executable milestone work for Pug.

## Rules

- active milestone files live in generation folders such as `g01/`
- file names use `NNN-slug.md` with numbering local to the generation
- references use roadmap IDs such as `g01.007`
- generation rollover is manual only
- backlog items belong in `backlog/`
- architecture belongs in `../architecture/`, not here

## Current Generation

- Active generation: `g03`
- Next generation: not yet planned

## Generation Map

- `g01` foundation, token system, contracts, primitive suite, workstation
  shells, and first Underlay bridge baseline
- `g02` advanced composites, docs/catalog depth, cleanup, and release baseline
- `g03` hardening, migration policy, parity automation, first downstream
  adoption tranches, validation, and mature extension support

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
- `backlog/README.md`

## Next Task

Execute `g03` in order, beginning with token evolution, migration, and
compatibility policy before downstream adoption begins.
