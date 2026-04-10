# PageHeader

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `PageHeader`
- Layer: `composites`
- Summary: a standardized title and action region for product pages, panels, or detail surfaces
- In scope: optional back link with contextual indicator, optional breadcrumbs region, title with optional count badge, section/title hierarchy, eyebrow label, subtitle, body slot, primary/secondary actions, optional banner region via slot or shortcut prop, configurable heading level
- Out of scope: app shell toolbars, global navigation, domain-specific command wiring

## 2. Anatomy

```text
[Root .page-header]  <header> data-align, aria-label
  ├── [Content .page-header__content]  <div>
  │     ├── [Breadcrumbs .page-header__breadcrumbs]  <div> (optional, breadcrumbs slot)
  │     └── [TitleBlock .page-header__title-block]  <div>
  │           ├── [Eyebrow .page-header__eyebrow]  <p> (optional)
  │           ├── [Section .page-header__section]  <p> (optional, when section + title)
  │           ├── [Title .page-header__title]  <h2..h6> (configurable level)
  │           │     ├── [TitleText]  <span>
  │           │     └── [Count .page-header__count]  <span> (optional, when count !== null)
  │           ├── [SectionTitle .page-header__section-title]  <p> (optional, when section + title)
  │           ├── [Subtitle .page-header__subtitle]  <p> (optional)
  │           ├── [Meta .page-header__meta]  <div> (optional, meta slot)
  │           └── [Body .page-header__body]  <div> (optional, default slot)
  ├── [ActionsRow .page-header__actions-row]  <div> (optional, when backHref or actions)
  │     ├── [BackLink .page-header__back]  <a> (optional, when backHref)
  │     │     ├── [BackIcon]  Icon "arrow-left"
  │     │     ├── [BackLabel]  <span> visible label text (defaults to "Back")
  │     │     └── [ContextDot .page-header__context-dot]  <span> (optional, when backIsContextual)
  │     └── [Actions .page-header__actions]  <div> (optional, actions slot; pushed right via margin-left: auto)
  └── [Banner .page-header__banner]  <div> (optional, banner slot or bannerMessage prop)
        └── [Callout]  Callout primitive (when bannerMessage shortcut used)
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Root | `<header>` | yes | Grid layout, `data-align` attribute |
| Content | `<div>` | yes | Grid container for breadcrumbs and title block |
| Breadcrumbs | `<div>` | no | Named slot for breadcrumb navigation |
| TitleBlock | `<div>` | yes | Contains eyebrow, section, title, subtitle, meta, body |
| Eyebrow | `<p>` | no | Small meta label above title |
| Section | `<p>` | no | Section label when two-level header (section + title both set) |
| Title | `<h2..h6>` | yes | Primary heading, level set via `level` prop |
| Count | `<span>` | no | Badge inline with title, shown when `count !== null` |
| SectionTitle | `<p>` | no | Secondary title below main heading in two-level header |
| Subtitle | `<p>` | no | Supporting copy below title |
| Meta | `<div>` | no | Named slot for metadata content below subtitle |
| Body | `<div>` | no | Default slot content below meta |
| ActionsRow | `<div>` | no | Flex row containing back link and actions; rendered when `backHref` or actions slot present |
| BackLink | `<a>` | no | Shown when `backHref` is set; renders arrow-left Icon + label text |
| BackIcon | `Icon` | no | Arrow-left icon inside the back link |
| BackLabel | `<span>` | no | Visible text next to the arrow icon (defaults to `"Back"`) |
| ContextDot | `<span>` | no | Shown when `backIsContextual` is true; `aria-hidden="true"` |
| Actions | `<div>` | no | Page-level action group via named slot; pushed right via `margin-left: auto` |
| Banner | `<div>` | no | Full-width banner spanning all grid columns |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | Primary heading text |
| `section` | `string \| null` | `null` | no | Optional section label; when both `section` and `title` are set, creates a two-level header |
| `count` | `number \| null` | `null` | no | Optional count badge rendered inline with the title |
| `subtitle` | `string \| null` | `null` | no | Supporting copy |
| `eyebrow` | `string \| null` | `null` | no | Small meta label |
| `backHref` | `string \| null` | `null` | no | Optional back-link target |
| `backLabel` | `string \| null` | `null` | no | Visible text label next to the arrow-left icon in the back link; defaults to `"Back"` when `backHref` is set |
| `backIsContextual` | `boolean` | `false` | no | Adds the contextual indicator dot on the back link |
| `bannerMessage` | `string \| null` | `null` | no | Shortcut banner message rendered via Callout below the header |
| `bannerTone` | `"neutral" \| "info" \| "success" \| "warning" \| "danger"` | `"warning"` | no | Tone for the shortcut banner |
| `align` | `"start" \| "between"` | `"between"` | no | Action alignment posture |
| `ariaLabel` | `string \| null` | `null` | no | Optional region label |
| `level` | `1 \| 2 \| 3 \| 4 \| 5 \| 6` | `2` | no | Heading level for the title element |

### Slots

| Slot | Purpose |
|------|---------|
| default | Body content rendered below subtitle in the title block |
| `breadcrumbs` | Pre-title navigation trail |
| `meta` | Metadata content rendered below subtitle in the title block |
| `actions` | Page-level action buttons (rendered in actions-row, pushed right) |
| `banner` | Custom banner content (overrides `bannerMessage` shortcut) |

### Controlled And Uncontrolled

- Declarative composite
- Actions, breadcrumbs, and banner content remain host-owned children

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| simple | title only | Compact title row |
| sectional | `section` and `title` both set | Two-level header hierarchy: section label, primary title, section title |
| descriptive | subtitle or eyebrow present | Expanded title block |
| with-count | `count` is not null | Count badge pill inline with title |
| actionable | actions slot present | Title and actions share row (between alignment) or stack |
| with-back | `backHref` set | Back link with arrow-left icon in actions-row alongside action buttons |
| with-banner | `bannerMessage` or banner slot | Full-width banner below header body |

### Component States

| State | Description |
|-------|-------------|
| `primaryTitle` (derived) | `title ?? section ?? ""` |
| `hasSectionTitleSplit` (derived) | `Boolean(section && title)` |
| `headingTag` (derived) | `h${level}` |

## 5. Events

No component-owned events beyond child action behavior.

## 6. Accessibility

### Semantics

- Root: `<header>` element with optional `aria-label`
- Title: rendered as `<h2>` by default (configurable via `level` prop)
- Count badge: `aria-label` with the count value
- Back link: `<a>` element containing arrow-left `Icon` and label `<span>`
- Context dot: `aria-hidden="true"`
- Banner via shortcut: Callout with `announceMode="polite"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | Reaches back link, breadcrumb links, banner actions, and header actions in logical order |

