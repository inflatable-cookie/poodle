# PageHeader

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `PageHeader`
- Layer: `composites`
- Summary: a standardized title and action region for product pages, panels, or
  detail surfaces
- In scope: title, subtitle/supporting text, optional breadcrumbs region,
  primary/secondary actions, status metadata
- Out of scope: app shell toolbars, global navigation, domain-specific command
  wiring

## 2. Anatomy

```text
[Root Header]
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
| Breadcrumbs Region | no | pre-title navigation trail | typography, text color |
| Title Block | yes | primary identity region | typography, spacing |
| Actions | no | page-level action group | gap, alignment |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | none | yes | primary heading text |
| `subtitle` | `string \| null` | `null` | no | supporting copy |
| `eyebrow` | `string \| null` | `null` | no | small meta label |
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
- Labeling rules: visible title should remain the primary accessible heading

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches breadcrumb links and header actions in logical order |

### Focus And Announcement

- focus entry: the header itself is not focusable by default
- focus exit: action focus order should not break heading readability
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
- composition rule: breadcrumbs, when present, remain above the title block and
  do not collapse into the same semantic role as the page title

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Header | spacing and separator roles | region spacing |
| Title Block | display/heading typography and text roles | hierarchy |
| Breadcrumbs Region | subdued text roles | path context |
| Actions | action spacing roles | command grouping |

## 9. Svelte Notes

- expected substrate: `Stack`/`Inline` composition with semantic heading
- wrapper strategy: slots for breadcrumbs and actions stay Pug-owned surface
  conventions

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::page_header`
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

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings headers, detail views, catalog pages
- future follow-up: pair with shell toolbars separately in workstation work

## Next Task

Use `PageHeader` for local page identity inside `DetailShell` and product pages,
and reserve app-wide toolbars for workstation or shell composites.
