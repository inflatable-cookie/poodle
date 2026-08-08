---
title: g04.001 gpui priority matrix and implementation order
status: completed
owner: nucleus
updated: 2026-03-13
tags: [logs, roadmap, gpui, parity]
---

## Summary

Completed `g04.001` by freezing the initial GPUI implementation order,
classifying the current review surface by parity mode, and making side-by-side
review targets explicit before larger GPUI component work begins.

## What changed

- added the normative spec `docs/specs/048-gpui-contract-audit-priority-and-side-by-side-review-baseline.md`
- completed `docs/roadmaps/g04/001-gpui-contract-audit-parity-priority-matrix-and-implementation-order.md`
- added the machine-readable priority artifact `packages/gpui/parity-priority-matrix.json`
- extended `packages/svelte/preview/scripts/lint-docs.ts` to validate:
  - required GPUI implementation waves
  - full section-target coverage across the current preview surface
  - supported priority and parity-mode values
  - explicit non-goals
- updated `packages/gpui/tokens/README.md` so the token crate points at the new GPUI implementation-order baseline
- rolled the next-task surfaces forward in:
  - `docs/specs/README.md`
  - `docs/roadmaps/g04/README.md`
  - `docs/roadmaps/generation-index.md`
  - `docs/roadmaps/README.md`
  - `docs/README.md`
  - `README.md`

## Validation

- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.001` is now explicit. The repo has a first GPUI priority matrix that says
which surfaces should aim for direct parity, which need native adaptation, and
which review sections deserve side-by-side comparison before the larger GPUI
build tranches begin.

## Next

Open `g04.002` and define the GPUI theme runtime, token application, and
native preview app baseline against the new implementation-order matrix.
