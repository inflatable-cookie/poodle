# ToastStack

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `ToastStack`
- Layer: `composites`
- Summary: a transient notification stack for low-interruption confirmations,
  warnings, and recoverable failures — renders a list of tone-styled toast
  articles with dismiss buttons, optional action affordances, and polite
  live-region posture
- In scope: toast ordering, title/message copy, optional action affordance,
  dismissal, tone-based styling (info/success/warning/danger), left accent
  bar, polite live-region posture, size-aware typography, density-aware
  spacing
- Out of scope: long-lived inline status, blocking errors, background queue
  persistence, system notification integration, auto-dismiss timers

## 2. Anatomy

```text
[Stack .toast-stack]  <ul>
  └── [Toast .toast]  <li> (repeated)
        ├── [AccentBar .toast::before]  pseudo-element
        ├── [DismissButton .toast__dismiss]  <button>
        │     └── [Icon]  Icon primitive (name="x")
        ├── [Copy .toast__copy]  <div>
        │     ├── [Title]  <strong>
        │     └── [Message]  <p> (optional)
        └── [Actions .toast__actions]  <div> (optional)
              └── [ActionButton]  Button primitive (variant="secondary")
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Stack | yes | `<ul>` container (native list semantics; `role="list"` is not permitted on `<section>`) | stack gap |
| Toast | yes | `<li>` (native list item; `role="listitem"` is not permitted on `<article>`) | tone color, border, background, elevation, radius |
| AccentBar | yes | `::before` pseudo-element left accent stripe | tone color (82% mix with white) |
| DismissButton | yes | absolute-positioned close button (x icon) | text-secondary, hover: text-primary |
| Copy | yes | title and optional message | layout only |
| Title | yes | `<strong>` toast heading | text-primary (inherited) |
| Message | no | `<p>` toast detail text | text-secondary, font-size 0.8125rem |
| Actions | no | container for action button(s) | layout only |
| ActionButton | no | `Button` primitive (variant="secondary") | (uses Button primitive tokens) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `ToastItem[]` | `[]` | no | toast items to display |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl" \| null` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `ariaLabel` | `string` | `"Notifications"` | no | accessible name for the stack section |

### Types

```typescript
type ToastTone = "info" | "success" | "warning" | "danger";

type ToastItem = {
  id: string;
  title: string;
  message?: string | null;
  tone?: ToastTone;
  actionLabel?: string | null;
};
```

### Controlled And Uncontrolled

- items list is externally driven; host owns add/remove logic
- no internal state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | `items` is empty | nothing rendered (empty section) |
| populated | `items` has entries | stack of toast articles |
| info | `tone="info"` (or undefined) | blue accent bar and border tint (fallback `#3b82f6`) |
| success | `tone="success"` | success-colored accent bar and border tint |
| warning | `tone="warning"` | warning-colored accent bar and border tint |
| danger | `tone="danger"` | danger-colored accent bar and border tint; `aria-live="assertive"` |

### Component States

No internal state. Toast list is externally managed.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

| Callback | When It Fires | Signature | Notes |
|----------|---------------|-----------|-------|
| `onDismiss` | dismiss button clicked | `(id: string) => void` | host removes toast from items |
| `onAction` | action button clicked | `(id: string) => void` | host handles the action (e.g. navigate, retry) |

## 6. Accessibility

### Semantics

- Stack: `<ul>` with `aria-label`, `aria-live="polite"`, `aria-atomic="false"`.
  Native list semantics — an explicit `role="list"` on a `<section>` is an
  invalid role/element combination
- Toast: `<li>` with `aria-atomic="true"` (native list-item semantics)
- Toast live region: `aria-live="polite"` by default; `aria-live="assertive"`
  for `tone="danger"`
