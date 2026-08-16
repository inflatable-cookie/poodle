# ModelConnectionSetup

Status: approved
Updated: 2026-08-14
Governing spec: `../../specs/067-model-connection-management.md`

## 1. Purpose

- Component name: `ModelConnectionSetup`
- Layer: `composites`
- Summary: an adaptive shell for choosing one exact model connection and,
  when required, completing host-owned setup or detection
- In scope: direct add, choose/configure navigation, selected-route summary,
  workflow actions, pending lock, safe feedback, configuration composition
- Out of scope: credentials, OAuth, discovery, validation, persistence,
  provider form schemas, overlay ownership, or arbitrary workflow steps

## 2. Anatomy

```text
[Root] <section>
  ├── choose
  │   ├── [ModelConnectionPicker]
  │   └── [Actions] Cancel + Continue or Add
  └── configure
      ├── [Header] selected provider and route
      ├── [Feedback] optional safe error/success
      ├── [Configuration snippet]
      ├── [Pending status] optional live region
      └── [Actions] Back, Cancel, Add
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | workflow region | surface, stack gap |
| Picker | choose | exact route selection; options declare whether configuration is required | delegated |
| Header | configure | selected route context | heading, secondary text |
| Configuration | configure | host-owned fields/actions | inset surface, stack gap |
| Feedback | no | form-level status | status tokens |
| Actions | configure | consistent workflow controls | separator, inline gap |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `stage` | `"choose" \| "configure" \| undefined` | `undefined` | no | controlled stage |
| `defaultStage` | `"choose" \| "configure"` | `"choose"` | no | uncontrolled initial stage |
| `options` | `ModelConnectionOption[]` | `[]` | no | forwarded to picker; `requiresConfiguration` selects direct or configured flow |
| `value` | `string \| null \| undefined` | `undefined` | no | controlled selected option id |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial selection |
| `query` | `string \| undefined` | `undefined` | no | controlled picker query |
| `pickerState` | `PickerState` | `"ready"` | no | picker catalogue posture |
| `title` | `string` | `"Add model connection"` | no | workflow heading |
| `description` | `string \| null` | `null` | no | workflow description |
| `canSubmit` | `boolean` | `false` | no | host-approved Add eligibility |
| `isPending` | `boolean` | `false` | no | locks workflow actions during host work |
| `pendingLabel` | `string` | `"Checking connection"` | no | announced pending copy |
| `error` | `string \| null` | `null` | no | safe form-level error |
| `success` | `string \| null` | `null` | no | safe form-level success |
| `continueLabel` | `string` | `"Continue"` | no | choose action |
| `submitLabel` | `string` | `"Add connection"` | no | configure submit action |
| `backLabel` | `string` | `"Back"` | no | configure back action |
| `cancelLabel` | `string` | `"Cancel"` | no | cancellation action |
| `ariaLabel` | `string \| null` | `null` | no | falls back to title |
| `onStageChange` | `((stage) => void) \| null` | `null` | no | stage request |
| `onValueChange` | `((id: string) => void) \| null` | `null` | no | picker selection request |
| `onQueryChange` | `((query: string) => void) \| null` | `null` | no | picker query request |
| `onSubmit` | `((id: string) => void) \| null` | `null` | no | add request for exact selected id |
| `onCancel` | `(() => void) \| null` | `null` | no | cancel request |

### Snippets / Render Props

| Name | Input | Purpose |
|------|-------|---------|
| `leading` | `{ option }` | passed to picker and selected summary |
| `configuration` | `{ option, isPending }` | host form, OAuth, detection, or guidance |
| `configureAside` | `{ option }` | optional secondary guidance |

### Controlled And Uncontrolled

Stage, selection, and query are independently controllable. A selected
available option with `requiresConfiguration=false` presents Add and submits
from `choose`; no configure stage is emitted. Other options present Continue
and move to `configure`. Back returns to `choose` without clearing selection
or host-owned field state.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| choose | initial stage | searchable picker plus Cancel and route-dependent Continue/Add |
| direct add | selected option does not require configuration | Add submits from choose; configure is skipped |
| configure | continued/controlled | selected summary and injected content |
| pending | `isPending` | spinner/status; navigation and submit locked |
| invalid | `canSubmit=false` | Add disabled; content remains interactive unless pending |
| error | `error` | danger feedback, no stage reset |
| success | `success` | positive feedback, host decides close timing |

### Behavior Machine

Behavior classification: machine-backed.

Context: controllable stage, selected id, query, option configuration
requirement, `canSubmit`, `isPending`.
Events: `SELECT`, `SET_QUERY`, `CONTINUE`, `BACK`, `SUBMIT`, `CANCEL`, and
programmatic setters. Pending guards every workflow event except programmatic
state replacement. `CONTINUE` requires an available selected option which
requires configuration. `SUBMIT` requires an exact selection and `canSubmit`;
from choose it additionally requires a selectable direct-add option. Effects
emit only the documented callbacks. Machinery dependencies: picker selection
machinery, id wiring, and polite status announcement.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onStageChange` | Continue or Back accepted | next stage | direct add does not emit a stage |
| `onValueChange` | picker selection changes | exact option id | pass-through |
| `onQueryChange` | picker query changes | string | pass-through |
| `onSubmit` | enabled Add activated | selected id | host performs validation/persistence |
| `onCancel` | enabled Cancel activated | none | host owns overlay closure |

## 6. Accessibility

- Root is a labelled region, not a dialog.
- Configured-flow stage changes move focus to the new visible heading; Back restores focus to
  the selected option when practical.
- Pending status uses `role="status"`, `aria-live="polite"`.
- Errors use `role="alert"`; host fields retain their own error association.
- Tab order follows visible content then workflow actions.
- GPUI later exposes equivalent region, busy, status, and focus movement.

## 7. Layout

- Root is a vertical grid with one scrollable body and fixed workflow actions.
- Configuration content may be one or two columns through `FormLayout`.
- At narrow widths, header metadata and actions stack.
- Outer modal/drawer width and maximum height belong to the host.

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `space.stack.*` | workflow rhythm |
| Header | `color.text.primary`, `color.text.secondary` | hierarchy |
| Configuration | `color.background.surface`, `color.border.subtle` | inset body |
| Actions | `color.border.subtle`, `space.inline.*` | separation |
| Feedback | semantic status tokens | safe outcome posture |

## 9. Svelte Notes

- Compose ModelConnectionPicker, Button, FormActions, Callout, and Spinner.
- Configuration is a snippet; never bind or retain its credential values.
- Do not create an internal Dialog.

## 10. GPUI Notes

- Not implemented in GPUI. The g14 adoption pipeline was rejected; native
  completion must be planned from this contract under the g15 release runway.
- Consumer-rendered configuration content remains the native composition seam.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] stage guards, focus movement, callbacks, and pending lock match
- [ ] direct routes submit from choose without emitting configure
- [ ] injected content receives the same selected option and pending state

### Tier 2: Visual Parity

- [ ] heading, body, feedback, and actions retain the same hierarchy

### Tier 3: Implementation Freedom

- [ ] host overlay and form implementation do not leak into the contract

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Native implementation missing | rejected g14 pilot supplied no adoption path | open runtime gap; not parity-certified | g15 release-gap inventory |

## 13. Approval And Adoption Notes

- contract status: `approved`
- approver: operator, 2026-08-14
- downstream adopter: Nucleus
- future follow-up: compile native completion from the g15 release-gap inventory
