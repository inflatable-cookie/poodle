# FormLayout

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `FormLayout`
- Layer: `composites`
- Summary: a responsive form grid with multi-column support, error/success
  messaging via Callout, field error summary with accessible alert, and
  action button area via FormActions — provides consistent form structure
  across dialogs, pages, and card content
- In scope: field grid layout with mixed column widths, form-level
  error/success messaging via Callout, field error summary list, action
  button placement via FormActions, responsive collapsing via container
  queries, description text
- Out of scope: form submission logic, validation orchestration, individual
  field state management

## 2. Anatomy

```text
[Root .poodle-form-layout]  <div>
  ├── [Description .poodle-form-layout__description]  <p> (optional)
  ├── [ErrorCallout]  Callout (tone="danger") (optional)
  ├── [SuccessCallout]  Callout (tone="success") (optional)
  ├── [FieldErrors .poodle-form-layout__field-errors]  <div> (optional)
  │     ├── [ErrorHeading]  <p> "Please fix the following errors:"
  │     └── [ErrorList]  <ul>
  │           └── [ErrorItem]  <li> per field error
  ├── [Grid .poodle-form-layout__grid]  <div>
  │     └── (snippet: children() — form fields)
  └── [Actions .poodle-form-layout__actions]  <div> (optional)
        └── [FormActions]  FormActions primitive
              └── (snippet: actions())
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | flex column container with container-type inline-size | gap (stack-lg) |
| Description | no | introductory help text | text-secondary, body typography |
| ErrorCallout | no | form-level error message via Callout | delegates to Callout contract (tone="danger") |
| SuccessCallout | no | form-level success message via Callout | delegates to Callout contract (tone="success") |
| FieldErrors | no | accessible error summary list | status-danger background/border, label typography |
| Grid | yes | responsive CSS grid for form fields | grid columns, row-gap (`stack-lg + 0.625rem`), column-gap (`inline-md`) |
| Actions | no | action buttons via FormActions | delegates to FormActions contract |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `columns` | `number` | `6` | no | base grid columns; 6 enables mixed 2-col and 3-col layouts |
| `error` | `string \| null` | `null` | no | form-level error message (displayed via Callout tone="danger") |
| `success` | `string \| null` | `null` | no | form-level success message (displayed via Callout tone="success") |
| `fieldErrors` | `Record<string, string> \| null` | `null` | no | per-field error map; keys are field names, values are error messages |
| `description` | `string \| null` | `null` | no | introductory text above the form |

### Snippets

| Snippet | Purpose |
|---------|---------|
| `children()` | form fields (placed in the grid; Field `span` prop controls `grid-column: span N`) |
| `actions()` | action buttons rendered inside FormActions |

### Grid Column System

The default 6-column grid enables flexible multi-column layouts using Field's
`span` prop:

| Span | Width | Use Case |
|------|-------|----------|
| `span={2}` | 1/3 width | three fields per row |
| `span={3}` | 1/2 width | two fields per row |
| `span={6}` | full width | full-width fields (textarea, notes) |

Rows can freely mix 2-col and 3-col divisions. For simple single-column
forms (e.g. inside a dialog), use `columns={1}`.

### Controlled And Uncontrolled

- All messaging state is host-owned
- Form submission logic is external
- No internal state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no messaging | grid with fields only |
| with-description | `description` set | help text above fields |
| error | `error` set | danger Callout above fields |
| success | `success` set | success Callout above fields |
| field-errors | `fieldErrors` populated with entries | accessible error list above fields |
| with-actions | actions slot populated | FormActions rendered below fields |

### Component States

No internal state. All visual variation is driven by props and snippets.

## 5. Events

None. FormLayout is a structural composite with no component-owned events.

## 6. Accessibility

### Semantics

- Field Error Summary: `role="alert"` with `aria-live="polite"` for automatic
  announcement when errors appear
- Error list heading: `<p>` with text "Please fix the following errors:"
- Error list: `<ul>` with `<li>` items showing `<strong>{field}</strong>: {message}`
- Individual field errors: managed by Field primitive's own accessibility
- Callout components: provide their own ARIA semantics

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through fields and actions in DOM order |

### Focus And Announcement

- Error summary announced automatically via `aria-live="polite"`
- Field-level errors announced by Field primitive

## 7. Layout

### Sizing

- Root: flex column with `gap: var(--poodle-space-stack-lg)`
- Fields grid: `repeat(columns, 1fr)` with `row-gap: calc(stack-lg + 0.625rem)`, `column-gap: inline-md`
- A `Field` that is the grid's only child spans the full row (`grid-column: 1 / -1`)
- Grid columns driven by CSS custom property `--fl-columns` (set inline from
  `columns` prop)
- Uses container queries (not viewport media queries) for responsive behavior

### Responsive Breakpoints (Container-Based)

- **>600px**: full grid (default 6 columns)
- **480--600px**: 2-column grid; `span 6` and `span -1` fields use
  `grid-column: 1 / -1` to stay full width; all other spans reset to
  `span 1`
- **<480px**: single column; all spans reset

### Composition

- Composes: `Callout` and `FormActions` from `@poodle/svelte`
- Parent expectations: page sections, Dialog body, card content
- Child expectations: Field-wrapped form controls, standalone controls
- Used by: FormDialog to provide consistent field layout inside dialogs

## 8. Token Usage — Exact Values

### Root `.poodle-form-layout`

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-direction | `column` |
| gap | `var(--poodle-space-stack-lg)` |
| container-type | `inline-size` |

### Description `.poodle-form-layout__description`

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `var(--poodle-typography-body-size, 0.875rem)` |
| line-height | `var(--poodle-typography-body-lineHeight, 1.5)` |

### Field Errors `.poodle-form-layout__field-errors`

| Property | Value |
|----------|-------|
| padding | `var(--poodle-space-panel-y, 0.75rem) var(--poodle-space-panel-x, 1rem)` |
| border-radius | `var(--poodle-radius-surface, 0.5rem)` |
| background | `color-mix(in srgb, var(--poodle-color-status-danger) 8%, transparent)` |
| border | `0.0625rem solid color-mix(in srgb, var(--poodle-color-status-danger) 40%, transparent)` |
| font-size | `var(--poodle-typography-label-size, 0.75rem)` |

### Field Errors Heading `.poodle-form-layout__field-errors p`

| Property | Value |
|----------|-------|
| margin | `0 0 0.25rem` |
| font-weight | `600` |

### Field Errors List `.poodle-form-layout__field-errors ul`

| Property | Value |
|----------|-------|
| margin | `0` |
| padding-left | `1.25rem` |

### Field Errors Item `.poodle-form-layout__field-errors li`

| Property | Value |
|----------|-------|
| margin-bottom | `0.125rem` |

### Grid `.poodle-form-layout__grid`

The grid declares two local custom properties on itself, then consumes them for the two gap axes:

| Custom Property | Value |
|-----------------|-------|
| `--poodle-form-layout-row-gap` | `calc(var(--poodle-space-stack-lg) + 0.625rem)` |
| `--poodle-form-layout-column-gap` | `var(--poodle-space-inline-md)` |

| Property | Value |
|----------|-------|
| display | `grid` |
| grid-template-columns | `repeat(var(--fl-columns, 6), 1fr)` |
| row-gap | `var(--poodle-form-layout-row-gap)` (= `space-stack-lg + 0.625rem`) |
| column-gap | `var(--poodle-form-layout-column-gap)` (= `space-inline-md`) |

The `--fl-columns` CSS variable is set inline from the `columns` prop.

### Grid Lone-Field Full-Span `.poodle-form-layout__grid .poodle-field:only-child`

| Property | Value |
|----------|-------|
| grid-column | `1 / -1` |

A `Field` that is the only child of the grid spans the full row width.

### Container Query Breakpoints

Uses `container-type: inline-size` on the root element.

#### `@container (max-width: 600px)`

| Selector | Property | Value |
|----------|----------|-------|
| `.poodle-form-layout__grid` | grid-template-columns | `repeat(2, 1fr)` |
| `.poodle-form-layout__grid :global([style*="grid-column"])` | grid-column | `span 1 !important` |
| `.poodle-form-layout__grid :global([style*="span 6"])` | grid-column | `1 / -1 !important` |
| `.poodle-form-layout__grid :global([style*="span -1"])` | grid-column | `1 / -1 !important` |

#### `@container (max-width: 480px)`

| Selector | Property | Value |
|----------|----------|-------|
| `.poodle-form-layout__grid` | grid-template-columns | `1fr` |

### Composed Primitives

| Part | Delegates To |
|------|-------------|
| ErrorCallout | Callout contract (foundation), `tone="danger"` |
| SuccessCallout | Callout contract (foundation), `tone="success"` |
| FormActions | FormActions contract (foundation) |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Reuses `Callout` primitive for error/success banners via `message` prop
- Reuses `FormActions` primitive for action button layout
- Root element uses `container-type: inline-size` for container queries
- Grid columns driven by CSS custom property `--fl-columns` set via
  `style:--fl-columns={columns}`
- Grid declares two local custom props `--poodle-form-layout-row-gap`
  (`calc(space-stack-lg + 0.625rem)`) and `--poodle-form-layout-column-gap`
  (`space-inline-md`), consumed by `row-gap`/`column-gap` (asymmetric: row gap
  is larger than column gap)
- A lone `.poodle-field:only-child` in the grid spans `grid-column: 1 / -1`
- Field `span` prop maps to `grid-column: span N` naturally
- At 2-col breakpoint, `span 6` fields use `grid-column: 1 / -1` to stay
  full width; all other spans reset to `span 1`
- `actions` snippet presence controls whether FormActions is rendered
- Callout receives `message` prop (not children/slot)

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::form_layout`
- Grid layout maps to flex with wrap in GPUI
- Responsive collapsing may use different thresholds per platform
- Field error summary must maintain `role="alert"` semantics

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] error/success messaging semantics match
- [ ] field error summary uses `role="alert"` with `aria-live="polite"`
- [ ] action button placement matches (below fields, inside FormActions)
- [ ] grid column system supports same span values

