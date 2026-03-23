# Meter

> **Surface elevation**: Meter is a surface consumer (80% subtle contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Meter`
- Layer: `foundation`
- Summary: a bounded measurement display for current level within a known range
- In scope: value, range, low/high/optimum hints, native meter semantics
- Out of scope: progress-task completion semantics and animated loading

## 2. Anatomy

```text
[Root .meter]
  ├── [Native <meter> .meter__native] (hidden, provides semantics)
  ├── [Track .meter__track]
  │     └── [Fill .meter__fill]
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `div` | yes | grid container for meter layout |
| Native | `<meter>` | yes | hidden native element providing browser meter semantics |
| Track | `span` | yes | visible track shell with rounded background |
| Fill | `span` | yes | visible fill bar representing current value |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `number` | `0` | no | current measurement value |
| `min` | `number` | `0` | no | minimum range bound |
| `max` | `number` | `100` | no | maximum range bound |
| `low` | `number \| null` | `null` | no | low threshold hint (passed to native meter) |
| `high` | `number \| null` | `null` | no | high threshold hint (passed to native meter) |
| `optimum` | `number \| null` | `null` | no | optimum value hint (passed to native meter) |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the meter |

### Controlled And Uncontrolled

- Controlled-only display primitive. All values are parent-owned.

### Computed Values

| Name | Formula |
|------|---------|
| `safeMax` | `max <= min ? min + 1 : max` |
| `safeValue` | `clamp(value, min, safeMax)` |
| `percentage` | `((safeValue - min) / (safeMax - min)) * 100` |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | `value = min` | fill width is 0% |
| partial | `min < value < max` | fill width reflects percentage |
| full | `value >= max` | fill width is 100% |

### Component States

No internal state. The `low`, `high`, and `optimum` props are passed to the
native `<meter>` element for browser semantics but do not change the visual
fill color in the current implementation.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | display primitive only |

## 6. Accessibility

### Semantics

- Role: native `<meter>` element provides meter semantics (hidden visually)
- The root `div` carries `aria-label` when provided
- The native `<meter>` receives `value`, `min`, `max`, and optional `low`,
  `high`, `optimum` attributes
- Labeling rules: `ariaLabel` should be provided when the meter's purpose is
  not clear from surrounding context

### Keyboard

| Key | Behavior |
|-----|----------|
| none | not interactive |

### Focus And Announcement

- Focus entry: not focusable by default
- Live-region behavior: parent-owned
- GPUI-native accessibility mapping notes: GPUI must expose bounded-value
  meter semantics distinct from progress semantics

## 7. Layout

### Sizing

- Width is parent-owned (`width: 100%`)
- Track minimum height is `0.5rem`
- Root uses grid layout with `gap: 0`

### Composition

- Parent expectations: health bars, storage usage, bounded scoring displays
- Child expectations: none (fill is internal)
- Resizing rules: fill width scales as a percentage of the track

## 8. Token Usage

### Root `.meter`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0` |
| `width` | `100%` |

### Native Meter `.meter__native`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `opacity` | `0` |
| `pointer-events` | `none` |

### Track `.meter__track`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `block` |
| `overflow` | `hidden` |
| `min-height` | `0.5rem` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary))` |

### Fill `.meter__fill`

| Property | Value |
|----------|-------|
| `display` | `block` |
| `height` | `100%` |
| `border-radius` | `inherit` |
| `background` | `linear-gradient(90deg, color-mix(in srgb, var(--poodle-color-status-success) 82%, white), var(--poodle-color-status-success))` |

### Fill — Inline Style

| Property | Value |
|----------|-------|
| `width` | `{percentage}%` where percentage = ((safeValue - min) / (safeMax - min)) * 100 |

### Token Reference

| Token | Role |
|-------|------|
| `--poodle-color-background-surface` | track background (mixed at 88% opacity) |
| `--poodle-color-status-success` | fill gradient endpoint and base |

## 9. Svelte Notes

- Uses a hidden native `<meter>` element for browser semantics while rendering
  a fully styled custom track and fill
- Fill uses percentage-based `width` (not `scaleX` transform as Progress does)
- The `low`, `high`, and `optimum` props are passed through to the native
  `<meter>` but do not affect custom visual styling

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::primitives::meter`
- GPUI must expose bounded-value meter semantics distinct from progress
  semantics through native accessibility APIs

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] bounded-value semantics match (value, min, max)
- [ ] low/high/optimum hints are exposed to accessibility layer
- [ ] progress-vs-meter meaning stays distinct

### Tier 2: Visual Parity

- [ ] track background uses `--poodle-color-background-surface` at 88% mix
- [ ] fill gradient uses `--poodle-color-status-success` at 82% mix with white
- [ ] border-radius 999px pill shape matches
- [ ] min-height of 0.5rem matches

### Tier 3: Implementation Freedom

- [ ] hidden native meter approach is Svelte-specific; GPUI uses native APIs
- [ ] fill rendering method (width vs other) is implementation choice

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native meter styling may vary | platform visuals are not the contract | allowed | keep measurement semantics strict |
| hidden native meter is Svelte-specific | GPUI uses native accessibility APIs directly | allowed | ensure semantic parity |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default (50%)

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default (50%) | `value=50`, `ariaLabel="Storage usage"` | Track with fill at 50% width; pill-shaped track and fill with success-gradient coloring |

### With thresholds

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With thresholds | `value=82`, `low=25`, `high=75`, `optimum=50`, `ariaLabel="CPU usage"` | Track with fill at 82% width; value is above the high threshold; annotated "82% -- above high threshold" |

### Low value (optimal range)

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Low value (optimal range) | `value=30`, `low=25`, `high=75`, `optimum=50`, `ariaLabel="Memory usage"` | Track with fill at 30% width; value is within the normal range; annotated "30% -- within normal range" |

### Custom range (0-500)

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Custom range (0-500) | `value=350`, `min=0`, `max=500`, `ariaLabel="API calls"` | Track with fill at 70% width (350/500); annotated "350 / 500 API calls used" |

## 14. Approval And Adoption Notes

- Contract status: `detailed contract`
- Approvers: pending
- Downstream adopters: health bars, storage usage, bounded scoring
- Future follow-up: consider adding color shifts for low/high/optimum zones
