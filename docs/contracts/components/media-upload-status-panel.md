# MediaUploadStatusPanel

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `MediaUploadStatusPanel`
- Layer: `composites`
- Summary: a status surface for duplicate-check, upload-progress, completion, and error postures in media upload workflows
- In scope: status copy, progress bar, duplicate prompt, success/error actions, spinner states
- Out of scope: file selection, upload execution, duplicate detection logic, media selection ownership

## 2. Anatomy

```text
[Root]
  ├── [Checking]
  │     ├── [Spinner]
  │     └── [Copy]  "Checking for duplicates..."
  ├── [Duplicate]
  │     ├── [Copy]  "This file already exists."
  │     ├── [DuplicateLabel]  <strong> (optional)
  │     └── [Actions]
  │           ├── [Upload Anyway Button]  secondary
  │           └── [Use Existing Button]   primary
  ├── [Uploading]
  │     ├── [Progress Bar]
  │     │     └── [Progress Fill]
  │     └── [Copy]  "Uploading... {percent}%"
  ├── [Finalising]
  │     ├── [Spinner]
  │     └── [Copy]  "Finalising..."
  ├── [Complete]
  │     ├── [Copy]  "Upload complete."
  │     └── [Actions]
  │           ├── [Upload Another Button]  secondary
  │           └── [Use This Media Button]  primary
  └── [Error]
        ├── [Copy]  error message or "Upload failed"
        └── [Try Again Button]  secondary
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | Grid container, centered, class `media-upload-status-panel` |
| spinner | `Spinner` | `variant="grid"`, `tone="accent"`, used in checking and finalising states |
| copy | `<p>` | Status text, secondary color |
| duplicate-label | `<strong>` | Duplicate item label, shown only when `duplicateLabel` is set |
| actions | `<div>` | Flex row of action buttons, centered, wrap-aware |
| progress-bar | `<div>` | Track container for upload progress |
| progress-fill | `<div>` | Filled portion of progress bar, accent color |
| buttons | `Button` | Action buttons using primary/secondary variants |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `uploadStep` | `MediaUploadWorkflowStep` | `"checking"` | no | Current workflow posture |
| `duplicateLabel` | `string \| null` | `null` | no | Label for the duplicate media item |
| `uploadProgress` | `number` | `0` | no | Percentage (0-100) shown during upload |
| `uploadError` | `string \| null` | `null` | no | Error message for the error state; falls back to "Upload failed" |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for text and progress bar sizing |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for spacing |

### Types

```ts
type MediaUploadWorkflowStep =
  | "checking"
  | "duplicate"
  | "uploading"
  | "finalising"
  | "complete"
  | "error";
```

### Slots

None.

### Controlled / Uncontrolled

All props are controlled. The host owns workflow state and transitions between steps.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| checking | `uploadStep === "checking"` | Spinner and "Checking for duplicates..." copy |
| duplicate | `uploadStep === "duplicate"` | Warning-toned container with existing-file message, optional label, two action buttons |
| uploading | `uploadStep === "uploading"` | Progress bar with animated fill, "Uploading... {percent}%" copy |
| finalising | `uploadStep === "finalising"` | Spinner and "Finalising..." copy |
| complete | `uploadStep === "complete"` | Success-toned container with "Upload complete." copy and two action buttons |
| error | `uploadStep === "error"` | Danger-toned container with error copy and retry button |

### Tone Modifiers

| Step | Modifier class | Color |
|------|---------------|-------|
| duplicate | `--warning` | `var(--poodle-color-warning-base)` |
| complete | `--success` | `var(--poodle-color-success-base)` |
| error | `--danger` | `var(--poodle-color-danger-base)` |

## 5. Callbacks

| Callback | When It Fires | Signature |
|----------|---------------|-----------|
| `onUploadAnyway` | User clicks "Upload as new" in duplicate state | `() => void` |
| `onSelectDuplicate` | User clicks "Use existing" in duplicate state | `() => void` |
| `onClearUpload` | User clicks "Upload another" (complete) or "Try again" (error) | `() => void` |
| `onSelectUploaded` | User clicks "Use this media" in complete state | `() => void` |

## 6. Accessibility

### Semantics

- Uses real `Button` primitives for each action
- Progress state keeps textual progress ("Uploading... 75%") alongside the bar for screen readers
- Error and success states remain readable without color alone (text content conveys meaning)
- Spinner provides visual indication of processing

### Keyboard

- Tab navigates between action buttons
- Enter/Space activates action buttons
- Standard `Button` primitive keyboard behavior

### Focus

- Action buttons use `Button` primitive focus behavior (focus-visible ring)

## 7. Layout

### Sizing

- Root: grid centered, gap from `space-stack-sm`, padding from `space-panel-y` / `space-panel-x`
- Progress bar: capped width `min(14rem, 100%)`, pill-shaped (`border-radius: 999px`)
- Actions: flex row, centered, gap from `space-inline-sm`, flex-wrap

### Composition

- Uses `Spinner` from primitives for checking and finalising states
- Uses `Button` from primitives for all action buttons
- Designed to be hosted inside richer upload workflow composites

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | root `<div>` | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | root `<div>` | `"compact"`, `"default"`, `"comfortable"` |

### Root `.media-upload-status-panel`

| Property | Value |
|----------|-------|
| display | `grid` |
| justify-items | `center` |
| gap | `var(--poodle-space-stack-sm)` |
| padding | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| text-align | `center` |

### Copy `p` and `strong`

| Property | Value |
|----------|-------|
| margin | `0` |

### Copy `p`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-secondary)` |

