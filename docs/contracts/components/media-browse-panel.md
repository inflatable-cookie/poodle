# MediaBrowsePanel

Status: detailed contract
Updated: 2026-05-18

## 1. Purpose

- Component name: `MediaBrowsePanel`
- Layer: `composites`
- Summary: a browse grid for media items with loading, error, empty, and load-more postures
- In scope: media card grid, selection events, empty/loading/error states, load-more action, thumbnail rendering via `MediaThumbnail`
- Out of scope: dialog ownership, server fetching, search inputs, upload orchestration, pagination state ownership

## 2. Anatomy

```text
[Root]
  ├── [State]          (loading | error | empty)
  │     ├── [Loading]  <p> centered copy
  │     ├── [Error]    Callout (danger tone)
  │     └── [Empty]    <p> centered copy
  └── [Ready]
        ├── [Grid]
        │     └── [Item...]  <button>
        │           ├── [MediaThumbnail]
        │           │     └── [Image]  <img> (optional, when thumbnailUrl present)
        │           ├── [Label]  <span>
        │           └── [Meta]   <span> (optional)
        └── [Actions]  (when hasMore)
              └── [Load More Button]
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | Container with class `media-browse-panel`, full width, `box-sizing: border-box`, `min-height: 18rem` |
| state | `<div>` | Centered state area for loading/empty postures |
| error | `Callout` | Danger-tone callout for error state |
| grid | `<div>` | CSS auto-fill grid of media items |
| item | `<button>` | Clickable media card with border, radius, background |
| thumbnail | `MediaThumbnail` | Media thumbnail shell with `compact` presentation, `square` aspect ratio |
| image | `<img>` | Thumbnail image when `thumbnailUrl` is present |
| label | `<span>` | Item label, bold, truncated |
| meta | `<span>` | Optional secondary text below label |
| actions | `<div>` | Centered action area for load-more button |
| load-more | `Button` | Secondary variant button; disables during loading |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `loading` | `boolean` | `false` | no | Shows the loading state and disables load-more |
| `error` | `string \| null` | `null` | no | Error callout message |
| `items` | `MediaPickerItem[]` | `[]` | no | Media items to render |
| `hasMore` | `boolean` | `false` | no | Whether to show the load-more action |
| `emptyMessage` | `string` | `"No media found"` | no | Empty-state message |
| `loadMoreLabel` | `string` | `"Load more"` | no | Label for the load-more button |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for browse card geometry and load-more action |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for card and grid spacing |

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

type MediaKind = "image" | "audio" | "video" | "document" | "embed";
```

### Slots

None.

### Controlled / Uncontrolled

All props are controlled. The component does not own any state; the host provides items, loading, and error.

Internally, the panel resolves semantic size and density from presentation
context, but it does not maintain browse, selection, or loading state of its
own.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| loading | `loading && items.length === 0` | Centered "Loading media..." copy |
| error | `error !== null` | Danger callout via `Callout` primitive |
| empty | `!loading && !error && items.length === 0` | Centered empty message copy |
| ready | `items.length > 0` | Media grid renders |
| loading-more | `loading && hasMore && items.length > 0` | Load-more button disabled, label switches to "Loading..." |

### Component States

| State | Description |
|-------|-------------|
| initial-loading | No items yet, loading is true |
| browsing | Items present, user can select |
| load-more-available | Items present and `hasMore` is true |

## 5. Callbacks

| Callback | When It Fires | Signature |
|----------|---------------|-----------|
| `onSelect` | User clicks a media item | `(item: MediaPickerItem) => void` |
| `onLoadMore` | User clicks the load-more button | `() => void` |

## 6. Accessibility

### Semantics

- Items are rendered as real `<button>` elements with `type="button"`
- Error state uses `Callout` primitive with `announceMode="polite"` for screen reader announcement
- Each `MediaThumbnail` receives `ariaLabel` from the item label
- When `thumbnailUrl` is present, the `<img>` has `alt` set to the item label

### Keyboard

- Tab navigates between grid items and load-more button
- Enter/Space selects a media item
- Standard button focus behavior

### Focus

- Grid items show border and background change on `:focus-visible`, outline suppressed
- Load-more button uses `Button` primitive focus behavior

## 7. Layout

### Sizing

- Root: `min-height: 18rem`
- Root: `width: 100%`, `box-sizing: border-box`
- Grid: CSS `auto-fill` grid with semantic size-driven minimum column width, density-aware gap, `margin-top` equal to grid gap
- Item: grid layout, density-aware gap and padding, border `0.0625rem solid border-subtle`, `radius-surface`
- Image: `display: block`, `width: 100%`, `height: 100%`, `object-fit: cover`
- State area: grid centered with `min-height: 18rem`

### Composition

