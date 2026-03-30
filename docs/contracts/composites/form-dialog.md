# FormDialog

Status: seed contract
Updated: 2026-03-28

## 1. Purpose

- Component name: `FormDialog`
- Layer: `composites`
- Summary: a dialog shell for form workflows — wraps a Dialog with a
  FormLayout body, submitting-state dismiss control, status callouts, optional
  default submit/cancel actions, and caller-owned custom actions when needed
- In scope: controlled open state, title and subtitle/description, form
  content via default slot, optional default submit and cancel buttons, custom
  actions slot, status callouts, dismiss prevention during submission, custom
  dialog width
- Out of scope: form validation logic, field-level errors (handled by
  individual Field components), multi-step forms, file upload handling

## 2. Anatomy

```text
[Dialog]  Dialog primitive (kind="dialog")
  ├── [Subtitle slot]  optional rich subtitle content
  ├── [FormLayout]  FormLayout composite
  │     ├── [ErrorDisplay]  (when error is set, handled by FormLayout)
  │     ├── [SuccessDisplay]  (when success is set, handled by FormLayout)
  │     └── [FormContent]  (via default slot)
  └── [Actions slot]
        ├── [Default actions]  optional cancel + submit buttons
        └── [Custom actions]  caller-owned slot content when shell mode is used
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Dialog | yes | Dialog primitive with controlled open state | delegates to Dialog contract |
| FormLayout | yes | FormLayout composite for form structure and error display | delegates to FormLayout contract |
| FormContent | yes (via slot) | form fields and content provided by consumer | layout delegated to FormLayout |
| CancelButton | yes | ghost Button for canceling; disabled during submission | delegates to Button contract |
| SubmitButton | yes | primary Button for submission; shows "Submitting..." text and is disabled during submission | delegates to Button contract |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null` | `null` | no | controlled open state; `null` means uncontrolled |
| `title` | `string` | — | yes | dialog title |
| `subtitle` | `string \| null` | `null` | no | primary description text used by shell-style callers |
| `description` | `string \| null` | `null` | no | alias/fallback description text |
| `submitLabel` | `string` | `"Submit"` | no | label for the submit button (shown when not submitting) |
| `cancelLabel` | `string` | `"Cancel"` | no | label for the cancel button |
| `submitting` | `boolean` | `false` | no | when true, disables dismiss and shows submitting state |
| `error` | `string \| null` | `null` | no | form-level error message passed to FormLayout |
| `success` | `string \| null` | `null` | no | form-level success message passed to FormLayout |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the dialog |
| `width` | `string \| null` | `null` | no | custom dialog width CSS value |
| `showDefaultActions` | `boolean` | `true` | no | when false, suppresses built-in cancel/submit buttons and expects an `actions` slot |

### Slots

| Slot | Provided Context | Description |
|------|-----------------|-------------|
| default | `{ submitting: boolean }` | form content (fields, inputs); receives submitting state for conditional rendering |
| `subtitle` | `{ submitting: boolean }` | optional rich subtitle content rendered below the title |
| `actions` | `{ submitting: boolean }` | optional custom footer actions; replaces default submit/cancel buttons |

### Controlled And Uncontrolled

- `open` is controlled externally; host must handle `openChange` event to
  update the value
- When `open` is `null`, the Dialog manages its own visibility

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | `open` is falsy/null | dialog not visible |
| open | `open` is true | dialog visible with form content and action buttons |
| submitting | `submitting` is true | submit button shows "Submitting..." text, both buttons disabled, dismiss on Escape and backdrop disabled |
| error | `error` is set | FormLayout displays error message above form content |
| success | `success` is set | FormLayout displays success message above form content |
| shell mode | `showDefaultActions` is false | caller supplies the full footer via the `actions` slot |

### Component States

No additional internal state. All state is externally controlled via props.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `submit` | user clicks the built-in submit button | `void` | host is responsible for async logic and setting `submitting`/`error` |
| `cancel` | dialog requests cancellation | `void` | fired for built-in cancel button and close/dismiss requests |
| `openChange` | dialog open state changes (cancel, Escape, backdrop) | `{ open: boolean }` | host must update `open` prop; prevented during submitting |

