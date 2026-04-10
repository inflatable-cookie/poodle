# FormDialog

Status: detailed contract
Updated: 2026-04-09

## 1. Purpose

- Component name: `FormDialog`
- Layer: `composites`
- Summary: a dialog shell for form workflows — wraps a Dialog with a
  FormLayout body, submitting-state dismiss control, status callouts, optional
  default submit/cancel actions, and caller-owned custom actions when needed
- In scope: controlled open state, title and subtitle/description, form
  content via default slot, optional default submit and cancel buttons, custom
  actions slot, status callouts via FormLayout, dismiss prevention during
  submission, custom dialog width via CSS custom property
- Out of scope: form validation logic, field-level errors (handled by
  individual Field components), multi-step forms, file upload handling

## 2. Anatomy

```text
[Dialog]  Dialog primitive (kind="dialog")
  ├── [Header .form-dialog__header]  (optional, when subtitle slot provided)
  │     └── [Subtitle .form-dialog__subtitle]  <div>
  │           └── (slot: subtitle)
  ├── [FormLayout]  FormLayout composite (columns={columns})  -- when bare=false
  │     ├── [ErrorDisplay]  (when error is set, handled by FormLayout)
  │     ├── [SuccessDisplay]  (when success is set, handled by FormLayout)
  │     └── [FormContent]  (via default slot)
  ├── [BareContent]  (via default slot, rendered directly)  -- when bare=true
  └── [Actions slot]
        ├── [Custom actions]  (slot: actions) — when provided
        └── [Default actions]  when resolvedShowActions=true and no actions slot
              ├── [CancelButton]  Button (variant="ghost")
              └── [SubmitButton]  Button (variant="primary")
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Dialog | yes | Dialog primitive with controlled open state | delegates to Dialog contract |
| Header | no | subtitle container rendered when subtitle slot is provided | margin-bottom spacing |
| Subtitle | no | rich subtitle content area | text-secondary, body typography |
| FormLayout | conditional | FormLayout composite for form structure and error/success display; rendered when `bare=false` | delegates to FormLayout contract |
| BareContent | conditional | slot content rendered directly without FormLayout wrapper; rendered when `bare=true` | none (consumer controls layout) |
| FormContent | yes (via slot) | form fields and content provided by consumer | layout delegated to FormLayout (or consumer when bare) |
| CancelButton | conditional | ghost Button for canceling; disabled during submission | delegates to Button contract |
| SubmitButton | conditional | primary Button for submission; shows "Submitting..." text when submitting | delegates to Button contract |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null` | `null` | no | controlled open state; `null` means uncontrolled (Dialog manages visibility) |
| `title` | `string` | — | yes | dialog title |
| `subtitle` | `string \| null` | `null` | no | primary description text; takes precedence over `description` |
| `description` | `string \| null` | `null` | no | alias/fallback description text; used when `subtitle` is null |
| `submitLabel` | `string` | `"Submit"` | no | label for the submit button (shown when not submitting) |
| `cancelLabel` | `string` | `"Cancel"` | no | label for the cancel button |
| `submitting` | `boolean` | `false` | no | when true, disables dismiss and shows submitting state |
| `error` | `string \| null` | `null` | no | form-level error message passed to FormLayout |
| `success` | `string \| null` | `null` | no | form-level success message passed to FormLayout |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the dialog |
| `width` | `string \| null` | `null` | no | custom dialog width CSS value (applied via `--poodle-form-dialog-width`) |
| `columns` | `number` | `6` | no | number of columns passed through to FormLayout; ignored when `bare=true` |
| `showDefaultActions` | `boolean` | `true` | no | when false, suppresses built-in cancel/submit buttons; expects an `actions` slot; automatically set to false when `bare=true` |
| `bare` | `boolean` | `false` | no | when true, renders the default slot directly without a FormLayout wrapper and automatically sets `showDefaultActions` to false; use for embedding reusable form components that already own their own FormLayout |
| `size` | `ControlSize \| null` | `null` | no | explicit semantic size override for dialog and action controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for dialog and action controls |

### Slots

| Slot | Provided Context | Description |
|------|-----------------|-------------|
| default | `{ submitting: boolean }` | form content (fields, inputs); receives submitting state for conditional rendering |
| `subtitle` | `{ submitting: boolean }` | optional rich subtitle content rendered below the title; when provided, the `description` prop is not passed to Dialog |
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
| bare mode | `bare` is true | slot content rendered directly without FormLayout wrapper; `showDefaultActions` automatically set to false |
| custom width | `width` is set | dialog surface uses the specified width (capped at 100%) |

### Component States

No additional internal state. All state is externally controlled via props.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `submit` | user clicks the built-in submit button | `void` | host is responsible for async logic and setting `submitting`/`error` |
| `cancel` | dialog requests cancellation (cancel button, Escape, backdrop, close button) | `void` | fired before `openChange` when applicable |
| `openChange` | dialog open state changes | `{ open: boolean }` | host must update `open` prop; cancel event also fires when closing while not submitting |

