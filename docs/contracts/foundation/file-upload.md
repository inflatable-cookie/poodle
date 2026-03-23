# FileUpload

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `FileUpload`
- Layer: `foundation`
- Summary: a drag-and-drop file upload zone with file list, progress tracking,
  image previews, and validation
- In scope: drag-and-drop, click-to-browse, file type and size validation,
  upload progress, image preview thumbnails, error states, multi-file support
- Out of scope: actual HTTP upload logic, server-side processing, chunked
  uploads, resumable uploads

## 2. Anatomy

```text
[Root .file-upload]  <div>
  ├── [Dropzone .file-upload__dropzone]  <div>
  │     ├── [Hidden Input .file-upload__input]  <input type="file">
  │     └── [Dropzone Content .file-upload__dropzone-content]
  │           ├── [Icon .file-upload__icon]  <svg>
  │           ├── [Label .file-upload__label]  <p>
  │           │     └── [Browse .file-upload__browse]  <span>
  │           └── [Hint .file-upload__hint]  <p>
  └── [File List .file-upload__list]  <ul> (conditional: files.length > 0)
        └── [File Item .file-upload__item]  <li> (repeated)
              ├── [Preview .file-upload__preview]  <img> (conditional: image file with showPreview)
              │   OR [File Icon .file-upload__file-icon]  <div> (non-image or no preview)
              ├── [Meta .file-upload__meta]
              │     ├── [Name .file-upload__name]  <span>
              │     └── [Size .file-upload__size]  <span>
              │         OR [Error Text .file-upload__error-text]  <span> (error state)
              ├── [Remove .file-upload__remove]  <button>
              └── [Progress .file-upload__progress]  <div> (conditional: uploading state)
                    └── [Progress Bar .file-upload__progress-bar]  <div>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | outer column container | gap |
| Dropzone | yes | drag-and-drop target area | border, radius, background, cursor |
| Hidden Input | yes | invisible file input element | offscreen positioning |
| Dropzone Content | yes | centered prompt content | gap, alignment |
| Icon | yes | upload arrow icon | size, color |
| Label | yes | instructional text | font-size, color |
| Browse | yes | clickable accent text | color, underline |
| Hint | no | file constraint description | font-size, color |
| File List | no | list of added files | gap |
| File Item | yes | single file row | padding, radius, background |
| Preview | no | image thumbnail | size, radius, object-fit |
| File Icon | no | generic file icon | size, color |
| Meta | yes | file name and size/error | gap |
| Name | yes | truncated file name | font-size, overflow |
| Size | yes | formatted file size | font-size, color |
| Error Text | no | error message for file | font-size, color |
| Progress | no | progress track | height, radius, background |
| Progress Bar | no | progress fill | background, transition |
| Remove | yes | delete file button | size, radius, color, hover |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `accept` | `string \| null` | `null` | no | accepted file types (MIME types, extensions) |
| `maxSize` | `number` | `10485760` | no | max file size in bytes (10 MB default) |
| `multiple` | `boolean` | `false` | no | allow multiple file selection |
| `maxFiles` | `number` | `10` | no | max number of files when multiple |
| `showPreview` | `boolean` | `true` | no | show image thumbnails |
| `disabled` | `boolean` | `false` | no | disables all interaction |
| `files` | `FileUploadItem[]` | `[]` | no | bindable file list |

### Public Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `updateProgress` | `(id: string, progress: number) => void` | update upload progress for a file |
| `setError` | `(id: string, message: string) => void` | set error state on a file |
| `clear` | `() => void` | remove all files and revoke preview URLs |

### Types

```typescript
interface FileUploadItem {
  file: File;
  id: string;
  progress: number;
  status: "pending" | "uploading" | "complete" | "error";
  previewUrl: string | null;
  error?: string;
}
```

### Controlled And Uncontrolled

- `files` is a bindable prop; the component manages the list internally but
  exposes it for parent binding
- Progress and error states are updated via public methods

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | initial render | empty dropzone with prompt |
| disabled | `disabled=true` | reduced opacity, no pointer events |
| drag active | file dragged over dropzone | accent border, tinted background |
| hover | pointer over dropzone | border and background change |
| has files | files added | file list visible below dropzone |
| file uploading | `status="uploading"` | progress bar visible on item |
| file complete | `status="complete"` | no progress bar |
| file error | `status="error"` | error-tinted background, error text |

### Component States

| State | Type | Initial |
|-------|------|---------|
| `files` | `FileUploadItem[]` | `[]` |
| `isDragActive` | `boolean` | `false` |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `change` | files added or removed | `{ files: FileUploadItem[] }` | full file list |
| `error` | file fails validation | `{ file: File, message: string }` | rejected file and reason |
| `remove` | file removed from list | `{ item: FileUploadItem }` | removed item |

## 6. Accessibility

### Semantics

- Dropzone: focusable `<div>` with `role="button"`, `tabindex="0"`
- Hidden input: `<input type="file">` with `accept` and `multiple` attributes
- File list: `<ul>` with `<li>` items
- Remove button: `<button>` with accessible label (e.g., "Remove filename")
- Disabled: modifier class `.file-upload--disabled` on root

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus to dropzone, then to remove buttons |
| `Enter` / `Space` | on dropzone, opens file picker dialog |
| `Enter` / `Space` | on remove button, removes the file |

### Focus And Announcement

- Dropzone shows focus-visible styles on keyboard focus
- File additions and errors should be announced to assistive technology

## 7. Layout

### Sizing

- Root fills parent width
- Dropzone min-height: `8rem`
- File item preview/icon: `2rem` square
- Remove button: `1.5rem` square

### Composition

- parent expectations: form fields, settings panels, upload dialogs
- child expectations: no child slots; content is self-contained

## 8. Token Usage — Exact Values

### Root `.file-upload`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `var(--poodle-space-stack-sm, 0.5rem)` |

### Root Disabled `.file-upload--disabled`

| Property | Value |
|----------|-------|
| `opacity` | `0.5` |
| `pointer-events` | `none` |

### Dropzone `.file-upload__dropzone` (default)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-height` | `8rem` |
| `padding` | `var(--poodle-space-panel-y, 1.5rem) var(--poodle-space-panel-x, 1.5rem)` |
| `border` | `0.125rem dashed var(--poodle-color-border-default, #444)` |
| `border-radius` | `var(--poodle-radius-surface, 0.5rem)` |
| `background` | `transparent` |
| `cursor` | `pointer` |
| `transition` | `border-color 0.15s, background 0.15s` |