### Focus And Announcement

- The header itself is not focusable by default
- Back link, breadcrumb links, and actions should not break heading readability
- Banner via Callout uses `aria-live="polite"`

## 7. Layout

### Sizing

- Root: CSS grid with `gap: var(--poodle-space-stack-md)`
- `align="between"`: `grid-template-columns: minmax(0, 1fr) auto`
- `align="start"`: single-column grid
- Responsive: at `max-width: 45rem`, between alignment collapses to single column
- Title block: internal grid with `gap: var(--poodle-space-inline-sm)`
- Banner: `grid-column: 1 / -1` (spans full width)

### Composition

- Composes: `Callout` primitive (for banner shortcut)
- Parent expectations: detail shells, settings pages, list/grid surfaces, product panels
- Child expectations: breadcrumb navigation, action buttons via slots
- Resizing rules: title remains visually dominant over actions

## 8. Token Usage -- Exact Values

### Recipe Custom Properties

| Property | Default |
|----------|---------|
| `--poodle-recipe-page-header-padding-block-start` | `0` |
| `--poodle-recipe-page-header-padding-inline` | `0` |
| `--poodle-recipe-page-header-padding-block-end` | `calc(var(--poodle-space-stack-md) + 0.125rem)` |
| `--poodle-recipe-page-header-fill` | `transparent` |
| `--poodle-recipe-page-header-border` | `transparent` |
| `--poodle-recipe-page-header-shadow` | `none` |
| `--poodle-recipe-page-header-radius` | `var(--poodle-radius-surface)` |

#### `.page-header` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-md)` |
| `align-items` | `end` |
| `padding` | `var(--poodle-recipe-page-header-padding-block-start) var(--poodle-recipe-page-header-padding-inline) var(--poodle-recipe-page-header-padding-block-end)` |
| `border` | `0.0625rem solid var(--poodle-recipe-page-header-border)` |
| `border-radius` | `var(--poodle-recipe-page-header-radius)` |
| `background` | `var(--poodle-recipe-page-header-fill)` |
| `box-shadow` | `var(--poodle-recipe-page-header-shadow)` |

#### `.page-header[data-align="between"]`

| Property | Value |
|----------|-------|
| `grid-template-columns` | `minmax(0, 1fr) auto` |

#### `.page-header__content`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-md)` |

#### `.page-header__actions-row`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-md)` |

#### `.page-header__back`

