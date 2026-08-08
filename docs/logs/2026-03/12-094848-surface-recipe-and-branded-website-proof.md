# Surface Recipe And Branded Website Proof

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- extended the appearance-recipe proof beyond `interactive` into shared
  `surface` and `surface-elevated` treatment roles
- wired those surface roles into
  `packages/svelte/components/src/Card.svelte`,
  `packages/svelte/components/src/PageHeader.svelte`, and
  `packages/svelte/workstation/src/PanelSurface.svelte` using stable CSS custom
  property hooks instead of one-off preview selectors
- updated the preview docs shell to use the broader `appearance treatment`
  framing and added a scoped branded website-style proof that shows Pug
  components inside an app-owned wrapper with gradients and raised surfaces
- made preview package resolution deterministic in
  `packages/svelte/preview/vite.config.ts` so cross-package imports no longer
  depend on Bun materializing nested workspace links in a particular state
- updated `docs/specs/026-appearance-recipes-and-downstream-override-strategy.md`
  and `packages/svelte/preview/README.md` so the current evidence matches the
  broader surface-level override model

## Validation

- `bun run docs:build`
- `git diff --check`

## Notes

- semantic tokens still stay typed and narrow; gradients and website-style
  polish remain scoped appearance-layer overrides rather than canonical token
  meaning
- the branded website proof is intentionally an app-owned wrapper example, not a
  signal that Pug core should become a marketing-site component kit

## Next Task

Return to `docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
and harden contract linting, docs completeness checks, and the publish
baseline on top of the now-clearer styling contract.
