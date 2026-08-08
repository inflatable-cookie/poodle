# Loophole Studio Monochrome Grey Neutralization

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- neutralized the `loophole-studio` shell/support palette in
  `packages/tokens/schema/modes/themes/loophole-studio.json` so its canvas,
  surface, panel, elevated, text, border, and icon values read as monochrome
  greys rather than green-tinted greys
- kept the lime accent, hover, focus, and success mappings intact so the theme
  still carries the new grey-plus-lime personality without washing the entire
  interface green
- regenerated the emitted theme artifact and verified the docs preview against
  the corrected loophole palette

## Validation

- `bun run tokens:build`
- `bun run docs:build`
- `git diff --check`

## Next Task

Do one more integrated browser review pass across `light`, `dark`, and
`loophole-studio` to confirm the final theme split feels intentional, or return
to `docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
once the Svelte review surface feels stable enough to freeze.
