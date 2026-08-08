# MetricTile

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `MetricTile`
- Layer: `composites`
- Summary: a compact metric display tile showing a label, current value,
  optional trend indicator with directional icon and label, and optional
  inline sparkline chart — rendered in a contained surface treatment
- In scope: label-value pairs with contained surface, code-style labels,
  trend direction with colored icon and label, inline sparkline SVG,
  accessible name computation, iceberg theme override
- Out of scope: interactive editing, multi-value display, full charts or
  gauges, click behavior

## 2. Anatomy

```text
[Root .state-tile]  <div>
  ├── [Label .state-tile__label]  <span>
  ├── [Body .state-tile__body]  <div>
  │     ├── [Value .state-tile__value]  <strong>
  │     └── [Sparkline .state-tile__sparkline]  <svg> (optional)
  └── [Trend .state-tile__trend]  <span> (optional)
        ├── [TrendArrow .state-tile__trend-arrow]  <span>
        │     └── Icon (trending-up | trending-down | arrow-right)
        └── [TrendLabel]  <span> (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | contained tile surface | background-surface, radius-surface, border |
| Label | yes | code-style metadata key | text-secondary, code-family |
| Body | yes | horizontal row holding value and optional sparkline | layout only |
| Value | yes | bold display value | body typography |
| Sparkline | no | inline SVG line chart rendered from data points | text-tertiary color |
| Trend | no | trend direction indicator row with icon and label | status colors |
| TrendArrow | no | Icon primitive showing directional icon | inherits trend color |
| TrendLabel | no | textual label for the trend (e.g. "+12.3%") | inherits trend color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `label` | `string` | — | yes | metadata key or signal name |
| `value` | `string` | — | yes | current value to display |
| `ariaLabel` | `string \| null` | `null` | no | overrides computed `"{label}: {value}"` accessible name |
| `trend` | `"up" \| "down" \| "flat" \| null` | `null` | no | trend direction indicator |
| `trendLabel` | `string \| null` | `null` | no | descriptive label for trend (e.g. "+12.3%", "-8%") |
| `sparklineData` | `number[] \| null` | `null` | no | data points for inline sparkline; requires 2+ values to render |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | overrides inherited UI presentation density for tile padding and internal spacing only |

### Slots

None.

### Controlled And Uncontrolled

Display composite only; all values are externally driven. No internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | render with label and value | contained tile with code-style label above bold value |
| with-trend | `trend` is non-null | trend row appears below body with directional icon and optional label |
| with-sparkline | `sparklineData` has 2+ values | inline SVG sparkline appears beside value in body row |
| trend-up | `trend="up"` | trend text and icon colored with success status color |
| trend-down | `trend="down"` | trend text and icon colored with danger status color |
| trend-flat | `trend="flat"` | trend text and icon colored with tertiary text color |

### Component States (Derived)

- `sparklinePath` (derived): SVG path string built from `sparklineData` via
  `buildSparkline()` pure function; `null` when data has fewer than 2 values

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Events

None. MetricTile is a non-interactive display component.

## 6. Accessibility

### Semantics

- Root: `<div>` with `aria-label` set to `ariaLabel ?? "{label}: {value}"`
- Sparkline SVG: `aria-hidden="true"` (decorative)
- Trend arrow: wrapping `<span>` has `aria-hidden="true"` (the trend label
  conveys meaning)
- Icon: rendered via Icon primitive with trend-specific name

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive; not focusable |

### Focus And Announcement

- Not focusable
- No live-region behavior

## 7. Layout

### Sizing

- Root fills available width by default (block-level)
- Minimum content height driven by label + value stack
- Sparkline: fixed `4rem` wide, `1.5rem` tall
- Internal gap between label, body, and trend rows
- Density affects padding and internal gaps, not typography or sparkline size

### Composition

- Composes: `Icon` from `@inflatable-cookie/poodle-svelte`
- Parent expectations: grids, stacks, signal panels, workspace surfaces
- Child expectations: none (self-contained)
- Resizing rules: tile stretches horizontally; value text wraps if needed

### Sparkline Path Generation

The `buildSparkline()` function generates an SVG `<path>` from data points:
- ViewBox: `0 0 64 24`
- Padding: 1px inset
- X-axis: data points distributed linearly across width
- Y-axis: values normalized to min/max range, inverted for SVG coordinates
- Output: `M`/`L` commands with 1-decimal precision

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-trend` | `.state-tile__trend` span | `"up"`, `"down"`, `"flat"` |

### Root `.state-tile`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-inline-sm)` |
| padding | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| border | `0.0625rem solid transparent` |
| border-radius | `var(--poodle-radius-surface)` |
| background | `color-mix(in srgb, var(--poodle-color-background-surface) 60%, transparent)` |

### Density Overrides

| `data-density` | Root Gap | Root Padding | Body Gap |
|----------------|----------|--------------|----------|
| `compact` | `0.375rem` | `0.5rem 0.75rem` | `0.5rem` |
| `default` | `var(--poodle-space-inline-sm)` | `0.625rem var(--poodle-space-panel-x)` | `var(--poodle-space-inline-md)` |
| `comfortable` | `0.625rem` | `0.75rem 1.25rem` | `0.875rem` |

### Label `.state-tile__label`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-secondary)` |
| font-family | `var(--poodle-typography-code-family)` |
| font-size | `0.75rem` |

