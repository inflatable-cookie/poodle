# DetailRow

> **Surface elevation**: DetailRow is a surface consumer (72% moderate contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `DetailRow`
- Layer: `foundation`
- Summary: a labeled key-value row for displaying metadata in detail views
  with optional description and action slot
- In scope: label, description, value display, value truncation, responsive
  single-column collapse, action slot
- Out of scope: editable values (see inline editing patterns), grouped detail
  sections

## 2. Anatomy

```text
[Root .detail-row]  <div>
  ├── [Label Block .detail-row__label-block]  <div>
  │   ├── [Label .detail-row__label]  <p>
  │   └── [Description .detail-row__description]  <p> (optional)
  ├── [Value .detail-row__value]  <p> (or value slot)
  └── [Action .detail-row__action]  (optional, via action slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | grid row container | grid layout, padding, border-radius, background |
| Label Block | yes | label and optional description group | gap |
| Label | yes | key/field name | color, typography |
| Description | no | supplementary label context | color, font-size |
| Value | no | field value display | color, typography, truncation |
| Action | no | trailing action slot | — |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `label` | `string` | — | yes | row label text |
| `description` | `string \| null` | `null` | no | supplementary description below label |
| `value` | `string \| null` | `null` | no | display value; overridden by value slot |
| `truncateValue` | `boolean` | `false` | no | truncates value text with ellipsis |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the row |

### Slots

| Slot | Purpose |
|------|---------|
| value | custom value content (overrides value prop) |
| action | trailing action element (button, link, etc.) |

### Controlled And Uncontrolled

- Display primitive only; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | three-column grid with label, value, action |
| truncated | `truncateValue=true` | value text clips with ellipsis |
| responsive | viewport < 45rem | single-column stack |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| — | — | — | DetailRow emits no events; delegates to slot content |

## 6. Accessibility

### Semantics

- Root: `<div>` with optional `aria-label`
- Label and value are associated by visual proximity and reading order
- No special ARIA roles needed for static display

### Keyboard

| Key | Behavior |
|-----|----------|
| — | Not focusable unless action slot contains interactive elements |

### Focus And Announcement

- Not focusable by default; action slot content manages its own focus

## 7. Layout

### Sizing

- Default: three-column grid `11.25rem minmax(0,1fr) auto`
- Responsive (max-width: 45rem): single-column `1fr`

### Composition

- parent expectations: detail sections, settings panels, metadata views
- child expectations: label text, value text or custom value content, optional action
- resizing: fills parent width, height auto-fits content

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `11.25rem minmax(0, 1fr) auto` |
| `gap` | `var(--pug-space-inline-md)` |
| `align-items` | `center` |
| `padding` | `0.75rem 0.875rem` |
| `border-radius` | `calc(var(--pug-radius-surface) - 0.0625rem)` |
| `background` | `color-mix(in srgb, var(--pug-surface) 72%, var(--pug-color-background-elevated))` |

### Label Block

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.25rem` |
| `min-width` | `0` |

### Label

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `var(--pug-typography-label-size)` |
| `line-height` | `var(--pug-typography-label-lineHeight)` |
| `margin` | `0` |

### Description

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.5` |
| `margin` | `0` |

### Value

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-primary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `var(--pug-typography-body-size)` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |
| `min-width` | `0` |
| `margin` | `0` |

### Value (truncated)

| Property | Value |
|----------|-------|
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |
| `white-space` | `nowrap` |

### Responsive (max-width: 45rem)

| Property | Value |
|----------|-------|
| `grid-template-columns` | `1fr` |

## 9. Svelte Notes

- `data-truncate` attribute when truncateValue is true
- Value slot takes precedence over value prop when both provided
- Action slot conditionally rendered when content provided
- Media query handled via CSS `@media (max-width: 45rem)`

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::detail_row`
- Spec struct: `DetailRowSpec` in primitives crate
- Component struct: `PugDetailRow` in components crate
- Three-column grid maps to flex layout with fixed-width label column
- Responsive breakpoint may use GPUI layout measurements
- Text truncation uses GPUI's text ellipsis support

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] label, description, value display correctly
- [ ] truncateValue clips with ellipsis
- [ ] value slot overrides value prop

### Tier 2: Visual Parity

- [ ] grid column widths match (11.25rem label column)
- [ ] padding and gap match
- [ ] background color-mix matches
- [ ] typography tokens match for label, description, value
- [ ] responsive breakpoint matches

### Tier 3: Implementation Freedom

- [ ] responsive detection method is platform-owned
- [ ] slot mechanism is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Responsive breakpoint detection | GPUI may use layout-based detection vs CSS media query | allowed | same visual result |

## 13. Specimen Definitions

### Basic Label-Value Pairs

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Name | `label="Name"`, `value="Pug Design System"` | Three-column grid row with label and value |
| Version | `label="Version"`, `value="2.1.0"` | Three-column grid row with label and value |
| License | `label="License"`, `value="MIT"` | Three-column grid row with label and value |

### With Description

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With description | `label="API endpoint"`, `value="https://api.example.com/v2"`, `description="Base URL for all API requests."`, `truncateValue` | Row with label, description below label, and truncated value with ellipsis |

### With Action Slot

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With action slot | `label="Email"`, `value="clay@example.com"`, action slot containing secondary small "Change" button | Row with label, value, and trailing action button |

### With Value Slot

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With value slot | `label="Status"`, value slot containing custom status badge | Row with label and custom rendered value content |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: detail sections, settings panels, metadata displays
- future follow-up: editable detail row variant, copy-to-clipboard action
