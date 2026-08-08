---
title: g03 closeout and next-program posture
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, closeout, generation]
---

## Summary

Completed `g03.014` by freezing an explicit generation closeout artifact,
marking the whole `g03` generation complete, and recording the carry-forward
gaps that still belong to a later program instead of being implied as solved.

## What changed

- added the normative spec `docs/specs/047-generation-closeout-and-next-program-posture.md`
- completed `docs/roadmaps/g03/014-generation-closeout-and-next-program-cutover.md`
- added the machine-readable closeout artifact `packages/g03-closeout.json`
- extended `packages/svelte/preview/scripts/lint-docs.ts` to validate:
  - completed milestone inventory
  - required `g03` stable surfaces
  - required carry-forward gaps
  - next-program posture metadata
- updated the generation and top-level docs surfaces in:
  - `docs/specs/README.md`
  - `docs/roadmaps/g03/README.md`
  - `docs/roadmaps/generation-index.md`
  - `docs/roadmaps/README.md`
  - `docs/README.md`
  - `README.md`

## Validation

- `bun run docs:lint`
- `bun run docs:build`
- `git diff --check`

## Outcome

`g03` is now explicitly complete. The repo has a concrete closeout artifact
covering stable surfaces and carry-forward gaps, and the docs no longer imply
that another generation is already active.

## Next

The three-generation `g01` to `g03` program is complete. If a new generation is
opened later, use the `g03` closeout artifact and carry-forward gaps as the
starting frame.
