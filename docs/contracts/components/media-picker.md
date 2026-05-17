# MediaPicker

Status: detailed contract
Updated: 2026-03-30

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
| root | `<div>` | Flex column container inside dialog body, class `media-picker` |
| tabs | `Tabs` | Browse / Upload tab switcher |
| search | `TextInput` | Filters browse items by label, wrapped in `.media-picker__search` |
| grid | `<div>` | CSS auto-fill grid of media items, `role="listbox"`, `aria-label="Media items"` |
| item | `<button>` | Selectable media item, `role="option"`, `aria-selected="false"` |
| thumbnail | `<img>` or `<div>` | Item thumbnail; placeholder SVG when no `thumbnailUrl` |
| label | `<span>` | Item label, truncated with ellipsis |
| empty | `<div>` | Centered empty message when no items match |
| upload | `FileUpload` | File upload dropzone in the upload tab |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null \| undefined` | `undefined` | no | Controls dialog visibility when supplied; omit for the internal closed path |
| `items` | `MediaPickerItem[]` | `[]` | no | Available media items to browse |
| `accept` | `string` | `"image/*"` | no | File type filter for uploads |
| `maxFileSize` | `number` | `25 * 1024 * 1024` (25 MB) | no | Maximum upload file size in bytes, passed as `maxSize` to FileUpload |
| `title` | `string` | `"Select media"` | no | Dialog title |
| `emptyMessage` | `string` | `"No media items found."` | no | Message when browse grid is empty after filtering |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for tabs, search field, and grid item geometry |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for browse grid spacing and item padding |

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

Supplying `open` makes dialog visibility host-owned through `onOpenChange`.
Omitting it leaves the picker on its internal closed/open path. `activeTab`,
`searchQuery`, and pending upload files are internal state.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| browse-tab | `activeTab="browse"` | Grid and search visible |
| upload-tab | `activeTab="upload"` | FileUpload component visible |
| item-hover | Mouse over grid item | Border color changes to `border-focus`, background to `background-panel` |
| item-focus | Focus-visible on grid item | Same as hover, outline suppressed |
| empty | No items match search | Centered empty message displayed, min-height `10rem` |

### Component States

| State | Description |
|-------|-------------|
| browsing | Default tab, showing media grid |
| uploading | Upload tab active, showing file upload zone |
| searching | Browse tab with active search query filtering results |

## 5. Callbacks

| Callback | When It Fires | Signature |
|----------|---------------|-----------|
| `onSelect` | User clicks a media item | `(item: MediaPickerItem) => void` |
| `onUpload` | Files added via upload tab | `(files: FileUploadItem[]) => void` |
| `onOpenChange` | Dialog open state changes | `(open: boolean) => void` |

Selecting an item calls both `onSelect` and `onOpenChange(false)` so the host
can close the dialog.

## 6. Accessibility

### Semantics

- Dialog provides modal semantics via the `Dialog` primitive (`kind="dialog"`)
- Browse grid has `role="listbox"` with `aria-label="Media items"`
- Each item has `role="option"` with `aria-selected="false"`
- Thumbnail images use `alt=""` (decorative, label provides accessible name)

### Keyboard

- Tab navigates between search, grid items, and upload area
- Enter/Space selects a media item (triggers `select` and auto-close)
- Escape closes the dialog (handled by `Dialog` primitive)

### Focus

- Grid items show border and background change on `:focus-visible`, outline suppressed
- Focus is managed by the `Dialog` primitive on open/close

## 7. Layout

### Sizing

- Root: flex column, `gap` from `--poodle-space-stack-sm`, `min-height: 20rem`
- Grid: CSS `auto-fill` grid with semantic size-driven minimum column width and density-aware gap, `max-height: 20rem`, `overflow-y: auto`
- Item: flex column, centered, density-aware padding, border `0.0625rem solid transparent`, `radius-control`
- Thumbnail: semantic size-driven square thumbnail
- Label: semantic label size, truncated with ellipsis, `max-width: 100%`

### Composition

Composed from `Dialog`, `Tabs`, `TextInput`, and `FileUpload` primitives. Wraps children in `UiPresentationProvider` with resolved size and density.

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | root `.media-picker` | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | root `.media-picker` | `"compact"`, `"default"`, `"comfortable"` |

### Root `.media-picker`

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-direction | `column` |
| gap | `var(--poodle-space-stack-sm, 0.5rem)` |
| min-height | `20rem` |

### Search `.media-picker__search`

| Property | Value |
|----------|-------|
| margin-top | `0.25rem` (default) |

### Grid `.media-picker__grid`

| Property | Value |
|----------|-------|
| display | `grid` |
| grid-template-columns | `repeat(auto-fill, minmax(5.5rem, 1fr))` (default) |
| gap | `0.375rem` (default) |
| max-height | `20rem` |
| overflow-y | `auto` |

### Item `.media-picker__item`

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-direction | `column` |
| align-items | `center` |
| gap | `0.25rem` |
| padding | `0.375rem` (default) |
| border | `0.0625rem solid transparent` |
| border-radius | `var(--poodle-radius-control, 0.375rem)` |
| background | `transparent` |
| cursor | `pointer` |
| transition | `border-color 0.1s, background 0.1s` |

#### Item States

| State | Property | Value |
|-------|----------|-------|
| `:hover` | border-color | `var(--poodle-color-border-focus, #888)` |
| `:hover` | background | `var(--poodle-color-background-panel, #1a1a1a)` |
| `:focus-visible` | border-color | `var(--poodle-color-border-focus, #888)` |
| `:focus-visible` | background | `var(--poodle-color-background-panel, #1a1a1a)` |
| `:focus-visible` | outline | `none` |

### Thumbnail `.media-picker__thumb`

| Property | Value |
|----------|-------|
| width | `4.5rem` (default, varies by size) |
| height | `4.5rem` (default, varies by size) |
| border-radius | `0.25rem` |
| object-fit | `cover` |

#### Placeholder Thumbnail `.media-picker__thumb--placeholder`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| background | `var(--poodle-color-background-panel, #1a1a1a)` |

#### Placeholder SVG

| Property | Value |
|----------|-------|
| width | `1.5rem` |
| height | `1.5rem` |
| color | `var(--poodle-color-text-tertiary, #666)` |

### Label `.media-picker__label`

| Property | Value |
|----------|-------|
| font-size | `var(--poodle-typography-label-size)` |
| color | `var(--poodle-color-text-secondary, #999)` |
| white-space | `nowrap` |
| overflow | `hidden` |
| text-overflow | `ellipsis` |
| max-width | `100%` |

### Empty `.media-picker__empty`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| min-height | `10rem` |

#### Empty Message `p`

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary, #999)` |
| font-size | `0.875rem` |

### Upload `.media-picker__upload`

| Property | Value |
|----------|-------|
| margin-top | `0.25rem` (default, varies by density) |

### Size Adjustments

| Size | Thumb size | Grid min column |
|------|-----------|-----------------|
| `xs` | `3.5rem` | `4.75rem` |
| `sm` | `4.25rem` | `5.25rem` |
| `md` (default) | `4.5rem` | `5.5rem` |
| `lg` | `5rem` | `6rem` |
| `xl` | `5.5rem` | `6.5rem` |

### Density Adjustments

| Density | Search offset | Grid gap | Item padding |
|---------|--------------|----------|-------------|
| `compact` | `0.125rem` | `0.25rem` | `0.25rem` |
| `default` | `0.25rem` | `0.375rem` | `0.375rem` |
| `comfortable` | `0.375rem` | `0.5rem` | `0.5rem` |

## 9. Svelte Notes

- Composes `Dialog`, `Tabs`, `TextInput`, `FileUpload` from `@poodle/svelte`
- Wraps content in `UiPresentationProvider` with resolved size and density
- `filteredItems` reactive statement filters by label case-insensitively
- Selecting an item calls both `onSelect` and `onOpenChange(false)` so the host
  can close the dialog
- `activeTab` is internal state toggling between `"browse"` and `"upload"`
- `uploadFiles` tracked internally for two-way state with `FileUpload`
- Uses `resolveSemanticControlSize()` to derive effective size

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] All props have the same meaning and defaults
- [ ] Event names and payloads match
- [ ] Dialog wrapper with open/close behavior
- [ ] Browse/upload tab switching

### Tier 2: Visual Parity

- [ ] Thumbnail grid layout matches
- [ ] Placeholder thumbnail rendering matches
- [ ] Empty state positioning and styling matches
- [ ] Size and density adjustments match

### Tier 3: Implementation Freedom

- [ ] Internal state management approach may differ
- [ ] FileUpload integration details may differ

## 12. Specimen Definitions

### Media Picker Dialog

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Media picker dialog | `open` controlled by button, 6 sample items with mix of thumbnails and placeholders, `title="Select an asset"` | Dialog with browse grid of items; selecting shows selected label; upload tab available |
