# InlineRemediation

> **Implementation note**: Web callers compose this shape with `Callout` by
> design. Native runtimes use the shared `InlineRemediation` renderer.

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `InlineRemediation`
- Layer: `composites`
- Summary: a compact inline recovery affordance attached to one or more form
  fields, combining tone-styled messaging with an optional remediation action.
  Distinct from `RemediationBanner` (which is form-level) in that it lives
  between fields and references specific field ids.
- In scope: tone-styled border treatment, short title, required message text,
  referenced-field tracking for cross-field error attribution, optional single
  remediation action
- Out of scope: form-level status summaries (use `FormShell.status_summary`),
  standalone announcements (use `Callout`), multi-action recovery flows
  (use `RemediationBanner`)

## 2. Anatomy

```text
[Root .inline-remediation]  <aside>
  ├── [Border .inline-remediation__border]  (tone-colored left border)
  ├── [Content .inline-remediation__content]
  │     ├── [Title .inline-remediation__title]  <strong> (optional)
  │     └── [Message .inline-remediation__message]  <p>
  └── [Action]  RemediationAction button (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | inline aside element between fields | padding, background |
| Border | yes | tone-colored left border | `border_token()` (tone) |
| Title | no | bold inline heading | typography-label |
| Message | yes | short recovery message | typography-body, text-secondary |
| Action | no | single action button | delegates to Button primitive |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `StatusTone` | `"info"` | no | border tone and optional leading-icon color |
| `title` | `string \| null` | `null` | no | inline heading above the message |
| `message` | `string` | — | yes | primary recovery message text |
| `referencedFieldIds` | `string[]` | `[]` | no | field ids this remediation applies to; used for `aria-describedby` wiring |
| `action` | `RemediationAction \| null` | `null` | no | single recovery action button |

### Types

```ts
type RemediationAction = {
  id: string;
  label: string;
  variant: ButtonVariant;
  isDisabled: boolean;
};
```

### Derived Helpers

- `reference_count()` — count of ids in `referenced_field_ids`
- `is_actionable()` — `true` when `action` is set

### Controlled And Uncontrolled

- Wholly controlled by parent — no internal state
- Action click dispatches through the caller-supplied handler; the component
  itself does not own recovery logic

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| info | `tone="info"` (default) | info-tinted border |
| warning | `tone="warning"` | warning-tinted border |
| danger | `tone="danger"` | danger-tinted border |
| with-action | `action` is set | action button rendered right-aligned |
| actionless | `action` is null | pure messaging treatment |

## 5. Accessibility

- Root element: `<aside>` with `aria-labelledby` on title when present
- When `referencedFieldIds` is populated, each referenced field should point
  its `aria-describedby` to this remediation's id
- Non-announcing by default — intended as ambient context, not an interrupt

## 6. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Border | tone → `color.status.*` | tone-colored left accent (from `border_token()`) |
| Root gap | `space.stack.sm` | title-to-message vertical gap (from `gap_token()`) |

## 7. Rust Spec

- Rust type: `poodle_specs::InlineRemediationSpec`
- File: `packages/contracts/components/src/inline_remediation.rs`
- Introduced by baseline: `054-gpui-form-validation-and-remediation-composite-baseline`

## 8. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Web uses `Callout` for inline recovery | The web primitive already owns tone, title, message, and action composition; a second public component would duplicate that surface | intentional | keep the convention documented rather than adding an alias |
