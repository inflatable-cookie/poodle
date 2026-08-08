---
title: g03.009 gpui multi-app validation baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, validation]
---

## Summary

Completed `g03.009` by freezing an explicit GPUI multi-app validation target
matrix and assumption inventory, without overclaiming runnable downstream GPUI
app evidence that this repo does not currently contain.

## What changed

- added the normative spec `docs/specs/042-gpui-multi-app-validation-target-matrix.md`
- completed `docs/roadmaps/g03/009-additional-gpui-app-adoption-and-multi-app-validation.md`
- added the machine-readable matrix `packages/gpui/tokens/multi-app-validation.json`
- expanded `packages/gpui/tokens/README.md` so token-only GPUI readiness is clearly separated from wider multi-app validation posture
- rolled the index and next-task surfaces forward in:
  - `docs/specs/README.md`
  - `docs/roadmaps/g03/README.md`
  - `docs/roadmaps/README.md`
  - `docs/README.md`
  - `README.md`

## Validation

- `bun run docs:lint`
- `bun run docs:build`
- `git diff --check`

## Outcome

`g03.009` is now explicit. The repo has a concrete GPUI validation target
matrix, a named list of hidden shared-layer assumptions, and an honest blocker
list that prevents the token-only GPUI package from being mistaken for wider
multi-app readiness.

## Next

Move to `g03.010` and perform accessibility plus assistive-technology audit
work using the adoption proofs and GPUI validation matrix as explicit context.
