# ConfirmAction

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `ConfirmAction`
- Layer: `composites`
- Summary: a convenience composite that pairs a trigger button with an
  AlertDialog for confirming destructive or significant actions — handles
  open/close state internally, supports custom trigger snippet and body content
- In scope: default trigger button, custom trigger snippet, configurable tone
  (danger/warning), confirm/cancel labels, AlertDialog composition, body
  content snippet, internal open state management, size and density support
- Out of scope: multi-step confirmation, undo workflows, inline confirmation
  patterns (non-dialog), form submission within the dialog, async loading
  state during confirmation

## 2. Anatomy

```text
[TriggerZone]
  ├── [TriggerSlot .confirm-action__trigger]  <span role="presentation"> (when trigger snippet provided)
  │     └── (snippet: trigger)
  └── [DefaultTrigger]  Button (variant="secondary", tone derived) (when no trigger snippet)

[AlertDialog]  AlertDialog primitive
  └── [BodySlot]  (optional, via children snippet)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| TriggerZone | yes | either a custom trigger snippet wrapper or default Button | delegates to Button or custom element |
| DefaultTrigger | conditional | Button with `variant="secondary"` and tone derived from the `tone` prop | delegates to Button contract |
| TriggerSlot | conditional | `<span role="presentation">` wrapper with click/keydown handlers | layout only |
| AlertDialog | yes | AlertDialog primitive managing the confirmation dialog | delegates to AlertDialog contract |
| BodySlot | no | additional content rendered inside the AlertDialog body | layout delegated to consumer |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | — | yes | AlertDialog title text |
| `description` | `string \| null` | `null` | no | AlertDialog description text |
| `tone` | `"danger" \| "warning"` | `"danger"` | no | visual tone; maps to AlertDialog tone and default trigger Button tone |
| `triggerLabel` | `string` | `"Delete"` | no | label for the default trigger Button (ignored when trigger snippet is used) |
| `confirmLabel` | `string` | `"Confirm"` | no | label for the AlertDialog confirm button |
| `cancelLabel` | `string` | `"Cancel"` | no | label for the AlertDialog cancel button |
| `onConfirm` | `(() => void \| Promise<void>) \| null` | `null` | no | callback invoked when the confirm action is accepted |
| `onCancel` | `(() => void) \| null` | `null` | no | callback invoked when the dialog is canceled or dismissed |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl" \| null` | `null` | no | explicit control size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Types

```ts
type AlertDialogTone = "danger" | "warning";
```

### Snippets

| Snippet | Description |
|------|-------------|
| `trigger` | custom trigger element; replaces the default Button |
| `children` | body content rendered inside the AlertDialog, between description and action buttons |

### Controlled And Uncontrolled

- `open` state is managed internally; no external control prop
- the dialog opens when the trigger is activated and closes on confirm,
  cancel, or backdrop/escape dismiss

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | initial / after confirm or cancel | only the trigger is visible |
| open | trigger activated | AlertDialog is shown with title, description, body, and action buttons |

### Component States

- `open` (internal boolean): controls AlertDialog visibility directly
- Derived: `triggerTone` — maps `tone === "danger"` to `"danger"` Button tone,
  all others to `"default"`

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Reclassified (g11 extraction sweep): ConfirmAction composes Dialog, which
runs the shared modal machine — escape/backdrop dismissal is machine-backed
via composition. The confirm/cancel flow itself is plain open state with no
component-owned behavioral logic; no separate machine is warranted.

## 5. Callbacks

| Prop | When It Fires | Payload | Notes |
|------|---------------|---------|-------|
| `onConfirm` | user clicks the confirm button in the AlertDialog | `void` | dialog closes after completion |
| `onCancel` | user clicks cancel, presses Escape, or clicks backdrop | `void` | dialog closes after firing |

## 6. Accessibility

### Semantics

- Default trigger: standard Button semantics (variant="secondary", derived tone)
- Custom trigger wrapper: `<span role="presentation">` with click and keydown
  handlers — the slotted element is expected to be an interactive element
  (Button, IconButton, etc.)
- AlertDialog: delegates all dialog accessibility to the AlertDialog primitive
  (role="alertdialog", aria-labelledby, aria-describedby, focus trap)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | activates the trigger (default button or custom trigger wrapper keydown handler) |
| `Escape` | closes the AlertDialog (delegated to AlertDialog) |
| `Tab` | cycles focus within the open AlertDialog (delegated to AlertDialog) |

### Focus And Announcement

