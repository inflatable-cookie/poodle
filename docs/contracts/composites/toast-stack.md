# ToastStack

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `ToastStack`
- Layer: `composites`
- Summary: a transient notification stack for low-interruption confirmations,
  warnings, and recoverable failures
- In scope: toast ordering, title/message copy, optional action affordance,
  dismissal, tone-based styling, left accent bar, polite live-region posture
- Out of scope: long-lived inline status, blocking errors, background queue
  persistence, system notification integration, auto-dismiss timers

## 2. Anatomy

```text
[Stack]
  └── [Toast...]
        ├── [DismissButton]
        ├── [Copy]
        │     ├── [Title]
        │     └── [Message]   (optional)
        └── [Actions]         (optional)
              └── [ActionButton]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Stack | yes | `<section>` container with `role="list"` | stack gap |
| Toast | yes | `<article>` with `role="listitem"` | tone color, border, background, elevation, radius |
| AccentBar | yes | `::before` pseudo-element left accent stripe | tone color (82% mix with white) |
| DismissButton | yes | absolute-positioned close button (x icon) | text-secondary, hover: text-primary |
| Copy | yes | title and optional message | layout only |
| Title | yes | `<strong>` toast heading | text-primary (inherited) |
| Message | no | `<p>` toast detail text | text-secondary, font-size 0.8125rem |
| Actions | no | container for action button(s) | layout only |
| ActionButton | no | `Button` primitive (variant="secondary", size="sm") | (uses Button primitive tokens) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `ToastItem[]` | `[]` | no | toast items to display |
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
| empty | `items` is empty | nothing rendered |
| populated | `items` has entries | stack of toast articles |
| info | `tone="info"` (or undefined) | blue accent bar and border tint (fallback `#3b82f6`) |
| success | `tone="success"` | success-colored accent bar and border tint |
| warning | `tone="warning"` | warning-colored accent bar and border tint |
| danger | `tone="danger"` | danger-colored accent bar and border tint; `aria-live="assertive"` |

### Component States

No internal state. Toast list is externally managed.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `dismiss` | dismiss button clicked | `{ id: string }` | host removes toast from items |
| `action` | action button clicked | `{ id: string }` | host handles the action (e.g. navigate, retry) |

## 6. Visual Rules

### Tone Colors

Each toast uses a CSS custom property `--pug-toast-tone` set per tone:

| Tone | Token | Fallback |
|------|-------|----------|
| `info` | `--pug-color-status-info` | `#3b82f6` (blue) |
| `success` | `--pug-color-status-success` | -- |
| `warning` | `--pug-color-status-warning` | -- |
| `danger` | `--pug-color-status-danger` | -- |

The tone color is used for:
- **Left accent bar**: 3px wide, `::before` pseudo-element, `inset: 0 auto 0 0`,
  color at 82% mix with 6% white
- **Border**: 1px solid, tone at 34% mixed with `--pug-color-border-default`
- **Background gradient**: left edge has 12% tone blended into elevated background,
  fading to 98% elevated background at 18% gradient stop

### Elevation

- toasts use `--pug-elevation-overlay` box shadow
- background uses `--pug-color-background-elevated` at 96% alpha mix

## 7. Accessibility

### Semantics

- Stack: `<section>` with `aria-label`, `aria-live="polite"`,
  `aria-atomic="false"`, `role="list"`
- Toast: `<article>` with `role="listitem"`, `aria-atomic="true"`
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

## 8. Layout

### Sizing

- stack gap: `--pug-space-stack-sm`
- toast padding: `--pug-space-panel-x` (with extra 1.5rem on right for dismiss button)
- toast internal gap: `--pug-space-stack-sm`
- toast border-radius: `calc(--pug-radius-surface - 0.125rem)`
- dismiss button: 1.25rem x 1.25rem, positioned top-right (0.375rem inset)
- accent bar width: 0.1875rem (3px)

### Composition

