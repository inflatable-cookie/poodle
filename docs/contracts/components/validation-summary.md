# ValidationSummary

> **Implementation note**: Implemented in the shared Rust renderer, Svelte, and React.

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `ValidationSummary`
- Layer: `composites`
- Summary: a grouped error-surface that lists all currently-invalid fields in
  a form, announcing them together after a failed submission attempt. Serves
  as the landing target for a "jump to first error" flow.
- In scope: entry list derived from per-field validation state, optional
  inclusion of pending fields, announcement mode for assistive tech, title,
  blocking-entry count derivation, accent vs danger border based on whether
  any entries are blocking
- Out of scope: per-field inline messaging (use `InlineRemediation` or a
  field's built-in message), recovery actions (use `RemediationBanner`),
  form-level status (use `FormShell.status_summary`)

## 2. Anatomy

```text
[Root .validation-summary]  <aside>
  ├── [Title .validation-summary__title]  <strong> (optional)
  └── [List .validation-summary__list]  <ul>
        └── [Entry .validation-summary__entry]  <li> (one per active entry)
              ├── [Label]  linked <a href="#field-id">
              └── [Message]  <span>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | grouped error surface | background, border, radius, padding |
| Title | no | heading above the list | typography-label |
| List | yes | unordered list of entries | list spacing |
| Entry | yes | one row per invalid field | spacing, text |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | heading text |
| `entries` | `ValidationSummaryEntry[]` | `[]` | no | all tracked entries; the component filters active ones |
| `announceMode` | `AnnouncementMode` | `"polite"` | no | `"polite"`, `"assertive"`, or `"none"` |
| `includePending` | `boolean` | `false` | no | include pending fields alongside invalid ones |

### Types

```ts
type ValidationSummaryEntry = {
  fieldId: string;
  label: string;
  message: string;
  validationState: ValidationState;
};

type AnnouncementMode = "none" | "polite" | "assertive";
```

### Derived Helpers

- `active_entries()` — entries filtered to `Invalid`, plus `Pending` when
  `include_pending` is true
- `blocking_entry_count()` — count of entries with `is_blocking()` true
  (currently synonymous with `Invalid`)
- `accessibility_role()` — `"status"` / `"alert"` / none per announce mode

### Controlled And Uncontrolled

- Wholly controlled by parent; `entries` reflect current validation state of
  the owning form
- When empty (no active entries), the component may render nothing or an
  explicit "clean" state depending on caller preference

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no active entries | component renders nothing |
| blocking | one or more `Invalid` entries | danger-bordered surface with entry list |
| pending-only | `includePending=true` and only Pending entries | accent-bordered surface |
| mixed | both Pending and Invalid | danger border takes precedence |
| polite | `announceMode="polite"` | `role="status"`, `aria-live="polite"` |
| assertive | `announceMode="assertive"` | `role="alert"`, `aria-live="assertive"` |

## 5. Accessibility

- Root element: a neutral container with the live-region role derived from
  `announceMode`. A silent host may use `<aside>` instead.
- Each entry renders as `<li>` containing an `<a href="#field-id">` that jumps
  focus to the referenced field when activated
- Component is typically announced once after failed submission; a subsequent
  fix should move it back to a polite or silent mode

## 6. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Fill | `color.background.panel` | base grouped surface |
| Border | `color.status.danger` (blocking) or `color.accent.base` (pending-only) | from `border_token()` |
| Radius | `radius.surface` | grouped-surface shape |
| Padding | `space.panel.x`, `space.panel.y` | internal spacing |
| Title | `typography.label` | summary heading |
| Entry | `typography.body`, `color.text.secondary` | linked validation copy |

## 7. Rust Spec

- Rust type: `poodle_specs::ValidationSummarySpec`
- File: `packages/contracts/components/src/validation_summary.rs`
- Introduced by baseline: `054-gpui-form-validation-and-remediation-composite-baseline`

## 8. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Focus jump via href anchor is web-only | GPUI and Jetstream must emulate with imperative focus calls | allowed | document platform-specific focus move |
| GPUI cannot emit aria-live | GPUI 0.2 has no ARIA surface | allowed | revisit with GPUI accessibility layer |
