# FormLayout

Status: seed contract
Updated: 2026-03-21

## 1. Purpose

- Component name: `FormLayout`
- Layer: `composites`
- Summary: a responsive form grid with multi-column support, error/success
  messaging, field error summary, and action button area
- In scope: field grid layout with mixed column widths, form-level messaging,
  field error summary, action button placement, responsive collapsing
- Out of scope: form submission logic, validation orchestration, individual
  field state management

## 2. Anatomy

```text
[Root]
  ├── [Description] (optional)
  ├── [Error Callout] (optional)
  ├── [Success Callout] (optional)
  ├── [Field Error Summary] (optional)
  ├── [Fields Grid]
  └── [Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | vertical flex container with container queries | gap |
| Description | no | introductory help text | typography, text-secondary |
| Error Callout | no | form-level error message | via Callout danger tone |
| Success Callout | no | form-level success message | via Callout success tone |
| Field Error Summary | no | accessible list of per-field errors | background, border, typography |
| Fields Grid | yes | responsive CSS grid for form fields | grid columns, gap |
| Actions | no | action buttons via FormActions | via FormActions primitive |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `columns` | `number` | `6` | no | base grid columns — 6 enables mixed 2-col and 3-col layouts |
| `error` | `string \| null` | `null` | no | form-level error message |
| `success` | `string \| null` | `null` | no | form-level success message |
| `fieldErrors` | `Record<string, string> \| null` | `null` | no | per-field error map |
| `description` | `string \| null` | `null` | no | introductory text |

### Slots

| Slot | Purpose |
|------|---------|
| default | form fields (Field components placed in the grid) |
| `actions` | action buttons (rendered inside FormActions) |

### Grid Column System

The default 6-column grid enables flexible multi-column layouts using Field's
`span` prop:

| Span | Width | Use case |
|------|-------|----------|
| `span={2}` | 1/3 width | three fields per row |
| `span={3}` | 1/2 width | two fields per row |
| `span={6}` | full width | full-width fields (textarea, notes) |

Rows can freely mix 2-col and 3-col divisions. For example, a row of three
name fields (`span={2}` each) followed by a row of two contact fields
(`span={3}` each) works naturally.

For simple single-column forms (e.g. inside a dialog), use `columns={1}`.

### Controlled And Uncontrolled

- all messaging state is host-owned
- form submission logic is external

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no messaging | grid with fields only |
| with-description | `description` set | help text above fields |
| error | `error` set | danger Callout above fields |
| success | `success` set | success Callout above fields |
| field-errors | `fieldErrors` populated | accessible error list above fields |
| with-actions | actions slot populated | FormActions below fields |

### Component States

State table is sufficient.

## 5. Events

No component-owned events.

## 6. Accessibility

### Semantics

- Field Error Summary uses `role="alert"` with `aria-live="polite"`
- Individual field errors are managed by Field primitive's own accessibility
- Callout components provide their own ARIA semantics

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through fields and actions in DOM order |

### Focus And Announcement

- Error summary announced automatically via `aria-live="polite"`
- Field-level errors announced by Field primitive

## 7. Layout

### Sizing

- Fields grid: `repeat(columns, 1fr)` at full container width
- Uses container queries (not viewport media queries) for responsive behavior
- Responsive breakpoints based on container width:
  - **>600px**: full grid (default 6 columns)
  - **480–600px**: 2-column grid — 3-col rows collapse to 2+1 across two rows,
    full-width fields span both columns, all other spans reset to 1
  - **<480px**: single column, all spans reset to 1
- Gap: `stack-lg` vertically, `inline-md` horizontally
- Field `span` prop controls `grid-column: span N`

### Composition

- parent expectations: page sections, Dialog body, card content
- child expectations: Field-wrapped form controls, standalone controls
- Used by FormDialog to provide consistent field layout inside dialogs

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `space-stack-lg` | vertical gap between sections |
| Description | `text-secondary`, `body-size` | help text styling |
| Field Error Summary | `status-danger` | error background/border tint |
| Fields Grid | `space-stack-lg`, `space-inline-md` | grid gap |
| Actions | via FormActions primitive | button layout |

### Token Usage — Exact CSS Values

#### `.form-layout` (Root)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `var(--flint-space-stack-lg)` |
| `container-type` | `inline-size` |

#### `.form-layout__description`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--flint-color-text-secondary)` |
| `font-size` | `var(--flint-typography-body-size, 0.875rem)` |
| `line-height` | `var(--flint-typography-body-lineHeight, 1.5)` |

