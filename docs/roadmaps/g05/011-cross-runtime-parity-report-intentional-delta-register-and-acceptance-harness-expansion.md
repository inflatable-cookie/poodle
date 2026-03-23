# g05.011 Cross-Runtime Parity Report, Intentional Delta Register, And Acceptance-Harness Expansion

Status: completed
Owner: Poodle Core
Updated: 2026-03-12
Depends on: g05.007, g05.008, g05.009, g05.010
Primary repos: `poodle`

## Goals

- [x] turn the new GPUI implementation work into explicit cross-runtime parity evidence
- [x] expand the acceptance harness so GPUI no longer sits mostly outside the
  machine-readable evidence surface
- [x] define how side-by-side Svelte and GPUI review evidence is captured
  without collapsing legitimate native deltas

## Execution Checklist

- [x] define a stronger parity report or artifact surface that includes GPUI evidence
- [x] create or update an intentional delta register rather than burying
  runtime differences in milestone notes
- [x] expand ecosystem acceptance and regression coverage to reflect real GPUI component evidence
- [x] define side-by-side comparison artifacts or review surfaces where they
  materially improve defect detection
- [x] keep manual-boundary honesty intact while automation grows

## Acceptance Criteria

- [x] GPUI parity evidence posture is explicit
- [x] intentional delta and acceptance-harness posture is explicit
- [x] side-by-side comparison posture is explicit

## Completed Work

- added the normative baseline `docs/specs/058-cross-runtime-parity-report-delta-register-and-acceptance-harness-expansion.md`
- added the machine-readable artifact `packages/gpui/cross-runtime-parity-report.json`
- updated `packages/svelte/preview/scripts/build-parity-report.ts` so the
  generated parity artifact now carries a cross-runtime summary, side-by-side
  section set, delta-register summary, and GPUI acceptance-harness alignment
- updated `packages/ecosystem-acceptance.json` so the GPUI suite now carries
  the widened GPUI package set plus parity and accessibility evidence instead
  of reading like a token-only matrix
- updated `packages/svelte/preview/src/parity.ts` and
  `docs/specs/025-parity-automation-and-harness-boundary.md` so the manual
  parity boundary now reflects artifact-backed GPUI evidence without implying
  mounted native proof
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new
  cross-runtime parity report is machine-checked against the Svelte parity
  registry, the GPUI priority matrix, the GPUI accessibility proof, and the
  ecosystem acceptance matrix
- rolled roadmap and status surfaces forward so the repo now points at `g05.012`

## Next Task

Open `g05.012` and prove GPUI adoption through a downstream reference app and
stronger multi-app implementation evidence.
