# g05.012 Shared Demo-App Audit, Gap Register, And Target-Shape Freeze

Status: completed
Owner: Flint Core
Updated: 2026-03-13
Depends on: g05.009, g05.010, g05.011
Primary repos: `flint`

## Goals

- [x] stop treating the current Svelte demo or preview surface as a stable
  parity target by implication
- [x] identify the concrete ways the current Svelte demo app fails as the UI
  GPUI is supposed to replicate
- [x] freeze the target shape for a rebuilt shared demo app before more GPUI
  adoption claims are made

## Execution Checklist

- [x] audit the current Svelte demo or preview surface for coverage gaps,
  structural messiness, indirect component usage, and poor side-by-side review
  posture
- [x] identify which public primitives, composites, and workstation exports are
  still missing or only indirectly demonstrated
- [x] separate docs-shell concerns from demo-app concerns so the target UI is
  explicit rather than mixed with catalog or preview plumbing
- [x] freeze the target app shape, section inventory, and quality bar for the
  rebuilt shared demo
- [x] avoid treating the existing preview surface as “good enough” merely
  because it exercises many sections loosely

## Acceptance Criteria

- [x] demo-app audit and gap register are explicit
- [x] target shape for the rebuilt shared demo is explicit
- [x] the repo is pointed at demo contracts and Svelte rebuild work before
  downstream GPUI proof resumes

## Completed Work

- added the normative baseline `docs/specs/059-shared-demo-app-audit-and-target-freeze-baseline.md`
- added the machine-readable audit artifact `packages/shared-demo-app-audit.json`
- froze the current source-surface posture explicitly:
  - `packages/svelte/preview/src/App.svelte` remains a 3328-line monolith
  - `packages/svelte/preview/src/app.css` remains a 2085-line global stylesheet
  - docs-only sections such as `catalog-hub`, `token-summary-section`, and
    `token-inspector` remain useful preview infrastructure but are not the main
    shared demo target GPUI should reproduce
- made the current coverage debt explicit using the generated parity report:
  - `@flint/svelte-primitives`: `14/63` previewed
  - `@flint/svelte-composites`: `18/20` previewed
  - `@flint/svelte-workstation`: `12/14` previewed
- froze the main audit findings: docs-shell leakage, monolithic entry shape,
  broad but shallow coverage, missing coherent app story, and excessive
  preview-glue ownership
- froze the first target shape for the rebuilt shared demo as six coherent
  screen families: shell or overview, form or validation, browse or table,
  detail and related data, picker or media, and command or workspace
- rolled the status surfaces forward so the repo now points at `g05.013`

## Next Task

Open `g05.013` and write the cross-runtime demo-app contract, section model,
and parity checklist for both Svelte and GPUI.
