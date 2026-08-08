# Dark Theme Orange Accent Shift

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- changed the default `dark` theme accent in
  `packages/tokens/schema/modes/themes/dark.json` from blue to orange so the
  dark baseline has a stronger, more distinct personality
- added `primitives.color.amber.400` in
  `packages/tokens/schema/primitives/color.json` to support a readable warm
  hover and focus accent on dark surfaces
- kept `light` and `loophole-studio` on their existing blue accent posture so
  the orange shift is specific to the default dark theme rather than a global
  system change
- regenerated the emitted token artifacts and verified the preview/docs surface
  against the new dark accent mapping

## Validation

- `bun run tokens:build`
- `bun run docs:build`
- `git diff --check`

## Next Task

Do one more integrated browser review pass across `light`, `dark`, and
`loophole-studio` to confirm the accent split feels intentional, or return to
`docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
once the Svelte review surface feels stable enough to freeze.
