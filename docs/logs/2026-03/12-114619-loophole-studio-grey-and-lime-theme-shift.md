# Loophole Studio Grey And Lime Theme Shift

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- retuned `packages/tokens/schema/modes/themes/loophole-studio.json` away from
  the previous blue-toned shell toward a mid-deep grey base with neutralized
  surface, panel, text, and border support values
- changed the `loophole-studio` accent, hover, and focus roles to lime green so
  the workstation theme now carries a distinct grey-plus-lime personality
- added `primitives.color.green.400` and shifted the existing green step in
  `packages/tokens/schema/primitives/color.json` so the lime accent has a
  readable hover/focus companion and success posture stays aligned
- regenerated the emitted token artifacts and verified the preview/docs surface
  against the new loophole theme mapping

## Validation

- `bun run tokens:build`
- `bun run docs:build`
- `git diff --check`

## Next Task

Do one more integrated browser review pass across `light`, `dark`, and
`loophole-studio` to confirm the three theme personalities now feel clearly
separated, or return to
`docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
once the Svelte review surface feels stable enough to freeze.
