---
title: g04.002 gpui theme runtime and preview app baseline
status: completed
owner: nucleus
updated: 2026-03-13
tags: [logs, roadmap, gpui, preview]
---

## Summary

Completed `g04.002` by freezing the first native GPUI theme-runtime and
preview-app baseline, including the wave-0 side-by-side review sections and the
required review controls for section, theme, density, and control size.

## What changed

- added the normative spec `docs/specs/049-gpui-theme-runtime-and-native-preview-app-baseline.md`
- completed `docs/roadmaps/g04/002-gpui-theme-runtime-token-application-and-native-preview-app-baseline.md`
- added the machine-readable preview baseline `packages/gpui/preview-app-baseline.json`
- extended `packages/svelte/preview/scripts/lint-docs.ts` to validate:
  - supported GPUI theme IDs
  - density and control-size review dimensions
  - wave-0 preview section coverage against the GPUI priority matrix
  - required preview controls and non-goals
- updated `packages/gpui/tokens/README.md` so the token crate points at the new runtime and preview baseline
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

`g04.002` is now explicit. The repo has a first native GPUI preview baseline
that says which sections come first, which review controls must exist, and how
theme and token inspection should begin before the larger GPUI component
surface is built out.

## Next

Open `g04.003` and implement the GPUI structural primitive tranche on top of
the now-explicit theme runtime and preview-app baseline.
