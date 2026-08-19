# Meter

> **Surface elevation**: Meter is a surface consumer (80% subtle contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-07-24

## 1. Purpose

- Component name: `Meter`
- Layer: `foundation`
- Summary: a bounded measurement display for current level within a known range,
  rendered as a horizontal bar or a circular ring
- In scope: value, range, low/high/optimum hints, native meter semantics,
  track-thickness sizing, `linear` / `ring` shape, threshold-driven fill tone,
  optional value readout
- Out of scope: progress-task completion semantics and animated loading

## 2. Anatomy

```text
[Root .meter]
  ├── [Native <meter> .meter__native] (hidden, provides semantics)
  ├── [Track .meter__track]
  │     └── [Fill .meter__fill]
  └── [Value .meter__value] (conditional: showValue)
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `div` | yes | grid container for meter layout; carries `data-shape`, `data-level`, `data-tone` |
| Native | `<meter>` | yes | hidden native element providing browser meter semantics |
| Track | `span` | yes | visible track shell — rounded bar (`linear`) or circle (`ring`) |
| Fill | `span` | yes | visible fill — width-scaled bar (`linear`) or masked conic arc (`ring`) |
| Value | `span` | no | value readout; centred in the ring, trailing the bar in `linear` |

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
| `shape` | `"linear" \| "ring"` | `"linear"` | no | bar or circular ring geometry; `ring` is intrinsically sized (no parent-owned width) |
| `tone` | `"success" \| "accent" \| "warning" \| "danger" \| "neutral"` | `"success"` | no | base fill tone; overridden to `warning` while the value sits at or above `high` |
| `showValue` | `boolean` | `false` | no | render the value readout part |
| `valueText` | `string \| null` | `null` | no | explicit readout text; when null the computed `"{round(percentage)}%"` is used |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl" \| null` | `null` | no | explicit track-thickness / ring-diameter size override |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role used to resolve inherited size scale |

### Controlled And Uncontrolled

- Controlled-only display primitive. All values are parent-owned.

### Computed Values

| Name | Formula |
|------|---------|
| `safeMax` | `max <= min ? min + 1 : max` |
| `safeValue` | `clamp(value, min, safeMax)` |
| `percentage` | `((safeValue - min) / (safeMax - min)) * 100` |
| `level` | `high !== null && safeValue >= high ? "high" : low !== null && safeValue <= low ? "low" : "normal"` |
| `computedValueText` | `valueText ?? "{round(percentage)}%"` |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | `value = min` | fill width is 0% (`linear`) / arc sweep is 0deg (`ring`) |
| partial | `min < value < max` | fill reflects percentage |
| full | `value >= max` | fill is 100% / a complete ring |
| ring | `shape="ring"` | track is a circle, fill is a masked conic arc starting at 12 o'clock, sweeping clockwise |
| above high | `high !== null && value >= high` | `data-level="high"`; fill switches to the warning tone regardless of `tone` |
| at or below low | `low !== null && value <= low` | `data-level="low"`; fill keeps the base tone (hook only, for host styling) |

### Component States

No internal state. `low` and `high` drive `data-level` and — for `high` only —
the fill tone; `optimum` is passed to the native `<meter>` for browser semantics
and does not change visual styling.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

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

- `linear`: width is parent-owned (`width: 100%`); track thickness is size-driven
- `ring`: intrinsically sized — diameter and ring thickness are size-driven, root
  is `inline-grid` and does not stretch
- Root uses grid layout with `gap: 0`

### Composition

- Parent expectations: health bars, storage usage, bounded scoring displays;
  `ring` for inline budget/quota indicators inside toolbars and composers
  (`AgentChatInput` context indicator)
- Child expectations: none (fill and value readout are internal)
- Resizing rules: `linear` fill width scales as a percentage of the track;
  `ring` arc sweep scales as a percentage of a full turn

## 8. Token Usage

### Root `.meter`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0` |
| `width` | `100%` |
| `--poodle-meter-track-thickness` | `0.5rem` |

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
| `min-height` | `var(--poodle-meter-track-thickness)` |
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
| `width` (`linear`) | `{percentage}%` where percentage = ((safeValue - min) / (safeMax - min)) * 100 |
| `--poodle-meter-percentage` (`ring`, set on Root) | `{percentage}` — drives the conic sweep |

### Ring Shape `.meter[data-shape="ring"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `display` | `inline-grid` |
| Root | `place-items` | `center` |
| Root | `width` / `height` | `var(--poodle-meter-ring-size)` |
| Track | `border-radius` | `50%` |
| Track | `background` | `color-mix(in srgb, var(--poodle-surface) 88%, var(--poodle-color-text-primary))` |
| Track | `mask` | `radial-gradient(farthest-side, transparent calc(100% - var(--poodle-meter-ring-thickness)), #000 calc(100% - var(--poodle-meter-ring-thickness)))` |
| Fill | `position` | `absolute` (inset 0) |
| Fill | `background` | `conic-gradient(from 0deg, var(--poodle-meter-fill-color) calc(var(--poodle-meter-percentage) * 1%), transparent 0)` |
| Fill | `mask` | same ring mask as Track |
| Value | `font-size` | `calc(var(--poodle-meter-ring-size) * 0.34)` |

The Track and Fill both carry the ring mask so the centre stays transparent over
any surface. The arc starts at 12 o'clock and sweeps clockwise.

### Fill Tone

`--poodle-meter-fill-color` resolves from `tone`, then the `high` override:

| `tone` | Colour |
|--------|--------|
| `success` (default) | `--poodle-color-status-success` |
| `accent` | `--poodle-color-accent-base` |
| `warning` | `--poodle-color-status-warning` |
| `danger` | `--poodle-color-status-danger` |
| `neutral` | `--poodle-color-text-secondary` |

`data-level="high"` forces `--poodle-color-status-warning` regardless of `tone`.
In `linear` the fill keeps its gradient treatment (fill colour mixed 82% with
white at the leading edge); in `ring` the arc is flat.

### Size Variants

| Size | Track thickness | Ring diameter | Ring thickness |
|------|-----------------|---------------|----------------|
| `xs` | `0.25rem` | `0.875rem` | `0.125rem` |
| `sm` | `0.375rem` | `1.125rem` | `0.1875rem` |
| `md` | `0.5rem` | `1.375rem` | `0.1875rem` |
| `lg` | `0.625rem` | `1.75rem` | `0.25rem` |
| `xl` | `0.75rem` | `2.25rem` | `0.3125rem` |

### Token Reference

| Token | Role |
|-------|------|
| `--poodle-surface` | track background base (mixed at 96% linear / 88% ring with `--poodle-color-text-primary`) |
| `--poodle-color-status-success` | default fill gradient endpoint and base |
| `--poodle-color-status-warning` | fill colour while `data-level="high"`, and the `warning` tone |
| `--poodle-color-status-danger` | `danger` tone fill |
| `--poodle-color-accent-base` | `accent` tone fill |
| `--poodle-color-text-secondary` | `neutral` tone fill, and value readout colour |

## 9. Svelte Notes

- Uses a hidden native `<meter>` element for browser semantics while rendering
  a fully styled custom track and fill
- Fill uses percentage-based `width` (not `scaleX` transform as Progress does)
- `size` resolves through presentation context; it changes track thickness in
  `linear` and diameter + ring thickness in `ring`
- `optimum` is passed through to the native `<meter>` but does not affect custom
  visual styling; `low` / `high` drive `data-level`, and `high` also drives the
  warning fill override
- Ring geometry is pure CSS (`conic-gradient` + `mask`); no SVG, no JS geometry

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::primitives::meter`
- GPUI must expose bounded-value meter semantics distinct from progress
  semantics through native accessibility APIs

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] bounded-value semantics match (value, min, max)
- [ ] low/high/optimum hints are exposed to accessibility layer
- [ ] progress-vs-meter meaning stays distinct
- [ ] `level` derivation matches (`high` wins over `low`)
- [ ] `high` forces the warning fill tone in every target
- [ ] value readout text matches (`valueText` wins over the computed percentage)

### Tier 2: Visual Parity

- [ ] track background uses `--poodle-surface` at 96% mix with text-primary
- [ ] fill gradient uses `--poodle-color-status-success` at 82% mix with white
- [ ] border-radius 999px pill shape matches
- [ ] track-thickness ladder matches across `xs`–`xl`
- [ ] ring diameter / thickness ladder matches across `xs`–`xl`
- [ ] ring arc starts at 12 o'clock and sweeps clockwise

### Tier 3: Implementation Freedom

- [ ] hidden native meter approach is Svelte-specific; GPUI uses native APIs
- [ ] fill rendering method (width vs other) is implementation choice
- [ ] ring arc rasterisation is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native meter styling may vary | platform visuals are not the contract | allowed | keep measurement semantics strict |
| hidden native meter is Svelte-specific | GPUI uses native accessibility APIs directly | allowed | ensure semantic parity |
| GPUI / Jetstream render the ring as a circular track plus the value readout, without the swept arc | neither runtime exposes conic gradients or arc geometry through the div/JsEl builders; the proportion stays legible through the readout, and the measurement semantics are unchanged | accepted | revisit if either runtime gains an arc/shader primitive |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Sizes

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Sizes | `shape="ring"`, `value=60`, `size=xs..xl` | One ring representative per size; diameter and ring thickness step across the full size ladder |

### Default usage

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default usage | `value=50`, `ariaLabel="Storage usage"` | Track with fill at 50% width; pill-shaped track and fill with success-gradient coloring |

### Threshold states

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Above high | `value=82`, `low=25`, `high=75`, `optimum=50`, `ariaLabel="CPU usage"` | Track with fill at 82% width; value is above the high threshold; annotated "82% -- above high threshold" |
| Within range | `value=30`, `low=25`, `high=75`, `optimum=50`, `ariaLabel="Memory usage"` | Track with fill at 30% width; value is within the normal range; annotated "30% -- within normal range" |

High and low threshold states share one Examples section.

### Custom range

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Custom range | `value=350`, `min=0`, `max=500`, `ariaLabel="API calls"` | Track with fill at 70% width (350/500); annotated "350 / 500 API calls used" |

### Ring shape and readout

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Ring (38%) | `shape="ring"`, `value=38`, `ariaLabel="Context used"` | Circular track with a success arc covering 38% of the turn from 12 o'clock |
| Ring above high | `shape="ring"`, `value=86`, `high=80`, `ariaLabel="Context used"` | Warning-toned arc at 86%; `data-level="high"` |
| Ring with readout | `shape="ring"`, `value=64`, `showValue`, `size="xl"` | Arc at 64% with "64%" centred inside the ring |

Ring scaling is taught once in the Sizes pane, not as a second Examples matrix.

### Ring tones

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Ring tones | `shape="ring"`, `value=60`, `tone=success/accent/warning/danger/neutral` | Arc colour steps through the tone set |

## 14. Approval And Adoption Notes

- Contract status: `detailed contract`
- Approvers: pending
- Downstream adopters: health bars, storage usage, bounded scoring,
  `AgentChatInput` context indicator
- Future follow-up: none outstanding — threshold colour shifts landed with the
  `ring` shape (2026-07-24)
