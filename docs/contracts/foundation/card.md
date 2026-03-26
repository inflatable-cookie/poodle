# Card

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Card`
- Layer: `foundation`
- Summary: a contained surface for grouping related content with optional media,
  header, body, and footer regions
- In scope: default, outlined, and elevated variants; vertical, horizontal, and
  compact layouts; interactive and selected states; media, header, body, and
  footer slots
- Out of scope: card carousels, card grids (see NavCardGrid), drag-and-drop
  reordering

## 2. Anatomy

```text
[Root .card]  <article>
  ├── [Media .card__media]  (optional, via media slot)
  ├── [Header .card__header]  (optional, via header slot)
  ├── [Body .card__body]  (default slot)
  └── [Footer .card__footer]  (optional, via footer slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | grid container with variant-driven styling | background, border, radius, shadow, padding, gap |
| Media | no | overflow-clipped region for images or video | overflow, border-radius |
| Header | no | title and metadata region | typography |
| Body | yes | primary content area via default slot | — |
| Footer | no | action or metadata row with top border | padding-top, border-top |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"default" \| "outlined" \| "elevated"` | `"default"` | no | visual treatment |
| `layout` | `"vertical" \| "horizontal" \| "compact"` | `"vertical"` | no | content arrangement |
| `interactive` | `boolean` | `false` | no | enables hover/focus states and pointer cursor |
| `selected` | `boolean` | `false` | no | accent border and shadow treatment |
| `media` | `boolean` | `false` | no | enables media slot region |
| `ariaLabel` | `string \| null` | `null` | no | accessible name when interactive |

### Slots

| Slot | Purpose |
|------|---------|
| media | image or video content, clipped to card radius |
| header | title, subtitle, metadata content |
| default | primary body content |
| footer | actions or supplementary metadata with top divider |

### Controlled And Uncontrolled

- Display component; no internal state. `interactive` and `selected` are
  externally controlled.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | subtle border, panel background, inset shadow |
| outlined | `variant="outlined"` | stronger border, same background |
| elevated | `variant="elevated"` | elevated background, multi-layer box-shadow |
| interactive hover | pointer enters (when interactive) | hover fill, hover border, hover shadow |
| selected | `selected=true` | accent border color, accent inset shadow |
| compact | `layout="compact"` | reduced padding and gap |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| — | — | — | Card itself emits no events; delegates to slot content |

## 6. Accessibility

### Semantics

- Root element: `<article>`
- When interactive: CSS `cursor: pointer` is applied; no ARIA roles, keyboard interaction, or focus management are currently implemented (see Known Deltas)
- Landmark: none

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus to next focusable element |

### Focus And Announcement

- non-interactive cards are not focusable
- interactive cards do not currently receive focus (see Known Deltas)

## 7. Layout

### Sizing

- Vertical: single-column grid, content stacks top to bottom
- Horizontal: two-column grid `auto 1fr`, media in left column spanning all rows
- Compact: reduced padding and gap

### Composition

- parent expectations: page grids, dashboard layouts, list views
- child expectations: media, header, body, footer via slots
- resizing: fills parent width, height auto-fits content

## 8. Token Usage — Exact Values

### CSS Custom Properties

| Var | Value |
|-----|-------|
| `--poodle-recipe-card-radius` | `var(--poodle-treatment-surface-radius, var(--poodle-radius-surface))` |
| `--poodle-recipe-card-fill` | `color-mix(in srgb, var(--poodle-color-background-panel) 98%, var(--poodle-color-background-elevated))` |
| `--poodle-recipe-card-border` | `color-mix(in srgb, var(--poodle-color-border-subtle) 18%, transparent)` |
| `--poodle-recipe-card-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 18%, transparent)` |
| `--poodle-recipe-card-divider` | `color-mix(in srgb, var(--poodle-color-border-subtle) 52%, transparent)` |
| `--poodle-recipe-card-hover-fill` | `var(--poodle-treatment-surface-hover-fill, color-mix(in srgb, var(--poodle-color-background-elevated) 94%, var(--poodle-color-background-panel)))` |
| `--poodle-recipe-card-hover-border` | `var(--poodle-treatment-surface-hover-border, color-mix(in srgb, var(--poodle-color-accent-base) 28%, var(--poodle-color-border-subtle)))` |
| `--poodle-recipe-card-hover-shadow` | `var(--poodle-treatment-surface-hover-shadow, var(--poodle-recipe-card-shadow))` |

### Root (base — default variant)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `align-content` | `start` |
| `gap` | `var(--poodle-space-stack-md)` |
| `padding` | `var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid var(--poodle-recipe-card-border)` |
| `border-radius` | `var(--poodle-recipe-card-radius)` |
| `background` | `var(--poodle-treatment-surface-fill, color-mix(in srgb, var(--poodle-surface) 88%, var(--poodle-color-text-primary)))` |
| `--poodle-surface` | `var(--poodle-treatment-surface-fill, var(--poodle-recipe-card-fill))` |
| `box-shadow` | `var(--poodle-treatment-surface-shadow, var(--poodle-recipe-card-shadow))` |

### Outlined variant

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-border-default) 76%, transparent)` |

### Elevated variant

