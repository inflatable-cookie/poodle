# Status Indicator

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `StatusIndicator`
- Layer: `foundation`
- Summary: a compact visual and textual signal for current state such as ready,
  warning, error, or pending
- In scope: status tone, colored dot, optional short label, pending animation
- Out of scope: full explanatory callouts or banners

## 2. Anatomy

```text
[Root .status-indicator]
  ├── [Dot .status-indicator__dot]
  └── [Label .status-indicator__label] (optional)
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `span` | yes | inline-flex host with status color variable |
| Dot | `span` | yes | colored circle indicating status |
| Label | `span` | conditional | short text label (rendered when `label` prop is set OR default slot has content) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `status` | `"neutral" \| "info" \| "success" \| "warning" \| "danger" \| "pending"` | `"neutral"` | no | semantic status tone |
| `label` | `string \| null` | `null` | no | optional short visible label text |
| `ariaLabel` | `string \| null` | `null` | no | explicit accessible label when visible label is absent or abbreviated |

### Controlled And Uncontrolled

- Display primitive only. No internal state.

### Content

- Label text comes from the `label` prop OR from default slot content
- When both are absent, only the dot is rendered

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral | `status="neutral"` (default) | dot uses `--pug-color-text-secondary` |
| info | `status="info"` | dot uses `--pug-color-accent-base` |
| success | `status="success"` | dot uses `--pug-color-status-success` |
| warning | `status="warning"` | dot uses `--pug-color-status-warning` |
| danger | `status="danger"` | dot uses `--pug-color-status-danger` |
| pending | `status="pending"` | dot uses `--pug-color-accent-base` with pulse animation |

### Component States

No internal state. Status is fully parent-controlled.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive by default |

## 6. Accessibility

### Semantics

- Role: root is a `<span>` element (inline semantics)
- Optional attributes: `aria-label={ariaLabel}` on root when provided
- Data attribute: `data-status={status}` on root
- Labeling rules: color alone must never be the only status signal; either
  `label`, `ariaLabel`, or slot content must provide text meaning

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive by default |

### Focus And Announcement

- Focus entry: not focusable by default
- Live-region behavior: parent-owned unless status changes must be announced
- GPUI-native accessibility mapping notes: GPUI must expose status meaning via
  text or accessible label, not color alone

## 7. Layout

### Sizing

- Root is `inline-flex` so it flows inline with text
- Root has `min-width: 0` to allow truncation in tight containers
- Dot is fixed at `0.5625rem` x `0.5625rem`
- Gap between dot and label is `0.4375rem`

### Composition

- Parent expectations: headers, lists, rows, status summaries
- Child expectations: dot (always) and optional label text
- Resizing rules: label truncates; dot never shrinks

## 8. Token Usage

### Root `.status-indicator`

| Property | Value |
|----------|-------|
| `--pug-status-color` | `var(--pug-color-text-secondary)` (default, overridden per status) |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.4375rem` |
| `color` | `var(--pug-color-text-primary)` |
| `min-width` | `0` |

### Root — Status Color Overrides via `data-status`

| Selector | `--pug-status-color` Value |
|----------|---------------------------|
| `[data-status="info"]` | `var(--pug-color-accent-base)` |
| `[data-status="success"]` | `var(--pug-color-status-success)` |
| `[data-status="warning"]` | `var(--pug-color-status-warning)` |
| `[data-status="danger"]` | `var(--pug-color-status-danger)` |
| `[data-status="pending"]` | `var(--pug-color-accent-base)` |

Note: `neutral` uses the default `--pug-status-color` value of
`var(--pug-color-text-secondary)` and does not need a data-status override.

### Dot `.status-indicator__dot`

| Property | Value |
|----------|-------|
| `flex` | `0 0 auto` |
| `width` | `0.5625rem` |
| `height` | `0.5625rem` |
| `border-radius` | `999px` |
| `background` | `var(--pug-status-color)` |
| `box-shadow` | `0 0 0 0.125rem color-mix(in srgb, var(--pug-status-color) 18%, transparent)` |

### Dot — Pending Animation

When `status="pending"`, the dot receives:

| Property | Value |
|----------|-------|
| `animation` | `status-pulse 1s ease-in-out infinite alternate` |

### Keyframes

```
@keyframes status-pulse {
  from { opacity: 0.55 }
  to { opacity: 1 }
}
```

### Label `.status-indicator__label`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1.3` |

### Token Reference

| Token | Role |
|-------|------|
| `--pug-color-text-secondary` | neutral dot color (default) |
| `--pug-color-text-primary` | label text color |
| `--pug-color-accent-base` | info and pending dot color |
| `--pug-color-status-success` | success dot color |
| `--pug-color-status-warning` | warning dot color |
| `--pug-color-status-danger` | danger dot color |
| `--pug-typography-label-family` | label font family |

## 9. Svelte Notes

- Status color is driven by a CSS custom property `--pug-status-color` set on
  the root, then consumed by the dot's `background` and `box-shadow`
- Data attribute `data-status` on root controls the color override via CSS
  selectors
- Label content can come from prop or default slot

## 10. GPUI Notes

- Expected crate/module surface: `pug_gpui::primitives::status_indicator`
- GPUI implementation must ensure a text or accessible-label path exists so the
  status remains perceivable to assistive technology
- Pending animation pulse is optional in GPUI if native animation support is
  limited

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all six status tones produce correct color mapping
- [ ] status meaning is not color-only in either runtime
- [ ] accessible labeling semantics match

### Tier 2: Visual Parity

- [ ] dot size is 0.5625rem with 999px border-radius
- [ ] dot box-shadow glow at 18% opacity matches
- [ ] gap of 0.4375rem between dot and label
- [ ] label typography: 0.75rem / 600 weight / 1.3 line-height
- [ ] pending pulse animation present

### Tier 3: Implementation Freedom

- [ ] dot/icon rendering details stay internal
- [ ] pending animation timing may vary slightly

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| pending animation details may differ | motion internals are runtime-specific | allowed | keep semantic labeling strict |
| CSS custom property pattern is Svelte-specific | GPUI uses direct color values | allowed | ensure same final colors |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### All statuses

Six status indicators stacked vertically, one per status with label prop:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Neutral | `status="neutral"`, `label="Neutral"` | Dot in text-secondary color with "Neutral" label |
| Info | `status="info"`, `label="Info"` | Dot in accent-base color with "Info" label |
| Success | `status="success"`, `label="Success"` | Dot in status-success color with "Success" label |
| Warning | `status="warning"`, `label="Warning"` | Dot in status-warning color with "Warning" label |
| Danger | `status="danger"`, `label="Danger"` | Dot in status-danger color with "Danger" label |
| Pending | `status="pending"`, `label="Pending"` | Dot in accent-base color with pulse animation and "Pending" label |

### Without labels (dot only)

Four status indicators in a horizontal row, dot only with ariaLabel:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Online | `status="success"`, `ariaLabel="Online"` | Success-colored dot only, no visible text |
| Away | `status="warning"`, `ariaLabel="Away"` | Warning-colored dot only, no visible text |
| Offline | `status="danger"`, `ariaLabel="Offline"` | Danger-colored dot only, no visible text |
| Unknown | `status="neutral"`, `ariaLabel="Unknown"` | Neutral-colored dot only, no visible text |

### Slot content

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Build passing | `status="success"`, default slot content "Build passing" | Success-colored dot with "Build passing" text from slot |

## 14. Approval And Adoption Notes

- Contract status: `detailed contract`
- Approvers: pending
- Downstream adopters: headers, status rows, lightweight summaries
- Future follow-up: connect to richer diagnostics surfaces later
