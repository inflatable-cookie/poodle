# g05.002 GPUI Theme Runtime, Token Application, And Native Preview App Baseline

Status: completed
Owner: Poodle Core
Updated: 2026-03-13
Depends on: g05.001
Primary repos: `poodle`

## Goals

- [x] define how GPUI consumes emitted tokens and themes as a real runtime
- [x] define the minimum GPUI preview or review app for repeat inspection and
  side-by-side comparison against the Svelte preview

## Execution Checklist

- [x] define GPUI theme ingestion, mode switching, and token application rules
- [x] define the minimum native preview app or inspection shell needed for
  repeat GPUI review
- [x] define which Svelte preview sections, states, and controls must have
  matching GPUI review surfaces for direct comparison
- [x] define what native screenshots, recordings, or inspection evidence are
  worth capturing in this generation
- [x] avoid treating browser preview routes as a substitute for native review

## Acceptance Criteria

- [x] GPUI runtime theming posture is explicit
- [x] GPUI preview app posture is explicit
- [x] side-by-side review coverage against the Svelte preview is explicit

## Completed Work

- added the normative baseline `docs/specs/049-gpui-theme-runtime-and-native-preview-app-baseline.md`
- added the machine-readable preview artifact `packages/gpui/preview-app-baseline.json`
- froze the wave-0 GPUI preview section set at `catalog-hub`, `token-summary-section`, and `token-inspector`
- defined required review controls for section, theme, density, and control size so the native review surface starts from the same inspection dimensions as the Svelte preview
- defined initial evidence-capture expectations for screenshots, inspection notes, and theme-plus-section snapshots
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the GPUI preview baseline remains machine-checked
- updated `packages/gpui/tokens/README.md` so the token crate points at the preview-app baseline as part of the wider GPUI runtime posture

## Next Task

Open `g05.003` and implement the GPUI structural primitive tranche on top of
the now-explicit theme runtime and native preview app baseline.
