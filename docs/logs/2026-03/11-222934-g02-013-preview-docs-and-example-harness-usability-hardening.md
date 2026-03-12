# g02.013 Preview Docs And Example Harness Usability Hardening

Status: completed
Date: 2026-03-11
Owner: Pug Core

## Summary

- completed `g02.013`
- hardened the Svelte docs/preview surface around everyday review ergonomics
  instead of continuing to treat it like a raw internal demo page
- simplified navigation by keeping section browsing in the sticky rail,
  adding local previous and next controls there, and removing the noisier
  duplicate browser from the main content area
- reduced left-rail noise by collapsing display controls, state probes, and
  reference material into disclosure groups
- improved current-section readability in the hero so package ownership,
  contract roots, and visible example types remain obvious while reviewing
- tightened the catalog hub layout so family and example cards stop stretching
  awkwardly and leaving large dead zones
- recorded remaining preview debt in `packages/svelte/preview/README.md`
  instead of implying the surface is already downstream-ready

## Validation

- `bun run preview:build`
- `git diff --check`

## Notes

- this tranche intentionally stops short of claiming downstream adoption
  readiness; the resulting surface is markedly easier to use, but package API
  cleanup and parity debt still need their own tranche
- the preview is now positioned as a practical internal review tool, while
  `g03` remains the first real downstream adoption generation

## Next Task

Open
`docs/roadmaps/g02/014-component-api-cleanup-package-ergonomics-and-parity-debt.md`
and tighten the public package surface before downstream repos depend on it.
