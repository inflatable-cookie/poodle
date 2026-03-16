# NavCard

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `NavCard`
- Layer: `foundation`
- Summary: a navigational card that links to a destination with icon, title,
  optional badge, description, and directional arrow
- In scope: link and button rendering, badge display, hover arrow reveal,
  disabled state, icon slot
- Out of scope: card grids (see NavCardGrid), nested navigation, breadcrumb
  integration

## 2. Anatomy

```text
[Root .nav-card]  <a> or <button>
  ├── [Icon .nav-card__icon]  (optional, via icon slot)
  ├── [Content .nav-card__content]  <div>
  │   ├── [Title .nav-card__title]  <span>
  │   │   ├── [Title text]
  │   │   └── [Badge .nav-card__badge]  <span> (optional)
  │   └── [Description .nav-card__description]  <span> (optional)
  └── [Arrow .nav-card__arrow]  <svg>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | link or button container | padding, border, radius, background, cursor |
| Icon | no | visual indicator slot | width, height, border-radius, background, color |
| Content | yes | title, badge, and description column | flex, gap |
| Title | yes | primary navigation label | font, color, weight |
| Badge | no | inline status indicator | padding, border-radius, background, color, font-size |
| Description | no | supplementary context text | font-size, color, line-height |
| Arrow | yes | directional indicator (reveals on hover) | width, height, color, opacity, transition |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | — | yes | navigation destination label |
| `description` | `string \| null` | `null` | no | supplementary description |
| `href` | `string \| null` | `null` | no | link destination; renders as `<a>` when provided |
| `badge` | `string \| null` | `null` | no | inline badge text |
| `isDisabled` | `boolean` | `false` | no | disables interaction |
| `ariaLabel` | `string \| null` | `null` | no | accessible name |

### Slots

| Slot | Purpose |
|------|---------|
| icon | custom icon content for leading indicator |

### Controlled And Uncontrolled

- Navigation component; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | subtle border, surface background, arrow hidden |
| hover | pointer enters | accent-tinted border, elevated background, arrow visible |
| focus | keyboard focus | accent focus ring |
| disabled | `isDisabled=true` | reduced opacity, not-allowed cursor |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `click` | card activated | `MouseEvent` | suppressed while disabled; native navigation for `<a>` |

## 6. Accessibility

### Semantics

- When `href` provided (and not disabled): renders as `<a>` with `href`
- When no `href` or disabled: renders as `<button>`
- `aria-label`: from prop when provided, otherwise derived from title
- When disabled: `aria-disabled="true"`, `href` removed for `<a>` rendering

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | activates card (navigates for links, fires click for buttons) |
| `Space` | activates card (button mode only) |
| `Tab` | moves focus to/from card |

### Focus And Announcement

- focus entry: card root receives visible focus ring
- focus exit: focus ring clears immediately

## 7. Layout

### Sizing

- Root: flex row, fills parent width
- Icon: fixed 2rem square
- Content: flex 1, min-width 0
- Arrow: fixed 1rem square, right-aligned

### Composition

- parent expectations: NavCardGrid, navigation panels, settings pages
- child expectations: icon via slot
- resizing: fills parent width

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.75rem` |
| `padding` | `0.875rem 1rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 32%, transparent)` |
| `border-radius` | `var(--pug-radius-surface)` |
| `background` | `var(--pug-color-background-surface)` |
| `color` | `inherit` |
| `cursor` | `pointer` |
| `text-decoration` | `none` |
| `text-align` | `left` |
| `font` | `inherit` |
| `width` | `100%` |
| `transition` | `background, border-color, box-shadow` at `motion-duration-interaction motion-easing-standard` |

### Root hover

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--pug-color-accent-base) 28%, var(--pug-color-border-subtle))` |
| `background` | `color-mix(in srgb, var(--pug-color-background-elevated) 52%, var(--pug-color-background-surface))` |

### Root focus

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

### Root disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Icon slot

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `flex-shrink` | `0` |
| `width` | `2rem` |
| `height` | `2rem` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 12%, transparent)` |
| `color` | `var(--pug-color-accent-base)` |
| `font-size` | `1rem` |

### Content

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `min-width` | `0` |
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.125rem` |

### Title

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `var(--pug-typography-label-size)` |
| `font-weight` | `600` |
| `color` | `var(--pug-color-text-primary)` |

### Badge

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `padding` | `0.0625rem 0.375rem` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent)` |
| `color` | `var(--pug-color-accent-base)` |
| `font-size` | `0.625rem` |
| `font-weight` | `600` |
| `text-transform` | `uppercase` |
| `letter-spacing` | `0.05em` |

### Description

| Property | Value |
|----------|-------|
| `font-size` | `0.8125rem` |
| `color` | `var(--pug-color-text-secondary)` |
| `line-height` | `1.4` |

### Arrow

| Property | Value |
|----------|-------|
| `flex-shrink` | `0` |
| `width` | `1rem` |
| `height` | `1rem` |
| `color` | `var(--pug-color-text-secondary)` |
| `opacity` | `0` |
| `transition` | `opacity` at `motion-duration-interaction motion-easing-standard` |

### Arrow (root hover)

| Property | Value |
|----------|-------|
| `opacity` | `1` |

## 9. Svelte Notes

- Renders `<a>` when `href` provided and not disabled, otherwise `<button>`
- `data-disabled` attribute on root when disabled
- Arrow SVG is always in DOM, opacity-toggled on hover
- Badge rendered inline within title row when badge prop is non-null
- Click event handler on root; `<a>` also navigates natively

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::nav_card`
- Spec struct: `NavCardSpec` in primitives crate
- Component struct: `PugNavCard` in components crate
- Link vs button rendering maps to different GPUI element types
- Arrow opacity animation may use GPUI's animation system
- Badge pill styling matches standalone Pill component patterns

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] renders as link when href provided, button otherwise
- [ ] disabled state suppresses interaction and removes href
- [ ] click event fires correctly
- [ ] keyboard activation matches

### Tier 2: Visual Parity

- [ ] padding and gap match
- [ ] border and border-radius match
- [ ] hover border and background match
- [ ] focus ring matches
- [ ] icon slot default styling matches
- [ ] badge appearance matches (pill, accent colors)
- [ ] description typography matches
- [ ] arrow opacity transition on hover matches
- [ ] disabled opacity matches

### Tier 3: Implementation Freedom

- [ ] transition timing is platform-owned
- [ ] link vs button element is platform-appropriate

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Arrow opacity animation | GPUI may use different animation approach | allowed | same visual result |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: NavCardGrid, navigation panels, settings pages, documentation hubs
- future follow-up: external link indicator, nested navigation support
