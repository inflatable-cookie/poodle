# Skeleton

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Skeleton`
- Layer: `foundation`
- Summary: a non-interactive loading placeholder that matches expected content
  shape, with optional shimmer animation and built-in presets for common layouts
- In scope: shape-based placeholders (line, block, circle), shimmer animation,
  preset skeletons for table rows, cards, list items, detail sections, and
  avatar-line layouts
- Out of scope: progress semantics, real content fallback logic, skeleton
  screens as full page layouts

## 2. Anatomy

### Single shape

```text
[Root .skeleton]  <div>
```

### Preset layout

```text
[Root .skeleton-preset]  <div aria-hidden="true">
  ├── [Skeleton .skeleton] ...
  └── [Skeleton .skeleton] ...
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root (single) | yes | placeholder block | background, radius, animation |
| Root (preset) | yes | flex container for preset layout | flex, gap, padding, border |
| Skeleton child | yes (in preset) | individual placeholder element | width, height, radius |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `shape` | `"line" \| "block" \| "circle"` | `"line"` | no | placeholder form factor |
| `preset` | `"table-row" \| "card" \| "list-item" \| "detail-section" \| "avatar-line" \| null` | `null` | no | predefined layout skeleton |
| `width` | `string \| null` | `null` | no | custom width override |
| `height` | `string \| null` | `null` | no | custom height override |
| `lines` | `number` | `3` | no | line count for detail-section preset |
| `animated` | `boolean` | `true` | no | shimmer animation toggle |

### Controlled And Uncontrolled

- display primitive only, no internal state beyond animation toggle

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| animated | `animated=true` (default) | shimmer gradient sweeps across placeholder |
| static | `animated=false` | fixed gradient placeholder with no motion |

### Component States

No internal state. Animation is purely CSS-driven.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | display primitive only |

## 6. Accessibility

### Semantics

- Role: none; decorative loading scaffold
- Preset containers: `aria-hidden="true"`
- Single skeletons: no semantic role
- Labeling rules: skeleton placeholders must not be exposed as real content;
  parent-owned loading state should provide `aria-busy` and status announcements

### Keyboard

| Key | Behavior |
|-----|----------|
| none | not interactive |

### Focus And Announcement

- focus entry: never focusable
- live-region behavior: parent-owned; the loading container should use
  `aria-busy="true"` and announce loading state changes
- GPUI-native accessibility mapping notes: GPUI must keep skeleton placeholders
  out of the accessible tree as decorative scaffolds

## 7. Layout

### Sizing

- Single shapes use resolved defaults based on `shape`:
  - `line`: width 100%, height 0.875rem (14px)
  - `circle`: width 2.5rem (40px), height 2.5rem (40px)
  - `block`: width 100%, height 6rem (96px)
- `width` and `height` props override resolved defaults
- Presets define their own internal sizing

### Composition

- parent expectations: loading states for cards, lists, forms, detail views,
  tables
- child expectations: none for single shape; presets compose internal children
- resizing rules: placeholder follows given width/height constraints; presets
  have fixed internal proportions

## 8. Token Usage — Exact Values

### Base skeleton `.skeleton`

