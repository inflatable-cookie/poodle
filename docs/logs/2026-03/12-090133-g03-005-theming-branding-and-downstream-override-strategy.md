# g03.005 Theming, Branding, And Downstream Override Strategy

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- completed `g03.005` early to freeze the styling-extension posture before more
  downstream-facing implementation depth accumulates
- added `docs/specs/026-appearance-recipes-and-downstream-override-strategy.md`
  as the normative baseline for pure semantic tokens, treatment roles, recipe
  variables, and app-owned branded wrappers
- made gradients and similar browser-only effects explicit appearance-layer
  treatments instead of broadening token meaning
- added a seeded Svelte implementation of shared interactive treatment roles so
  buttons, tabs, and text-entry chrome can take one scoped branded treatment
- kept the active next tranche on `g03.003`, since contract linting and docs
  completeness remain the next uncompleted hardening batch

## Validation

- `bun run tokens:build`
- `bun run docs:build`
- `git diff --check`

## Notes

- this tranche intentionally does not turn Pug core into a marketing-site
  component kit; structural brand expression still belongs in app-owned wrappers
- the seed treatment proof is Svelte-first and web-capable, while the spec keeps
  the cross-runtime extension lane narrow enough for later GPUI mapping

## Next Task

Return to `docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
and harden contract linting plus docs completeness checks.
