# PageLoading

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `PageLoading`
- Layer: `composites`
- Summary: a loading surface with shared spinner, optional progress bar, status message, and cancel button that can render either as a full-page modal overlay or an inline centered state
- In scope: indeterminate spinner, determinate progress bar, status message, cancel action, backdrop with blur, modal-level z-index, inline centered loading state
- Out of scope: skeleton screens, progress toast notifications, retry logic

## 2. Anatomy

```text
[Root .page-loading]  <div role="status"> aria-label, aria-live="polite"
  ├── [Backdrop .page-loading__backdrop]  <div> (overlay only, aria-hidden)
  └── [Card .page-loading__card]  <div>
        ├── [Spinner]  Spinner primitive (variant="ring", sizeRole="prominent", tone="accent")
        ├── [Progress .page-loading__progress]  <div> (optional, when value !== null)
        │     └── [ProgressBar]  Progress primitive
        ├── [Message .page-loading__message]  <p> (optional, when message is provided)
        └── [CancelButton .page-loading__cancel]  <button> (optional, when canCancel)
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Root | `<div>` | yes | `role="status"`, `aria-live="polite"`, centered flex; conditionally rendered based on `visible` |
| Backdrop | `<div>` | no | Overlay-only; semi-transparent background with `backdrop-filter: blur(2px)`, `aria-hidden="true"` |
| Card | `<div>` | yes | Elevated card in overlay mode; simplified transparent container in inline mode |
| Spinner | `Spinner` primitive | yes | Shared spinner with `variant="ring"`, `sizeRole="prominent"`, `tone="accent"` |
| Progress | `<div>` wrapper | no | Shown when `value` is not null; contains `Progress` primitive |
| Message | `<p>` | no | Status text, centered |
| CancelButton | `<button>` | no | Shown when `canCancel` is true |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `visible` | `boolean` | `true` | no | Controls whether the component is rendered |
| `presentation` | `"overlay" \| "inline"` | `"overlay"` | no | Whether the component renders as a modal overlay or inline centered loading state |
| `value` | `number \| null` | `null` | no | Progress value; `null` = indeterminate (spinner only) |
| `max` | `number` | `100` | no | Maximum value for the progress bar |
| `message` | `string \| null` | `null` | no | Status message text |
| `canCancel` | `boolean` | `false` | no | Show cancel button |
| `ariaLabel` | `string \| null` | `null` | no | Accessible label; defaults to `"Loading"` |

### Slots

None.

### Controlled / Uncontrolled

- `visible` is controlled externally
- `value` and `message` are controlled to reflect external loading state

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| hidden | `visible=false` | Component not rendered |
| inline | `presentation="inline"` | Inline centered loading shell, no backdrop |
| overlay | `presentation="overlay"` | Full-screen modal treatment with backdrop |
| indeterminate | `value=null` | Spinning arc animation, no progress bar |
| determinate | `value` is a number | Spinner shown plus progress bar |
| with-message | `message` provided | Status text below spinner/progress |
| cancellable | `canCancel=true` | Cancel button visible below message |
| cancel-hover | Mouse over cancel button | Subtle background change |
| cancel-focus | Focus-visible on cancel button | Focus ring |

### Component States

| State | Description |
|-------|-------------|
| `isIndeterminate` (derived) | `value === null` |
| `isOverlay` (derived) | `presentation === "overlay"` |

## 5. Callbacks

| Callback | When It Fires | Signature | Notes |
|----------|---------------|-----------|-------|
| `onCancel` | User clicks the cancel button | `() => void` | Only available when `canCancel` is true |

## 6. Accessibility

### Semantics

- Root: `role="status"`, `aria-live="polite"`, `aria-label` (defaults to `"Loading"`)
- Backdrop: `aria-hidden="true"` in overlay mode
- Spinner: `aria-hidden="true"` (via Spinner primitive)
- Progress bar (when present): uses the `Progress` primitive with its own `ariaLabel` (defaults to `message ?? "Loading progress"`)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | Cancel button is keyboard-focusable when visible |
| `Enter` / `Space` | Activates cancel button |

### Focus

- Cancel button: `border-width-focus` solid `accent-focusRing`, offset `0.125rem`
- Overlay mode uses modal visual treatment; inline mode participates in normal page flow

## 7. Layout

### Sizing

- Root: flex centered in both modes
- Overlay: `position: fixed`, `inset: 0`, modal z-index
- Inline: `position: relative`, `min-height: 12rem`, `padding: 3rem 1rem`
- Backdrop: absolute inset 0, `background-base` at 62% opacity, `backdrop-filter: blur(2px)`
- Card (overlay): flex column centered, `gap: 1rem`, `min-width: 14rem`, `max-width: 20rem`
- Card (inline): `min-width: auto`, `max-width: 24rem`, no border/bg/shadow
- Progress: full width of card
- Cancel button: inline padding `0.375rem 0.875rem`

### Composition

- Composes: `Spinner` primitive, `Progress` primitive
- Parent expectations: page-level loading states, form submission overlays, file upload progress
- Resizing rules: card is centered in viewport (overlay) or parent (inline)

## 8. Token Usage -- Exact Values

#### `.page-loading` (Root)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |

#### `.page-loading[data-presentation="overlay"]`

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `inset` | `0` |
| `z-index` | `var(--poodle-overlay-z-modal, 1000)` |

#### `.page-loading[data-presentation="inline"]`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `min-height` | `12rem` |
| `padding` | `3rem 1rem` |

#### `.page-loading__backdrop`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `0` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-base, #000) 62%, transparent)` |
| `backdrop-filter` | `blur(2px)` |

