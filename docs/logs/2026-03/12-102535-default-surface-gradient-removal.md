# Default Surface Gradient Removal

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- removed default gradients from the preview's left-rail shell surfaces in
  `packages/svelte/preview/src/app.css`
- changed `control-rail` and `rail-details` to use tonal fills instead of
  gradient fills so default component and panel chrome stays neutral
- updated `docs/specs/023-svelte-visual-hierarchy-and-contrast-baseline.md` to
  make the rule explicit: gradients belong in shell atmosphere or opt-in brand
  overrides, not in default component fills

## Validation

- `bun run docs:build`
- `git diff --check`

## Next Task

Continue targeted visual cleanup where it still improves review clarity, or
return to
`docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
once the Svelte review surface feels stable enough to freeze.