### Body `.state-tile__body`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `var(--poodle-space-inline-md)` |

### Value `.state-tile__value`

| Property | Value |
|----------|-------|
| font-size | `1rem` |

### Sparkline `.state-tile__sparkline`

| Property | Value |
|----------|-------|
| width | `4rem` |
| height | `1.5rem` |
| color | `var(--poodle-color-text-tertiary)` |
| flex-shrink | `0` |
| SVG viewBox | `0 0 64 24` |
| SVG fill | `none` |
| path stroke | `currentColor` |
| path stroke-width | `1.5` |
| path stroke-linecap | `round` |
| path stroke-linejoin | `round` |

### Trend `.state-tile__trend`

| Property | Value |
|----------|-------|
| display | `inline-flex` |
| align-items | `center` |
| gap | `0.25rem` |
| font-size | `0.75rem` |
| font-family | `var(--poodle-typography-code-family)` |
| color | `var(--poodle-color-text-secondary)` |

### Trend Colors By `data-trend`

| data-trend | color |
|------------|-------|
| `up` | `var(--poodle-color-status-success, #22c55e)` |
| `down` | `var(--poodle-color-status-danger, #ef4444)` |
| `flat` | `var(--poodle-color-text-tertiary)` |

### Trend Arrow `.state-tile__trend-arrow`

| Property | Value |
|----------|-------|
| font-size | `0.875rem` |
| line-height | `1` |

### Trend Icon Mapping

| trend | Icon name |
|-------|-----------|
| `up` | `trending-up` |
| `down` | `trending-down` |
| `flat` | `arrow-right` |

### Iceberg Theme Overrides

| Selector | Property | Value |
|----------|----------|-------|
| `:global([data-theme="iceberg"]) .state-tile` | background | `var(--poodle-treatment-surface-fill)` |

### Composed Primitives

| Part | Delegates To |
|------|-------------|
| TrendArrow Icon | Icon contract (foundation) |

## 9. Svelte Notes

- Root uses `<div class="state-tile">` with CSS class prefix `state-tile__`
  (not `metric-tile__`)
- `density` resolves from UI presentation context when not provided explicitly
- Sparkline is built via a pure `buildSparkline()` function that generates an
  SVG path from data points
- Trend icons use the `Icon` primitive (no explicit `size` prop — inherits
  from font-size of parent `trend-arrow` span)
- Iceberg theme override uses `:global([data-theme="iceberg"])` selector with
  `--poodle-treatment-surface-fill` token
- `sparklinePath` is a reactive derived value computed when `sparklineData`
  has 2+ entries

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::metric_tile`
- Render as vertical stack of styled text within contained surface
- Sparkline rendering may use platform-specific drawing primitives
- Trend colors must map to the same status tokens
- Light theme surface fill override must be honored

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] accessible name computation matches (`"{label}: {value}"`)
- [ ] trend prop values and visual mapping match
- [ ] sparklineData produces consistent visual output (same path generation)
- [ ] trend icon mapping matches (trending-up, trending-down, arrow-right)

### Tier 2: Visual Parity

- [ ] tile surface treatment matches (60% surface background, transparent border)
- [ ] typography hierarchy matches (code label, bold value)
- [ ] trend colors map to correct status tokens
- [ ] sparkline dimensions and stroke match (4rem x 1.5rem, stroke-width 1.5)
- [ ] iceberg theme override matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal
- [ ] sparkline path generation algorithm may differ in implementation details

## 12. Specimen Definitions

### Basic Tiles

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Components | `label="Components"`, `value="85"` | contained tile with code-style label and bold value |
| Coverage | `label="Coverage"`, `value="94%"` | contained tile with code-style label and bold value |
| Open issues | `label="Open issues"`, `value="12"` | contained tile with code-style label and bold value |
| Build time | `label="Build time"`, `value="1.8s"` | contained tile with code-style label and bold value |

Tiles are displayed in a responsive grid layout (`auto-fit, minmax(10rem, 1fr)`).

### With Trend Indicators

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Active users | `label="Active users"`, `value="2,847"`, `trend="up"`, `trendLabel="+12.3%"` | tile with green upward trend icon and positive trend label |
| Error rate | `label="Error rate"`, `value="0.04%"`, `trend="down"`, `trendLabel="-8%"` | tile with red downward trend icon and negative trend label |
| Latency | `label="Latency"`, `value="42ms"`, `trend="flat"`, `trendLabel="No change"` | tile with tertiary flat trend icon and neutral trend label |
| Revenue | `label="Revenue"`, `value="$14.2k"`, `trend="up"`, `trendLabel="+3.1%"` | tile with green upward trend icon and positive trend label |

### With Sparklines

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Requests/min | `label="Requests/min"`, `value="1,204"`, `trend="up"`, `trendLabel="+5%"`, `sparklineData=[800,920,850,1100,980,1050,1204]` | tile with trend indicator and inline sparkline chart beside value |
| CPU usage | `label="CPU usage"`, `value="62%"`, `trend="down"`, `trendLabel="-4%"`, `sparklineData=[75,72,68,70,65,63,62]` | tile with trend indicator and inline sparkline chart beside value |
| Memory | `label="Memory"`, `value="4.2 GB"`, `sparklineData=[3.8,3.9,4.0,4.1,4.0,4.1,4.2]` | tile with sparkline chart beside value, no trend indicator |
