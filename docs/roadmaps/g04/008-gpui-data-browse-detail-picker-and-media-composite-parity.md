# g04.008 GPUI Data, Browse, Detail, Picker, And Media Composite Parity

Status: completed
Owner: Pug Core
Updated: 2026-03-12
Depends on: g04.003, g04.004, g04.005, g04.006, g04.007
Primary repos: `pug`

## Goals

- [x] implement the broader GPUI composite families that make the shared system
  feel real outside the primitive layer
- [x] define where native-runtime media, list, table, and picker behavior
  departs intentionally from the browser surface

## Execution Checklist

- [x] implement GPUI data-table, browse-shell, detail-shell, picker, relation,
  media-preview, and related composites where the contracts already exist
- [x] align loading, empty, no-results, error, and recovery posture to the contracts
- [x] document virtualization, media, and list-navigation deltas explicitly
- [x] verify these composites are sufficient to support downstream GPUI app proofs

## Acceptance Criteria

- [x] GPUI data and browse composite posture is explicit
- [x] GPUI detail, picker, and media parity posture is explicit

## Completed Work

- added the normative baseline `docs/specs/055-gpui-data-browse-detail-picker-and-media-composite-baseline.md`
- added the machine-readable artifact `packages/gpui/data-browse-detail-picker-media-baseline.json`
- expanded `packages/gpui/composites` with:
  - `DataTableSpec`
  - `ListShellSpec`
  - `GridShellSpec`
  - `DetailShellSpec`
  - `FilterToolbarSpec`
  - `PaginationSummarySpec`
  - `EmptyStateSpec`
  - `PickerShellSpec`
  - `RelationPickerSpec`
  - `SelectionSummarySpec`
  - `MediaThumbnailSpec`
  - `MediaPreviewSpec`
- added shared GPUI composite support types for browse state, picker posture, media state, table columns and rows, selection summaries, and scroll ownership
- froze browse-state differentiation, visible-scope selection, detail-shell state posture, picker summary posture, and media fallback structure inside the GPUI composite layer so downstream apps inherit the same contract semantics as Svelte
- added crate tests for visible-scope table selection, browse-shell state distinctions, detail-shell and empty-state posture, picker selection summaries, and media fallback behavior
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI data or browse or detail or picker or media baseline artifact is machine-checked
- updated package and roadmap surfaces so the repo now points at `g04.009`

## Next Task

Open `g04.009` and implement the GPUI workstation shell, command discovery,
and layout orchestration tranche.
