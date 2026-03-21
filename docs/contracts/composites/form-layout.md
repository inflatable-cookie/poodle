# FormLayout

Status: seed contract
Updated: 2026-03-21

## 1. Purpose

- Component name: `FormLayout`
- Layer: `composites`
- Summary: a responsive form grid with error/success messaging, field error
  summary, and action button area
- In scope: field grid layout, form-level messaging, field error summary,
  action button placement
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
| Root | yes | vertical flex container | gap |
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
| `columns` | `number` | `2` | no | grid columns at full width |
| `error` | `string \| null` | `null` | no | form-level error message |
| `success` | `string \| null` | `null` | no | form-level success message |
| `fieldErrors` | `Record<string, string> \| null` | `null` | no | per-field error map |
| `description` | `string \| null` | `null` | no | introductory text |

### Slots

| Slot | Purpose |
|------|---------|
| default | form fields (Field components placed in the grid) |
| `actions` | action buttons (rendered inside FormActions) |

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

- Fields grid: `repeat(columns, 1fr)` at full width
- @media ≤640px: collapses to single column
- Gap: `stack-sm` vertically, `inline-md` horizontally
- Field `span` prop controls `grid-column: span N` for full-width fields

### Composition

- parent expectations: page sections, Dialog body, card content
- child expectations: Field-wrapped form controls, standalone controls
- Used by FormDialog to provide consistent field layout inside dialogs

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `space-stack-sm` | vertical gap between sections |
| Description | `text-secondary`, `body-size` | help text styling |
| Field Error Summary | `status-danger` | error background/border tint |
| Fields Grid | `space-stack-sm`, `space-inline-md` | grid gap |
| Actions | via FormActions primitive | button layout |

## 9. Svelte Notes

- reuses Callout primitive for error/success banners
- reuses FormActions primitive for action button layout
- grid columns driven by CSS custom property `--fl-columns`
- Field `span` prop maps to `grid-column: span N` naturally

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::form_layout`
- grid layout maps to flex with wrap in GPUI

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] error/success messaging semantics match
- [ ] field error summary accessibility matches
- [ ] action button placement matches

### Tier 2: Visual Parity

- [ ] grid layout and responsive breakpoints comparable
- [ ] spacing and typography use equivalent token roles

### Tier 3: Implementation Freedom

- [ ] grid vs flex realization stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| grid vs flex layout | GPUI lacks CSS grid | allowed | maintain equivalent visual result |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Two-column grid

A two-column form with spanning fields:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Two-column grid | `description="Fill in the details..."`, default columns, fields: first/last name (1 col each), email (span 2), role/region selects, notes textarea (span 2), actions: Cancel + Create user | responsive 2-column grid with full-width spanning fields |

### Single column

A single-column form:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Single column | `columns={1}`, fields: display name, bio textarea, checkbox, actions: Save profile | single-column stacked form |

### With error and field errors

A form showing error state:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With error and field errors | `error="Unable to save..."`, `fieldErrors={Email: "...", Role: "..."}`, fields with invalid validation state | danger callout, field error list, and invalid fields |

### With success message

A form showing success state:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With success message | `success="Settings saved successfully."`, `columns={1}`, field: site name, actions: Save | success callout above form |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: FormDialog, page-level settings forms, onboarding flows
