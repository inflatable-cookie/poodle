# DetailItem

Status: detailed contract
Updated: 2026-05-17

## 1. Purpose

- Component name: `DetailItem`
- Layer: `foundation`
- Summary: a label/value pair for displaying read-only metadata within detail
  sections, supporting inline and stacked layouts, optional info description
  popovers, optional trailing action content, surface presentation for elevated cards,
  and density-driven spacing adjustments
- In scope: label/value display, optional custom value content via snippets,
  optional trailing action content, description popover (info icon with Popover), inline
  grid layout, stacked layout, surface presentation variant, responsive
  breakpoint collapse, full/half column span in parent grids
- Out of scope: section headers, inline editing, complex metadata composition,
  interactive values

## 2. Anatomy

```text
[Root .detail-item]  <div>
  ├── [Label Block .detail-item__label-block]  <div>
  │     └── [Label .detail-item__label]  <dt>
  │           ├── label text
  │           └── [Info Popover] (conditional, when description prop set)
  │                 └── [Info Trigger .detail-item__info-trigger]  <span>
  │                       └── [Info Icon .detail-item__info-icon]  <span>
  │                             └── Icon name="info"
  ├── [Value .detail-item__value]  <dd>
  │     ├── valueContent() OR children() OR text value OR emptyText
  └── [Action .detail-item__action]  <div> (conditional, when action snippet present)
        └── action()
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | grid container; layout varies by `layout` and `presentation` props | gap, grid-template-columns, padding, radius, background |
| Label Block | yes | flex wrapper for label and info icon | gap |
| Label | yes | semantic `<dt>` element for the metadata key | color, font-family, font-size, line-height |
| Info Popover | no | Popover component with info Icon, shown when `description` is set | background, border-radius, color |
| Info Trigger | no | inline-flex wrapper for the info icon button | alignment |
| Info Icon | no | circular badge containing the info icon | width, height, background, color, border-radius |
| Value | yes | semantic `<dd>` element for the metadata value | color, font-family, font-size, line-height, word-break |
| Action | no | container for action slot content (e.g. buttons, links) | none (container) |

### Snippets (Svelte)

| Snippet | Description |
|---------|-------------|
| `valueContent()` | Custom value content; replaces text value rendering |
| `children()` | Fallback custom value content when `valueContent()` is not used |
| `action()` | Optional trailing action content (for example a button or link) rendered in the action column |

## 3. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | explicit density override for layout spacing; when null, resolves from inherited presentation |
| `label` | `string` | -- | yes | visible detail label |
| `description` | `string \| null` | `null` | no | when set, renders an info icon Popover next to the label (matches Field pattern) |
| `value` | `string \| number \| null` | `null` | no | simple text value fallback |
| `emptyText` | `string` | `"—"` | no | display text when value is null and no slot content provided (em-dash) |
| `truncateValue` | `boolean` | `false` | no | truncates value with ellipsis overflow |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the root element |
| `layout` | `"inline" \| "stacked"` | `"inline"` | no | inline renders label and value side-by-side; stacked renders them vertically |
| `presentation` | `"simple" \| "surface"` | `"surface"` | no | surface adds elevated card-like styling with background and padding |
| `span` | `"full" \| "half" \| 1 \| 2 \| 3 \| 4 \| null` | `null` | no | `"full"` spans all columns via `grid-column: 1 / -1`; `"half"`/`2` span 2 columns; `3`/`4` span the named column count |
| `valueContent` | `Snippet` | -- | no | custom value content; replaces text value rendering (see Snippets table) |
| `action` | `Snippet` | -- | no | trailing action content rendered in the action column |
| `children` | `Snippet` | -- | no | fallback custom value content when `valueContent` is not used |

## 4. Behavior

- When `valueContent()` is present, it replaces the simple text value rendering
- When `children()` is present (and no `valueContent()` is present), children content is rendered
- When neither snippet is present, `value` prop text is displayed; if `value` is null, `emptyText` is shown
- When `truncateValue` is true, value text overflows with ellipsis
- When `description` is set, an info icon appears next to the label that opens a Popover on click
- When `span="full"`, root spans all columns in the parent grid
- When `action()` is present, an action column is rendered after the value
- When `density` is omitted, spacing resolves from inherited UI presentation density
- When `density` is set explicitly, root gap, inline column gap, label gap, and surface padding/gaps follow that density

### Responsive Behavior

The root establishes `container-type: inline-size`; collapse is driven by container queries, not viewport media queries.

At container widths at or below `26rem`, the inline layout collapses to a single column:
- `grid-template-columns` becomes `1fr`
- All children (label-block, value, action) return to auto grid-row placement
- Surface presentation (and surface + stacked) also collapses to single column
- Any `data-span` value collapses to `1 / -1`

At container widths at or below `21rem`, spacing tightens further (inline/surface column gap `0.5rem`, surface padding-x `0.75rem`) and label/value font-size step down (label `0.75rem`/lh `1.35`, value `0.9375rem`/lh `1.4`).

## 5. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty value | `value` is null and no slot content | `emptyText` displayed (default `"—"` em-dash) |
| truncated | `truncateValue=true` | value overflow hidden with ellipsis |
| info hover | pointer enters info icon | icon background darkens, icon color shifts to text-primary |
| info focus | keyboard focus on info trigger | focus ring around info icon |
| info open | click info icon | Popover appears with description text |
| compact density | `density="compact"` or inherited compact presentation | tighter row gap, inline column gap, and surface padding |
| comfortable density | `density="comfortable"` or inherited comfortable presentation | looser row gap, inline column gap, and surface padding |

## 6. Accessibility

- Renders semantic `<dt>` and `<dd>` pairs within a description list context
- `ariaLabel` prop sets `aria-label` on the root element when provided
- Info icon Popover has `ariaLabel="More information"` on both the Popover and the icon span
- Info icon trigger receives keyboard focus; `focus-visible` shows focus ring
- Popover trigger `focus-visible` outline is suppressed (the info icon handles its own ring)

## 7. Layout

### Inline Layout (`layout="inline"`)

- Grid layout: `grid-template-columns: minmax(8rem, 11.25rem) minmax(0, 1fr) auto`
- Gap:
  - `compact`: `0.1875rem` row gap, `var(--poodle-space-inline-sm)` column gap
  - `default`: `0.25rem` row gap, `var(--poodle-space-inline-md)` column gap
  - `comfortable`: `0.3125rem` row gap, `0.875rem` column gap
- All three children (label-block, value, action) placed on `grid-row: 1`
- Alignment: `baseline`

### Stacked Layout (`layout="stacked"`)

- Grid layout: single column (default)
- Gap:
  - `compact`: `0.1875rem`
  - `default`: `0.25rem`
  - `comfortable`: `0.3125rem`
- Label and value stack vertically

### Surface Presentation (`presentation="surface"`)

- Grid layout: `grid-template-columns: 11.25rem minmax(0, 1fr) auto`
- Gap:
  - `compact`: `var(--poodle-space-inline-sm)`
  - `default`: `var(--poodle-space-inline-md)`
  - `comfortable`: `0.875rem`
- Alignment: `center`
- Padding:
  - `compact`: `0.5rem 0.75rem`
  - `default`: `0.625rem var(--poodle-space-panel-x)`
  - `comfortable`: `0.75rem 1rem`
- Border radius: `calc(var(--poodle-radius-surface) - 0.0625rem)`
- Background: `color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary))`

### Surface + Stacked

- Grid layout: `grid-template-columns: minmax(0, 1fr) auto`
- Alignment: `start`
- Gap:
  - `compact`: `0.125rem`
  - `default`: `0.1875rem`
  - `comfortable`: `0.25rem`
- Label block spans all columns (`grid-column: 1 / -1`)
- Label color shifts to `var(--poodle-color-text-tertiary)`, font-size `0.75rem`, line-height `1.35`
- Value font-size increases to `1rem`, font-weight to `600`

### Responsive (container queries)

At `@container (max-width: 26rem)` all layout modes collapse to `grid-template-columns: 1fr` with auto row placement and any span collapses to `1 / -1`. At `@container (max-width: 21rem)` column gaps and surface padding-x tighten and label/value font-sizes step down.

### Composition

- Parent expectations: detail sections, metadata grids, settings panels
- Child expectations: text values, custom snippet content, optional action buttons
- Resizing: stretches to parent width

## 8. Token Usage -- Exact Values

### Root `.detail-item`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.25rem` |

