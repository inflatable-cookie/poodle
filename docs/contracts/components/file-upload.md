# FileUpload

Status: detailed contract
Updated: 2026-03-30

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
| `describedBy` | `string \| null` | `null` | no | `aria-describedby` target for the native file input |
| `files` | `FileUploadItem[]` | `[]` | no | bindable file list |
| `validate` | `(file: File) => string \| null` | `undefined` | no | app-owned custom validation callback |
| `compress` | `boolean` | `false` | no | compress raster images before they are added |
| `compressionOptions` | `ImageCompressionOptions` | `DEFAULT_COMPRESSION` | no | image compression settings |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

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
  originalFile?: File;
}

interface ImageCompressionOptions {
  maxWidth?: number;
  maxHeight?: number;
  quality?: number;
  format?: "image/jpeg" | "image/png" | "image/webp";
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

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onChange` | files added, removed, or status-updated | `FileUploadItem[]` | full file list |
| `onUpload` | validated files added | `File[]` | ready for app-owned upload orchestration |
| `onError` | file fails validation | `{ file: File, message: string }` | rejected file and reason |
| `onRemove` | file removed from list | `FileUploadItem` | removed item |

## 6. Accessibility

### Semantics

- Dropzone: non-interactive `<div>` drop target. It must NOT be a focusable
  `role="button"`: it contains the file input and the browse button, and nesting
  interactive controls is invalid HTML with ambiguous activation semantics
- Hidden input: `<input type="file">` with `accept` and `multiple` attributes.
  Visually hidden but focusable, and carries the accessible label — it is the
  real control that keyboard and assistive technology use
- `describedBy`, when provided, is applied to the native file input and to any
  renderer-owned focusable dropzone shell
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
- Remove button: `1.75rem` square

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
| `font-size` | `0.8125rem` |
| `color` | `var(--poodle-color-text-tertiary, #777)` |

### File List `.file-upload__list`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.5rem` |
| `margin` | `0` |
| `padding` | `0` |
| `list-style` | `none` |

### File Item `.file-upload__item`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `auto 1fr auto` |
| `align-items` | `center` |
| `gap` | `0.75rem` |
| `padding` | `0.75rem` |
| `border-radius` | `var(--poodle-radius-surface, 0.5rem)` |
| `background` | `var(--poodle-color-background-panel, #1a1a1a)` |

### File Item Error `.file-upload__item--error`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-danger-base, #ef4444) 10%, var(--poodle-color-background-panel, #1a1a1a))` |

### Preview `.file-upload__preview`

| Property | Value |
|----------|-------|
| `width` | `2rem` |
| `height` | `2rem` |
| `border-radius` | `0.375rem` |
| `object-fit` | `cover` |

### File Icon `.file-upload__file-icon`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `2rem` |
| `height` | `2rem` |
| `border-radius` | `0.375rem` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface, #111) 82%, transparent)` |
| `color` | `var(--poodle-color-text-secondary, #999)` |

### File Icon SVG `.file-upload__file-icon svg`

| Property | Value |
|----------|-------|
| `width` | `1.25rem` |
| `height` | `1.25rem` |

### Meta `.file-upload__meta`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.125rem` |
| `min-width` | `0` |

### Name `.file-upload__name`

| Property | Value |
|----------|-------|
| `font-size` | `0.875rem` |
| `color` | `var(--poodle-color-text-primary, #f5f5f5)` |
| `white-space` | `nowrap` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |

### Size `.file-upload__size`

| Property | Value |
|----------|-------|
| `font-size` | `0.8125rem` |
| `color` | `var(--poodle-color-text-tertiary, #777)` |

### Error Text `.file-upload__error-text`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-danger-base, #ef4444)` |

### Progress `.file-upload__progress`

| Property | Value |
|----------|-------|
| `grid-column` | `2` |
| `height` | `0.25rem` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface, #111) 82%, transparent)` |
| `overflow` | `hidden` |

### Progress Bar `.file-upload__progress-bar`

| Property | Value |
|----------|-------|
| `height` | `100%` |
| `background` | `var(--poodle-color-accent-default, #6366f1)` |
| `transition` | `width 0.15s ease` |

### Remove Button `.file-upload__remove` (default)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.75rem` |
| `height` | `1.75rem` |
| `padding` | `0` |
| `border` | `none` |
| `border-radius` | `999px` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary, #999)` |
| `cursor` | `pointer` |

### Remove Button Hover `.file-upload__remove:hover`, `.file-upload__remove:focus-visible`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface, #111) 82%, transparent)` |
| `color` | `var(--poodle-color-text-primary, #f5f5f5)` |
| `outline` | `none` |

### Remove Button SVG `.file-upload__remove svg`

| Property | Value |
|----------|-------|
| `width` | `0.875rem` |
| `height` | `0.875rem` |

### Size adjustments

| Size | dropzone min-height | icon size | label font-size | hint font-size |
|------|---------------------|-----------|-----------------|----------------|
| `xs` | `5rem` | `1.5rem` | `0.75rem` | `0.6875rem` |
| `sm` | `6rem` | _(base)_ | _(base)_ | _(base)_ |
| `md` | `8rem` | `2rem` | `0.875rem` | `0.8125rem` |
| `lg` | `10rem` | _(base)_ | `0.9375rem` | _(base)_ |
| `xl` | `12rem` | `2.5rem` | `1rem` | _(base)_ |

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
- `data-size` attribute on root reflects the resolved size for CSS variant styling
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

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

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] dropzone min-height 8rem matches
- [ ] dropzone dashed border 0.125rem matches
- [ ] dropzone active accent border and 8% tint matches
- [ ] dropzone hover border-focus and 50% panel background matches
- [ ] file item grid layout (auto 1fr auto) with 0.75rem gap and 0.75rem padding matches
- [ ] file item border-radius (radius-surface) matches
- [ ] file item error background color-mix 10% danger-base matches
- [ ] preview thumbnail 2rem square with 0.375rem radius matches
- [ ] progress bar height 0.25rem with 999px radius matches
- [ ] progress bar accent background matches
- [ ] remove button 1.75rem square with 999px pill radius matches
- [ ] remove hover surface color-mix background matches
- [ ] disabled opacity 0.5 matches
- [ ] icon 2rem square, file icon SVG 1.25rem matches
- [ ] name font-size 0.875rem with ellipsis truncation matches
- [ ] size font-size 0.8125rem matches
- [ ] hint font-size 0.8125rem matches
- [ ] meta gap 0.125rem matches
- [ ] file list gap 0.5rem matches
- [ ] browse text accent color with underline matches

### Tier 3: Implementation Freedom

- [ ] drag event handling internals stay platform-owned
- [ ] file reading and preview generation internals stay platform-owned
- [ ] file size formatting internals stay implementation-owned

## 11a. Jetstream Notes

- `FileUpload::from_spec(spec, theme).on_remove(...)`, carrying the removed
  file's name — the identity the list itself displays.
- `onUpload` and `onChange` need file drops, which the runtime does not raise.

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native file dialog appearance varies by platform | OS-native dialog | allowed | accept filter applied post-selection with honest rejection (g15.007) |
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

## 15. Rust Binding Notes (g15.007)

Status: **bound** — generic single-file selection/read lands with `g15.007`
Batch A.

- The dropzone/browse affordance wires a generic browse intent:
  `FileUploadHandlers.on_browse` fires on dropzone activation; the OS dialog,
  the file read, and the accept rule are runtime-owned, never component
  logic. LicenceActivation composes this capability; it contains no
  OS-dialog code itself.
- GPUI 0.2.2's `PathPromptOptions` has no accept-filter field. The configured
  `accept` rule (and the web default 10 MB size rule) is therefore enforced
  *after* selection and a rejection is reported honestly
  (`poodle_gpui_node_backend::file_capability::finish_file_pick`), never
  claimed as OS-filtered. Rule semantics mirror
  `packages/core/src/file-upload.ts` (`poodle_headless::file_upload`):
  `.ext` tokens match the extension; `type/*` and exact MIME rules need a MIME
  read, which GPUI cannot supply — such rules refuse honestly rather than
  guess.
- Headless evidence injects a selected fixture path/bytes through the same
  generic seam (`SingleFileSource` + `finish_file_pick`) that the live OS
  prompt uses (`OsFilePrompt`), so a static filename or a prefilled
  credential is never proof of the selection/read path.
- The picked file's payload is encoded to bare base64 (no data-URL prefix)
  through `poodle_headless::file_upload::base64_encode`; credential contents
  never render.
- The licence-file route (LicenceActivation) reads through this capability
  with the host's `fileAccept` rule and reports read/accept failures with the
  web reference's copy.
