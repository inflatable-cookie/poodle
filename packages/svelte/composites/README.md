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
- `DebugDialog`
- `DetailItem`
- `DetailSection`
- `DetailShell`
- `DockRegion`
- `EditableList`
- `EmbedInput`
- `EmbedPreview`
- `EmptyState`
- `ErrorBoundary`
- `FilterToolbar`
- `FormDialog`
- `FormLayout`
- `InlineListSection`
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
- `OrderBy`
- `PageHeader`
- `PageLoading`
- `PaginationSummary`
- `PickerShell`
- `RelationPicker`
- `ResizeHandle`
- `SelectionSummary`
- `SidebarNav`
- `SplitView`
- `StatusBar`
- `ToastHost`
- `ToastStack`
- `VideoPlayer`
- `computeFileHash`
- `createResetMediaBrowseState`
- `detectParsedEmbed`
- `getProviderAccent`
- `getThumbnailUrl`
- `loadMediaBrowsePage`
- `lookupMeta`
- `mergeMediaBrowseItems`
- `parseEmbed`
- `parseWorkspaceLayoutSnapshot`
- `renderEmbed`
- `resolveEmbedParseState`
- `runMediaUploadWorkflow`
- `serializeWorkspaceLayoutSnapshot`
- `uploadMediaWithKnownHash`
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
