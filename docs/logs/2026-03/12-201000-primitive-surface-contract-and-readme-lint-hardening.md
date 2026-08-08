# Primitive Surface Contract And README Lint Hardening

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- hardened `docs:lint` so the public `@pug/svelte-primitives` component export
  surface must stay aligned with foundation contract files
- added README inventory validation so the primitive package public-surface
  list cannot silently drift from the actual exported components
- updated the docs completeness baseline to treat public primitive surface
  alignment as a first-class docs gate

## Validation

- `bun run docs:lint`
- `bun run docs:build`
- `git diff --check`

## Risks

- this batch hardens the primitive package only; composite and workstation
  package README surfaces are not yet linted the same way
- parity depth is still route-based and review-oriented, not full runtime
  parity automation

## Next Task

If hardening continues, extend the same package-surface discipline to
`@pug/svelte-composites` and `@pug/svelte-workstation`, or shift fully into
parity review and evidence cleanup instead of more surface growth.
