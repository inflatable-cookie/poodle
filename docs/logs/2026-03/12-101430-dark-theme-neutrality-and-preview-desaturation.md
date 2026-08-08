# Dark Theme Neutrality And Preview Desaturation

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- retuned the default dark theme overlay in
  `packages/tokens/schema/modes/themes/dark.json` so canvas, panel, surface,
  and elevated backgrounds read as neutral charcoal rather than cool blue
  chrome
- kept the accent/focus/action blue intact while softening the dark theme's
  secondary text and border neutrals to match the more monochrome shell
  direction
- updated the preview shell in `packages/svelte/preview/src/app.css` so the
  default `dark` theme no longer gets the broad accent wash that now belongs to
  `loophole-studio`
- documented the distinction in
  `docs/specs/023-svelte-visual-hierarchy-and-contrast-baseline.md` so the
  default dark theme stays neutral while atmospheric tinting remains opt-in

## Validation

- `bun run tokens:build`
- `bun run docs:build`
- `git diff --check`

## Notes

- this keeps the main dark baseline closer to the light theme's neutrality,
  while preserving `loophole-studio` as the more characterful workstation
  option
- the change was applied at the token/theme and preview-shell layers together so
  the browser surface reflects the intended distinction immediately

## Next Task

Continue targeted visual refinement where it still meaningfully improves review
quality, or return to
`docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
once the Svelte surface feels stable enough to freeze.
