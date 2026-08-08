# Svelte Preview Hardening And `g03.003` Docs Gate

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- hardened the Svelte preview/docs shell in
  `packages/svelte/preview/src/app.css` with more deliberate rail spacing,
  clearer disclosure affordances, and stronger hover/focus treatment for docs
  tiles and rail selectors without reintroducing default gradient chrome
- re-reviewed the live preview in `light`, `dark`, and `loophole-studio`
  through local Chrome screenshots and kept the final loophole shell neutral
  grey with lime accents rather than lime-tinted chrome
- completed `g03.003` by adding
  `docs/specs/027-docs-completeness-contract-linting-and-publish-pipeline.md`,
  wiring `bun run docs:lint` and `bun run docs:check`, and adding
  `packages/svelte/preview/scripts/lint-docs.ts` to validate contract indexes,
  minimum seed-contract structure, docs catalog coverage, and parity wiring
- rolled the roadmap and readme surfaces forward so the docs authority now
  points at `g03.004`

## Validation

- `bun run docs:lint`
- `bun run docs:check`
- `git diff --check`

## Risks

- the new contract linter intentionally enforces the current minimum
  seed-contract structure rather than the full component template everywhere;
  expanding to full-template enforcement remains future hardening once more of
  the contract catalog is normalized

## Next Task

Open `docs/roadmaps/g03/004-performance-render-cost-and-memory-profile-hardening.md`
and harden performance, render-cost, and memory posture without reopening the
new docs/publish validation baseline.