### Root -- inline layout `.detail-item[data-layout="inline"]`

| Property | Value |
|----------|-------|
| `grid-template-columns` | `minmax(8rem, 11.25rem) minmax(0, 1fr) auto` |
| `gap` | `0.25rem var(--poodle-space-inline-md)` |
| `align-items` | `baseline` |

### Root -- full span `.detail-item[data-span="full"]`

| Property | Value |
|----------|-------|
| `grid-column` | `1 / -1` |

### Label Block `.detail-item__label-block`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `baseline` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-width` | `0` |

### Label `.detail-item__label`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `line-height` | `var(--poodle-typography-label-lineHeight)` |

### Value `.detail-item__value`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |
| `word-break` | `break-word` |
| `margin` | `0` |

### Value -- truncated `.detail-item__value.truncate`

| Property | Value |
|----------|-------|
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |
| `white-space` | `nowrap` |

### Info Icon `.detail-item__info-icon`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.25em` |
| `height` | `1.25em` |
| `border-radius` | `var(--poodle-radius-pill)` |
| `background` | `color-mix(in srgb, var(--poodle-color-text-secondary) 14%, transparent)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |
| `flex-shrink` | `0` |
| `transition` | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Info Icon SVG

| Property | Value |
|----------|-------|
| `width` | `0.75em` |
| `height` | `0.75em` |

### Info Icon -- hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-text-secondary) 26%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |

### Info Icon -- focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

### Info Popover Surface

| Property | Value |
|----------|-------|
| `min-width` | `10rem` |
| `max-width` | `22rem` |
| `padding` | `0.5rem 0.625rem` |

### Info Content `.detail-item__info-content`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `0.75rem` |
| `line-height` | `1.5` |

### Surface Presentation `.detail-item[data-presentation="surface"]`

| Property | Value |
|----------|-------|
| `grid-template-columns` | `11.25rem minmax(0, 1fr) auto` |
| `gap` | `var(--poodle-space-inline-md)` |
| `align-items` | `center` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border-radius` | `calc(var(--poodle-radius-surface) - 0.0625rem)` |
| `background` | `color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary))` |

