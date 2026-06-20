# DetailSection

Status: detailed contract
Updated: 2026-05-17

## 1. Purpose

- Component name: `DetailSection`
- Layer: `composites`
- Summary: a titled grouping of related detail rows, supporting a description,
  optional section-level actions, multi-column body layout, a divider posture
  for visual separation between stacked sections, and density-driven spacing
- In scope: section heading with optional description, actions snippet for
  section-level controls, multi-column body grid (1/2/3 columns), responsive
  collapse to single column on narrow viewports, separated/unseparated visual
  posture, accessible region labeling
- Out of scope: page-level header identity, domain-specific row contents,
  editable form submission logic

## 2. Anatomy

```text
[Root Section]  <section>
  ├── [Header .detail-section__header]  (when title, description, or actions)
  │     ├── [TitleBlock .detail-section__title-block]
  │     │     ├── [Title .detail-section__title]  <h3> (optional)
  │     │     └── [Description .detail-section__description]  <p> (optional)
  │     └── [Actions .detail-section__actions]  <div> (optional)
  │           └── (snippet: actions())
  └── [Body .detail-section__body]  <div>
        └── (snippet: children())
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Section | yes | `<section>` element with grid layout and optional separation | spacing, separator |
| Header | conditional | flex row with title block and actions; rendered when title, description, or actions slot is present | gap |
| TitleBlock | conditional | grid column with title and description | typography, gap |
| Title | no | `<h3>` section heading | heading-family, font-size, line-height |
| Description | no | `<p>` supporting text | text-secondary, body typography |
| Actions | no | slot container for section-level action buttons | — |
| Body | yes | grid container for child content | gap, optional multi-column grid |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | section heading text |
| `description` | `string \| null` | `null` | no | supporting description text below the title |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | explicit density override for section spacing; when null, resolves from inherited presentation |
| `separated` | `boolean` | `true` | no | controls `data-separated` attribute for visual separation styling |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the root `<section>` when no visible title exists |
| `columns` | `"auto" \| 1 \| 2 \| 3 \| 4` | `"auto"` | no | body grid column mode; `"auto"` uses responsive auto-fit capped by `maxAutoColumns`, fixed values force that column count |
| `itemMinColumnWidth` | `string \| null` | `null` | no | min column width for `"auto"` mode; sets `--poodle-detail-section-item-min` (default `12rem`) |
| `maxAutoColumns` | `2 \| 3 \| 4 \| 5` | `4` | no | caps the column count `"auto"` mode can expand to |

### Snippets

| Snippet | Purpose |
|---------|---------|
| `children()` | detail rows or custom body content |
| `actions()` | section-level action content rendered in the header |

### Controlled And Uncontrolled

- Declarative grouping composite; all content is host-owned.
- No internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| simple | no title, description, or actions | body grid only, no header |
| titled | `title` present | header rendered with heading-led section |
| described | `description` present | description text below title in title block |
| actionable | actions slot populated | header shows split layout with title block on start and actions on end |
| separated | `separated=true` | `data-separated="true"` on root; a `0.0625rem` rule (`border-subtle` 72% mix) is drawn via `::before` above the section, suppressed for the first child, with density-driven top padding |
| multi-column | `columns="auto"` / `2` / `3` / `4` | body grid uses multi-column layout (`"auto"` = responsive auto-fit capped by `maxAutoColumns`) |
| compact density | `density="compact"` or inherited compact presentation | tighter section gap, header gap, title gap, and body gap |
| comfortable density | `density="comfortable"` or inherited comfortable presentation | looser section gap, header gap, title gap, and body gap |

### Component States

No internal state. All visual variation is driven by props and snippets.

## 5. Events

None. DetailSection is a grouping composite with no component-owned events.

## 6. Accessibility

### Semantics

- Root: `<section>` element
- `aria-label`: set from `ariaLabel` prop when provided; enables the section
  to be treated as a named region
- Title: `<h3>` heading element when `title` is present
- Actions slot: contains focusable action buttons in logical order

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through section actions and body content in DOM order |

### Focus And Announcement

- The section container is not focusable by default
- Section-level actions do not interrupt child row tab order
- No live-region behavior

## 7. Layout

### Sizing

- Root fills available parent width; sets `container-type: inline-size`
- Body spacing is consistent across simple and descriptive headers
- Fixed multi-column body uses `repeat(N, minmax(0, 1fr))`; `"auto"` mode uses
  `repeat(auto-fit, minmax(…item-min…, 1fr))` bounded by `maxAutoColumns`
- Responsive collapse is driven by container queries: at `<= 44rem` columns
  `3`/`4` collapse to 2; at `<= 32rem` all column modes collapse to single
  column; at `<= 28rem` the header stretches and the title shifts to an
  uppercase secondary-color treatment

### Composition

- Parent expectations: detail shells, settings pages, inspector sections
- Child expectations: `DetailItem`, `Card`, or custom body content
- Resizing rules: actions may wrap below the title block on narrow widths
  (via `flex-wrap: wrap` on header)

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-separated` | root `<section>` | `"true"`, `"false"` |
| `data-columns` | root `<section>` | `"auto"`, `"1"`, `"2"`, `"3"`, `"4"` |
| `data-max-auto-columns` | root `<section>` | `"2"`, `"3"`, `"4"`, `"5"` |

### Root `.detail-section`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-detail-section-root-gap)` |

### `.detail-section[data-separated="true"]`

| Property | Value |
|----------|-------|
| padding-top | `var(--poodle-detail-section-separated-gap)` (density-driven; `0` for `:first-child`) |
| position | `relative` |

Separator rule via `::before`: `height: 0.0625rem`, `background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent)`, inset by `--poodle-detail-section-separated-inset`, hidden for `:first-child`.