### Dropzone Hover/Focus `.file-upload__dropzone:hover`, `.file-upload__dropzone:focus-visible`

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-border-focus, #888)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel, #1a1a1a) 50%, transparent)` |
| `outline` | `none` |

### Dropzone Active `.file-upload__dropzone--active`

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-default, #6366f1)` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-default, #6366f1) 8%, transparent)` |

### Hidden Input `.file-upload__input`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `width` | `0` |
| `height` | `0` |
| `overflow` | `hidden` |
| `opacity` | `0` |

### Dropzone Content `.file-upload__dropzone-content`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `text-align` | `center` |

### Icon `.file-upload__icon`

| Property | Value |
|----------|-------|
| `width` | `2rem` |
| `height` | `2rem` |
| `color` | `var(--poodle-color-text-secondary, #999)` |

### Label `.file-upload__label`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `font-size` | `0.875rem` |
| `color` | `var(--poodle-color-text-secondary, #999)` |

### Browse `.file-upload__browse`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-accent-default, #6366f1)` |
| `text-decoration` | `underline` |

### Hint `.file-upload__hint`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `font-size` | `0.75rem` |
| `color` | `var(--poodle-color-text-tertiary, #666)` |

### File List `.file-upload__list`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.25rem` |
| `margin` | `0` |
| `padding` | `0` |
| `list-style` | `none` |

### File Item `.file-upload__item`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.5rem` |
| `padding` | `0.375rem 0.5rem` |
| `border-radius` | `var(--poodle-radius-control, 0.375rem)` |
| `background` | `var(--poodle-color-background-panel, #1a1a1a)` |

### File Item Error `.file-upload__item--error`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-text-danger, #ef4444) 10%, var(--poodle-color-background-panel, #1a1a1a))` |

### Preview `.file-upload__preview`

| Property | Value |
|----------|-------|
| `width` | `2rem` |
| `height` | `2rem` |
| `border-radius` | `0.25rem` |
| `object-fit` | `cover` |

### File Icon `.file-upload__file-icon`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `2rem` |
| `height` | `2rem` |
| `flex-shrink` | `0` |

### File Icon SVG `.file-upload__file-icon svg`

| Property | Value |
|----------|-------|
| `width` | `1.25rem` |
| `height` | `1.25rem` |
| `color` | `var(--poodle-color-text-tertiary, #666)` |

### Meta `.file-upload__meta`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.0625rem` |
| `flex` | `1` |
| `min-width` | `0` |

### Name `.file-upload__name`

| Property | Value |
|----------|-------|
| `font-size` | `0.8125rem` |
| `white-space` | `nowrap` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |

### Size `.file-upload__size`

| Property | Value |
|----------|-------|
| `font-size` | `0.75rem` |
| `color` | `var(--poodle-color-text-secondary, #999)` |

### Error Text `.file-upload__error-text`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-danger, #ef4444)` |

### Progress `.file-upload__progress`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `bottom` | `0` |
| `left` | `0` |
| `right` | `0` |
| `height` | `0.125rem` |
| `border-radius` | `0 0 var(--poodle-radius-control, 0.375rem) var(--poodle-radius-control, 0.375rem)` |
| `background` | `var(--poodle-color-border-default, #444)` |
| `overflow` | `hidden` |

