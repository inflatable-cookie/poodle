# FilterToolbar

Status: seed contract
Updated: 2026-03-21

## 1. Purpose

- Component name: `FilterToolbar`
- Layer: `composites`
- Summary: a compact, optionally collapsible control strip for search, filtering,
  sorting, and result summary affordances above lists or grids
- In scope: search and filter control grouping, responsive grid layout, collapsible
  state, result summary, action buttons, secondary actions
- Out of scope: result data ownership, domain-specific filter logic, active filter
  pill management, command palette behavior

## 2. Anatomy

```text
[Root Toolbar]
  ├── [Header]
  │     ├── [CollapseToggle] (optional)
  │     ├── [Summary] (optional)
  │     └── [Actions] (optional)
  ├── [Controls Grid] (hidden when collapsed)
  └── [Secondary Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Toolbar | yes | control grouping container | surface, spacing, border, radius |
| Header | yes | row containing toggle, summary, and actions | gap, alignment |
| CollapseToggle | no | chevron toggle to show/hide controls | via CollapseToggle primitive |
| Summary | no | result count or active-filter summary text | typography, text-secondary |
| Actions | no | icon buttons (refresh, etc.) aligned right | gap |
| Controls Grid | yes | responsive grid of filter controls | grid columns, gap |
| Secondary Actions | no | clear, create, export, or related actions | gap |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `ariaLabel` | `string` | `"Filters"` | no | toolbar/group label |
| `summaryText` | `string \| null` | `null` | no | visible summary copy in header |
| `collapsible` | `boolean` | `false` | no | whether controls can be collapsed |
| `collapsed` | `boolean` | `false` | no | current collapsed state (bindable) |
| `columns` | `number` | `4` | no | number of grid columns at full width |
| `minItemWidth` | `string` | `"10rem"` | no | minimum width per grid item |
| `isSticky` | `boolean` | `false` | no | sticky positioning when host supports it |

### Slots

| Slot | Purpose |
|------|---------|
| default | filter controls (SearchField, Select, etc.) placed in the grid |
| `actions` | icon buttons in the header row (refresh, settings, etc.) |
| `secondary` | trailing actions below the grid (reset, export, etc.) |

### Controlled And Uncontrolled

- `collapsed` is bindable for two-way control
- filter state is fully host-owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| expanded | default or `collapsed=false` | header + controls grid visible |
| collapsed | `collapsible=true`, `collapsed=true` | header only, controls hidden |
| summarized | `summaryText` present | summary text in header row |
| with-actions | actions slot populated | icon buttons right-aligned in header |
| sticky | `isSticky=true` | sticky positioning with elevation |

### Component States

State table is sufficient.

## 5. Events

No component-owned events beyond child control behavior and collapse toggle.

## 6. Accessibility

### Semantics

- Role: `toolbar` with accessible label
- Required attributes: `aria-label` for the control group
- Optional attributes: summary description relationship when helpful
- Labeling rules: the summary supplements the toolbar; it does not replace the
  accessible label

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through contained controls in DOM order |
| `Enter`/`Space` | toggles collapse when CollapseToggle is focused |
| toolbar-internal arrows | only where child controls define them |

### Focus And Announcement

- focus entry: the toolbar container is not focusable by default
- focus exit: sticky positioning must not create duplicate focus order
- live-region behavior: summary changes may be announced only when the host
  decides the result update is material
- collapse toggle: managed by CollapseToggle primitive accessibility

## 7. Layout

### Sizing

- controls grid uses CSS grid with responsive breakpoints:
  - full width: `columns` columns (default 4)
  - ≤960px: 2 columns
  - ≤640px: 1 column
- each grid item has a minimum width of `minItemWidth` (default `10rem`)
- summary and actions remain in the header row at all widths

### Composition

- parent expectations: browser-style pages, settings views with search/filter
  affordances
- child expectations: search fields, selects, segmented controls, buttons,
  icon buttons
- resizing rules: primary controls remain first in reading and focus order

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Toolbar | `border-subtle`, `radius-surface`, `background-elevated` | grouping shell |
| Header | `space-inline-sm` | header row gap |
| Summary | `text-secondary`, `label-size` | result metadata |
| Controls Grid | `space-inline-sm` | grid gap |
| Secondary Actions | `space-inline-sm` | trailing controls gap |
| Sticky posture | `elevation-surface` | persistent browse chrome |

### Token Usage — Exact CSS Values

#### `.filter-toolbar` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-sm)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent)` |

