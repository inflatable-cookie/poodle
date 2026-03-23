# g03.013 Reference Apps, Onboarding Depth, And Public Examples

Status: completed
Owner: Flint Core
Updated: 2026-03-12
Depends on: g03.003, g03.012
Primary repos: `flint`

## Goals

- [x] define exemplar apps or example packages that teach the system clearly
- [x] deepen adopter onboarding and public discoverability

## Execution Checklist

- [x] define reference apps or example packages
- [x] deepen adopter onboarding
- [x] deepen public discoverability through examples

## Acceptance Criteria

- [x] reference-example posture is explicit
- [x] onboarding depth is explicit

## Completed Work

- added the normative baseline `docs/specs/046-reference-apps-onboarding-and-public-example-baseline.md`
- added the machine-readable reference and onboarding matrix `packages/reference-apps.json`
- extended `packages/svelte/preview/scripts/lint-docs.ts` so required reference shapes, onboarding lanes, evidence paths, and example-section coverage remain machine-checked
- aligned the main package and preview README surfaces with the new onboarding posture instead of leaving them on older generation next-task language
- rolled the roadmap and index surfaces forward to generation closeout

## Next Task

Open `g03.014` and close the current program deliberately.
