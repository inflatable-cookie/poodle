# MediaPicker

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `MediaPicker`
- Layer: `composites`
- Summary: a dialog-based media selection composite with browse grid, search filtering, and file upload tab
- In scope: tabbed browse/upload interface, thumbnail grid, search filtering, file upload integration, item selection, dialog open/close
- Out of scope: image editing/cropping, multi-select, drag-and-drop reordering, media library management, pagination, server-side search

## 2. Anatomy

```text
[Dialog]
  └── [Root]
        ├── [Tabs]  (Browse | Upload)
        ├── [Browse Tab]
        │     ├── [Search Input]
        │     ├── [Grid]  role="listbox"
        │     │     └── [Item...]  role="option"
        │     │           ├── [Thumbnail]  <img> or placeholder SVG
        │     │           └── [Label]
        │     └── [Empty Message]  (when no items match)
        └── [Upload Tab]
              └── [FileUpload]
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| dialog | `Dialog` | Wraps the entire picker; controlled via `open` prop |
| root | `<div>` | Flex column container inside dialog body |
| tabs | `Tabs` | Browse / Upload tab switcher |
| search | `TextInput` | Filters browse items by label |
| grid | `<div>` | CSS grid of media items, `role="listbox"` |
| item | `<button>` | Selectable media item, `role="option"` |
| thumbnail | `<img>` or `<div>` | Item thumbnail; placeholder SVG when no `thumbnailUrl` |
| label | `<span>` | Item label, truncated with ellipsis |
| empty | `<div>` | Centered empty message when no items match |
| upload | `FileUpload` | File upload dropzone in the upload tab |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null` | `null` | no | Controls dialog visibility |
| `items` | `MediaPickerItem[]` | `[]` | no | Available media items to browse |
| `accept` | `string` | `"image/*"` | no | File type filter for uploads |
| `maxFileSize` | `number` | `25 * 1024 * 1024` (25 MB) | no | Maximum upload file size in bytes |
| `title` | `string` | `"Select media"` | no | Dialog title |
| `emptyMessage` | `string` | `"No media items found."` | no | Message when browse grid is empty |

### Types

```ts
type MediaPickerItem = {
  id: string;
  label: string;
  thumbnailUrl?: string | null;
  mimeType?: string | null;
  kind?: MediaKind;
};

type MediaKind = "image" | "audio" | "video" | "document" | "embed";
```

### Slots

None.

### Controlled / Uncontrolled

`open` is controlled externally. The component dispatches `openChange` to request state changes.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| browse-tab | `activeTab="browse"` | Grid and search visible |
| upload-tab | `activeTab="upload"` | FileUpload component visible |
| item-hover | Mouse over grid item | Border color changes to `border-focus`, background to `background-panel` |
| item-focus | Focus-visible on grid item | Same as hover |
| empty | No items match search | Centered empty message displayed |

### Component States

| State | Description |
|-------|-------------|
| browsing | Default tab, showing media grid |
| uploading | Upload tab active, showing file upload zone |
| searching | Browse tab with active search query filtering results |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `select` | User clicks a media item | `{ item: MediaPickerItem }` |
| `upload` | Files added via upload tab | `{ files: FileUploadItem[] }` |
| `openChange` | Dialog open state changes | `{ open: boolean }` |

## 6. Accessibility

### Semantics

- Dialog provides modal semantics via the `Dialog` primitive
- Browse grid has `role="listbox"` with `aria-label="Media items"`
- Each item has `role="option"` with `aria-selected="false"`

### Keyboard

- Tab navigates between search, grid items, and upload area
- Enter/Space selects a media item
- Escape closes the dialog (handled by `Dialog` primitive)

### Focus

- Grid items show border and background change on `focus-visible`
- Focus is managed by the `Dialog` primitive on open/close

## 7. Layout

### Sizing

- Root: flex column, `gap: space-stack-sm`, `min-height: 20rem`
- Grid: CSS auto-fill grid, `minmax(5.5rem, 1fr)` columns, `0.375rem` gap, `max-height: 20rem` with overflow scroll
- Item: flex column, centered, padding `0.375rem`, border `0.0625rem solid transparent`, `radius-control`
- Thumbnail: `4.5rem x 4.5rem`, `0.25rem` radius, `object-fit: cover`
- Label: `0.6875rem` font-size, truncated with ellipsis

### Composition

Composed from `Dialog`, `Tabs`, `TextInput`, and `FileUpload` primitives.

## 8. Token Usage

| Property | Token |
|----------|-------|
| Root gap | `space-stack-sm` |
| Item radius | `radius-control` |
| Item hover/focus border | `color-border-focus` |
| Item hover/focus background | `color-background-panel` |
| Thumbnail placeholder bg | `color-background-panel` |
| Thumbnail placeholder icon | `color-text-tertiary` |
| Label color | `color-text-secondary` |
| Empty message color | `color-text-secondary` |

## 9. Svelte Notes

- Composes `Dialog`, `Button`, `TextInput`, `FileUpload`, and `Tabs` from `@flint/svelte-primitives`
- `filteredItems` reactive statement filters by label case-insensitively
- Selecting an item dispatches both `select` and `openChange(false)` to auto-close
- `activeTab` is internal state toggling between `"browse"` and `"upload"`

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

| Feature | Svelte | GPUI | Jetstream |
|---------|--------|------|-----------|
| Dialog wrapper | Yes | -- | -- |
| Browse/Upload tabs | Yes | -- | -- |
| Search filtering | Yes | -- | -- |
| Thumbnail grid | Yes | -- | -- |
| Placeholder thumbnail | Yes | -- | -- |
| File upload integration | Yes | -- | -- |
| Empty state | Yes | -- | -- |

## 12. Known Deltas

None yet (single implementation).

## 13. Specimen Definitions

### Media Picker Dialog

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Media picker dialog | `open` controlled by button, 6 sample items, `title="Select an asset"` | Dialog with browse grid of items; selecting shows selected label |

## 14. Approval And Adoption Notes

Use `MediaPicker` for selecting from an existing media library or uploading new files. The component assumes media items are provided as a flat array; server-side search and pagination should be handled by the consuming application, updating the `items` prop reactively.
