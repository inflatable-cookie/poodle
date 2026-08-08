---
title: g03.012 ecosystem acceptance baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, acceptance, regression]
---

## Summary

Completed `g03.012` by freezing an explicit ecosystem acceptance matrix and a
named long-tail regression taxonomy, then wiring the docs lint surface to keep
those acceptance records internally coherent.

## What changed

- added the normative spec `docs/specs/045-ecosystem-acceptance-and-long-tail-regression-baseline.md`
- completed `docs/roadmaps/g03/012-ecosystem-acceptance-suites-and-long-tail-regression-coverage.md`
- added the machine-readable acceptance matrix `packages/ecosystem-acceptance.json`
- extended `packages/svelte/preview/scripts/lint-docs.ts` to validate:
  - required acceptance suites
  - required long-tail regression classes
  - evidence artifact existence
  - covered package references against the release manifest
  - matrix-only blocker honesty
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

`g03.012` is now explicit. Pug has a bounded ecosystem acceptance model across
preview, Underlay, Loophole, and GPUI validation surfaces, plus a named list
of long-tail regression classes that stops “ecosystem readiness” from being a
vague umbrella claim.

## Next

Move to `g03.013` and deepen reference apps, onboarding, and public-facing
examples using the now-explicit ecosystem acceptance baseline as the adoption
frame.