### Surface + Stacked

| Property | Value |
|----------|-------|
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `align-items` | `start` |
| label-block `grid-column` | `1 / -1` |
| label `color` | `var(--poodle-color-text-tertiary)` |
| label `font-size` | `0.75rem` |
| label `line-height` | `1.35` |
| value `font-size` | `1rem` |
| value `font-weight` | `600` |

## 9. Svelte Notes

- Uses `data-layout`, `data-presentation`, and `data-span` attributes on the root element
- Description info icon follows the same Popover + Icon pattern used in `Field`
- The Popover uses `placement="top"` and `offset={6}`
- Value rendering priority: `valueContent` snippet > `children` snippet > `value` prop text > `emptyText`
- Root sets `container-type: inline-size`; responsive collapse uses `@container (max-width: 26rem)` and a tighter `@container (max-width: 21rem)` step, not viewport media queries

## 10. Parity Checklist

### Tier 1: Strict Parity

- [ ] label and value rendering matches
- [ ] description popover behavior matches
- [ ] layout modes (inline/stacked) match
- [ ] presentation modes (simple/surface) match
- [ ] span prop behavior matches

### Tier 2: Visual Parity

- [ ] inline grid columns match (`minmax(8rem, 11.25rem) minmax(0, 1fr) auto`)
- [ ] label typography matches (label-family, label-size, text-secondary)
- [ ] value typography matches (body-family, body-size, text-primary)
- [ ] info icon styling matches (pill radius, secondary color mix)
- [ ] surface presentation background matches (93% surface mix)
- [ ] responsive breakpoint at 45rem matches

### Tier 3: Implementation Freedom

- [ ] popover implementation details are platform-owned
- [ ] responsive behavior may differ by platform
- [ ] transition timing is platform-owned

## 11. Specimen Definitions

All preview apps must render the following specimens identically.

### Simple inline

Two detail items in inline layout:

| Label | Value | Description |
|-------|-------|-------------|
| Status | Active | null |
| Created | 2026-01-15 | "Date the record was created" |

### Surface presentation

One detail item with surface presentation:

| Label | Value | Layout | Presentation |
|-------|-------|--------|-------------|
| Total | 42 | inline | surface |

### With action slot

One detail item with an action button:

| Label | Value | Action |
|-------|-------|--------|
| API Key | sk-... | Copy button |

## 12. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: detail sections, settings panels, metadata views
- future follow-up: consider validation state if needed for editable detail items
