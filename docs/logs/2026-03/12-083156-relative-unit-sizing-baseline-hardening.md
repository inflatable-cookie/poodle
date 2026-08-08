# Relative Unit Sizing Baseline Hardening

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- shifted the Svelte and token sizing baseline away from raw `px` values and
  toward relative units
- converted token primitives and mode overlays for spacing, control size,
  typography, radius, and related dimensions to `rem`
- converted the Svelte preview/docs shell and component styles from hard-coded
  pixel values to relative units across the current implementation surface
- added shared `pxToRem` and `pxToEm` helpers at
  `packages/svelte/tokens/src/units.ts` for runtime-generated sizing cases
- updated the visual-system baseline so relative-unit posture is documented, not
  just implied by code

## Validation

- `bun run tokens:build`
- `bun run docs:build`
- `git diff --check`

## Notes

- this tranche intentionally targeted the token baseline plus the current
  high-visibility Svelte surface first, so later GPUI work has a clearer sizing
  posture to follow
- the goal was not aesthetic redesign; it was to make scale, spacing, and type
  respond to the root sizing baseline more coherently

## Next Task

Continue Svelte-side visual refinement or return to `g03.003` once the review
surface feels stable enough to lock down linting and publish-pipeline work.
