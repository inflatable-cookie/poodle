# g03.010 Accessibility Audit And Assistive-Technology Conformance

Status: completed
Owner: Poodle Core
Updated: 2026-03-12
Depends on: g03.002, g03.003, g03.004, g03.005
Primary repos: `poodle`

## Goals

- [x] audit the shared component suite for accessibility depth
- [x] define where cross-framework accessibility expectations differ and how
  those deltas are documented

## Execution Checklist

- [x] audit the shared suite for accessibility depth
- [x] define cross-framework accessibility deltas
- [x] document how those deltas are recorded and reviewed

## Acceptance Criteria

- [x] accessibility audit posture is explicit
- [x] cross-framework accessibility delta handling is explicit

## Completed Work

- added the normative baseline `docs/specs/043-accessibility-audit-and-cross-runtime-delta-handling-baseline.md`
- added the section-based accessibility audit manifest `packages/svelte/preview/src/accessibility.ts`
- added the generated audit artifact `packages/svelte/preview/artifacts/accessibility-report.json`
- wired audit validation into `packages/svelte/preview/scripts/lint-docs.ts` and report generation into `docs:check`
- made manual review areas plus GPUI accessibility blockers explicit instead of implied

## Next Task

Open `g03.011` and define deprecation, change-control, and release-channel
operations now that accessibility audit posture and cross-runtime delta
handling are explicit.
