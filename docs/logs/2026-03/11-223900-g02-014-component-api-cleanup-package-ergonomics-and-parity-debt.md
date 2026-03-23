# g02.014 Component API Cleanup Package Ergonomics And Parity Debt

Status: completed
Date: 2026-03-11
Owner: Poodle Core

## Summary

- completed `g02.014`
- tightened the JS-side public package boundary by giving the Svelte packages
  explicit `svelte` entry points, explicit `./types` subpaths where applicable,
  and constrained `files` lists instead of leaving the package surface implicit
- added local token wrapper modules so `@poodle/svelte-tokens` exposes generated
  themes and metadata through package-owned paths rather than repo reach-through
- added package readmes for the Svelte and GPUI token layers and revised the
  Svelte package readmes to distinguish public entry points, stability notes,
  and current adoption blockers
- added the normative package and parity-debt baseline at
  `docs/specs/021-public-package-api-stability-and-parity-debt-baseline.md`
  so the adoption gate is explicit rather than inferred

## Validation

- `bun run preview:build`
- `git diff --check`

## Notes

- this tranche intentionally treats GPUI component parity as explicit debt,
  not as something implied by the presence of generated token bindings
- the package surface is now much clearer, but release policy, semantic
  versioning, and adoption guarantees still belong to `g02.015`

## Next Task

Open `docs/roadmaps/g02/015-packaging-release-and-versioning-baseline.md` and
turn the newly explicit package surface into a real packaging and release
baseline.
