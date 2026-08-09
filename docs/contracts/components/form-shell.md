# FormShell

> **Implementation note**: no standalone Svelte component. This contract exists as a shared spec for GPUI/Jetstream composite renderers so they don't reinvent field grouping, validation exposure, or submission-blocking logic. In Svelte, form orchestration is handled compositionally by the caller.

Status: detailed contract
Updated: 2026-04-11

## 1. Purpose

- Component name: `FormShell`
- Layer: `composites`
- Summary: an orchestrated form surface that owns title, description, section
  grouping, field-state tracking, form-level status summary, and submission
  gating. Exists as a shared Rust spec so GPUI composite renderers don't
  reinvent field grouping, validation exposure, or submission-blocking logic.
- In scope: sectioned layout with per-section field-id lists, per-field
  validation state tracking, form-level status summary with tone, disabled /
  busy gating, submission block derivation from field validation state,
  resolved-status-tone derivation
- Out of scope: individual field rendering (delegated to primitives), validator
  execution, async submission orchestration, network error surfacing beyond
  status-summary text

## 2. Anatomy

```text
[Root .form-shell]  <form>
  ├── [Title .form-shell__title]  <h2> (optional)
  ├── [Description .form-shell__description]  <p> (optional)
  ├── [Sections .form-shell__sections]
  │     └── [Section .form-shell__section]  (one per FormSectionSpec)
  │           ├── [SectionTitle]  <h3>
  │           ├── [SectionDescription]  <p> (optional)
  │           └── [Fields]  (rendered by host, referenced via field_ids)
  ├── [StatusSummary]  <Callout> (optional, when status_summary is set)
  └── [Actions .form-shell__actions]  (FormActionLayout, align + count)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | form element with stack layout | stack gap, padding |
| Title | no | heading text | typography-heading |
| Description | no | supporting description | typography-body, text-secondary |
| Sections | no | ordered list of field groupings | section gap |
| StatusSummary | no | form-level status Callout | callout token chain |
| Actions | yes | submit / cancel action row | inline gap, padding |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | — | yes | unique form id; used for aria-labelledby and submission handlers |
| `title` | `string \| null` | `null` | no | form heading text |
| `description` | `string \| null` | `null` | no | supporting description below the title |
| `sections` | `FormSectionSpec[]` | `[]` | no | ordered section definitions with field id lists |
| `fields` | `FormFieldState[]` | `[]` | no | per-field validation state tracking |
| `actions` | `FormActionLayout` | `{ align: "end", actionCount: 0 }` | no | action row alignment and count |
| `statusSummary` | `FormStatusSummary \| null` | `null` | no | form-level status with tone and message |
| `isDisabled` | `boolean` | `false` | no | blocks all interaction across the form |
| `isBusy` | `boolean` | `false` | no | indicates in-flight submission; implies disabled semantics |

### Types

```ts
type FormSectionSpec = {
  id: string;
  title: string;
  description: string | null;
  fieldIds: string[];
};

type FormFieldState = {
  id: string;
  label: string;
  validationState: ValidationState;
  message: string | null;
  isRequired: boolean;
  isDisabled: boolean;
};

type FormActionLayout = {
  align: "start" | "center" | "end" | "between";
  actionCount: number;
};

type FormStatusSummary = {
  tone: StatusTone;
  message: string;
};
```

### Derived Helpers

Defined on `FormShellSpec`:

- `invalid_field_count()` — count of fields with `validation_state == Invalid`
- `pending_field_count()` — count of fields with `validation_state == Pending`
- `blocks_submission()` — `is_disabled || is_busy || invalid_field_count > 0`
- `resolved_status_tone()` — derives effective tone from field state:
  - `Danger` when any field is invalid
  - `Pending` when any field is pending or `is_busy`
  - otherwise falls back to `status_summary.tone` or `Neutral`

### Controlled And Uncontrolled

- `fields` and `status_summary` are externally owned — the caller updates them
  as validators run
- Submission is host-driven; the component exposes submission gating via
  `blocks_submission()` rather than owning a submit handler

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | default, no invalid fields | actions enabled, neutral status |
| busy | `isBusy=true` | fields read-only, spinner in status summary, actions disabled |
| blocked | any field invalid | status tone danger, actions disabled |
| pending | any field pending validation | status tone pending, actions disabled |
| disabled | `isDisabled=true` | entire form dimmed via `state.opacity.disabled`, no interaction |

## 5. Accessibility

- Root element: `<form aria-labelledby>` pointing to title id
- Status summary is a `<Callout>` with `announceMode="polite"` (or assertive
  when tone is danger and submission was attempted)
- Per-field validation message is wired via `aria-describedby` on each field
- Disabled state must not suppress programmatic focus management

## 6. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `space.stack.lg` | outer stack gap (from `stack_gap_token()`) |
| Sections | `space.stack.md` | gap between sections (from `section_gap_token()`) |
| Title | `typography.heading.size` | form title |
| Description | `typography.body.size`, `color.text.secondary` | supporting text |
| StatusSummary | delegates to `Callout` token chain | form-level status |

## 7. Rust Spec

- Rust type: `poodle_specs::FormShellSpec`
- File: `packages/contracts/components/src/form_shell.rs`
- Introduced by baseline: `054-gpui-form-validation-and-remediation-composite-baseline`

## 8. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Svelte equivalent may be `FormLayout` composite | Svelte's form composite landed under a different name; field-groupings and validation tracking overlap but the two are not literally identical | allowed | consolidate names in a future pass |
| GPUI does not render per-field messages via aria-describedby | GPUI 0.2 has no ARIA surface | allowed | revisit when GPUI accessibility lands |
