# 043 Accessibility Audit And Cross-Runtime Delta Handling Baseline

Status: active
Updated: 2026-03-12
Depends on: `003-accessibility-and-assistive-technology-baseline.md`, `019-advanced-catalog-accessibility-focus-keyboard-and-state-rules.md`, `025-parity-automation-and-harness-boundary.md`, `042-gpui-multi-app-validation-target-matrix.md`

## Purpose

Freeze the minimum accessibility-audit posture Pug should enforce during
`g03`, without pretending that browser preview coverage alone proves native
assistive-technology conformance.

The goal of this milestone is to make accessibility review explicit, section
based, and honest about where Svelte evidence stops and GPUI or native evidence
is still blocked or remains only manual proof rather than explicit conformance.

## Core Rule

Accessibility audit posture must be machine-readable, section based, and honest
about cross-runtime deltas.

Pug should not treat accessibility as an implied outcome of contract docs,
preview examples, or visual parity. The audit surface must state which areas
are explicit, which still require manual review, and which GPUI claims remain
blocked or manual-only.

## Required Baseline

The current baseline must provide one audit target per docs section that
records:

- audit-area status for semantics, focus, keyboard, announcements, and GPUI
- automated checks that can be validated mechanically
- manual checks that still require human review
- stable preview review routes for repeat inspection
- GPUI delta notes and blockers where native proof does not yet exist or still
  remains manual-only

## Current Audit Artifacts

The current accessibility audit baseline lives in:

- `packages/svelte/preview/src/accessibility.ts`
- `packages/svelte/preview/scripts/build-accessibility-report.ts`
- `packages/svelte/preview/artifacts/accessibility-report.json`
- `packages/gpui/native-accessibility-proof.json`

## Cross-Runtime Delta Rule

Pug must document cross-runtime accessibility deltas explicitly whenever Svelte
or browser-native behavior does part of the work automatically and GPUI will
need different implementation machinery.

Common acceptable delta examples include:

- DOM live regions versus native announcement APIs
- browser focus traps or dialog scope versus native focus-scope ownership
- semantic HTML table, form, or list structure versus explicit native
  accessible-tree nodes and relationships
- browser media or document affordances versus native preview or playback
  surfaces

## Honesty Rule

Pug may say:

- the current accessibility audit surface is explicit
- a section has stable review routes and named manual checks
- a GPUI accessibility claim is blocked until native evidence exists
- a GPUI accessibility claim remains manual until mounted runtime proof exists

Pug may not say:

- browser preview coverage proves native assistive-technology conformance
- GPUI accessibility is validated without runnable native evidence
- cross-runtime accessibility deltas are negligible unless they are explicitly
  documented and reviewed

## Current `g03.010` Seed Baseline

The current seed baseline is:

- this normative spec
- `packages/svelte/preview/src/accessibility.ts`
- `packages/svelte/preview/scripts/build-accessibility-report.ts`
- `packages/svelte/preview/artifacts/accessibility-report.json`
- `packages/gpui/native-accessibility-proof.json`
- `docs/roadmaps/g03/010-accessibility-audit-and-assistive-technology-conformance.md`

## Next Task

Carry this baseline into `g03.011` so change-control and release-channel
operations do not weaken accessibility audit posture or turn blocked native
claims into implied release readiness.