#### `.filter-toolbar[data-sticky="true"]`

| Property | Value |
|----------|-------|
| `box-shadow` | `var(--poodle-elevation-surface)` |

#### `.filter-toolbar__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |

#### `.filter-toolbar[data-collapsed="true"] .filter-toolbar__header`

| Property | Value |
|----------|-------|
| `cursor` | `pointer` |

#### `.filter-toolbar__summary`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `flex` | `1` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `var(--poodle-typography-label-size, 0.75rem)` |
| `line-height` | `var(--poodle-typography-label-lineHeight, 1.4)` |

#### `.filter-toolbar__actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-xs, 0.25rem)` |
| `margin-left` | `auto` |

#### `.filter-toolbar__controls`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(var(--ft-columns, 4), minmax(var(--ft-min-width, 10rem), 1fr))` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `align-items` | `end` |

The `--ft-columns` and `--ft-min-width` CSS variables are set inline from the `columns` and `minItemWidth` props.

#### `.filter-toolbar__secondary`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `align-items` | `center` |

### Responsive Breakpoint: `max-width: 960px`

#### `.filter-toolbar__controls`

| Property | Value |
|----------|-------|
| `grid-template-columns` | `repeat(2, minmax(var(--ft-min-width, 10rem), 1fr))` |

### Responsive Breakpoint: `max-width: 640px`

#### `.filter-toolbar__controls`

| Property | Value |
|----------|-------|
| `grid-template-columns` | `1fr` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-sticky` | `.filter-toolbar` root | enables elevation shadow for sticky positioning |
| `data-collapsed` | `.filter-toolbar` root | controls collapsed state styling (header cursor) |

## 9. Svelte Notes

- uses CollapseToggle primitive for collapse affordance
- controls grid uses CSS custom properties (`--ft-columns`, `--ft-min-width`)
  driven by props
- `collapsed` prop supports `bind:collapsed` for two-way binding

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::filter_toolbar`
- implementation-only details: GPUI may realize sticky behavior with native
  layout containers, but must preserve labeled-group semantics and control order

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] labeled-group semantics and control order match
- [ ] summary meaning and placement in the accessibility tree match
- [ ] collapse behavior hides/shows controls consistently

### Tier 2: Visual Parity

- [ ] control grouping, spacing, and grid layout use comparable token roles
- [ ] responsive breakpoints produce equivalent column reduction

### Tier 3: Implementation Freedom

- [ ] wrap, sticky, and grid mechanics stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| sticky realization may differ | runtime layout systems differ | allowed | keep order and label semantics strict |
| grid breakpoints may vary | viewport detection differs per platform | allowed | ensure column reduction at equivalent widths |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Responsive grid layout

A toolbar with four filter controls in a responsive grid:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Responsive grid layout | `summaryText="Showing 24 of 156 items"`, children: SearchField + 3 Selects (status, type, owner) | toolbar with summary text and 4 controls in responsive grid |

### Collapsible with actions

A collapsible toolbar with action buttons, expanded by default:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Collapsible with actions | `collapsible`, `summaryText="Showing 24 of 156 items"`, actions slot: refresh IconButton, children: SearchField + 2 Selects | toolbar with collapse toggle, summary, refresh button, and filter controls |

### Collapsed by default

A collapsible toolbar starting in collapsed state:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Collapsed by default | `collapsible`, `collapsed`, `summaryText="3 filters active"`, actions slot: refresh IconButton, children: SearchField + Select | compact header row with expand toggle, summary text, and refresh button |

### With secondary slot

A toolbar with secondary actions below the grid:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With secondary slot | `columns={3}`, children: SearchField + 2 Selects; secondary slot: Reset all button | toolbar with 3-column grid and trailing Reset all button |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings search/filter bars, collection browsers,
  inspector filter strips
- future follow-up: consumers may compose active filter pills in the secondary
  slot or header for richer filter feedback