### Header `.detail-section__header`

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-wrap | `wrap` |
| justify-content | `space-between` |
| gap | `var(--poodle-detail-section-header-gap)` |
| align-items | `start` |

### TitleBlock `.detail-section__title-block`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-detail-section-title-gap)` |

### Title `.detail-section__title` and Description `.detail-section__description`

| Property | Value |
|----------|-------|
| margin | `0` |

### Title `.detail-section__title`

| Property | Value |
|----------|-------|
| font-family | `var(--poodle-typography-heading-family)` |
| font-weight | `700` |
| font-size | `1.125rem` |
| line-height | `1.2` |

### Description `.detail-section__description`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-secondary)` |
| font-size | `var(--poodle-typography-body-size)` |
| line-height | `var(--poodle-typography-body-lineHeight)` |

### Body `.detail-section__body`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-detail-section-body-gap)` |

### Density Variables

`root-gap`/`header-gap`/`title-gap`/`body-gap` (also drives body row-gap and column-gap) plus `separated-gap` (separated top padding).

| Density | `root-gap` | `header-gap` | `title-gap` | `body-gap` | `separated-gap` |
|---------|------------|--------------|-------------|------------|-----------------|
| `compact` | `0.75rem` | `var(--poodle-space-inline-sm)` | `0.25rem` | `0.625rem` | `0.875rem` |
| `default` | `calc(var(--poodle-space-stack-md) + 0.125rem)` | `0.75rem` | `0.375rem` | `0.75rem` | `1rem` |
| `comfortable` | `calc(var(--poodle-space-stack-lg) - 0.125rem)` | `0.875rem` | `0.5rem` | `1rem` | `1.125rem` |

The base `.detail-section` (no `data-density`) seeds defaults via tokens: `header-gap: var(--poodle-space-inline-md)`, `body-gap: var(--poodle-space-stack-sm)`, `item-min: 12rem`, `title-weight: 700`.

### Multi-Column Body

| Selector | Property | Value |
|----------|----------|-------|
| `[data-columns="auto"] .detail-section__body` | grid-template-columns | `repeat(auto-fit, minmax(min(100%, item-min), 1fr))`, bounded per `data-max-auto-columns` (2–5) |
| `[data-columns="2"] .detail-section__body` | grid-template-columns | `repeat(2, minmax(0, 1fr))` |
| `[data-columns="3"] .detail-section__body` | grid-template-columns | `repeat(3, minmax(0, 1fr))` |
| `[data-columns="4"] .detail-section__body` | grid-template-columns | `repeat(4, minmax(0, 1fr))` |

### Responsive Breakpoints (container queries)

| Selector | Breakpoint | Property | Value |
|----------|-----------|----------|-------|
| `[data-columns="3"\|"4"] .detail-section__body` | `@container (max-width: 44rem)` | grid-template-columns | `repeat(2, minmax(0, 1fr))` |
| `[data-columns="auto"\|"2"\|"3"\|"4"] .detail-section__body` | `@container (max-width: 32rem)` | grid-template-columns | `1fr` |
| `.detail-section__header` | `@container (max-width: 28rem)` | align-items / gap | `stretch` / `0.5rem` |
| `.detail-section__title` | `@container (max-width: 28rem)` | font / transform | `0.8125rem`, weight `650`, `0.04em` tracking, `uppercase`, `text-secondary` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Root element is a `<section>` with scoped CSS class `detail-section`
- Header is conditionally rendered when `title`, `description`, or `actions`
  is present
- Title is an `<h3>` element; description is a `<p>` element
- Multi-column layout driven by `data-columns` + `data-max-auto-columns` attributes on root; `"auto"` mode reads `--poodle-detail-section-item-min` (set from `itemMinColumnWidth`, default `12rem`)
- Root sets `container-type: inline-size`; responsive collapse uses container queries (`44rem` / `32rem` / `28rem`), not a viewport media query
- Separator is a `::before` rule (border-subtle 72% mix), not `border-top`; suppressed for the first child
- `actions` snippet presence controls the header action area

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::detail_section`
- GPUI may use grouped layout views and named subtrees instead of HTML
  sections, but addressable-section semantics remain required
- Multi-column body can be realized as a flex-wrap container with fixed-fraction
  child widths

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] heading/group semantics match (`<section>` with `<h3>`)
- [ ] header renders only when title, description, or actions are present
- [ ] body grid supports 1/2/3 column modes
- [ ] multi-column layouts collapse on narrow viewports

### Tier 2: Visual Parity

- [ ] section spacing, dividers, and title hierarchy use comparable token roles
- [ ] title-block gap matches (0.375rem)
- [ ] header flex-wrap behavior matches

### Tier 3: Implementation Freedom

- [ ] body slot mounting and wrapping behavior stay internal
- [ ] responsive breakpoint mechanism may differ (media query vs container query)

## 12. Specimen Definitions

### With Title And Rows

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With title and rows | `title="Project details"`, `description="Core metadata for this project."`, four DetailItems (Name, Owner, Created, Status) | heading-led section with description text and vertically stacked detail rows |

### With Actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With actions | `title="Billing"`, actions slot with secondary sm Edit button, three DetailItems (Plan, Billing cycle, Next invoice) | header split layout with title on start and Edit button on end, detail rows below |

### Multi-Column Layout

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Multi-column layout | `title="Server metrics"`, `columns={3}`, six DetailItems arranged in a 3-column grid | three-column body grid with rows distributed across columns |

### DetailItem With Description

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| DetailItem with description | `title="Configuration"`, two DetailItems with `description` prop; first row has `truncateValue` | detail rows with label, value, and supporting description; first value truncates with ellipsis |
