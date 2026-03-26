# Pill

Status: detailed contract
Updated: 2026-03-26

## 1. Purpose

- Component name: `Pill`
- Layer: `foundation`
- Summary: a rounded inline label for compact categorization or metadata
- In scope: tone, appearance, semantic sizing, optional monospace styling
- Out of scope: removable chips, multi-select tag inputs

## 2. Anatomy

```text
[Root .pill]
  └── [Content (default slot)]
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `<span>` | yes | rounded metadata shell with inline-flex layout |
| Content | slot | yes | short label text |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `"neutral" \| "success" \| "danger"` | `"neutral"` | no | semantic tone controlling fill/border/text color |
| `appearance` | `"solid" \| "subtle" \| "badge"` | `"solid"` | no | fill opacity variant |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit pill size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `font` | `"normal" \| "mono"` | `"normal"` | no | content font variant |
| `muted` | `boolean` | `false` | no | visual de-emphasis via reduced opacity |
| `ariaLabel` | `string \| null` | `null` | no | optional explicit accessible name |

### Controlled And Uncontrolled

- Display primitive only; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral | default | neutral fill, subtle border, secondary text |
| success | `tone="success"` | green-tinted fill and border, primary text |
| danger | `tone="danger"` | red-tinted fill and border, primary text |
| subtle | `appearance="subtle"` | 50% transparent fill overlay |
| muted | `muted=true` | reduced opacity (0.72) |
| xs | `size="xs"` | smallest metadata chip sizing |
| sm | `size="sm"` | compact metadata chip sizing |
| md | inherited default or `size="md"` | default metadata chip sizing |
| lg | `size="lg"` | emphasized metadata chip sizing |
| xl | `size="xl"` | largest metadata chip sizing |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive display primitive |

## 6. Accessibility

### Semantics

- Role: inline text by default (no ARIA role)
- Required attributes: none
- Optional attributes: `aria-label` when visible text is abbreviated or symbolic
- Labeling rules: pills stay non-interactive unless a higher-order contract
  wraps them

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive by default |

### Focus And Announcement

- focus entry: not focusable by default
- live-region behavior: none
- GPUI-native accessibility mapping notes: expose pill content as text-like
  metadata, not as a control

## 7. Layout

### Sizing

- Pill sizes to content with compact padding
- Content may truncate according to parent layout rules
- Uses `white-space: nowrap` to prevent wrapping

### Composition

- parent expectations: metadata rows, headers, cards, filter summaries
- child expectations: short text content

## 8. Token Usage

### Root `.pill` (default / neutral / md)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-height` | `1.25rem` |
| `padding` | `0.1875rem 0.5rem` |
| `border` | `0.0625rem solid var(--poodle-pill-border)` |
| `border-radius` | `999px` |
| `background` | `var(--poodle-pill-fill)` |
| `color` | `var(--poodle-pill-text)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `line-height` | `1` |
| `white-space` | `nowrap` |

### Component custom properties (neutral default)

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-fill` | `color-mix(in srgb, var(--poodle-color-background-surface) 90%, transparent)` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent)` |
| `--poodle-pill-text` | `var(--poodle-color-text-secondary)` |

### Tone: success `.pill[data-tone="success"]`

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-fill` | `color-mix(in srgb, var(--poodle-color-status-success) 14%, var(--poodle-color-background-surface))` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-color-status-success) 38%, var(--poodle-color-border-subtle))` |
| `--poodle-pill-text` | `var(--poodle-color-text-primary)` |

### Tone: danger `.pill[data-tone="danger"]`

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-fill` | `color-mix(in srgb, var(--poodle-color-status-danger) 14%, var(--poodle-color-background-surface))` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-color-status-danger) 38%, var(--poodle-color-border-subtle))` |
| `--poodle-pill-text` | `var(--poodle-color-text-primary)` |

### Appearance: subtle `.pill[data-appearance="subtle"]`

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-fill` | `color-mix(in srgb, var(--poodle-pill-fill) 50%, transparent)` |

### Appearance: badge `.pill[data-appearance="badge"]`

| Custom Property / Property | Value |
|----------------------------|-------|
| `--poodle-pill-fill` | accent-tinted fill (tone-specific color at low opacity) |
| `--poodle-pill-border` | `transparent` |
| `--poodle-pill-text` | tone-specific accent color |
| `text-transform` | `uppercase` |
| `font-weight` | `700` |

### Size: sm `.pill[data-size="sm"]`

| Property | Value |
|----------|-------|
| `min-height` | `1rem` |
| `padding` | `0.125rem 0.375rem` |
| `font-size` | `0.625rem` |

### Size: xs `.pill[data-size="xs"]`

| Property | Value |
|----------|-------|
| `min-height` | `0.875rem` |
| `padding` | `0.0625rem 0.3125rem` |
| `font-size` | `0.5625rem` |

### Size: lg `.pill[data-size="lg"]`

| Property | Value |
|----------|-------|
| `min-height` | `1.375rem` |
| `padding` | `0.25rem 0.625rem` |
| `font-size` | `0.75rem` |

### Size: xl `.pill[data-size="xl"]`

| Property | Value |
|----------|-------|
| `min-height` | `1.5rem` |
| `padding` | `0.3125rem 0.75rem` |
| `font-size` | `0.8125rem` |

### Font: mono `.pill[data-font="mono"]`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-code-family)` |
| `letter-spacing` | `0.02em` |

### Muted `.pill[data-muted="true"]`

| Property | Value |
|----------|-------|
| `opacity` | `0.72` |

## 9. Svelte Notes

- Renders as a styled inline `<span>` with a default slot
- Tone, appearance, size, font, and muted state are driven via `data-*`
  attributes for CSS selector targeting
- Component custom properties (`--poodle-pill-fill`, `--poodle-pill-border`,
  `--poodle-pill-text`) are set on the root element and consumed by the same
  element's CSS, enabling tone overrides without class proliferation

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::pill`
- keep semantics non-interactive unless wrapped by a control-specific contract
- `color-mix` blending should be replicated using equivalent alpha-blended color
  calculations in GPUI's color system

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] non-interactive metadata semantics match
- [ ] tone custom property overrides produce equivalent colors

### Tier 2: Visual Parity

- [ ] `xs | sm | md | lg | xl` sizes produce correct min-height, padding, and font-size
- [ ] mono font variant uses code family with correct letter-spacing
- [ ] subtle appearance halves fill opacity
- [ ] muted state applies 0.72 opacity

### Tier 3: Implementation Freedom

- [ ] truncation and rendering internals stay internal
- [ ] `color-mix` may be replaced by pre-computed equivalents

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| `color-mix` implementation | GPUI may pre-compute blended colors rather than using CSS `color-mix` | allowed | ensure visual equivalence across themes |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Tones

Five pills in a horizontal row with 8px gap:

| Content | Tone |
|---------|------|
| Neutral | neutral |
| Success | success |
| Danger | danger |

### Sizes

Five pills in a horizontal row with 8px gap:

| Content | Size |
|---------|------|
| Extra small | xs |
| Small | sm |
| Medium | md |
| Large | lg |
| Extra large | xl |

### Code font

Five pills with `font: "mono"` in a horizontal row:

| Content | Tone | Font |
|---------|------|------|
| v2.4.1 | neutral | mono |
| stable | success | mono |
| beta | danger | mono |

### Muted

Three muted pills in a horizontal row:

| Content | Tone | Muted |
|---------|------|-------|
| Muted neutral | neutral | true |
| Muted success | success | true |
| Muted danger | danger | true |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: metadata displays, labels, status tags, card headers
- future follow-up: add dismissible-chip semantics separately if needed