| Property | Value |
|----------|-------|
| `border-radius` | `var(--poodle-treatment-surface-elevated-radius, var(--poodle-recipe-card-radius))` |
| `border-color` | `color-mix(in srgb, var(--poodle-treatment-surface-elevated-border, var(--poodle-color-border-default)) 82%, var(--poodle-color-border-default))` |
| `background` | `var(--poodle-treatment-surface-elevated-fill, color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel)))` |

### Elevated variant box-shadow (dark mode)

| Property | Value |
|----------|-------|
| `box-shadow` | `0 1.125rem 2.5rem color-mix(in srgb, black 38%, transparent), 0 0.375rem 0.875rem color-mix(in srgb, black 24%, transparent), inset 0 0.0625rem 0 color-mix(in srgb, var(--poodle-color-text-inverse) 10%, transparent), 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-default) 12%, transparent)` |

### Elevated variant box-shadow (light mode)

| Property | Value |
|----------|-------|
| `box-shadow` | `0 0.875rem 1.75rem rgba(49,66,85,0.1), 0 0.25rem 0.625rem rgba(49,66,85,0.06), inset 0 0.0625rem 0 rgba(255,255,255,0.72), 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-default) 10%, transparent)` |

### Selected state

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-base)` |
| `box-shadow` | `0 0 0 0.0625rem var(--poodle-color-accent-base), inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |

### Interactive hover

| Property | Value |
|----------|-------|
| `cursor` | `pointer` |
| `border-color` | `var(--poodle-recipe-card-hover-border)` |
| `background` | `var(--poodle-recipe-card-hover-fill)` |
| `box-shadow` | `var(--poodle-recipe-card-hover-shadow)` |

### Horizontal layout

| Property | Value |
|----------|-------|
| `grid-template-columns` | `auto 1fr` |

### Horizontal media

| Property | Value |
|----------|-------|
| `grid-row` | `1 / -1` |
| `width` | `8rem` |

### Compact layout

| Property | Value |
|----------|-------|
| `padding` | `0.5rem 0.625rem` |
| `gap` | `var(--poodle-space-stack-sm)` |

### Media

| Property | Value |
|----------|-------|
| `overflow` | `hidden` |
| `border-radius` | `calc(var(--poodle-recipe-card-radius) - 0.1875rem)` |

### Footer

| Property | Value |
|----------|-------|
| `padding-top` | `var(--poodle-space-stack-sm)` |
| `border-top` | `0.0625rem solid var(--poodle-treatment-surface-divider, var(--poodle-recipe-card-divider))` |

## 9. Svelte Notes

- `data-variant`, `data-layout`, `data-interactive`, `data-selected` data attributes
- CSS custom properties set on root element style attribute
- Media slot conditionally rendered when `media` is true
- Footer slot conditionally rendered when footer slot has content
- Interactive mode adds click handler and keyboard activation

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::card`
- Spec struct: `CardSpec` in primitives crate
- Component struct: `PoodleCard` in components crate
- Elevated shadow requires light/dark mode branching
- CSS custom property fallback chains map to Rust conditionals
- Horizontal layout grid can be modeled as flex with fixed-width media column

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] variant, layout, interactive, selected props produce matching behavior
- [ ] interactive card is keyboard activatable
- [ ] selected card shows accent border treatment
- [ ] ARIA roles match (button when interactive)

### Tier 2: Visual Parity

- [ ] all three variant color schemes match
- [ ] elevated shadow matches per light/dark mode
- [ ] selected border and shadow match
- [ ] hover treatment matches for interactive cards
- [ ] compact padding and gap match
- [ ] horizontal layout grid matches
- [ ] media border-radius inset matches
- [ ] footer divider matches

### Tier 3: Implementation Freedom

- [ ] hover transition timing is platform-owned
- [ ] elevated shadow light/dark mode detection method is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Elevated shadow light vs dark | GPUI may detect color scheme differently than CSS media query | allowed | same visual result |
| CSS custom property fallback chains | Rust conditionals vs CSS var() fallback | allowed | same visual result |
| Interactive card accessibility | `aria-selected`, `role="button"`, `tabindex="0"`, and Enter/Space keyboard activation are documented in the contract but NOT implemented in Svelte; interactive cards only apply `cursor: pointer` | known gap | implement in a future pass |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default variant

Two cards in a horizontal row with 16px gap:

| Title | Body | Footer |
|-------|------|--------|
| Project Alpha | "A collaborative workspace for your team to plan, build, and ship products." | "Updated 2 days ago" |
| Monthly report | "Revenue grew 12% month-over-month with improved conversion rates." | — (no footer) |

### Outlined variant

One card with visible border:

| Title | Body | Variant |
|-------|------|---------|
| Settings | "Configure your workspace preferences and notification settings." | outlined |

### Elevated variant

One card with drop shadow:

| Title | Body | Variant |
|-------|------|---------|
| Dashboard | "View real-time metrics and performance indicators." | elevated |

### Interactive

One clickable card with hover state:

| Title | Body | Variant | Props |
|-------|------|---------|-------|
| Learn more | "Click to explore documentation and guides." | default | `interactive: true` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: dashboard layouts, content listings, settings panels, media galleries
- future follow-up: card group component, drag-and-drop support

> **Surface elevation**: Card is a surface creator — see [surface-elevation.md](./surface-elevation.md).