- when opened: focus moves to the AlertDialog (delegated to AlertDialog primitive)
- when closed: focus returns to the trigger element
- custom trigger wrapper handles `keydown` for Enter and Space with
  `e.preventDefault()` to avoid double-activation

## 7. Layout

### Sizing

- trigger: inline; sized by the Button or custom content
- custom trigger wrapper: `<span>` wrapper does not add layout
- AlertDialog: centered modal (delegated to AlertDialog contract)

### Composition

- composes: `AlertDialog` and `Button` from `@poodle/svelte`
- parent expectations: toolbars, list item actions, settings forms
- child expectations: AlertDialog primitive, Button primitive, optional body
  content via `children` snippet
- resizing rules: trigger is inline, dialog is modal overlay

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | trigger `<span>` (custom trigger) | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | trigger `<span>` (custom trigger) | `"compact"`, `"default"`, `"comfortable"` |

### Trigger Tone Derivation

| `tone` prop | Default trigger Button `tone` |
|-------------|------------------------------|
| `"danger"` | `"danger"` |
| `"warning"` | `"default"` |

### AlertDialog Props Mapping

| ConfirmAction Prop | AlertDialog Prop |
|-------------------|-----------------|
| `open` | `open` |
| `title` | `title` |
| `description` | `description` |
| `tone` | `tone` |
| `confirmLabel` | `confirmLabel` |
| `cancelLabel` | `cancelLabel` |

### Composed Primitives

All token usage delegates to the respective primitive contracts:

| Part | Delegates To |
|------|-------------|
| DefaultTrigger | Button contract (foundation), variant="secondary", tone derived |
| AlertDialog | AlertDialog contract (foundation) |

### Light Theme Overrides

None.

## 9. Svelte Notes

- composes `AlertDialog` and `Button` from `@poodle/svelte`
- `open` is passed straight through to AlertDialog
- handles `openChange` from AlertDialog to sync internal open state
- trigger tone derivation: `tone === "danger" ? "danger" : "default"`
- custom trigger wrapper `<span>` receives `data-size` and `data-density`
  resolved from `resolveSemanticControlSize` and `getUiPresentation`
- `AlertDialogTone`, `ControlSize`, `SemanticControlSizeRole`, `ControlDensity`
  types imported from `@poodle/svelte`
- size resolves via `resolveSemanticControlSize` with `sizeRole="control"`

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::confirm_action`
- spec struct: `ConfirmActionSpec` with title, description, tone, labels,
  size, density
- compose AlertDialog and Button primitives
- internal open state managed by the composite

## 10a. Jetstream Notes

- `ConfirmAction::from_spec(spec, theme).on_trigger(...).on_confirm(...).on_cancel(...)`.
  All three forward to the composed `Button` and `AlertDialog` rather than being
  re-implemented, so `on_cancel` covers the cancel button and every dismissal
  route, as `AlertDialog` does.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] callback names and payloads match
- [ ] trigger tone derivation logic matches (danger->danger, warning->default)
- [ ] custom trigger snippet behavior matches (click and keyboard activation)
- [ ] open state passes straight through to AlertDialog
- [ ] dialog closes after confirm and cancel events

### Tier 2: Visual Parity

- [ ] default trigger renders as secondary Button with correct tone
- [ ] AlertDialog appearance delegates correctly
- [ ] size and density propagation matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal
- [ ] custom trigger wrapper element may differ

## 12. Specimen Definitions

### Default Trigger (Danger)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default trigger (danger) | `title="Delete this record?"`, `description="This record will be permanently removed."`, `triggerLabel="Delete record"`, `confirmLabel="Delete"` | danger-toned secondary button; clicking opens AlertDialog with danger styling |

### Warning Tone

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Warning tone | `title="Archive this project?"`, `description="The project will be moved to the archive..."`, `tone="warning"`, `triggerLabel="Archive project"`, `confirmLabel="Archive"` | default-toned secondary button; AlertDialog uses warning tone |

### Custom Trigger Slot

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Custom trigger | `title="Remove all filters?"`, `description="This will clear all active filters..."`, `tone="warning"`, `confirmLabel="Clear all"`, trigger snippet contains ghost Button "Clear filters" | ghost button as trigger; clicking opens warning AlertDialog |

### With Body Content

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With body content | `title="Revoke API key?"`, `description="This key will immediately stop working."`, `confirmLabel="Revoke"`, default slot shows code block with API key preview | danger trigger button; AlertDialog includes custom body content between description and actions |
