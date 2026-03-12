# g03.012 Ecosystem Acceptance Suites And Long-Tail Regression Coverage

Status: completed
Owner: Pug Core
Updated: 2026-03-12
Depends on: g03.002, g03.003, g03.004, g03.005, g03.006, g03.007, g03.008, g03.009, g03.010, g03.011
Primary repos: `pug`

## Goals

- [x] define end-to-end acceptance coverage across representative consumers
- [x] define long-tail regression posture for tokens, contracts, and components

## Execution Checklist

- [x] define representative consumer acceptance suites
- [x] define long-tail regression coverage for tokens, contracts, and
  components
- [x] define what evidence is required before declaring broad ecosystem
  readiness

## Acceptance Criteria

- [x] ecosystem acceptance posture is explicit
- [x] long-tail regression posture is explicit

## Completed Work

- added the normative baseline `docs/specs/045-ecosystem-acceptance-and-long-tail-regression-baseline.md`
- added the machine-readable ecosystem acceptance matrix `packages/ecosystem-acceptance.json`
- extended `packages/svelte/preview/scripts/lint-docs.ts` so required suites, regression classes, and evidence artifacts stay machine-checked
- made the readiness gate explicit across preview, Underlay, Loophole, and GPUI acceptance surfaces
- rolled the roadmap and index surfaces forward to the reference-app and onboarding tranche

## Next Task

Open `g03.013` and deepen reference apps, onboarding, and public-facing
examples now that ecosystem acceptance posture and long-tail regression classes
are explicit.
