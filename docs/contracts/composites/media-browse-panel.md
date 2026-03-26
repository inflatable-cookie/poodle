# MediaBrowsePanel

Status: seed contract
Updated: 2026-03-26

## 1. Purpose

- Component name: `MediaBrowsePanel`
- Layer: `composites`
- Summary: a browse grid for media items with loading, error, empty, and load-more postures
- In scope: media card grid, selection events, empty/loading/error states, load-more action
- Out of scope: dialog ownership, server fetching, search inputs, upload orchestration, pagination state ownership

## 2. Anatomy

```text
[Root]
  ├── [State]          (loading | error | empty)
  └── [Ready]
        ├── [Grid]
        │     └── [Item...]
        │           ├── [MediaThumbnail]
        │           ├── [Label]
        │           └── [Meta]
        └── [Load More Action]
```

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `loading` | `boolean` | `false` | no | Shows the loading state and disables load-more |
| `error` | `string \| null` | `null` | no | Error callout message |
| `items` | `MediaPickerItem[]` | `[]` | no | Media items to render |
| `hasMore` | `boolean` | `false` | no | Whether to show the load-more action |
| `emptyMessage` | `string` | `"No media found"` | no | Empty-state message |
| `loadMoreLabel` | `string` | `"Load more"` | no | Label for the load-more button |
| `size` | `ControlSize \| null` | `null` | no | explicit semantic size override for browse card geometry and load-more action |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for card and grid spacing |

### Types

```ts
type MediaPickerItem = {
  id: string;
  label: string;
  thumbnailUrl?: string | null;
  mimeType?: string | null;
  kind?: MediaKind;
  meta?: string | null;
};
```

## 4. States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| loading | `loading && items.length === 0` | Centered loading copy |
| error | `error !== null` | Danger callout |
| empty | `!loading && !error && items.length === 0` | Centered empty copy |
| ready | items available | Media grid renders |
| loading-more | `loading && hasMore` | Load-more button disables and label switches to loading copy |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `select` | User clicks a media item | `{ item: MediaPickerItem }` |
| `loadMore` | User clicks the load-more action | `void` |

## 6. Accessibility

- Items are rendered as real buttons
- Error state uses `Callout`
- Labels and meta text remain visible outside thumbnail-only treatment

## 7. Composition Notes

- Uses `MediaThumbnail` for the media shell
- Keeps fetching and cursor ownership outside the component
- Works as the browse surface inside richer selectors like Underlay `MediaPicker`
- Grid density and browse-card padding now respond to semantic presentation context rather than fixed rem-only spacing

## 8. Adoption Notes

Use `MediaBrowsePanel` when the host already owns item loading and pagination but
needs a consistent media-library browse presentation. Do not move duplicate
checks, upload handshake orchestration, or modal workflow state into this
component.
