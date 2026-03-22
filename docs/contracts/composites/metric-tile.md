# MetricTile

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `MetricTile`
- Layer: `composites`
- Summary: a compact metric display tile showing a label, current value, optional
  trend indicator, and optional sparkline chart
- In scope: label-value pairs with contained surface treatment, code-style labels,
  trend direction with icon and label, inline sparkline SVG
- Out of scope: interactive editing, multi-value display, full charts or gauges

## 2. Anatomy

```text
[Root]
  ├── [Label]
  ├── [Body]
  │     ├── [Value]
  │     └── [Sparkline]  (optional)
  └── [Trend]            (optional)
        ├── [TrendArrow]
        └── [TrendLabel]  (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | contained tile surface | background, radius, border |
| Label | yes | code-style metadata key | typography (code family), secondary color |
| Body | yes | horizontal row holding value and optional sparkline | layout only |
| Value | yes | bold display value | typography (body), primary color |
| Sparkline | no | inline SVG line chart rendered from data points | tertiary color for stroke |
| Trend | no | trend direction indicator row | status colors (success, danger, tertiary) |
| TrendArrow | no | icon indicating direction (trending-up, trending-down, arrow-right) | inherits trend color |
| TrendLabel | no | textual label for the trend (e.g. "+12.3%") | inherits trend color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `label` | `string` | -- | yes | metadata key or signal name |
| `value` | `string` | -- | yes | current value to display |
| `ariaLabel` | `string \| null` | `null` | no | overrides computed `label: value` accessible name |
| `trend` | `"up" \| "down" \| "flat" \| null` | `null` | no | trend direction indicator |
| `trendLabel` | `string \| null` | `null` | no | descriptive label for trend (e.g. "+12.3%", "-8%") |
| `sparklineData` | `number[] \| null` | `null` | no | data points for inline sparkline; requires 2+ values |

### Controlled And Uncontrolled

- display composite only; all values are externally driven

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | render | contained tile with label above bold value |
| with-trend | `trend` is non-null | trend row appears below body with directional icon and optional label |
| with-sparkline | `sparklineData` has 2+ values | inline SVG sparkline appears beside value |
| trend-up | `trend="up"` | trend text and icon colored with success status color |
| trend-down | `trend="down"` | trend text and icon colored with danger status color |
| trend-flat | `trend="flat"` | trend text and icon colored with tertiary text color |

### Component States

No internal state. MetricTile is a pure display component. The sparkline path is
derived from `sparklineData` via a pure function.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive |

## 6. Accessibility

### Semantics

- Role: generic container (`<div>`)
- Required attributes: none
- Optional attributes: `aria-label` for composite accessible name
- Labeling rules: defaults to `"label: value"` pattern when no explicit
  `ariaLabel` is provided
- Sparkline SVG is `aria-hidden="true"` (decorative)
- Trend arrow icon is `aria-hidden="true"` (the trend label conveys meaning)

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive |

### Focus And Announcement

- focus entry: not focusable
- live-region behavior: none
- GPUI-native accessibility mapping notes: expose as labeled value group

## 7. Layout

### Sizing

- fills available width by default (block-level)
- minimum content height driven by label + value stack
- sparkline has fixed dimensions: 4rem wide, 1.5rem tall
- internal gap: 0.375rem between label, body, and trend rows

### Composition

- parent expectations: grids, stacks, signal panels, workspace surfaces
- child expectations: none (self-contained)
- resizing rules: tile stretches horizontally, value text truncates if needed

## 8. Token Usage And Precise CSS

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-trend` | trend `<span>` | `"up"`, `"down"`, `"flat"` |

### Root

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `0.375rem` |
| padding | `0.875rem` |
| border | `0.0625rem solid transparent` |
| border-radius | `var(--pug-radius-surface)` |
| background | `color-mix(in srgb, var(--pug-color-background-surface) 60%, transparent)` |

### Label

| Property | Value |
|----------|-------|
| color | `var(--pug-color-text-secondary)` |
| font-family | `var(--pug-typography-code-family)` |
| font-size | `0.75rem` |

