# Package Export Preview Coverage Hardening

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- added a machine-readable package export coverage manifest in the preview
  parity layer so public Svelte exports are now classified as either
  `previewed` or `contract-only`
- extended the docs lint baseline to verify that the coverage manifest matches
  the real public export surfaces of `@pug/svelte-primitives`,
  `@pug/svelte-composites`, and `@pug/svelte-workstation`
- extended the generated parity report so review artifacts now expose both
  section-level parity targets and export-level preview coverage debt

## Validation

- `bun run docs:lint`
- `bun run parity:report`
- `bun run docs:build`
- `git diff --check`

## Risks

- many primitive exports remain intentionally `contract-only`, which is honest
  but still means the preview is broader at family level than at one-specimen-
  per-export depth
- export coverage is now explicit for Svelte component packages, but token
  package review coverage still stays section-oriented rather than export-
  oriented

## Next Task

Use the new export-coverage manifest to decide the next preview expansion
deliberately: either add direct preview specimens for the highest-value
`contract-only` primitives, or freeze preview breadth here and move into
`g03.004` performance and render-cost hardening.
