# Table

> **Surface elevation**: Table header is a surface consumer (60% medium-strong contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Table`
- Layer: `foundation`
- Summary: a low-level structured data surface that preserves native table
  semantics without pulling sorting, selection, or row actions into the same
  contract
- In scope: column headers, row and cell relationships, row-header posture,
  empty state, caption, horizontal overflow in shell
- Out of scope: sorting, row selection, row actions, bulk actions,
  virtualization, pagination policy

## 2. Anatomy

```text
[Shell .table-shell]  <div>
  └── [Table .table]  <table>
        ├── [Caption .table__caption]  <caption> (optional)
        ├── [Header .table__head]  <thead>
        │     └── [Header Row]  <tr>
        │           └── [Column Header .table__header]...  <th>
        └── [Body .table__body]  <tbody>
              ├── [Row .table__row]...  <tr>
              │     └── [Cell .table__cell]...  <td> or <th>
              └── [Empty Row]  <tr> (conditional, when rows empty)
                    └── [Empty Cell .table__empty]  <td>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Shell | yes | overflow container | border, radius, background |
| Table | yes | native table element | color, typography |
| Caption | no | visible table label | color, typography, spacing |
| Header | yes | column header row | background, color, typography, border |
| Cell | yes | data cell | color, spacing, border |
| Empty Cell | no | empty-state message | color, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `columns` | `TableColumn[]` | — | yes | column definitions |
| `rows` | `TableRow[]` | `[]` | no | row data |
| `caption` | `string \| null` | `null` | no | visible caption text |
| `emptyMessage` | `string` | `"No rows available."` | no | shown when rows is empty |
| `ariaLabel` | `string \| null` | `null` | no | accessible name when no caption |

### Type Definitions

```
TableColumn: { id: string; label: string; align?: "start" | "end"; isRowHeader?: boolean }
TableRow: { id: string; cells: Record<string, string> }
```

### Controlled And Uncontrolled

- data-driven read-only component, no persistent value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | rows has items | table renders header and body rows |
| empty | rows is empty | table renders header and empty-state row |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `rows.length > 0` | body rows rendered from row data |
| empty | `rows.length === 0` | single row spanning all columns with emptyMessage |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | — | — | baseline table has no interactive events |

## 6. Accessibility

### Semantics

- Role: native `<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>` elements
- `<th scope="col">` on column headers
- `<th scope="row">` on cells in columns where `isRowHeader` is true
- `<caption>` for visible table label; `aria-label` on `<table>` when no caption
- Empty row cell uses `colspan` spanning all columns

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus through any focusable content within cells |

### Focus And Announcement

- focus entry: standard document navigation through native table semantics
- focus exit: standard tab order
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must expose table, row, column-header, and cell semantics through the native accessibility tree with stable header-to-cell relationships

## 7. Layout

### Sizing

- Shell: `min-width: 0`, `overflow-x: auto` to handle wide tables
- Table: `width: 100%` within shell
- overflow behavior: horizontal scroll within shell container

### Composition

- parent expectations: panels, data views, settings surfaces
- child expectations: columns and rows data only
- resizing rules: shell stretches to parent width; table may overflow horizontally

## 8. Token Usage — Exact Values

### Shell `.table-shell`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `overflow-x` | `auto` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 78%, transparent)` |
| `border-radius` | `var(--flint-radius-surface)` |
| `background` | `color-mix(in srgb, var(--flint-color-background-panel) 96%, transparent)` |

### Table `.table`

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `border-collapse` | `collapse` |
| `color` | `var(--flint-color-text-primary)` |
| `font-family` | `var(--flint-typography-body-family)` |
| `font-size` | `var(--flint-typography-body-size)` |
| `line-height` | `var(--flint-typography-body-lineHeight)` |

### Caption `.table__caption`

| Property | Value |
|----------|-------|
| `padding` | `var(--flint-space-panel-y) var(--flint-space-panel-x) 0` |
| `color` | `var(--flint-color-text-secondary)` |
| `font-size` | `0.75rem` |
| `text-align` | `left` |

### Column Header `.table__header`

| Property | Value |
|----------|-------|
| `padding` | `0.6875rem 0.875rem` |
| `border-bottom` | `0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 72%, transparent)` |
| `text-align` | `left` |
| `vertical-align` | `middle` |
| `color` | `var(--flint-color-text-secondary)` |
| `font-family` | `var(--flint-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.04em` |
| `text-transform` | `uppercase` |
| `background` | `color-mix(in srgb, var(--flint-surface) 60%, var(--flint-color-background-elevated))` |

### Cell `.table__cell`

| Property | Value |
|----------|-------|
| `padding` | `0.6875rem 0.875rem` |
| `border-bottom` | `0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 72%, transparent)` |
| `text-align` | `left` |
| `vertical-align` | `middle` |

### Cell — row header (column has `isRowHeader: true`)

| Property | Value |
|----------|-------|
| `font-weight` | `600` |

### Cell / Header — `align="end"`

| Property | Value |
|----------|-------|
| `text-align` | `right` |

### Last row cell `.table__row:last-child .table__cell`

| Property | Value |
|----------|-------|
| `border-bottom` | `0` |

### Empty cell `.table__empty`

| Property | Value |
|----------|-------|
| `padding` | `0.6875rem 0.875rem` |
| `color` | `var(--flint-color-text-secondary)` |
| `text-align` | `left` |
| `vertical-align` | `middle` |

## 9. Svelte Notes

- Uses real `<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>` elements
- `data-align="end"` data attribute on cells for alignment targeting
- Row header cells rendered as `<th scope="row">` rather than `<td>`
- Shell wrapper provides border-radius clipping and scroll containment
- Richer interactivity (sorting, selection) should compose above this primitive

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::primitives::table`
- Spec struct: `TableSpec` in primitives crate
- GPUI must expose table, row, column-header, and cell semantics through accessibility tree
- Row header posture must be preserved in accessibility output
- Shell border-radius and overflow handling may differ from CSS approach
- The `color-mix` formulas for borders and backgrounds should be replicated as closely as possible

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] columns, rows, caption, emptyMessage, ariaLabel mean the same thing
- [ ] header-to-cell relationships match
- [ ] row-header posture (scope="row") matches
- [ ] empty-state posture matches
- [ ] accessible name from caption or ariaLabel matches

### Tier 2: Visual Parity

- [ ] shell border, radius, and background match
- [ ] column header typography (uppercase, 0.6875rem, 600 weight, 0.04em spacing) matches
- [ ] cell padding (0.6875rem 0.875rem) matches
- [ ] border-bottom color-mix formula matches
- [ ] header background color-mix formula matches
- [ ] align="end" right-alignment matches
- [ ] last-row border removal matches
- [ ] empty-state secondary color matches

### Tier 3: Implementation Freedom

- [ ] exact overflow handling may differ by runtime
- [ ] transition or animation behavior is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact overflow handling may differ | layout internals differ by runtime | allowed | keep structured table meaning strict |
| color-mix formula rendering | GPUI may approximate color-mix | allowed | match visual result as closely as possible |

## 13. Specimen Definitions

### Standard Table

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Standard table | 4 columns (Name as row header, Role, Status, Hours end-aligned), 4 data rows | Full table with header row and body rows; Name column bold, Hours right-aligned |

### With Caption

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With caption | same columns and rows as standard, `caption="Q1 team allocation"` | Table with visible caption text above header row |

### Minimal Key-Value

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Minimal key-value | 2 columns (Property as row header, Value), 3 rows (Version, License, Bundle size) | Compact two-column table for key-value display |

### Empty State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty state | same 4 columns, `rows={[]}`, `emptyMessage="No team members found."` | Table with header row and single empty-state row spanning all columns |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: simple structured reports, low-interaction admin tables,
  read-only data surfaces, DataTable composite
- future follow-up: sorting, selection, and row actions belong in DataTable composite