- Uses `MediaThumbnail` for the media shell (compact presentation, square aspect ratio)
- Uses `Button` for load-more action (secondary variant)
- Uses `Callout` for error state (danger tone)
- Wraps children in `UiPresentationProvider` with resolved size and density
- Keeps fetching and cursor ownership outside the component

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | root `.media-browse-panel` | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | root `.media-browse-panel` | `"compact"`, `"default"`, `"comfortable"` |

### Root `.media-browse-panel`

| Property | Value |
|----------|-------|
| width | `100%` |
| box-sizing | `border-box` |
| min-height | `18rem` |

### Grid `.media-browse-panel__grid`

| Property | Value |
|----------|-------|
| display | `grid` |
| grid-template-columns | `repeat(auto-fill, minmax(11rem, 1fr))` (default) |
| gap | `var(--poodle-space-stack-sm)` (default) |
| margin-top | `var(--poodle-space-stack-sm)` (default, same as gap) |

### Item `.media-browse-panel__item`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `0.375rem` (default) |
| padding | `0.75rem` (default) |
| border | `0.0625rem solid var(--poodle-color-border-subtle)` |
| border-radius | `var(--poodle-radius-surface)` |
| background | `color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent)` |
| color | `inherit` |
| text-align | `left` |
| cursor | `pointer` |

#### Item States

| State | Property | Value |
|-------|----------|-------|
| `:hover` | border-color | `var(--poodle-color-border-focus)` |
| `:hover` | background | `color-mix(in srgb, var(--poodle-color-background-elevated) 90%, transparent)` |
| `:hover` | outline | `none` |
| `:focus-visible` | border-color | `var(--poodle-color-border-focus)` |
| `:focus-visible` | background | `color-mix(in srgb, var(--poodle-color-background-elevated) 90%, transparent)` |
| `:focus-visible` | outline | `none` |

### Image `.media-browse-panel__image`

| Property | Value |
|----------|-------|
| display | `block` |
| width | `100%` |
| height | `100%` |
| object-fit | `cover` |

### Label `.media-browse-panel__label`

| Property | Value |
|----------|-------|
| font-size | `var(--poodle-typography-body-size)` |
| font-weight | `600` |
| overflow | `hidden` |
| text-overflow | `ellipsis` |
| white-space | `nowrap` |

### Meta `.media-browse-panel__meta` and State `p`

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.8125rem` |
| line-height | `1.45` |

### Actions `.media-browse-panel__actions`

| Property | Value |
|----------|-------|
| display | `flex` |
| width | `100%` |
| align-items | `center` |
| justify-content | `center` |
| margin-top | same as grid gap |

### State `.media-browse-panel__state`

| Property | Value |
|----------|-------|
| display | `grid` |
| place-items | `center` |
| min-height | `18rem` |
| text-align | `center` |

### Size Adjustments

| Size | Grid min column |
|------|-----------------|
| `xs` | `8.5rem` |
| `sm` | `10rem` |
| `md` (default) | `11rem` |
| `lg` | `12rem` |
| `xl` | `13rem` |

### Density Adjustments

| Density | Grid gap | Item gap | Item padding |
|---------|----------|----------|-------------|
| `compact` | `0.375rem` | `0.25rem` | `0.5rem` |
| `default` | `var(--poodle-space-stack-sm)` | `0.375rem` | `0.75rem` |
| `comfortable` | `0.75rem` | `0.5rem` | `0.875rem` |

## 9. Svelte Notes

- Uses `MediaThumbnail` for the media shell with `compact` presentation and `square` aspect ratio
- Uses `toMediaKind()` helper to default `kind` to `"image"` when undefined
- Load-more button label switches to `"Loading..."` when `loading` is true and `hasMore`
- Wraps content in `UiPresentationProvider` with resolved size scale and density
- Uses `resolveSemanticControlSize()` and `getUiPresentation()` for presentation context
- Root and load-more action row both pin `width: 100%` so the panel and footer
  align consistently inside broader specimen or shell layouts

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] All props have the same meaning and defaults
- [ ] Event names and payloads match
- [ ] Loading/error/empty/ready state transitions match
- [ ] Load-more behavior and disabling match

### Tier 2: Visual Parity

- [ ] Grid layout and card styling match
- [ ] MediaThumbnail integration matches
- [ ] State area centering and sizing match
- [ ] Size and density adjustments match

### Tier 3: Implementation Freedom

- [ ] Internal state management approach may differ
- [ ] MediaThumbnail implementation may differ across targets

## 12. Specimen Definitions

### Browse Grid With Load More

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Browse grid | 6 items with mix of thumbnails and meta, `hasMore=true`, `loadMoreLabel="Load more"` | Grid of media cards with thumbnails, labels, meta; centered load-more button below |

### Loading State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Loading | `loading=true`, `items=[]` | Centered "Loading media..." text |

### Error State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Error | `error="Failed to load media"` | Danger callout with error message |

### Empty State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty | `items=[]`, `emptyMessage="No media found"` | Centered empty message |
