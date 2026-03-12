# Pug Docs

Pug uses a Northstar-shaped documentation system inside `docs/`.

## Core Sections

- `vision/`
- `architecture/`
- `roadmaps/`
- `logs/`

## Add-On Sections In Use

- `contracts/`
- `research/`
- `specs/`

## Start Here

1. Read `vision/001-pug-vision.md`.
2. Read `architecture/001-pug-system-shape.md`.
3. Read `roadmaps/README.md`.
4. Read the current generation under `roadmaps/g03/`.

## Local Inspection Surface

For a runnable browser surface while the larger docs-site program remains in
`g02.012`, use `packages/svelte/preview`.

From the repo root:

```sh
bun install
bun run tokens:build
bun run docs:dev
```

The docs surface now covers theme inspection, tokens, advanced composites,
workstation shell depth, dock and split orchestration, discoverability
metadata, and serialized layout inspection.

## Working Rule

- use `vision/` for long-range intent
- use `architecture/` for package boundaries, layering, and contract rules
- use `roadmaps/` for executable milestone work
- use `contracts/` for per-component source-of-truth semantics
- use `logs/` for completed batches and decision evidence
- use `research/` when external systems must be compared before architecture is frozen

## Next Task

Open `docs/roadmaps/g03/001-token-evolution-migration-and-compatibility-policy.md`
and begin `g03` with explicit migration and compatibility policy.