### Body

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.625rem` |

### Value

| Property | Value |
|----------|-------|
| font-size | `1rem` |

### Sparkline

| Property | Value |
|----------|-------|
| width | `4rem` |
| height | `1.5rem` |
| color | `var(--pug-color-text-tertiary)` |
| flex-shrink | `0` |
| SVG viewBox | `0 0 64 24` |
| SVG stroke-width | `1.5` |
| SVG stroke-linecap | `round` |
| SVG stroke-linejoin | `round` |

### Trend

| Property | Value |
|----------|-------|
| display | `inline-flex` |
| align-items | `center` |
| gap | `0.25rem` |
| font-size | `0.75rem` |
| font-family | `var(--pug-typography-code-family)` |
| color | `var(--pug-color-text-secondary)` |

#### Trend Colors By Data-Trend

| data-trend | color |
|------------|-------|
| `up` | `var(--pug-color-status-success, #22c55e)` |
| `down` | `var(--pug-color-status-danger, #ef4444)` |
| `flat` | `var(--pug-color-text-tertiary)` |

### Trend Arrow

| Property | Value |
|----------|-------|
| font-size | `0.875rem` |
| line-height | `1` |

### Light Theme Overrides

| Selector | Property | Value |
|----------|----------|-------|
| `:global([data-theme="light"]) .state-tile` | background | `var(--pug-treatment-surface-fill)` |

## 9. Svelte Notes

- root uses `<div class="state-tile">` with CSS class prefix `state-tile__`
- sparkline is built via a pure `buildSparkline()` function that generates an SVG path
  from data points (viewBox 0 0 64 24, stroke-width 1.5, round linecap/linejoin)
- trend icons use the `Icon` primitive with `size="sm"`:
  `trending-up`, `trending-down`, `arrow-right`
- light theme override uses `--pug-treatment-surface-fill`

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::metric_tile`
- render as vertical stack of styled text within contained surface
- sparkline rendering may use platform-specific drawing primitives

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] label and value props have the same meaning
- [ ] accessible name computation matches
- [ ] trend prop values and visual mapping match
- [ ] sparklineData produces consistent visual output

### Tier 2: Visual Parity

- [ ] tile surface treatment matches
- [ ] typography hierarchy matches (code label, bold value)
- [ ] trend colors map to correct status tokens
- [ ] sparkline dimensions and stroke match

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal
- [ ] sparkline path generation algorithm may differ

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Specimen Definitions

### Basic Tiles

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Components | `label="Components"`, `value="85"` | Contained tile with code-style label and bold value |
| Coverage | `label="Coverage"`, `value="94%"` | Contained tile with code-style label and bold value |
| Open issues | `label="Open issues"`, `value="12"` | Contained tile with code-style label and bold value |
| Build time | `label="Build time"`, `value="1.8s"` | Contained tile with code-style label and bold value |

Tiles are displayed in a responsive grid layout (`auto-fit, minmax(10rem, 1fr)`).

### With Trend Indicators

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Active users | `label="Active users"`, `value="2,847"`, `trend="up"`, `trendLabel="+12.3%"` | Tile with upward trend indicator and positive trend label |
| Error rate | `label="Error rate"`, `value="0.04%"`, `trend="down"`, `trendLabel="-8%"` | Tile with downward trend indicator and negative trend label |
| Latency | `label="Latency"`, `value="42ms"`, `trend="flat"`, `trendLabel="No change"` | Tile with flat trend indicator and neutral trend label |
| Revenue | `label="Revenue"`, `value="$14.2k"`, `trend="up"`, `trendLabel="+3.1%"` | Tile with upward trend indicator and positive trend label |

### With Sparklines

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Requests/min | `label="Requests/min"`, `value="1,204"`, `trend="up"`, `trendLabel="+5%"`, `sparklineData=[800,920,850,1100,980,1050,1204]` | Tile with trend indicator and inline sparkline chart |
| CPU usage | `label="CPU usage"`, `value="62%"`, `trend="down"`, `trendLabel="-4%"`, `sparklineData=[75,72,68,70,65,63,62]` | Tile with trend indicator and inline sparkline chart |
| Memory | `label="Memory"`, `value="4.2 GB"`, `sparklineData=[3.8,3.9,4.0,4.1,4.0,4.1,4.2]` | Tile with sparkline chart, no trend indicator |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: workspace signals, command state panels, configuration
  displays, runtime metadata
- future follow-up: consider whether a `tone` prop is needed for warning/error
  state tiles, or whether a slot-based value variant is needed for richer content