## 6. Accessibility

### Semantics

- Dialog: `kind="dialog"` — standard dialog semantics (not alertdialog)
- Dialog: optional `ariaLabel` for custom accessible name
- CancelButton and SubmitButton: standard button semantics
- Both buttons: `disabled` attribute set during submitting state
- Dismiss: Escape and backdrop dismiss are disabled during submitting
  (`dismissOnEscape={!submitting}`, `dismissOnBackdrop={!submitting}`)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Escape` | closes the dialog (when not submitting); delegated to Dialog |
| `Tab` | cycles focus within the dialog (focus trap); delegated to Dialog |
| `Enter` / `Space` | activates focused button |

### Focus And Announcement

- Focus trap: delegated to Dialog primitive
- Initial focus: delegated to Dialog primitive (typically first focusable element)
- Focus return: delegated to Dialog primitive (returns to trigger on close)

## 7. Layout

### Sizing

- Dialog: default Dialog sizing unless `width` is supplied
- FormLayout: `columns={1}` — single-column form layout
- Actions: standard Dialog actions slot layout (flex row, right-aligned)

### Composition

- Parent expectations: any view needing a modal form (user creation, settings edit, etc.)
- Child expectations: Dialog primitive, FormLayout composite, Button primitive
- Resizing rules: delegated to Dialog; form content scrolls if it exceeds dialog height

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Dialog | (delegates to Dialog) | all Dialog tokens |
| FormLayout | (delegates to FormLayout) | error display, form spacing tokens |
| CancelButton | (delegates to Button) | ghost variant tokens |
| SubmitButton | (delegates to Button) | primary variant tokens |

No component-specific tokens; FormDialog is purely compositional.

## 9. Svelte Notes

- Uses `createEventDispatcher` for `submit`, `cancel`, and `openChange` events
- Composes `Dialog`, `Button` from `@poodle/svelte-primitives` and `FormLayout`
  from `@poodle/svelte-composites`
- Submit button text switches between `submitLabel` and `"Submitting..."`
- Cancel handler calls internal `setOpen(false)` which emits `openChange`
  when `open !== null`
- `handleOpenChange` prevents close during submitting state
- `subtitle` takes precedence over `description`
- custom width is applied through the Dialog content style hook

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::form_dialog`
- Compose Dialog, FormLayout, and Button from their respective crates
- Submitting state must prevent dismiss and disable buttons

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all shell props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] submitting state disables dismiss and buttons
- [ ] submit button text changes during submitting
- [ ] slot context provides submitting boolean

### Tier 2: Visual Parity

- [ ] action button layout matches (ghost cancel left, primary submit right)
- [ ] error display via FormLayout matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| `subtitle` aliases `description` | supports shell-style callers without introducing a second dialog component | approved | remove ambiguity later if callers converge |

## 13. Specimen Definitions

### Basic Form Dialog

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Basic form dialog | `title="Add new user"`, `description="Invite a user to this workspace."`, `submitLabel="Add user"`, form contains Field(name) + Field(role as Select) | trigger Button opens dialog; submitting state shows "Submitting..." then closes after 1.2s |

### With Error State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With error state | `title="Create account"`, `submitLabel="Create"`, form contains Field(email), error set after 0.8s submit | dialog shows inline error message "A user with this email already exists." via FormLayout |

### Shell Mode With Custom Actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Shell mode with custom actions | `title="Edit workspace settings"`, `subtitle="Update shared defaults for this workspace."`, `showDefaultActions=false`, `width="40rem"`, success set after save, actions slot contains caller-owned FormActions | dialog renders custom footer actions, shows success callout, and uses wider shell sizing |

## 14. Approval And Adoption Notes

- Contract status: `seed contract`
- Approvers: pending
- Downstream adopters: CRUD forms, settings dialogs, user management,
  any modal form workflow
- Future follow-up: consider adding `onSubmit` return value for promise-based
  auto-submitting management; consider form-level validation callback;
  consider `size` prop pass-through to Dialog
