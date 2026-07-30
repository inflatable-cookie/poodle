# Poodle Docs

Poodle uses a docs-first structure under `docs/` so product intent, architecture, contracts, specs, and execution history stay aligned.

## Core Sections

- `vision/`
- `architecture/`
- `roadmaps/`
- `logs/`

## Supporting Sections

- `contracts/`
- `guides/`
- `research/`
- `specs/`

## Start Here

1. Read `vision/001-poodle-vision.md`.
2. Read `architecture/001-poodle-system-shape.md`.
3. Read `roadmaps/README.md`.
4. Read `roadmaps/generation-index.md`.
5. Use `specs/README.md`, `contracts/README.md`, and `guides/README.md` when implementing or reviewing component work.

## Current Working Posture

- `vision/` defines long-range product intent and ecosystem boundaries
- `architecture/` defines package ownership, layering, and renderer boundaries
- `roadmaps/` holds executable milestone work and generation sequencing
- `specs/` holds normative repo-wide rules and artifact baselines
- `contracts/` holds per-component source-of-truth semantics and anatomy
- `guides/` packages reusable implementation recipes for downstream developers
- `logs/` records completed batches and evidence
- `research/` captures external comparison work before architectural decisions harden

## Local Inspection

For the browser preview surface:

```sh
bun install
bun packages/tokens/scripts/build-tokens.ts
bun run --cwd packages/svelte/preview dev
```

The repo-root `bun install` is the intended install root for normal
development and mounted-consumer hydration. Package-local `node_modules` under
`packages/svelte/*` are preview or direct-package execution details, not the
default repo contract.

For the default repo validation pass:

```sh
effigy health
```

## Next Task

Use `roadmaps/g12/README.md` as the current planning gate.
`roadmaps/generation-index.md` remains the top-level generation authority.
