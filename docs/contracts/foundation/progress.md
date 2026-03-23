# Progress

> **Surface elevation**: Progress is a surface consumer (80% subtle contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Progress`
- Layer: `foundation`
- Summary: a determinate or indeterminate progress indicator for task
  completion status
- In scope: value range, current progress, indeterminate state, accessible
  status semantics
- Out of scope: stepper workflows, upload-specific shell wrappers

## 2. Anatomy

```text
[Root .progress]
  └── [Indicator .progress__indicator]
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `div` | yes | progress host and track background |
| Indicator | `span` | yes | completed or active fill bar |

The root element doubles as the track. There is no separate track element.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `number \| null` | `null` | no | current progress value |
| `max` | `number` | `100` | no | maximum range |
| `isIndeterminate` | `boolean` | `false` | no | active progress with no fixed value |
| `ariaLabel` | `string \| null` | `null` | no | optional accessible name when context needs it |
| `valueText` | `string \| null` | `null` | no | human-readable progress text (e.g. "3 of 10") |

### Controlled And Uncontrolled

- Controlled-only display primitive. All values are parent-owned.

### Computed Values

| Name | Formula |
|------|---------|
| `safeMax` | `max <= 0 ? 100 : max` |
| `safeValue` | `clamp(value, 0, safeMax)` |
| `percentage` | `safeValue / safeMax` |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| determinate | `value` present and `isIndeterminate=false` | indicator scaled to `scaleX(percentage)` |
| indeterminate | `isIndeterminate=true` | indicator animates continuously across track |
| complete | `value >= max` | indicator fills entire track (`scaleX(1)`) |

### Component States

Determinate vs indeterminate is the only state axis. The component has no
internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | display primitive only |

## 6. Accessibility

### Semantics

- Role: `role="progressbar"` on root element
- Required attributes (determinate): `aria-valuemin="0"`, `aria-valuemax={safeMax}`, `aria-valuenow={safeValue}`
- Required attributes (indeterminate): no aria-valuemin, aria-valuemax, or aria-valuenow (omitted entirely)
- Optional attributes: `aria-label={ariaLabel}` when provided, `aria-valuetext={valueText}` when provided
- Data attribute: `data-indeterminate` present on root when `isIndeterminate=true`
- Labeling rules: when progress meaning is unclear from surrounding text, an
  explicit `ariaLabel` is required

### Keyboard

| Key | Behavior |
|-----|----------|
| none | not interactive |

### Focus And Announcement

- Focus entry: not focusable by default
- Live-region behavior: parent-owned unless progress updates must be announced
  explicitly
- GPUI-native accessibility mapping notes: GPUI must expose progress semantics
  and determinate/indeterminate meaning through native accessibility APIs

## 7. Layout

### Sizing

- Width is parent-owned (`width: 100%`)
- Minimum height is `0.5rem`
- No maximum height constraint (height can be overridden by parent)

### Composition

- Parent expectations: status rows, loading shells, forms, task flows
- Child expectations: none (indicator is internal)
- Resizing rules: indicator fill scales with parent width via `scaleX` transform

## 8. Token Usage

### Root `.progress`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `overflow` | `hidden` |
| `width` | `100%` |
| `min-height` | `0.5rem` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--flint-surface) 96%, var(--flint-color-text-primary))` |

### Indicator `.progress__indicator`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `0` |
| `transform-origin` | `left center` |
| `border-radius` | `inherit` |
| `background` | `linear-gradient(90deg, color-mix(in srgb, var(--flint-color-accent-base) 88%, white), var(--flint-color-accent-base))` |
| `transition` | `transform var(--flint-motion-duration-standard) var(--flint-motion-easing-standard)` |

### Indicator — Determinate State

| Property | Value |
|----------|-------|
| `transform` | `scaleX({percentage})` where percentage = safeValue / safeMax |

### Indicator — Indeterminate State

| Property | Value |
|----------|-------|
| `width` | `40%` |
| `transform` | `translateX(-100%)` (initial) |
| `animation` | `progress-indeterminate 1.2s ease-in-out infinite` |

### Keyframes

```
@keyframes progress-indeterminate {
  to { transform: translateX(250%) }
}
```

### Token Reference

| Token | Role |
|-------|------|
| `--flint-color-background-surface` | track background (mixed at 92% opacity) |
| `--flint-color-accent-base` | indicator gradient endpoint and base |
| `--flint-motion-duration-standard` | determinate transition duration |
| `--flint-motion-easing-standard` | determinate transition easing |

## 9. Svelte Notes

- Root is a `<div>` with `role="progressbar"` rather than a native `<progress>`
  element, enabling full visual control
- Determinate indicator uses `scaleX` transform rather than width for
  GPU-accelerated animation
- Indeterminate uses CSS `@keyframes` animation, not JavaScript-driven motion

## 10. GPUI Notes

- Expected crate/module surface: `flint_gpui::primitives::progress`
- GPUI implementation must intentionally expose determinate value and
  indeterminate progress semantics rather than presenting only a visual bar

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] determinate/indeterminate meaning matches
- [ ] progress accessibility semantics match (`role="progressbar"`, aria-value attributes)
- [ ] `data-indeterminate` attribute present when indeterminate

### Tier 2: Visual Parity

- [ ] track background uses `--flint-color-background-surface` at 92% mix
- [ ] indicator gradient uses `--flint-color-accent-base` at 88% mix with white
- [ ] indicator border-radius inherits from track (999px pill)
- [ ] min-height of 0.5rem matches

### Tier 3: Implementation Freedom

- [ ] indeterminate animation internals may differ (timing, easing)
- [ ] transform vs width approach for fill is implementation choice

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| indeterminate animation details may differ | motion internals are runtime-specific | allowed | keep progress meaning strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Determinate

Four progress bars in a vertical stack with 16px gap, each full width:

| Label | Value | Max | Notes |
|-------|-------|-----|-------|
| Empty | 0 | 100 | 0% filled |
| 35% | 35 | 100 | partial fill |
| 72% | 72 | 100 | partial fill |
| Complete | 100 | 100 | full bar |

### Indeterminate

One indeterminate progress bar, full width:

| Label | Value | Notes |
|-------|-------|-------|
| Loading | `undefined` | animated sliding indicator |

### Custom max

One progress bar with a non-100 maximum:

| Label | Value | Max | Notes |
|-------|-------|-----|-------|
| Steps | 3 | 5 | shows 60% fill |

## 14. Approval And Adoption Notes

- Contract status: `detailed contract`
- Approvers: pending
- Downstream adopters: loading states, task indicators
- Future follow-up: pair with richer loading wrappers later
