# Rail Spacing And Hover Affordance Hardening

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- increased spacing rhythm in the preview rail so helper copy, group headings,
  and option tiles no longer visually crowd each other
- updated `packages/svelte/preview/src/app.css` so `control-group`,
  `rail-details__body`, `group-head`, and segmented/pill stacks all use more
  deliberate vertical spacing
- strengthened inactive hover affordance for selectable tiles by adding a clear
  border, fill, and shadow delta before click instead of leaving hover
  readability mostly to the active state
- updated `docs/specs/023-svelte-visual-hierarchy-and-contrast-baseline.md` so
  control-group spacing and visible inactive hover response are now explicit
  baseline requirements

## Validation

- `bun run docs:build`
- `git diff --check`

## Next Task

Do one more integrated browser review pass across `light`, `dark`, and
`loophole-studio` to catch any remaining hierarchy mismatches, or return to
`docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
once the Svelte review surface feels stable enough to freeze.