| Property | Value |
|----------|-------|
| `display` | `block` |
| `width` | `var(--poodle-skeleton-width, 100%)` |
| `height` | `var(--poodle-skeleton-height, 0.875rem)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `linear-gradient(90deg, color-mix(in srgb, var(--poodle-color-background-elevated) 88%, transparent) 0%, color-mix(in srgb, var(--poodle-color-background-surface) 92%, white) 48%, color-mix(in srgb, var(--poodle-color-background-elevated) 88%, transparent) 100%)` |
| `background-size` | `220% 100%` |

### Shape override: circle

| Property | Value |
|----------|-------|
| `border-radius` | `999rem` |

Resolved defaults when no width/height props: width `2.5rem`, height `2.5rem`.

### Shape override: block

| Property | Value |
|----------|-------|
| `border-radius` | `calc(var(--poodle-radius-surface) - 0.25rem)` |

Resolved defaults when no width/height props: width `100%`, height `6rem`.

### Animation `.skeleton[data-animated="true"]`

| Property | Value |
|----------|-------|
| `animation` | `skeleton-shimmer 1.6s linear infinite` |

Preset containers also animate children via
`.skeleton-preset[data-animated="true"] .skeleton` with the same animation.

### Keyframes `@keyframes skeleton-shimmer`

| Step | Property | Value |
|------|----------|-------|
| `from` | `background-position` | `200% 0` |
| `to` | `background-position` | `-20% 0` |

### Preset shared `.skeleton-preset`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |

All preset roots set `aria-hidden="true"` and `data-animated={animated}`.

### Preset: table-row `.skeleton-preset--table-row`

| Property | Value |
|----------|-------|
| `flex-direction` | `row` |
| `gap` | `0.75rem` |
| `padding` | `0.625rem 0` |
| `border-bottom` | `1px solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent)` |

#### Cell `.skeleton--cell`

| Property | Value |
|----------|-------|
| `height` | `0.875rem` |
| `flex` | `1` |

4 cells total. Width overrides per cell index: i=0 `40%`, i=1 `60%`, i=2 `60%`,
i=3 `20%`.

### Preset: card `.skeleton-preset--card`

| Property | Value |
|----------|-------|
| `gap` | `0.75rem` |
| `padding` | `1rem` |
| `border` | `1px solid color-mix(in srgb, var(--poodle-color-border-default) 42%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |

#### Block header `.skeleton--block-header`

| Property | Value |
|----------|-------|
| `height` | `6rem` |
| `border-radius` | `calc(var(--poodle-radius-surface) - 0.375rem)` |

#### Card body `.skeleton-preset__card-body`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.375rem` |

3 lines with widths: `80%`, `100%`, `60%`.

#### Card footer `.skeleton-preset__card-footer`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `gap` | `0.5rem` |
| `padding-top` | `0.25rem` |

#### Pill `.skeleton--pill`

| Property | Value |
|----------|-------|
| `width` | `3.5rem` |
| `height` | `1.25rem` |
| `border-radius` | `999rem` |

### Preset: list-item `.skeleton-preset--list-item`

| Property | Value |
|----------|-------|
| `flex-direction` | `row` |
| `align-items` | `center` |
| `gap` | `0.75rem` |
| `padding` | `0.5rem 0` |

#### Avatar `.skeleton--avatar`

| Property | Value |
|----------|-------|
| `width` | `2.25rem` |
| `height` | `2.25rem` |
| `flex-shrink` | `0` |
| `border-radius` | `999rem` |

#### List text `.skeleton-preset__list-text`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.375rem` |
| `flex` | `1` |
| `min-width` | `0` |

#### Line `.skeleton--line`

| Property | Value |
|----------|-------|
| `height` | `0.875rem` |
| `width` | `60%` |

#### Line-sm `.skeleton--line-sm`

| Property | Value |
|----------|-------|
| `height` | `0.6875rem` |
| `width` | `40%` |

### Preset: detail-section `.skeleton-preset--detail`

| Property | Value |
|----------|-------|
| `gap` | `0.625rem` |

#### Heading `.skeleton--heading`

| Property | Value |
|----------|-------|
| `width` | `8rem` |
| `height` | `1rem` |
| `margin-bottom` | `0.25rem` |

#### Detail row `.skeleton-preset__detail-row`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `gap` | `1rem` |
| `align-items` | `center` |

#### Label `.skeleton--label`

| Property | Value |
|----------|-------|
| `width` | `6rem` |
| `height` | `0.75rem` |
| `flex-shrink` | `0` |

#### Value `.skeleton--value`

| Property | Value |
|----------|-------|
| `height` | `0.75rem` |
| `flex` | `1` |
| `max-width` | `14rem` |

Number of detail rows is controlled by the `lines` prop.

### Preset: avatar-line `.skeleton-preset--avatar-line`

| Property | Value |
|----------|-------|
| `flex-direction` | `row` |
| `align-items` | `center` |
| `gap` | `0.75rem` |

Composes: avatar (2.25rem circle, same as list-item avatar) + line (width
`10rem`).

## 9. Svelte Notes

- Single shape renders a `<div>` with class `skeleton` and `data-animated`
  attribute
- Shape variants applied via class: `skeleton--line`, `skeleton--circle`,
  `skeleton--block`
- Width/height applied as CSS custom properties `--poodle-skeleton-width` and
  `--poodle-skeleton-height` via inline style
- Presets render a wrapper `<div>` with `skeleton-preset` class, containing
  multiple child `<div class="skeleton">` elements
- No events, no slots, no lifecycle hooks beyond animation

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::skeleton`
- GPUI color-mix: `color-mix(in srgb, X 88%, transparent)` maps to
  `color.opacity(color.a * 0.88)`
- GPUI linear-gradient: must compose 3-stop gradient for shimmer base
- Animation: GPUI must implement shimmer as a periodic background-position
  shift using `gpui::Animation` or equivalent
- GPUI must keep skeleton placeholders decorative and out of the accessible tree
- Preset layouts: GPUI should implement as compound components or factory
  methods that produce the correct child arrangement

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `aria-hidden="true"` on preset containers
- [ ] skeletons remain unfocusable and unannounced
- [ ] shape prop resolves correct defaults for width/height
- [ ] preset prop produces correct internal layout

### Tier 2: Visual Parity

- [ ] shimmer gradient 3-stop color-mix matches
- [ ] background-size 220% 100% matches
- [ ] animation duration 1.6s linear infinite matches
- [ ] keyframe positions 200% to -20% match
- [ ] shape radius: line uses `--poodle-radius-control`, circle uses `999rem`,
      block uses `calc(--poodle-radius-surface - 0.25rem)`
- [ ] all preset spacing (gap, padding) matches exactly
- [ ] table-row: 4 cells with 40%/60%/60%/20% widths
- [ ] card: block-header 6rem, body 3 lines at 80%/100%/60%, pill 3.5rem
- [ ] list-item: avatar 2.25rem circle, line 60%, line-sm 40%
- [ ] detail-section: heading 8rem, label 6rem, value max-width 14rem
- [ ] avatar-line: avatar 2.25rem + line 10rem

### Tier 3: Implementation Freedom

- [ ] shimmer implementation details (CSS vs programmatic) stay platform-owned
- [ ] preset composition model (compound component vs factory) is flexible

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| animation technique may differ | GPUI uses programmatic animation vs CSS keyframes | allowed | keep shimmer timing 1.6s linear |

## 13. Specimen Definitions

### Group: Basic shapes

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Line shape | `<Skeleton shape="line" width="12rem" />` | Rectangular placeholder bar, 12rem wide, default line height (0.875rem), with shimmer animation |
| Circle shape | `<Skeleton shape="circle" width="2.5rem" height="2.5rem" />` | Circular placeholder, 2.5rem diameter, with shimmer animation |
| Block shape | `<Skeleton shape="block" width="8rem" height="3rem" />` | Large rectangular placeholder block, 8rem wide by 3rem tall, with shimmer animation |

### Group: Preset: avatar-line

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Avatar-line preset | `<Skeleton preset="avatar-line" />` | Circle avatar placeholder (2.25rem) alongside a text line placeholder (10rem), arranged horizontally with 0.75rem gap |

### Group: Preset: list-item (x3)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Three list items | 3x `<Skeleton preset="list-item" />` | Three rows, each with a circle avatar (2.25rem), a primary line (60% width), and a secondary smaller line (40% width), stacked vertically |

### Group: Preset: table-row (x3)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Three table rows | 3x `<Skeleton preset="table-row" />` | Three horizontal rows, each with 4 cells at widths 40%/60%/60%/20%, separated by subtle bottom borders |

### Group: Preset: card

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Two cards | 2x `<Skeleton preset="card" />` in a 2-column grid | Two card placeholders side by side, each with a block header (6rem), three body lines (80%/100%/60%), and pill-shaped footer elements, all within a bordered card frame |

### Group: Preset: detail-section

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Detail section | `<Skeleton preset="detail-section" lines={4} />` | A heading placeholder (8rem) followed by 4 label-value rows, each with a fixed-width label (6rem) and a flexible value placeholder |

### Group: Static (no animation)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Static line | `<Skeleton shape="line" width="10rem" animated={false} />` | Line placeholder (10rem wide) with gradient fill but no shimmer animation |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: loading cards, list skeletons, table skeletons, detail
  view placeholders, avatar placeholders
- future follow-up: none expected
