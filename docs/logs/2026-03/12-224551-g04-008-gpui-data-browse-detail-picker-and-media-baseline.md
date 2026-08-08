---
title: g04.008 gpui data browse detail picker and media composite baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, rust, composites, browse]
---

## Summary

Completed `g04.008` by widening `pug-gpui-composites` into the first broad
GPUI data, browse, detail, picker, and media composite baseline.

## What changed

- added the normative baseline `docs/specs/055-gpui-data-browse-detail-picker-and-media-composite-baseline.md`
- completed `docs/roadmaps/g04/008-gpui-data-browse-detail-picker-and-media-composite-parity.md`
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
- added shared GPUI composite support types for browse state, picker posture,
  media state, table columns and rows, selection summaries, and scroll
  ownership
- pinned browse-state differentiation, visible-scope selection, detail-shell
  state posture, picker summary posture, and media fallback structure inside
  the GPUI composite layer so downstream apps inherit the same contract
  semantics as Svelte
- added crate tests for visible-scope table selection, browse-shell state
  distinctions, detail-shell and empty-state posture, picker selection
  summaries, and media fallback behavior
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI
  data or browse or detail or picker or media baseline artifact is
  machine-checked
- rolled the package and roadmap surfaces forward to `g04.009`

## Validation

- `cargo fmt --manifest-path packages/gpui/composites/Cargo.toml`
- `cargo check --manifest-path packages/gpui/composites/Cargo.toml`
- `cargo test --manifest-path packages/gpui/composites/Cargo.toml`
- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.008` is now explicit. Pug has a materially broader GPUI composite
baseline for shared browse, table, picker, detail, and media review surfaces,
which reduces another major “Svelte-only by default” gap before workstation
parity.

## Next

Open `g04.009` and implement the GPUI workstation shell, command discovery,
and layout orchestration tranche on top of the widened primitive and composite
surface.