| Property | Value |
|----------|-------|
| `width` | `fit-content` |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.35rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.2` |
| `text-decoration` | `none` |

The back link renders an `Icon` with `name="arrow-left"` followed by a `<span>` containing the `backLabel` text (defaulting to `"Back"`). When `backIsContextual` is true, a context dot is appended.

#### `.page-header__back:hover`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

#### `.page-header__context-dot`

| Property | Value |
|----------|-------|
| `width` | `0.375rem` |
| `height` | `0.375rem` |
| `border-radius` | `999px` |
| `background` | `var(--poodle-color-fill-info-strong, var(--poodle-color-border-info))` |
| `flex` | `none` |

#### `.page-header__title-block`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-inline-sm)` |

#### `.page-header__title`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.5rem` |
| `font-family` | `var(--poodle-typography-heading-family)` |
| `font-size` | `1.75rem` |
| `line-height` | `1.1` |
| `font-weight` | `700` |

#### `.page-header__section`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.75rem` |
| `font-weight` | `700` |
| `letter-spacing` | `0.08em` |
| `text-transform` | `uppercase` |

#### `.page-header__section-title`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-heading-family)` |
| `font-size` | `1rem` |
| `line-height` | `1.25` |
| `font-weight` | `600` |

#### `.page-header__count`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-width` | `1.75rem` |
| `min-height` | `1.75rem` |
| `padding` | `0 0.5rem` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-color-fill-secondary) 72%, transparent)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.875rem` |
| `font-weight` | `600` |
| `line-height` | `1` |

#### `.page-header__eyebrow`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.12em` |
| `text-transform` | `uppercase` |

#### `.page-header__subtitle`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

#### `.page-header__body`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

#### `.page-header__meta`

| Property | Value |
|----------|-------|
| `margin-top` | `0.125rem` |

#### `.page-header__actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `0.375rem` |
| `align-items` | `center` |
| `margin-left` | `auto` |

#### `.page-header__banner`

| Property | Value |
|----------|-------|
| `grid-column` | `1 / -1` |

### Responsive Breakpoints

#### `@media (max-width: 45rem)` -- `.page-header[data-align="between"]`

| Property | Value |
|----------|-------|
| `grid-template-columns` | `1fr` |

#### `@media (max-width: 45rem)` -- `.page-header__actions-row`

| Property | Value |
|----------|-------|
| `flex-wrap` | `wrap` |

#### `@media (max-width: 45rem)` -- `.page-header__actions`

| Property | Value |
|----------|-------|
| `margin-left` | `0` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-align` | root `<header>` | Controls grid column layout (`"start"` or `"between"`) |

## 9. Svelte Notes

- Uses Svelte 5 `$props()` syntax with `Props` interface
- Composes `Callout` and `Icon` primitives from `@poodle/svelte-primitives` for banner shortcut and back link arrow
- `primaryTitle` derived as `title ?? section ?? ""`
- `hasSectionTitleSplit` derived as `Boolean(section && title)`
- Heading tag is dynamic via `<svelte:element this={headingTag}>`
- Banner slot takes priority over `bannerMessage` prop (if both provided, slot wins)
- Back link and actions are grouped in a shared actions-row; back link sits left, actions push right via `margin-left: auto`
- At tablet breakpoint (max-width: 45rem), actions-row wraps and actions lose their auto margin

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::page_header`
- GPUI heading semantics may use named region and text hierarchy APIs rather than HTML headings
- Accessible structure must be explicit even without native heading elements

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] title hierarchy and heading semantics match
- [ ] breadcrumbs and actions remain logically grouped and ordered
- [ ] section/title split produces correct two-level hierarchy
- [ ] count badge renders inline with title
- [ ] back link with contextual dot works correctly
- [ ] banner renders via slot or prop shortcut

### Tier 2: Visual Parity

- [ ] spacing, heading weight, and action alignment use comparable token roles
- [ ] count badge pill styling matches
- [ ] section and section-title typography matches
- [ ] responsive breakpoint behavior matches

### Tier 3: Implementation Freedom

- [ ] wrap strategy and layout breakpoints stay internal

## 12. Specimen Definitions

### Basic

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Basic | `title="Components"`, `subtitle="Browse and manage your component library."` | Title with subtitle text below |

### With Eyebrow And Actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With eyebrow and actions | `title="Button"`, `eyebrow="Primitive"`, `subtitle="Primary interactive control for triggering actions."`, actions slot with secondary "View source" button and primary "Edit" button (both `size="sm"`) | Eyebrow above title, subtitle below, action buttons trailing |

### Title Only

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Title only | `title="Settings"` | Compact title row with no subtitle, no eyebrow, no actions |
