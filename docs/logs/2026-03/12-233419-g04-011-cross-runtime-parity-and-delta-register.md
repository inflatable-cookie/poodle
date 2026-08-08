---
title: g04.011 cross-runtime parity and delta register
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, parity, acceptance]
---

## Summary

Completed `g04.011` by freezing the first explicit cross-runtime parity report,
intentional delta register, and widened GPUI acceptance-harness surface.

## What changed

- added the normative baseline `docs/specs/058-cross-runtime-parity-report-delta-register-and-acceptance-harness-expansion.md`
- completed `docs/roadmaps/g04/011-cross-runtime-parity-report-intentional-delta-register-and-acceptance-harness-expansion.md`
- added the machine-readable artifact `packages/gpui/cross-runtime-parity-report.json`
- updated `packages/svelte/preview/scripts/build-parity-report.ts` so the
  generated parity artifact now includes a cross-runtime summary, side-by-side
  section set, delta-register summary, and GPUI acceptance-harness alignment
- updated `packages/ecosystem-acceptance.json` so the GPUI suite now carries
  the widened GPUI package set plus parity and accessibility evidence instead
  of reading like a token-only matrix
- updated `packages/svelte/preview/src/parity.ts` and
  `docs/specs/025-parity-automation-and-harness-boundary.md` so the manual
  parity boundary now reflects artifact-backed GPUI evidence without implying
  mounted native proof
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the cross-runtime
  parity report is checked against the Svelte parity registry, GPUI priority
  matrix, GPUI accessibility proof, and ecosystem acceptance suite
- refreshed `packages/svelte/preview/artifacts/parity-report.json` and rolled
  the roadmap or index surfaces forward so the repo now points at `g04.012`

## Validation

- `bun run --cwd packages/svelte/preview parity:report`
- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.011` is now explicit. Pug has one machine-readable place that ties the
Svelte review harness, GPUI crate baselines, GPUI accessibility posture,
side-by-side review targets, intentional deltas, and ecosystem acceptance
surface together instead of leaving those as separate implied stories.

## Next

Open `g04.012` and prove GPUI adoption through a downstream reference app and
stronger multi-app implementation evidence instead of treating artifact-backed
parity as the end state.