- Dismiss button: `aria-label="Dismiss {item.title}"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | navigates between dismiss buttons and action buttons |
| `Enter` / `Space` | activates focused button (dismiss or action) |

### Focus And Announcement

- focus entry: dismiss button of first toast, or action button if present
- transient notifications announce politely and avoid stealing focus
- danger toasts escalate to assertive announcement
- GPUI-native accessibility mapping notes: GPUI must preserve transient
  notification meaning and dismiss/action reachability even where there is
  no web-style live region

## 7. Layout

### Sizing

- stack gap: `var(--poodle-space-stack-sm)`
- toast padding: `var(--poodle-space-panel-x)` (with extra 1.5rem on right for dismiss button)
- toast internal gap: `var(--poodle-space-stack-sm)`
- toast border-radius: `calc(var(--poodle-radius-surface) - 0.125rem)`
- dismiss button: 1.25rem x 1.25rem, positioned top-right (0.375rem inset)
- accent bar width: 0.1875rem (3px)

### Composition

- composes: `Button` primitive (for action buttons), `Icon` primitive (for dismiss x)
- parent expectations: positioned container (fixed/absolute) for toast overlay
- child expectations: none (self-contained)
- resizing rules: toasts fill container width

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-tone` | toast `<article>` | `"info"`, `"success"`, `"warning"`, `"danger"` |
| `data-size` | stack `<section>` | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | stack `<section>` | `"compact"`, `"default"`, `"comfortable"` |

### Stack (`.toast-stack`)

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-sm)` |

### Toast (`.toast`)

| Property | Value |
|----------|-------|
| `--poodle-toast-tone` | `var(--poodle-color-status-info, #3b82f6)` (default) |
| display | `grid` |
| gap | `var(--poodle-space-stack-sm)` |
| padding | `var(--poodle-space-panel-x)` |
| padding-right | `calc(var(--poodle-space-panel-x) + 1.5rem)` |
| border | `0.0625rem solid color-mix(in srgb, var(--poodle-toast-tone) 34%, var(--poodle-color-border-default))` |
| border-radius | `calc(var(--poodle-radius-surface) - 0.125rem)` |
| background | `linear-gradient(90deg, color-mix(in srgb, var(--poodle-toast-tone) 12%, transparent), color-mix(in srgb, var(--poodle-color-background-elevated) 98%, transparent) 18%), color-mix(in srgb, var(--poodle-color-background-elevated) 96%, transparent)` |
| box-shadow | `var(--poodle-elevation-overlay)` |
| position | `relative` |
| overflow | `hidden` |

### Toast Accent Bar (`.toast::before`)

| Property | Value |
|----------|-------|
| content | `""` |
| position | `absolute` |
| inset | `0 auto 0 0` |
| width | `0.1875rem` |
| background | `color-mix(in srgb, var(--poodle-toast-tone) 82%, white 6%)` |

### Tone Custom Property Values

| `data-tone` | `--poodle-toast-tone` |
|-------------|-------------------|
| `info` (default) | `var(--poodle-color-status-info, #3b82f6)` |
| `success` | `var(--poodle-color-status-success)` |
| `warning` | `var(--poodle-color-status-warning)` |
| `danger` | `var(--poodle-color-status-danger)` |

### Dismiss Button (`.toast__dismiss`)

| Property | Value |
|----------|-------|
| position | `absolute` |
| top | `0.375rem` |
| right | `0.375rem` |
| display | `inline-flex` |
| align-items | `center` |
| justify-content | `center` |
| width | `1.25rem` |
| height | `1.25rem` |
| padding | `0` |
| border | `none` |
| border-radius | `var(--poodle-radius-sm, 0.25rem)` |
| background | `transparent` |
| color | `var(--poodle-color-text-secondary)` |
| cursor | `pointer` |

#### Dismiss Button Hover

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-primary)` |
| background | `color-mix(in srgb, var(--poodle-color-background-surface) 60%, transparent)` |

### Copy Container (`.toast__copy`)

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `0.25rem` |
| `strong`, `p` margin | `0` |

### Message Text (`.toast__copy p`)

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.8125rem` |
| line-height | `1.5` |

### Actions Container (`.toast__actions`)

| Property | Value |
|----------|-------|
| display | `flex` |
| justify-content | `flex-start` |

### Size Adjustments

#### `data-size="xs"`