## 6. Accessibility

### Semantics

- Dialog: `kind="dialog"` — standard dialog semantics (not alertdialog)
- Dialog: `showCloseButton={true}` — always shows the Dialog close button
- Dialog: optional `ariaLabel` for custom accessible name
- CancelButton and SubmitButton: standard button semantics
- Both buttons: `disabled` attribute set during submitting state
- Dismiss: Escape and backdrop dismiss disabled during submitting
  (`dismissOnEscape={!submitting}`, `dismissOnBackdrop={!submitting}`)
- Description: when subtitle slot is provided, `description` is not passed to
  Dialog to avoid duplicate description announcement

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
- Custom width: applied via `--poodle-form-dialog-width` CSS custom property,
  capped with `min(var(--poodle-form-dialog-width, 34rem), 100%)`
- FormLayout: `columns={columns}` (default 6) — passed through to FormLayout
- When `bare=true`: no FormLayout wrapper; slot content rendered directly,
  allowing embedded components that own their own FormLayout
- Actions: standard Dialog actions slot layout (flex row, right-aligned)

### Composition

- composes: `Dialog` and `Button` from `@poodle/svelte-primitives`,
  `FormLayout` from composites
- parent expectations: any view needing a modal form (user creation, settings
  edit, etc.)
- child expectations: form fields and action primitives via slots
- resizing rules: delegated to Dialog; form content scrolls if it exceeds
  dialog height

## 8. Token Usage — Exact Values

### Dialog Surface Width (`:global(.form-dialog__surface)`)

| Property | Value |
|----------|-------|
| width | `min(var(--poodle-form-dialog-width, 34rem), 100%)` |

### Header (`.form-dialog__header`)

| Property | Value |
|----------|-------|
| margin-bottom | `var(--poodle-space-stack-md)` |

### Subtitle (`.form-dialog__subtitle`)

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-secondary)` |
| font-size | `var(--poodle-typography-body-size, 0.875rem)` |
| line-height | `var(--poodle-typography-body-lineHeight, 1.5)` |

### Composed Primitives

All other token usage delegates to the respective primitive/composite
contracts:

| Part | Delegates To |
|------|-------------|
| Dialog | Dialog contract (foundation) |
| FormLayout | FormLayout contract (composites) |
| CancelButton | Button contract (foundation), variant="ghost" |
| SubmitButton | Button contract (foundation), variant="primary" |

### Light Theme Overrides

None.

## 9. Svelte Notes

- uses `createEventDispatcher` for `submit`, `cancel`, and `openChange` events
- composes `Dialog`, `Button` from `@poodle/svelte-primitives` and `FormLayout`
  from `@poodle/svelte-composites`
- submit button text switches between `submitLabel` and `"Submitting..."`
- cancel handler calls internal `setOpen(false)` which emits `openChange`
  when `open !== null`
- `handleOpenChange` prevents cancel event during submitting state but
  always forwards `openChange`
- `subtitle` takes precedence over `description` via `resolvedDescription`
  reactive binding
- when subtitle slot is provided, `description` is set to null on the Dialog
  to avoid duplication
- custom width is applied through `contentStyle` and `contentClassName`
  props on Dialog; the global `.form-dialog__surface` class sets the width
- Dialog receives `showCloseButton={true}` always
- `bare` mode: when true, the default slot is rendered directly without a
  FormLayout wrapper; `resolvedShowActions` is derived as `bare ? false : showDefaultActions`
- `columns` prop (default 6) is passed through to `FormLayout` as `{columns}`;
  ignored when bare mode is active since FormLayout is not rendered
- `size`, `sizeRole`, and `density` are resolved via `getUiPresentation()` and
  passed through to Dialog and action Buttons

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::form_dialog`
- spec struct: `FormDialogSpec` with open, title, description, submit/cancel
  labels, submitting, error, success, width
- compose Dialog, FormLayout, and Button from their respective crates
- submitting state must prevent dismiss and disable buttons

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all shell props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] submitting state disables dismiss and buttons
- [ ] submit button text changes to "Submitting..." during submitting
- [ ] slot context provides submitting boolean
- [ ] subtitle takes precedence over description
- [ ] cancel event fires on Escape/backdrop/cancel button (not during submitting)
- [ ] `bare` mode renders slot directly without FormLayout, sets showDefaultActions to false
- [ ] `columns` prop passes through to FormLayout (default 6)

### Tier 2: Visual Parity

- [ ] action button layout matches (ghost cancel left, primary submit right)
- [ ] error/success display via FormLayout matches
- [ ] custom width CSS treatment matches
- [ ] subtitle typography matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Specimen Definitions

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