#### `.page-loading__card`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `flex` |
| `flex-direction` | `column` |
| `align-items` | `center` |
| `gap` | `1rem` |
| `min-width` | `14rem` |
| `max-width` | `20rem` |
| `padding` | `2rem 2.5rem` |
| `border` | `1px solid color-mix(in srgb, var(--poodle-color-border-default) 42%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-elevated)` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |

#### `.page-loading[data-presentation="inline"] .page-loading__card`

| Property | Value |
|----------|-------|
| `min-width` | `auto` |
| `max-width` | `24rem` |
| `padding` | `0` |
| `border` | `none` |
| `background` | `transparent` |
| `box-shadow` | `none` |

#### `.page-loading__progress`

| Property | Value |
|----------|-------|
| `width` | `100%` |

#### `.page-loading__message`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `font-size` | `var(--poodle-typography-label-size, 0.8125rem)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `text-align` | `center` |
| `line-height` | `1.4` |

#### `.page-loading__cancel`

| Property | Value |
|----------|-------|
| `padding` | `0.375rem 0.875rem` |
| `border` | `1px solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font` | `inherit` |
| `font-size` | `var(--poodle-typography-label-size, 0.8125rem)` |
| `cursor` | `pointer` |
| `transition` | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.page-loading__cancel:hover`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 72%, transparent)` |

#### `.page-loading__cancel:focus-visible`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-presentation` | `.page-loading` root | Switches between overlay and inline layout modes |

## 9. Svelte Notes

- Conditionally renders entire component based on `visible`
- Composes the shared `Spinner` primitive with `variant="ring"`, `sizeRole="prominent"`, `tone="accent"`
- Composes the `Progress` primitive for determinate mode
- `isIndeterminate` reactive: `value === null`
- `presentation` toggles between overlay/backdrop treatment and inline flow layout
- Uses the `onCancel` callback for the cancel action

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::page_loading`
- GPUI backdrop blur differs from Svelte (GPUI backdrop effects do not map directly to CSS `backdrop-filter`)
- GPUI cancel focus treatment uses component-level approximations rather than CSS outline + offset

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] `onCancel` fires with the same trigger
- [ ] indeterminate vs determinate logic matches
- [ ] overlay vs inline presentation matches
- [ ] visible toggle fully removes/adds the component

### Tier 2: Visual Parity

- [ ] backdrop opacity and blur match
- [ ] card chrome (border, radius, shadow) matches
- [ ] inline card has no chrome
- [ ] cancel button styling and hover match
- [ ] focus ring on cancel matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal
- [ ] backdrop blur implementation may differ by platform

## 12. Specimen Definitions

### Indeterminate (Spinner Only)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Indeterminate | `visible` toggled by button, `message="Loading data..."` | Full-page overlay with spinning arc and message |

### Determinate (With Progress Bar)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Determinate | `visible` toggled, `value` animates 0-100, `message="Uploading files... N%"` | Overlay with spinner, progress bar, and percentage message |

### With Cancel Button

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Cancellable | `visible` toggled, `message="Processing request..."`, `canCancel=true` | Overlay with spinner, message, and cancel button; cancel dismisses |