### Warning Modifier `.media-upload-status-panel--warning`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-warning-base)` |

### Success Modifier `.media-upload-status-panel--success`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-success-base)` |

### Danger Modifier `.media-upload-status-panel--danger`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-danger-base)` |

### Actions `.media-upload-status-panel__actions`

| Property | Value |
|----------|-------|
| display | `flex` |
| justify-content | `center` |
| gap | `var(--poodle-space-inline-sm)` |
| flex-wrap | `wrap` |

### Progress Bar `.media-upload-status-panel__progress`

| Property | Value |
|----------|-------|
| width | `min(14rem, 100%)` |
| height | `0.375rem` |
| border-radius | `999px` |
| overflow | `hidden` |
| background | `color-mix(in srgb, var(--poodle-color-background-surface) 82%, transparent)` |

### Progress Fill `.media-upload-status-panel__progress-fill`

| Property | Value |
|----------|-------|
| height | `100%` |
| background | `var(--poodle-color-accent-default)` |
| transition | `width 0.1s ease-out` |
| width | `{uploadProgress}%` (dynamic) |

### Size Adjustments

| Size | Font size | Progress height | Progress max-width |
|------|----------|----------------|-------------------|
| `xs` | `0.6875rem` | `0.25rem` | `min(11rem, 100%)` |
| `sm` | `0.75rem` | `0.3125rem` | `min(12.5rem, 100%)` |
| `md` (default) | inherit | `0.375rem` | `min(14rem, 100%)` |
| `lg` | `0.875rem` | `0.4375rem` | `min(15.5rem, 100%)` |
| `xl` | `0.9375rem` | `0.5rem` | `min(17rem, 100%)` |

### Density Adjustments

| Density | Padding |
|---------|---------|
| `compact` | `0.5rem` |
| `default` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `comfortable` | `1rem` |

## 9. Svelte Notes

- Each `uploadStep` value renders a different conditional block (`{#if}` chain)
- Uses `Spinner` with `variant="grid"` and `tone="accent"` for loading states
- Uses `Button` with `variant="secondary"` or `variant="primary"` for actions
- Progress percentage displayed via `uploadProgress.toFixed(0)`
- Error message falls back to `"Upload failed"` when `uploadError` is null/empty
- Uses `resolveSemanticControlSize()` and `getUiPresentation()` for presentation context

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] All props have the same meaning and defaults
- [ ] Event names and payloads match
- [ ] All six workflow step postures render correctly
- [ ] Button labels match ("Upload as new", "Use existing", "Upload another", "Use this media", "Try again")

### Tier 2: Visual Parity

- [ ] Progress bar dimensions and styling match
- [ ] Tone modifier colors match per step
- [ ] Size and density adjustments match

### Tier 3: Implementation Freedom

- [ ] Internal rendering approach may differ
- [ ] Spinner implementation may differ

## 12. Specimen Definitions

### Checking State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Checking | `uploadStep="checking"` | Spinner with "Checking for duplicates..." text |

### Duplicate State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Duplicate found | `uploadStep="duplicate"`, `duplicateLabel="photo-2024.jpg"` | Warning-toned panel with duplicate label, "Upload as new" and "Use existing" buttons |

### Uploading State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Uploading | `uploadStep="uploading"`, `uploadProgress=65` | Progress bar at 65%, "Uploading... 65%" text |

### Complete State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Complete | `uploadStep="complete"` | Success-toned panel with "Upload complete.", "Upload another" and "Use this media" buttons |

### Error State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Error | `uploadStep="error"`, `uploadError="Network timeout"` | Danger-toned panel with error message and "Try again" button |
