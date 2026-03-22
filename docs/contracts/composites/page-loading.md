# PageLoading

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `PageLoading`
- Layer: `composites`
- Summary: a full-page modal loading overlay with animated spinner, optional progress bar, status message, and cancel button
- In scope: indeterminate spinner, determinate progress bar, status message, cancel action, backdrop with blur, modal-level z-index
- Out of scope: inline loading indicators, skeleton screens, progress toast notifications, retry logic

## 2. Anatomy

```text
[Root]  role="status", position: fixed, inset: 0
  ├── [Backdrop]  aria-hidden, semi-transparent blur overlay
  └── [Card]
        ├── [Spinner]  aria-hidden, animated SVG
        ├── [Progress]  (optional, when value is not null)
        │     └── Progress primitive
        ├── [Message]  (optional, when message is provided)
        └── [Cancel Button]  (optional, when canCancel is true)
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | Fixed overlay, `role="status"`, `aria-live="polite"`, centered flex |
| backdrop | `<div>` | Semi-transparent background with `backdrop-filter: blur(2px)` |
| card | `<div>` | Elevated card centered in viewport |
| spinner | `<div>` | Animated spinning SVG arc |
| progress | `Progress` | Determinate progress bar (only shown when `value` is not null) |
| message | `<p>` | Status text, centered |
| cancel-button | `<button>` | Dismissal action |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `isVisible` | `boolean` | `true` | no | Controls whether the overlay is rendered |
| `value` | `number \| null` | `null` | no | Progress value; `null` = indeterminate (spinner only) |
| `max` | `number` | `100` | no | Maximum value for the progress bar |
| `message` | `string \| null` | `null` | no | Status message text |
| `canCancel` | `boolean` | `false` | no | Show cancel button |
| `ariaLabel` | `string \| null` | `null` | no | Accessible label; defaults to `"Loading"` |

### Slots

None.

### Controlled / Uncontrolled

`isVisible` is controlled externally. `value` and `message` are controlled to reflect external loading state.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| hidden | `isVisible=false` | Component not rendered |
| indeterminate | `value=null` | Spinning arc animation, no progress bar |
| determinate | `value` is a number | Spinner shown plus progress bar |
| with-message | `message` provided | Status text below spinner |
| cancellable | `canCancel=true` | Cancel button visible below message |
| cancel-hover | Mouse over cancel button | Subtle background change |
| cancel-focus | Focus-visible on cancel button | Focus ring |

### Component States

| State | Description |
|-------|-------------|
| indeterminate | Spinner only, no progress indication |
| determinate | Spinner plus progress bar showing numeric progress |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `cancel` | User clicks the cancel button | `void` |

## 6. Accessibility

### Semantics

- Root has `role="status"` and `aria-live="polite"` for screen reader announcements
- Root carries `aria-label` (defaults to `"Loading"`)
- Backdrop is `aria-hidden="true"`
- Spinner is `aria-hidden="true"`
- Progress bar (when present) uses the `Progress` primitive with its own `ariaLabel`

### Keyboard

- Cancel button is keyboard-focusable when visible
- Focus is trapped within the overlay when visible (modal behavior via fixed positioning and z-index)

### Focus

- Cancel button: `border-width-focus` solid `accent-focusRing`, offset `0.125rem`

## 7. Layout

### Sizing

- Root: `position: fixed`, `inset: 0`, `z-index: overlay-z-modal`, flex centered
- Backdrop: absolute inset 0, `background-base` at 62% opacity, `backdrop-filter: blur(2px)`
- Card: flex column centered, gap `1rem`, min-width `14rem`, max-width `20rem`, padding `2rem 2.5rem`
- Card border: `1px solid border-default` at 42% opacity, `radius-surface`
- Card background: `background-elevated`, shadow: `elevation-overlay`
- Spinner: `2.5rem x 2.5rem`
- Progress: full width of card
- Cancel button: padding `0.375rem 0.875rem`

### Composition

Uses the `Progress` primitive for determinate progress display.

## 8. Token Usage

| Property | Token |
|----------|-------|
| Root z-index | `overlay-z-modal` |
| Backdrop bg | `color-background-base` at 62% mix |
| Card bg | `color-background-elevated` |
| Card border | `color-border-default` at 42% mix |
| Card radius | `radius-surface` |
| Card shadow | `elevation-overlay` |
| Spinner color | `color-accent-base` |
| Message font-size | `typography-label-size` |
| Message color | `color-text-secondary` |
| Cancel border | `color-border-default` |
| Cancel radius | `radius-control` |
| Cancel text | `color-text-secondary` |
| Cancel font-size | `typography-label-size` |
| Cancel hover bg | `color-background-surface` at 72% mix |
| Cancel focus ring | `color-accent-focusRing`, `border-width-focus` |
| Motion duration | `motion-duration-interaction` |
| Motion easing | `motion-easing-standard` |

## 9. Svelte Notes

- Conditionally renders entire component based on `isVisible`
- Spinner uses CSS `@keyframes` animation rotating 360deg over 1s linear infinite
- Composes the `Progress` primitive for determinate mode
- `isIndeterminate` reactive: `value === null`

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

| Feature | Svelte | GPUI | Jetstream |
|---------|--------|------|-----------|
| Indeterminate spinner | Yes | -- | -- |
| Determinate progress | Yes | -- | -- |
| Status message | Yes | -- | -- |
| Cancel button | Yes | -- | -- |
| Backdrop blur | Yes | -- | -- |
| Focus ring on cancel | Yes | -- | -- |

## 12. Known Deltas

None yet (single implementation).

## 13. Specimen Definitions

### Indeterminate (Spinner Only)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Indeterminate | `isVisible` toggled by button, `message="Loading data..."` | Full-page overlay with spinning arc and message |

### Determinate (With Progress Bar)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Determinate | `isVisible` toggled, `value` animates 0-100, `message="Uploading files... N%"` | Overlay with spinner, progress bar, and percentage message |

### With Cancel Button

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Cancellable | `isVisible` toggled, `message="Processing request..."`, `canCancel=true` | Overlay with spinner, message, and cancel button; cancel dismisses |

## 14. Approval And Adoption Notes

Use `PageLoading` for blocking operations that prevent user interaction with the entire page (file uploads, data migrations, long-running server operations). For non-blocking loading states, use inline `Progress` or `Skeleton` components instead. The overlay uses modal-level z-index and should be used sparingly to avoid disrupting user flow.