### Progress Bar `.file-upload__progress-bar`

| Property | Value |
|----------|-------|
| `height` | `100%` |
| `background` | `var(--poodle-color-accent-default, #6366f1)` |
| `transition` | `width 0.2s` |

### Remove Button `.file-upload__remove` (default)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `flex-shrink` | `0` |
| `width` | `1.5rem` |
| `height` | `1.5rem` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `0.25rem` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary, #999)` |
| `cursor` | `pointer` |

### Remove Button Hover `.file-upload__remove:hover`

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-background-elevated, #2a2a2a)` |
| `color` | `var(--poodle-color-text-default, #eee)` |

### Remove Button SVG `.file-upload__remove svg`

| Property | Value |
|----------|-------|
| `width` | `0.875rem` |
| `height` | `0.875rem` |

## 9. Svelte Notes

- Hidden `<input type="file">` is triggered programmatically via
  `inputEl.click()` on dropzone click or Enter/Space
- Drag events (`dragenter`, `dragover`, `dragleave`, `drop`) on the dropzone
  toggle `isDragActive` state
- File validation checks: size against `maxSize`, type against `accept` (supports
  extensions like `.jpg`, MIME wildcards like `image/*`, and exact MIME types)
- Image preview URLs are created via `URL.createObjectURL()` for `image/*` files
  and revoked on remove or component destroy
- `updateProgress(id, progress)` sets `item.progress` and changes status to
  `"uploading"` (or `"complete"` at 100)
- `setError(id, message)` sets `item.status = "error"` and `item.error`
- `clear()` revokes all preview URLs and empties the file list
- File size formatting: bytes, KB, MB display

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::file_upload`
- Drag-and-drop: GPUI must integrate with platform drag-and-drop APIs for file
  drops
- File picker: GPUI must open a native file dialog on click/keyboard activation
- Image preview: GPUI renders thumbnail from file bytes; preview URL mechanism
  differs from web blob URLs
- `color-mix` for dropzone active: GPUI computes
  `accent.opacity(accent.a * 0.08)`
- `color-mix` for dropzone hover: GPUI computes
  `panel_bg.opacity(panel_bg.a * 0.50)`
- `color-mix` for error item: GPUI blends danger at 10% with panel background
- Progress bar width is set as percentage of track width

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `accept` file type filtering matches
- [ ] `maxSize` validation and error reporting matches
- [ ] `multiple` and `maxFiles` constraints match
- [ ] `files` bindable state shape matches `FileUploadItem` interface
- [ ] `change`, `error`, `remove` event payloads match
- [ ] `updateProgress`, `setError`, `clear` public API matches
- [ ] file status enum values match: pending, uploading, complete, error
- [ ] keyboard activation (Enter/Space on dropzone) matches

### Tier 2: Visual Parity

- [ ] dropzone min-height 8rem matches
- [ ] dropzone dashed border 0.125rem matches
- [ ] dropzone active accent border and 8% tint matches
- [ ] dropzone hover border-focus and 50% panel background matches
- [ ] file item padding 0.375rem 0.5rem matches
- [ ] file item error background color-mix 10% danger matches
- [ ] preview thumbnail 2rem square with 0.25rem radius matches
- [ ] progress bar height 0.125rem matches
- [ ] progress bar accent background matches
- [ ] remove button 1.5rem square matches
- [ ] remove hover elevated background matches
- [ ] disabled opacity 0.5 matches
- [ ] icon 2rem square, file icon SVG 1.25rem matches
- [ ] name font-size 0.8125rem with ellipsis truncation matches
- [ ] size/hint font-size 0.75rem matches
- [ ] browse text accent color with underline matches

### Tier 3: Implementation Freedom

- [ ] drag event handling internals stay platform-owned
- [ ] file reading and preview generation internals stay platform-owned
- [ ] file size formatting internals stay implementation-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native file dialog appearance varies by platform | OS-native dialog | allowed | ensure accept filter is applied |
| image preview mechanism differs (blob URL vs native image loading) | platform API differences | allowed | preview must display and be revocable |
| drag-and-drop visual feedback may differ slightly in GPUI | platform DnD API differences | allowed | must indicate active drop target state |

## 13. Specimen Definitions

### Image Upload With Preview

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Image upload with preview | `accept="image/*"`, `multiple`, `maxFiles={5}`, `maxSize={5 * 1024 * 1024}` | Dropzone accepting image files, multiple selection enabled, preview thumbnails shown for added images |

### Document Upload (Single File)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Document upload (single file) | `accept=".pdf,.doc,.docx,.txt"`, `maxSize={10 * 1024 * 1024}`, `showPreview={false}` | Dropzone accepting document types, single file only, no image previews in file list |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `disabled` | Dropzone at reduced opacity with no pointer events |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: upload forms, attachment fields, media libraries,
  profile image pickers
- future follow-up: chunked upload support, drag reordering of file list,
  camera capture integration
