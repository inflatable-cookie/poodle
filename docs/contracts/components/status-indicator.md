# Status Indicator

Status: detailed contract
Updated: 2026-04-01

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
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `typography` | `"label" \| "inherit"` | `"label"` | no | label typography by default; use `"inherit"` when parent inline text should own the label font metrics |

### Controlled And Uncontrolled

- Display primitive only. No internal state.

### Content

- Label text comes from the `label` prop OR from default slot content
- When both are absent, only the dot is rendered

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral | `status="neutral"` (default) | dot uses `--poodle-color-text-secondary` |
| info | `status="info"` | dot uses `--poodle-color-status-info` (fallback `#3b82f6`) |
| success | `status="success"` | dot uses `--poodle-color-status-success` |
| warning | `status="warning"` | dot uses `--poodle-color-status-warning` |
| danger | `status="danger"` | dot uses `--poodle-color-status-danger` |
| pending | `status="pending"` | dot uses `--poodle-color-accent-base` with pulse animation |

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
- `typography="inherit"` keeps the selected size preset, but expresses font,
  dot, and gap metrics in `em` so the whole indicator scales with parent text

### Composition

- Parent expectations: headers, lists, rows, status summaries
- Child expectations: dot (always) and optional label text
- Resizing rules: label truncates; dot never shrinks

## 8. Token Usage

### Root `.status-indicator`

| Property | Value |
|----------|-------|
| `--poodle-status-color` | `var(--poodle-color-text-secondary)` (default, overridden per status) |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.4375rem` |
| `color` | `var(--poodle-color-text-primary)` |
| `min-width` | `0` |

### Root — Status Color Overrides via `data-status`

| Selector | `--poodle-status-color` Value |
|----------|---------------------------|
| `[data-status="info"]` | `var(--poodle-color-status-info, #3b82f6)` |
| `[data-status="success"]` | `var(--poodle-color-status-success)` |
| `[data-status="warning"]` | `var(--poodle-color-status-warning)` |
| `[data-status="danger"]` | `var(--poodle-color-status-danger)` |
| `[data-status="pending"]` | `var(--poodle-color-accent-base)` |

Note: `neutral` uses the default `--poodle-status-color` value of
`var(--poodle-color-text-secondary)` and does not need a data-status override.

### Dot `.status-indicator__dot`

| Property | Value |
|----------|-------|
| `flex` | `0 0 auto` |
| `width` | `0.5625rem` |
| `height` | `0.5625rem` |
| `border-radius` | `999px` |
| `background` | `var(--poodle-status-color)` |
| `box-shadow` | `0 0 0 0.125rem color-mix(in srgb, var(--poodle-status-color) 18%, transparent)` |

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
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1.3` |

When `typography="inherit"`:

| Property | Value |
|----------|-------|
| `font` | `inherit` |
| `font-weight` | `600` |
| `font-size` | size-specific `em` value derived from the selected preset |

### Size adjustments

| Size | Dot size | Label font-size |
|------|----------|----------------|
| `xs` | `0.375rem` | `0.625rem` |
| `sm` | `0.4375rem` | `0.6875rem` |
| `md` | `0.5625rem` | `0.75rem` |
| `lg` | `0.6875rem` | `0.8125rem` |
| `xl` | `0.8125rem` | `0.875rem` |

When `typography="inherit"`:

| Size | Root font-size | Dot size | Gap |
|------|----------------|----------|-----|
| `xs` | `0.7143em` | `0.6em` | `0.5em` |
| `sm` | `0.7857em` | `0.6364em` | `0.5455em` |
| `md` | `0.8571em` | `0.75em` | `0.5833em` |
| `lg` | `0.9286em` | `0.8462em` | `0.6154em` |
| `xl` | `1em` | `0.9286em` | `0.6429em` |

### Token Reference

| Token | Role |
|-------|------|
| `--poodle-color-text-secondary` | neutral dot color (default) |
| `--poodle-color-text-primary` | label text color |
| `--poodle-color-status-info` | info dot color (fallback `#3b82f6`) |
| `--poodle-color-accent-base` | pending dot color |
| `--poodle-color-status-success` | success dot color |
| `--poodle-color-status-warning` | warning dot color |
| `--poodle-color-status-danger` | danger dot color |
| `--poodle-typography-label-family` | label font family |

## 9. Svelte Notes

- Status color is driven by a CSS custom property `--poodle-status-color` set on
  the root, then consumed by the dot's `background` and `box-shadow`
- Data attribute `data-status` on root controls the color override via CSS
  selectors
- `data-size` on root reflects the resolved size
- `data-density` on root reflects the resolved density value (`compact`, `default`, or `comfortable`)
- `typography="inherit"` uses the proportional-inherit rule from
  `docs/contracts/001-working-rules.md`: the selected size preset is converted
  from token `rem` values into equivalent `em` values for label, dot, and gap
- Label content can come from prop or default slot

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::primitives::status_indicator`
- GPUI implementation must ensure a text or accessible-label path exists so the
  status remains perceivable to assistive technology
- Pending animation pulse is optional in GPUI if native animation support is
  limited
- for `typography="inherit"`, non-CSS runtimes may approximate parent-owned
  `em` behavior with ratio-preserving metrics from a 1rem baseline until
  parent-relative inline layout exists

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
| Jetstream dot glow and label metrics | current Jetstream `JsEl` surface does not yet expose box-shadow, line-height, or full text styling parity | allowed | add shadow and richer text metrics support, then apply glow and label metrics literally |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### All statuses

Six status indicators stacked vertically, one per status with label prop:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Neutral | `status="neutral"`, `label="Neutral"` | Dot in text-secondary color with "Neutral" label |
| Info | `status="info"`, `label="Info"` | Dot in status-info color with "Info" label |
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