### Tier 2: Visual Parity

- [ ] grid layout and responsive breakpoints comparable
- [ ] spacing and typography use equivalent token roles
- [ ] multi-column mixing (2-col + 3-col rows) supported
- [ ] field error summary background/border treatment matches

### Tier 3: Implementation Freedom

- [ ] grid vs flex realization stays internal
- [ ] container query vs media query approach stays internal

## 12. Specimen Definitions

### Two-Column Layout (span 3 = half)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Two-column layout | `description="Fill in the details..."`, default 6-col grid, fields: first/last name (span 3 each), email (span 6), role/region selects (span 3 each), notes textarea (span 6), actions: Cancel + Create user | two fields per row with full-width spanning fields |

### Mixed 2-Col And 3-Col Rows

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Mixed 2-col and 3-col rows | `description="Mixing..."`, fields: first/middle/last name (span 2 each = 3-col row), email/phone (span 3 each = 2-col row), role/region/country (span 2 each = 3-col row), bio (span 6 = full), actions: Cancel + Save | mixed column widths with 3-col and 2-col rows in same form |

### Single Column (columns=1)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Single column | `columns={1}`, fields: display name, bio textarea, checkbox, actions: Save profile | single-column stacked form |

### With Error And Field Errors

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With error and field errors | `error="Unable to save..."`, `fieldErrors={Email: "...", Role: "..."}`, fields with invalid validation state (span 3 each) | danger callout, field error list with role="alert", and invalid fields |

### With Success Message

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With success message | `success="Settings saved successfully."`, `columns={1}`, field: site name, actions: Save | success callout above form |
