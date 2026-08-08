# Default Shell Neutrality Hardening

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- removed remaining accent-led atmosphere from the default `light` and `dark`
  preview shell backgrounds in `packages/svelte/preview/src/app.css`
- changed non-interactive docs-shell chips, tags, and mode readouts back to
  neutral surface-toned fills instead of accent-tinted fills
- kept the more expressive tinting scoped to `loophole-studio` and the branded
  website proof rather than letting it read as the default component/system
  language
- updated `docs/specs/023-svelte-visual-hierarchy-and-contrast-baseline.md` so
  neutral informational chips remain part of the documented baseline

## Validation

- `bun run docs:build`
- `git diff --check`

## Next Task

Do one more integrated browser review pass across `light`, `dark`, and
`loophole-studio` to catch any remaining hierarchy mismatches, or return to
`docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
once the Svelte review surface feels stable enough to freeze.