#### `.form-layout__field-errors`

| Property | Value |
|----------|-------|
| `padding` | `var(--flint-space-panel-y, 0.75rem) var(--flint-space-panel-x, 1rem)` |
| `border-radius` | `var(--flint-radius-surface, 0.5rem)` |
| `background` | `color-mix(in srgb, var(--flint-color-status-danger) 8%, transparent)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--flint-color-status-danger) 40%, transparent)` |
| `font-size` | `var(--flint-typography-label-size, 0.75rem)` |

#### `.form-layout__field-errors p`

| Property | Value |
|----------|-------|
| `margin` | `0 0 0.25rem` |
| `font-weight` | `600` |

#### `.form-layout__field-errors ul`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `padding-left` | `1.25rem` |

#### `.form-layout__field-errors li`

| Property | Value |
|----------|-------|
| `margin-bottom` | `0.125rem` |

#### `.form-layout__grid`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(var(--fl-columns, 6), 1fr)` |
| `gap` | `var(--flint-space-stack-lg) var(--flint-space-inline-md)` |

The `--fl-columns` CSS variable is set inline from the `columns` prop.

### Container Query Breakpoints

Uses `container-type: inline-size` on the root element (not viewport media queries).

#### `@container (max-width: 600px)`

| Selector | Property | Value |
|----------|----------|-------|
| `.form-layout__grid` | `grid-template-columns` | `repeat(2, 1fr)` |
| `.form-layout__grid :global([style*="grid-column"])` | `grid-column` | `span 1 !important` |
| `.form-layout__grid :global([style*="span 6"])`, `:global([style*="span -1"])` | `grid-column` | `1 / -1 !important` |

#### `@container (max-width: 480px)`

| Selector | Property | Value |
|----------|----------|-------|
| `.form-layout__grid` | `grid-template-columns` | `1fr` |

## 9. Svelte Notes

- reuses Callout primitive for error/success banners
- reuses FormActions primitive for action button layout
- root element uses `container-type: inline-size` for container queries
- grid columns driven by CSS custom property `--fl-columns`
- Field `span` prop maps to `grid-column: span N` naturally
- at 2-col breakpoint, `span 6` fields use `grid-column: 1 / -1` to stay
  full width; all other spans reset to `span 1`

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::composites::form_layout`
- grid layout maps to flex with wrap in GPUI
- responsive collapsing may use different thresholds per platform

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] error/success messaging semantics match
- [ ] field error summary accessibility matches
- [ ] action button placement matches

### Tier 2: Visual Parity

- [ ] grid layout and responsive breakpoints comparable
- [ ] spacing and typography use equivalent token roles
- [ ] multi-column mixing (2-col + 3-col rows) supported

### Tier 3: Implementation Freedom

- [ ] grid vs flex realization stays internal
- [ ] container query vs media query approach stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| grid vs flex layout | GPUI lacks CSS grid | allowed | maintain equivalent visual result |
| responsive thresholds | container size detection differs per platform | allowed | ensure equivalent column reduction |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Two-column layout (span 3 = half)

A form using span 3 for half-width fields:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Two-column layout | `description="Fill in the details..."`, default 6-col grid, fields: first/last name (span 3 each), email (span 6), role/region selects (span 3 each), notes textarea (span 6), actions: Cancel + Create user | two fields per row with full-width spanning fields |

### Mixed 2-col and 3-col rows

A form mixing half-width and third-width fields:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Mixed 2-col and 3-col rows | `description="Mixing..."`, fields: first/middle/last name (span 2 each = 3-col row), email/phone (span 3 each = 2-col row), role/region/country (span 2 each = 3-col row), bio (span 6 = full), actions: Cancel + Save | mixed column widths with 3-col and 2-col rows in same form |

### Single column (columns=1)

A single-column form:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Single column | `columns={1}`, fields: display name, bio textarea, checkbox, actions: Save profile | single-column stacked form |

### With error and field errors

A form showing error state:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With error and field errors | `error="Unable to save..."`, `fieldErrors={Email: "...", Role: "..."}`, fields with invalid validation state (span 3 each) | danger callout, field error list, and invalid fields |

### With success message

A form showing success state:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With success message | `success="Settings saved successfully."`, `columns={1}`, field: site name, actions: Save | success callout above form |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: FormDialog, page-level settings forms, onboarding flows
