# Badge

> **Surface elevation**: Badge (muted variant) is a surface consumer (78% subtle contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Badge`
- Layer: `foundation`
- Summary: a compact emphasis label for counts, status hints, or lightweight
  categorization
- In scope: short inline content, accent and muted variants
- Out of scope: dismissible chips, selection pills, interactive badges

## 2. Anatomy

```text
[Root .badge]  <span>
  └── [Content] (slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | compact label shell | background, radius, text, padding |
| Content | yes | short text or count | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"accent" \| "muted"` | `"accent"` | no | appearance family |
| `ariaLabel` | `string \| null` | `null` | no | explicit accessible name |

### Controlled And Uncontrolled

- display primitive only, no state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| accent | default | accent-tinted background, primary text, uppercase |
| muted | `variant="muted"` | surface-tinted background, secondary text, uppercase |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive by default |

## 6. Accessibility

### Semantics

- Role: inline `<span>`, no interactive role
- `aria-label`: from prop (optional, for abbreviated/symbolic content)

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive, not focusable |

### Focus And Announcement

- Not focusable by default
- No live-region behavior

## 7. Layout

### Sizing

- `display: inline-flex`, sizes to content
- `min-height: 1.25rem`
- Non-wrapping by default

### Composition

- parent expectations: headers, cards, rows, status text, inline with other text
- child expectations: short text or numeric content

## 8. Token Usage — Exact Values

### Root (accent variant — default)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-height` | `1.25rem` |
| `padding` | `0.125rem 0.4375rem` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent)` |
| `color` | `var(--pug-color-text-primary)` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `700` |
| `letter-spacing` | `0.04em` |
| `line-height` | `1` |
| `text-transform` | `uppercase` |
| `white-space` | `nowrap` |

### Root (muted variant)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-surface) 78%, var(--pug-color-background-elevated))` |
| `color` | `var(--pug-color-text-secondary)` |

## 9. Svelte Notes

- Simple `<span>` with `data-variant` attribute
- Slot-based content model
- No events, no state, no lifecycle hooks

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::badge`
- GPUI color-mix: accent 18% transparent → `accent.opacity(accent.a * 0.18)`
- GPUI color-mix: surface 88% transparent → `surface.opacity(surface.a * 0.88)`
- Text-transform uppercase: GPUI must uppercase the label string programmatically

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] non-interactive inline semantics match
- [ ] variant prop meaning matches

### Tier 2: Visual Parity

- [ ] accent background color-mix (18%) matches
- [ ] muted background color-mix (88%) matches
- [ ] font-size 0.6875rem matches
- [ ] font-weight 700 matches
- [ ] letter-spacing 0.04em matches
- [ ] uppercase text-transform matches
- [ ] padding 0.125rem 0.4375rem matches
- [ ] min-height 1.25rem matches
- [ ] border-radius 999px matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | n/a | n/a | n/a |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: counts, labels, status hints, data tables
- future follow-up: additional variants (success, warning, danger) if needed
