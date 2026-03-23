# 055 GPUI Data, Browse, Detail, Picker, And Media Composite Baseline

Status: active
Updated: 2026-03-12
Depends on: `054-gpui-form-validation-and-remediation-composite-baseline.md`

## Purpose

Freeze the broader GPUI composite layer that makes the shared system usable for
real review surfaces outside the primitive tier. This baseline adds the core
data, browse, detail, picker, and media shells so downstream GPUI apps can
compose the same UI families as Svelte against one contract-owned catalogue.

## Package Rule

The `g04.008` tranche extends `poodle-gpui-composites` with:

- `DataTableSpec`
- `DetailShellSpec`
- `FilterToolbarSpec`
- `PaginationSummarySpec`
- `EmptyStateSpec`
- `PickerShellSpec`
- `RelationPickerSpec`
- `SelectionSummarySpec`
- `MediaThumbnailSpec`
- `MediaPreviewSpec`

These exports stay preview-channel public-intent GPUI composites and build on
the widened GPUI primitive and form-shell baseline.

## Contract Coverage Rule

The crate must stay aligned to the existing shared composite contracts for:

- `data-table`
- `detail-shell`
- `filter-toolbar`
- `pagination-summary`
- `empty-state`
- `picker-shell`
- `relation-picker`
- `selection-summary`
- `media-thumbnail`
- `media-preview`

## Browse And Data Rule

This baseline freezes the shared browse posture that later GPUI shells must
reuse:

- browse-state differentiation between `empty`, `loading`, `error`, and
  `no-results`
- shell-owned scroll posture remaining explicit
- visible-scope selection semantics in data tables
- card-grid neutrality around child item semantics
- filter and pagination summaries remaining textual and reviewable

## Detail And Picker Rule

This tranche also freezes the shared detail and selection-workflow posture:

- detail shells stay distinct from browse shells and do not adopt
  `no-results`
- picker workflows preserve explicit title, query, result, and selection
  summary posture
- relation pickers preserve single or multiple selection meaning explicitly
- selected-summary surfaces remain textual and removable

## Media Rule

Media composites must preserve the same review semantics as Svelte:

- framed thumbnails and richer previews keep fallback structure explicit
- loading, error, and empty media states remain readable rather than blank
  surfaces
- metadata and footer actions remain available outside the rendered media frame

## Runtime Honesty Rule

This tranche remains explicit about current depth:

- state vocabulary, selection posture, fallback structure, and token-backed
  hierarchy are explicit
- mounted native tables, virtualization, media renderer plumbing, and final
  keyboard or accessibility proof still belong to later `g04` milestones

The repo may expose these composites as contract-backed GPUI specs before all
of them are rendered as fully mounted native shells.

## Token Rule

These data and media composites must continue resolving from the existing GPUI
token and primitive baselines for at least:

- browse-shell and panel surface roles
- selected-row and accent emphasis roles
- empty or error or pending message hierarchy
- spacing and gap cadence for tables, lists, grids, pickers, and media frames
- footer action and summary-chip posture inherited from lower layers

## Seed Evidence

- `packages/gpui/data-browse-detail-picker-media-baseline.json`
- `packages/gpui/composites/README.md`
- `packages/gpui/composites/src/lib.rs`
- `packages/gpui/composites/src/data_table.rs`
- `packages/gpui/composites/src/detail_shell.rs`
- `packages/gpui/composites/src/filter_toolbar.rs`
- `packages/gpui/composites/src/pagination_summary.rs`
- `packages/gpui/composites/src/empty_state.rs`
- `packages/gpui/composites/src/picker_shell.rs`
- `packages/gpui/composites/src/relation_picker.rs`
- `packages/gpui/composites/src/selection_summary.rs`
- `packages/gpui/composites/src/media_thumbnail.rs`
- `packages/gpui/composites/src/media_preview.rs`

## Next Task

Carry this widened GPUI composite baseline into `g04.009`, implementing the
workstation shell, command discovery, and layout orchestration tranche on top
of the broadened primitive and composite surface.