| Part | Property | Value |
|------|----------|-------|
| `.toast__dismiss` | width, height | `1rem` |
| `.toast__dismiss` | top, right | `0.25rem` |
| `.toast__copy strong` | font-size | `0.71875rem` |
| `.toast__copy p` | font-size | `0.6875rem` |

#### `data-size="sm"`

| Part | Property | Value |
|------|----------|-------|
| `.toast__dismiss` | width, height | `1.125rem` |
| `.toast__copy p` | font-size | `0.75rem` |

#### `data-size="md"` (default)

No overrides — uses base values.

#### `data-size="lg"`

| Part | Property | Value |
|------|----------|-------|
| `.toast__dismiss` | width, height | `1.5rem` |
| `.toast__dismiss` | top, right | `0.5rem` |
| `.toast__copy strong` | font-size | `0.9375rem` |
| `.toast__copy p` | font-size | `0.875rem` |

#### `data-size="xl"`

| Part | Property | Value |
|------|----------|-------|
| `.toast__dismiss` | width, height | `1.75rem` |
| `.toast__dismiss` | top, right | `0.5rem` |
| `.toast__copy strong` | font-size | `1rem` |
| `.toast__copy p` | font-size | `0.9375rem` |

### Density Adjustments

#### `data-density="compact"`

| Part | Property | Value |
|------|----------|-------|
| `.toast-stack` | gap | `var(--poodle-space-stack-sm)` |
| `.toast` | padding | `calc(var(--poodle-space-panel-x) * 0.75)` |
| `.toast` | padding-right | `calc(var(--poodle-space-panel-x) * 0.75 + 1.25rem)` |

#### `data-density="comfortable"`

| Part | Property | Value |
|------|----------|-------|
| `.toast-stack` | gap | `var(--poodle-space-stack-lg)` |
| `.toast` | padding | `calc(var(--poodle-space-panel-x) * 1.25)` |
| `.toast` | padding-right | `calc(var(--poodle-space-panel-x) * 1.25 + 1.75rem)` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- `data-size` attribute on stack reflects the resolved size
- `data-density` attribute on stack reflects resolved density (`compact`, `default`, `comfortable`)
- uses `createEventDispatcher` for `dismiss` and `action` events
- uses `Button` primitive (variant="secondary") for action buttons
- uses `Icon` primitive (name="x") for dismiss button icon
- toast tone set via `data-tone` attribute and `--poodle-toast-tone` CSS custom property
- items keyed by `item.id` in `{#each}` block
- `ToastItem` and `ToastTone` types imported from shared `types.ts`
- resolves size via `resolveSemanticControlSize` from inherited `getUiPresentation`
- resolves density via `getUiPresentation` store

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::toast_stack`
- spec struct: `ToastStackSpec` with items, size, density
- tone-based accent bar may use platform-specific drawing
- assertive announcement for danger tone must be preserved

## 10a. Jetstream Notes

- `ToastStack::from_spec(spec, theme).on_dismiss(...).on_action(...)`, each
  carrying the toast's id.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] ToastItem type matches
- [ ] tone mapping matches (info/success/warning/danger)
- [ ] aria-live escalation for danger tone matches
- [ ] dismiss button aria-label includes toast title

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] left accent bar rendering matches
- [ ] tone color treatment matches (border, gradient, accent)
- [ ] elevation and background treatment match
- [ ] dismiss button placement matches
- [ ] density spacing matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal
- [ ] animation/transition approach may differ

## 12. Specimen Definitions

### Interactive Stack

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Success toast | `title="Changes saved"`, `message="Your settings have been updated."`, `tone="success"` | Toast with success styling, dismiss affordance |
| Info toast with action | `title="New version available"`, `message="Update to v2.1 for the latest features."`, `tone="info"`, `actionLabel="Update"` | Toast with info styling, action button, dismiss affordance |
| Warning toast | `title="Rate limit warning"`, `message="You are approaching your API limit."`, `tone="warning"` | Toast with warning styling, dismiss affordance |

The specimen includes an "Add toast" button that appends new toasts cycling
through info, success, warning, and danger tones. Dismiss and action handlers
remove toasts from the stack. Toasts are rendered in a stacked layout within
a positioned container.
