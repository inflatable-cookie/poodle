# Loophole Shell Accent Wash Removal

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- removed the remaining theme-specific accent wash from the
  `loophole-studio` preview shell in `packages/svelte/preview/src/app.css`
- changed the shell background treatment from an accent-tinted atmospheric
  gradient to a neutral grey tonal blend so the theme now reads as monochrome
  grey chrome with lime accents rather than green-tinted chrome
- verified the preview/docs surface against the corrected loophole shell
  presentation

## Validation

- `bun run docs:build`
- `git diff --check`

## Next Task

Do one more integrated browser review pass across `light`, `dark`, and
`loophole-studio` to confirm the final theme split feels intentional, or return
to `docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
once the Svelte review surface feels stable enough to freeze.
