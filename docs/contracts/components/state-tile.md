# StateTile

> **Implementation note**: Svelte component not yet built. Contract is complete — implementation is pending.

Status: detailed contract
Updated: 2026-04-11

## 1. Purpose

- Component name: `StateTile`
- Layer: `foundation`
- Summary: a lightweight label-value display tile with optional trend
  indicator and sparkline flag. Simpler than `MetricTile`: no interactive
  affordances, no rich formatting, no light-theme override. Suitable for
  compact state dashboards where many tiles share the same visual treatment.
- In scope: label and value text, optional trend token (`"up"`, `"down"`,
  or arbitrary string), optional trend label, boolean "has sparkline" flag
  that tells the host whether to render an inline sparkline
- Out of scope: interactive editing, click handlers, sparkline data (owned
  by the host; StateTile only signals that the caller should render one),
  multi-value display, theme overrides

## 2. Anatomy

```text
[Root .state-tile]  <div>
  ├── [Label .state-tile__label]  <span>
  ├── [Value .state-tile__value]  <strong>
  ├── [Trend .state-tile__trend]  <span> (optional, when trend set)
  │     ├── [TrendIndicator]  directional icon or text
  │     └── [TrendLabel]  <span> (optional)
  └── [Sparkline .state-tile__sparkline]  <svg> (when has_sparkline is true)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | tile container | background, border, radius, padding |
| Label | yes | caption text | typography-label, text-secondary |
| Value | yes | primary value display | typography-heading |
| Trend | no | trend indicator and label | trend color token |
| Sparkline | no | inline chart area | delegated to host |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `label` | `string` | — | yes | caption above the value |
| `value` | `string` | — | yes | primary display value (pre-formatted by caller) |
| `trend` | `string \| null` | `null` | no | trend token; `"up"` / `"down"` map to success / danger colors, any other string is neutral |
| `trendLabel` | `string \| null` | `null` | no | optional label beside the trend indicator (e.g. "+12%") |
| `hasSparkline` | `boolean` | `false` | no | when true, host should render an inline sparkline in the Sparkline slot |

### Controlled And Uncontrolled

- Wholly static; StateTile has no internal state or event surface
- Trend direction and sparkline data are computed by the caller

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | all defaults | label + value, no trend, no sparkline |
| up-trend | `trend="up"` | success-colored trend indicator |
| down-trend | `trend="down"` | danger-colored trend indicator |
| neutral-trend | `trend` set to any other string | text-secondary colored indicator |
| with-sparkline | `hasSparkline=true` | sparkline region reserved; host renders chart |

## 5. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root fill | `color.background.panel` (from `fill_token()`) | base surface |
| Root border | `color.border.subtle` (from `border_token()`) | subtle outline |
| Trend color | `color.status.success` / `color.status.danger` / `color.text.secondary` | from `trend_color_token()` |

## 6. Rust Spec

- Rust type: `poodle_specs::StateTileSpec`
- File: `packages/contracts/components/src/state_tile.rs`

## 7. Accessibility

- Root stays accessibility-neutral by default. Do not imply button, link, or
  status semantics unless a host wrapper owns that behavior.
- Label and value must remain plain text content so assistive technology reads
  them in source order without extra role noise.
- Trend text must be expressed in readable text, not icon-only direction
  markers. If an icon is present, treat it as decorative and keep the trend
  meaning in text.
- Sparkline content is host-owned. When a host renders one, it must decide
  whether the chart is decorative or needs its own accessible summary.
- If a host promotes StateTile into a live status region or interactive card,
  that wrapper owns the additional semantics and keyboard behavior.

## 8. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Overlaps with `MetricTile` visually | StateTile is the stripped-down shape (5 fields) while MetricTile (25 fields) carries interactive, formatting, and theme-override surface | allowed | both exist as different grain levels; consumers pick based on needs |
| `trend` is a string rather than an enum | allows callers to pass domain-specific values like `"flat"` or locale tokens without contract churn | allowed | tighten to an enum if a shared vocabulary emerges |
