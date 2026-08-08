# g03.002 Parity Automation And Visual Or Interaction Harnesses

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- completed `g03.002`
- added `docs/specs/025-parity-automation-and-harness-boundary.md` to freeze
  what parity evidence is automated now and what must remain manual
- added a machine-readable parity registry at
  `packages/svelte/preview/src/parity.ts` tied directly to the live docs/catalog
  sections
- made the Svelte preview review state URL-addressable by section, theme,
  density, and control size so evidence can point at stable routes
- added `packages/svelte/preview/scripts/build-parity-report.ts` and the
  generated artifact `packages/svelte/preview/artifacts/parity-report.json`
- added root-level parity commands so report generation and docs build can run
  as one repeatable batch
- rolled the active roadmap/spec surfaces forward to `g03.003`

## Validation

- `bun run parity:report`
- `bun run docs:build`
- `git diff --check`

## Notes

- this tranche intentionally chose an honest route-based harness baseline
  instead of overclaiming screenshot or GPUI runtime parity before the repo can
  sustain it
- the new report makes manual-review boundaries explicit instead of leaving them
  implied in roadmap prose

## Next Task

Open `docs/roadmaps/g03/003-contract-linting-docs-completeness-and-publish-pipeline.md`
and harden contract linting plus docs completeness checks.
