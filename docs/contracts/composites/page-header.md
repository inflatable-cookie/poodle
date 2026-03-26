# PageHeader

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `PageHeader`
- Layer: `composites`
- Summary: a standardized title and action region for product pages, panels, or
  detail surfaces
- In scope: optional back link, title, subtitle/supporting text, optional
  breadcrumbs region, primary/secondary actions, status metadata
- Out of scope: app shell toolbars, global navigation, domain-specific command
  wiring

## 2. Anatomy

```text
[Root Header]
  ├── [Back Link] (optional)
  ├── [Breadcrumbs Region] (optional)
  ├── [Title Block]
  │     ├── [Eyebrow or Meta] (optional)
  │     ├── [Title]
  │     └── [Subtitle] (optional)
  └── [Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Header | yes | header container | spacing, separator |
| Back Link | no | lightweight return link above the title block | typography, text color, hover treatment |
| Breadcrumbs Region | no | pre-title navigation trail | typography, text color |
| Title Block | yes | primary identity region | typography, spacing |
| Actions | no | page-level action group | gap, alignment |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | none | yes | primary heading text |
| `count` | `number \| null` | `null` | no | optional count badge rendered inline with the title |
| `subtitle` | `string \| null` | `null` | no | supporting copy |
| `eyebrow` | `string \| null` | `null` | no | small meta label |
| `backHref` | `string \| null` | `null` | no | optional back-link target rendered above breadcrumbs and title |
| `backLabel` | `string \| null` | `null` | no | optional back-link text, defaults to `Back` when `backHref` is set |
| `align` | `"start" \| "between"` | `"between"` | no | action alignment posture |
| `ariaLabel` | `string \| null` | `null` | no | optional region label when header is independently addressable |

### Controlled And Uncontrolled

- declarative composite
- actions and breadcrumbs content remain host-owned children

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| simple | title only | compact title row |
| descriptive | subtitle or eyebrow present | expanded title block |
| actionable | actions present | title and actions share row/stack |

### Component States

State table is sufficient.

## 5. Events

No component-owned events beyond child action behavior.

## 6. Accessibility

### Semantics

- Role: header region or neutral section heading container depending on parent
  context
- Required attributes: heading semantics for the visible title
- Optional attributes: region label when the header is independently navigable
- Labeling rules: visible title should remain the primary accessible heading,
  and back-link wording should stay short and destination-oriented

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches back link, breadcrumb links, and header actions in logical order |

### Focus And Announcement

- focus entry: the header itself is not focusable by default
- focus exit: back link, breadcrumb links, and actions should not break heading
  readability or invert expected reading order
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must preserve heading
  hierarchy and action grouping without flattening the title block into plain
  text with unlabeled buttons nearby

## 7. Layout

### Sizing

- header should support narrow stacked layouts and wider split layouts
- actions may wrap below the title block when width is constrained

### Composition

- parent expectations: detail shells, settings pages, list/grid surfaces,
  product panels
- child expectations: breadcrumbs and action primitives are optional children
- resizing rules: the title remains visually dominant over actions
- composition rule: back link and breadcrumbs, when present, remain above the
  title block and do not collapse into the same semantic role as the page title

## 8. Token Usage And Precise CSS

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-align` | root `<header>` | `"start"`, `"between"` |

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

### Root

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-md)` |
| align-items | `end` |
| padding | `var(--poodle-recipe-page-header-padding-block-start) var(--poodle-recipe-page-header-padding-inline) var(--poodle-recipe-page-header-padding-block-end)` |
| border | `0.0625rem solid var(--poodle-recipe-page-header-border)` |
| border-radius | `var(--poodle-recipe-page-header-radius)` |
| background | `var(--poodle-recipe-page-header-fill)` |
| box-shadow | `var(--poodle-recipe-page-header-shadow)` |

#### Root Alignment Variant (`[data-align="between"]`)

| Property | Value |
|----------|-------|
| grid-template-columns | `minmax(0, 1fr) auto` |

### Content

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-md)` |

### Title Block

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `0.375rem` |

### Title (h2)

| Property | Value |
|----------|-------|
| margin | `0` |
| font-family | `var(--poodle-typography-heading-family)` |
| font-size | `1.75rem` |
| line-height | `1.1` |
| font-weight | `700` |

### Eyebrow

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.6875rem` |
| font-weight | `600` |
| letter-spacing | `0.12em` |
| text-transform | `uppercase` |

### Subtitle

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `var(--poodle-typography-body-size)` |
| line-height | `var(--poodle-typography-body-lineHeight)` |

### Actions

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-wrap | `wrap` |
| gap | `var(--poodle-space-inline-md)` |
| justify-content | `flex-end` |
| align-items | `start` |

### Responsive Breakpoints

| Breakpoint | Selector | Property | Value |
|------------|----------|----------|-------|
| `max-width: 45rem` | `[data-align="between"]` | grid-template-columns | `1fr` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- expected substrate: `Stack`/`Inline` composition with semantic heading
- wrapper strategy: slots for breadcrumbs and actions stay Poodle-owned surface
  conventions

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::page_header`
- implementation-only details: GPUI heading semantics may use named region and
  text hierarchy APIs rather than HTML headings, but the accessible structure
  still needs to be explicit

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] title hierarchy and heading semantics match
- [ ] breadcrumbs and actions remain logically grouped and ordered

### Tier 2: Visual Parity

- [ ] spacing, heading weight, and action alignment use comparable token roles

### Tier 3: Implementation Freedom

- [ ] wrap strategy and layout breakpoints stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| line wrapping may differ slightly | text metrics differ by runtime | allowed | keep hierarchy and order strict |

## 13. Specimen Definitions

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

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings headers, detail views, catalog pages
- future follow-up: pair with shell toolbars separately in workstation work

## Next Task

Use `PageHeader` for local page identity inside `DetailShell` and product pages,
and reserve app-wide toolbars for workstation or shell composites.