- composes: `Button` primitive (for action buttons), `Icon` primitive (for dismiss x)
- parent expectations: positioned container (fixed/absolute) for toast overlay
- child expectations: none (self-contained)
- resizing rules: toasts fill container width

## 9. Token Usage And Precise CSS

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-tone` | toast `<article>` | `"info"`, `"success"`, `"warning"`, `"danger"` |

### Stack

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--pug-space-stack-sm)` |

### Toast

| Property | Value |
|----------|-------|
| `--pug-toast-tone` | (set per tone, see below) |
| display | `grid` |
| gap | `var(--pug-space-stack-sm)` |
| padding | `var(--pug-space-panel-x)` |
| padding-right | `calc(var(--pug-space-panel-x) + 1.5rem)` |
| border | `0.0625rem solid color-mix(in srgb, var(--pug-toast-tone) 34%, var(--pug-color-border-default))` |
| border-radius | `calc(var(--pug-radius-surface) - 0.125rem)` |
| background | `linear-gradient(90deg, color-mix(in srgb, var(--pug-toast-tone) 12%, transparent), color-mix(in srgb, var(--pug-color-background-elevated) 98%, transparent) 18%), color-mix(in srgb, var(--pug-color-background-elevated) 96%, transparent)` |
| box-shadow | `var(--pug-elevation-overlay)` |
| position | `relative` |
| overflow | `hidden` |

### Toast Accent Bar (`::before`)

| Property | Value |
|----------|-------|
| content | `""` |
| position | `absolute` |
| inset | `0 auto 0 0` |
| width | `0.1875rem` |
| background | `color-mix(in srgb, var(--pug-toast-tone) 82%, white 6%)` |

### Tone Custom Property Values

| `data-tone` | `--pug-toast-tone` |
|-------------|-------------------|
| `info` (default) | `var(--pug-color-status-info, #3b82f6)` |
| `success` | `var(--pug-color-status-success)` |
| `warning` | `var(--pug-color-status-warning)` |
| `danger` | `var(--pug-color-status-danger)` |

### Dismiss Button

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
| border-radius | `var(--pug-radius-sm, 0.25rem)` |
| background | `transparent` |
| color | `var(--pug-color-text-secondary)` |

#### Dismiss Button Hover

| Property | Value |
|----------|-------|
| color | `var(--pug-color-text-primary)` |
| background | `color-mix(in srgb, var(--pug-color-background-surface) 60%, transparent)` |

### Copy Container

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `0.25rem` |
| `strong`, `p` margin | `0` |

### Message Text (`p`)

| Property | Value |
|----------|-------|
| color | `var(--pug-color-text-secondary)` |
| font-size | `0.8125rem` |
| line-height | `1.5` |

### Actions Container

| Property | Value |
|----------|-------|
| display | `flex` |
| justify-content | `flex-start` |

### Light Theme Overrides

None.

## 10. Svelte Notes

- uses `createEventDispatcher` for `dismiss` and `action` events
- uses `Button` primitive (variant="secondary", size="sm") for action buttons
- uses `Icon` primitive (name="x", size="sm") for dismiss button
- toast tone set via `data-tone` attribute and CSS custom property
- items keyed by `item.id` in `{#each}` block
- `ToastItem` type imported from shared `types.ts`

## 11. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::toast_stack`
- tone-based accent bar may use platform-specific drawing
- assertive announcement for danger tone must be preserved

## 12. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] ToastItem type matches
- [ ] tone mapping matches (info/success/warning/danger)
- [ ] aria-live escalation for danger tone matches

### Tier 2: Visual Parity

- [ ] left accent bar rendering matches
- [ ] tone color treatment matches (border, gradient, accent)
- [ ] elevation and background treatment match
- [ ] dismiss button placement matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal
- [ ] animation/transition approach may differ

## 13. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 14. Specimen Definitions

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

## 15. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: application-level notification systems, workspace
  feedback, recoverable error display
- future follow-up: use `ToastStack` for transient confirmations and
  recoverable warnings while keeping long-lived or blocking conditions on
  persistent inline surfaces; consider auto-dismiss timer support
