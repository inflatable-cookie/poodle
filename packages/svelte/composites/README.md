# Poodle Svelte Composites

First Svelte composite implementation surface for contract-backed Poodle browse
and data components.

## Public Surface

- `ActionDiscoveryPanel`
- `AppHeader`
- `AudioPlayer`
- `Breadcrumbs`
- `BlockEditor`
- `BulkActionBar`
- `Card`
- `CardRadioGroup`
- `CommandPalette`
- `ConfirmAction`
- `DataTable`
- `DetailRow`
- `DetailSection`
- `DetailShell`
- `DockRegion`
- `EditableList`
- `EmbedInput`
- `EmbedPreview`
- `EmptyState`
- `FilterToolbar`
- `FormDialog`
- `FormLayout`
- `ListCard`
- `ListContainer`
- `LogList`
- `MarkdownEditor`
- `MediaBrowsePanel`
- `MediaPicker`
- `MediaPreview`
- `MediaThumbnail`
- `MediaUploadStatusPanel`
- `MetricTile`
- `NavCard`
- `NavCardGrid`
- `OrderBy`
- `PageHeader`
- `PageLoading`
- `PaginationSummary`
- `PickerShell`
- `RelationPicker`
- `ResizeHandle`
- `ReorderableList`
- `SelectionSummary`
- `SidebarNav`
- `SplitView`
- `StatusBar`
- `ToastStack`
- `VideoPlayer`
- `parseWorkspaceLayoutSnapshot`
- `detectParsedEmbed`
- `resolveEmbedParseState`
- `serializeWorkspaceLayoutSnapshot`
- root import: `@poodle/svelte-composites`
- type-only import: `@poodle/svelte-composites/types`

## Stability Notes

- public entry points are the package root and `./types`
- row rendering, async data policy, ranking, and workflow orchestration remain
  host-owned semantics even when the composites own shell posture
- virtualization strategy, embed runtimes, and richer asset playback stay out
  of the public package contract for now
- GPUI parity for this family is still mostly documented debt, not shipped code

## Next Task

Use this package surface while following the direct-consumer and shell-oriented
onboarding lanes, keeping workflow orchestration host-owned and treating these
composites as reusable shells rather than app templates.
