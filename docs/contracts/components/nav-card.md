# NavCard

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `NavCard`
- Layer: `foundation`
- Summary: a navigational card that links to a destination with icon, title,
  optional badge, description, and directional arrow
- In scope: link and button rendering, badge display, hover arrow reveal,
  disabled state, icon snippet
- Out of scope: card grids (see NavCardGrid), nested navigation, breadcrumb
  integration

## 2. Anatomy

```text
[Root .nav-card]  <a> or <button>
  ├── [Icon .nav-card__icon]  (optional, via icon snippet)
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
| Icon | no | visual indicator snippet region | width, height, border-radius, background, color |
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
| `disabled` | `boolean` | `false` | no | disables interaction |
| `ariaLabel` | `string \| null` | `null` | no | accessible name |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | overrides inherited UI presentation density for card padding, icon box size, and internal spacing |
| `onClick` | `((event: MouseEvent) => void) \| null` | `null` | no | called when the card is activated; suppressed while disabled |

### Snippets

| Snippet | Purpose |
|---------|---------|
| `icon()` | custom icon content for leading indicator |

### Controlled And Uncontrolled

- Navigation component; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | subtle border, surface background, arrow hidden |
| hover | pointer enters | accent-tinted border, elevated background, arrow visible |
| focus | keyboard focus | accent focus ring |
| disabled | `disabled=true` | reduced opacity, not-allowed cursor |

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|----------|--------------|---------|-------|
| `onClick` | card activated | `MouseEvent` | suppressed while disabled; anchor roots still use native navigation |

## 6. Accessibility

### Semantics

- When `href` provided (and not disabled): renders as `<a>` with `href`
- When no `href` or disabled: renders as `<button>`
- `aria-label`: from prop when provided, otherwise derived from title
- When disabled: `aria-disabled="true"`, `href` removed for `<a>` rendering

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | activates card (navigates for links, calls `onClick` for buttons) |
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
- Density affects padding, row gaps, and icon box size; typography stays fixed

### Composition

- parent expectations: NavCardGrid, navigation panels, settings pages
- child expectations: icon via `icon()` snippet
- resizing: fills parent width
- hierarchy guidance: keep `title` as the leaf destination label; do not embed
  breadcrumb chains or multi-segment hierarchy strings in the title
- if parent context is needed, prefer `description` for one short supporting
  line rather than breadcrumb-style title composition
- if the UI needs true breadcrumb or hierarchy treatment with dimmed ancestors
  and chevrons, use `PageHeader` or `ListCard`, not `NavCard`

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.75rem` |
| `padding` | `0.875rem 1rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `inherit` |
| `cursor` | `pointer` |
| `text-decoration` | `none` |
| `text-align` | `left` |
| `font` | `inherit` |
| `width` | `100%` |
| `transition` | `background, border-color, box-shadow` at `motion-duration-interaction motion-easing-standard` |

### Density Overrides

| `data-density` | Root Gap | Root Padding | Icon Size | Content Gap | Title Gap |
|----------------|----------|--------------|-----------|-------------|-----------|
| `compact` | `0.625rem` | `0.5rem 0.75rem` | `1.75rem` | `0.0625rem` | `0.3125rem` |
| `default` | `0.75rem` | `0.625rem var(--poodle-space-panel-x)` | `2rem` | `0.125rem` | `0.375rem` |
| `comfortable` | `0.875rem` | `0.75rem 1.25rem` | `2.25rem` | `0.1875rem` | `0.4375rem` |

### Root hover

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 28%, var(--poodle-color-border-subtle))` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 52%, var(--poodle-color-background-surface))` |

### Root focus

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

### Root disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Icon

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `flex-shrink` | `0` |
| `width` | `2rem` |
| `height` | `2rem` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |
| `color` | `var(--poodle-color-accent-base)` |
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
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `600` |
| `color` | `var(--poodle-color-text-primary)` |

### Badge

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `padding` | `0.0625rem 0.375rem` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent)` |
| `color` | `var(--poodle-color-accent-base)` |
| `font-size` | `0.625rem` |
| `font-weight` | `600` |
| `text-transform` | `uppercase` |
| `letter-spacing` | `0.05em` |

### Description

| Property | Value |
|----------|-------|
| `font-size` | `0.8125rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `line-height` | `1.4` |

### Arrow

| Property | Value |
|----------|-------|
| `flex-shrink` | `0` |
| `width` | `1rem` |
| `height` | `1rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `opacity` | `0` |
| `transition` | `opacity` at `motion-duration-interaction motion-easing-standard` |

### Arrow (root hover)

| Property | Value |
|----------|-------|
| `opacity` | `1` |

## 9. Svelte Notes

- Renders `<a>` when `href` provided and not disabled, otherwise `<button>`
- `data-disabled` attribute on root when disabled
- `density` resolves from UI presentation context when not provided explicitly
- Arrow SVG is always in DOM, opacity-toggled on hover
- Badge rendered inline within title row when badge prop is non-null
- `onClick` handler on root; `<a>` still navigates natively when `href` is present

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::nav_card`
- Spec struct: `NavCardSpec` in primitives crate
- Component struct: `PoodleNavCard` in components crate
- Link vs button rendering maps to different GPUI element types
- Arrow opacity animation may use GPUI's animation system
- Badge pill styling matches standalone Pill component patterns

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] renders as link when href provided, button otherwise
- [ ] disabled state suppresses interaction and removes href
- [ ] `onClick` runs correctly
- [ ] keyboard activation matches

### Tier 2: Visual Parity

- [ ] padding and gap match
- [ ] border and border-radius match
- [ ] hover border and background match
- [ ] focus ring matches
- [ ] icon snippet default styling matches
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

## 13. Specimen Definitions

### Navigation Card Grid (2 Columns)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Getting Started | `title`, `description`, icon snippet (home), click handler | NavCard with icon, title, description, arrow hidden until hover |
| Components | `title`, `description`, `badge="New"`, icon snippet (layers), click handler | NavCard with icon, title, inline badge, description |
| Tokens | `title`, `description`, icon snippet (sliders-horizontal), click handler | NavCard with icon, title, description |
| API Reference | `title`, `description`, `disabled`, icon snippet (file-text) | Disabled NavCard at reduced opacity with not-allowed cursor |

Cards arranged in a 2-column NavCardGrid.

### Single Card (As Link)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| View Documentation | `title`, `description`, `href="#"` | NavCard rendered as `<a>` element with link behavior |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: NavCardGrid, navigation panels, settings pages, documentation hubs
- future follow-up: external link indicator, nested navigation support
