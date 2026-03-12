# Pug Svelte Composites

First Svelte composite implementation surface for contract-backed Pug browse
and data components.

## Public Surface

- `Card`
- `PageHeader`
- `Breadcrumbs`
- `DetailShell`
- `DetailSection`
- `DetailRow`
- `DataTable`
- `BulkActionBar`
- `PaginationSummary`
- `FilterToolbar`
- `ListShell`
- `GridShell`
- `SelectionSummary`
- `PickerShell`
- `RelationPicker`
- `MediaThumbnail`
- `MediaPreview`
- `EmbedShell`
- `EmptyState`
- `ToastStack`
- root import: `@pug/svelte-composites`
- type-only import: `@pug/svelte-composites/types`

## Stability Notes

- public entry points are the package root and `./types`
- row rendering, async data policy, ranking, and workflow orchestration remain
  host-owned semantics even when the composites own shell posture
- virtualization strategy, embed runtimes, and richer asset playback stay out
  of the public package contract for now
- GPUI parity for this family is still mostly documented debt, not shipped code

## Next Task

Use this package surface while executing `g02.016`, confirming which composite
entry points are stable enough to carry into the first downstream-adoption
generation.
