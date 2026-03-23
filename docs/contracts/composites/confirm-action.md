# ConfirmAction

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `ConfirmAction`
- Layer: `composites`
- Summary: a convenience composite that pairs a trigger button with an
  AlertDialog for confirming destructive or significant actions — handles
  open/close state internally
- In scope: default trigger button, custom trigger slot, configurable tone
  (danger/warning), confirm/cancel labels, AlertDialog composition, body
  content slot, internal open state management
- Out of scope: multi-step confirmation, undo workflows, inline confirmation
  patterns (non-dialog), form submission within the dialog

## 2. Anatomy

```text
[TriggerZone]
  ├── [TriggerSlot]  (when "trigger" slot is provided)
  │     └── <span role="presentation"> wrapper with click/keydown handlers
  └── [DefaultTrigger]  (when no "trigger" slot)
        └── Button (variant="secondary", tone derived from prop)

[AlertDialog]  AlertDialog primitive
  └── [BodySlot]  (optional, via default slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| TriggerZone | yes | either a custom trigger slot or default Button | delegates to Button or custom element |
| DefaultTrigger | conditional | Button with `variant="secondary"` and tone matching the ConfirmAction tone | delegates to Button contract |
| AlertDialog | yes | AlertDialog primitive managing the confirmation dialog | delegates to AlertDialog contract |
| BodySlot | no | additional content rendered inside the AlertDialog body | layout delegated to consumer |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | — | yes | AlertDialog title text |
| `description` | `string \| null` | `null` | no | AlertDialog description text |
| `tone` | `AlertDialogTone` | `"danger"` | no | visual tone; maps to AlertDialog tone and default trigger Button tone |
| `triggerLabel` | `string` | `"Delete"` | no | label for the default trigger Button (ignored when trigger slot is used) |
| `confirmLabel` | `string` | `"Confirm"` | no | label for the AlertDialog confirm button |
| `cancelLabel` | `string` | `"Cancel"` | no | label for the AlertDialog cancel button |

### Types

```ts
type AlertDialogTone = "danger" | "warning";
```

### Slots

| Slot | Description |
|------|-------------|
| `trigger` | custom trigger element; replaces the default Button |
| default | body content rendered inside the AlertDialog, between description and action buttons |

### Controlled And Uncontrolled

- `open` state is managed internally; no external control prop
- The dialog opens when the trigger is activated and closes on confirm, cancel,
  or backdrop/escape dismiss

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | initial / after confirm or cancel | only the trigger is visible |
| open | trigger activated | AlertDialog is shown with title, description, body, and action buttons |

### Component States

- `open` (internal boolean): controls AlertDialog visibility
- Derived: `triggerTone` — maps `"danger"` tone to `"danger"` Button tone,
  all others to `"default"`

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `confirm` | user clicks the confirm button in the AlertDialog | `void` | dialog closes after firing |
| `cancel` | user clicks cancel, presses Escape, or clicks backdrop | `void` | dialog closes after firing |

## 6. Accessibility

### Semantics

- Default trigger: standard Button semantics
- Custom trigger wrapper: `<span role="presentation">` — the slotted element
  is expected to be an interactive element (Button, IconButton, etc.)
- AlertDialog: delegates all dialog accessibility to the AlertDialog primitive
  (role="alertdialog", aria-labelledby, aria-describedby, focus trap)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | activates the trigger (default button or custom trigger wrapper) |
| `Escape` | closes the AlertDialog (delegated to AlertDialog) |
| `Tab` | cycles focus within the open AlertDialog (delegated to AlertDialog) |

### Focus And Announcement

- When opened: focus moves to the AlertDialog (delegated to AlertDialog primitive)
- When closed: focus returns to the trigger element
- Custom trigger wrapper handles `keydown` for Enter and Space

## 7. Layout

### Sizing

- Trigger: inline; sized by the Button or custom content
- AlertDialog: centered modal (delegated to AlertDialog contract)

### Composition

- Parent expectations: toolbars, list item actions, settings forms
- Child expectations: AlertDialog primitive (foundation), Button primitive (foundation)
- Resizing rules: trigger is inline, dialog is modal overlay

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| DefaultTrigger | (delegates to Button) | Button variant="secondary" with tone |
| AlertDialog | (delegates to AlertDialog) | all AlertDialog tokens |

No component-specific tokens; ConfirmAction is purely compositional.

## 9. Svelte Notes

- Uses `createEventDispatcher` for `confirm` and `cancel` events
- Composes `AlertDialog` and `Button` from `@poodle/svelte-primitives`
- `open` is passed to AlertDialog as `open || null` (falsy → null for
  uncontrolled initial state)
- Handles `openChange` from AlertDialog to sync internal open state
- Trigger tone derivation: `tone === "danger" ? "danger" : "default"`

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::confirm_action`
- Compose AlertDialog and Button primitives
- Internal open state managed by the composite

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] trigger tone derivation logic matches
- [ ] custom trigger slot behavior matches (click and keyboard activation)

### Tier 2: Visual Parity

- [ ] default trigger renders as secondary Button with correct tone
- [ ] AlertDialog appearance delegates correctly

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Specimen Definitions

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
| Custom trigger slot | `title="Remove all filters?"`, `description="This will clear all active filters..."`, `tone="warning"`, `confirmLabel="Clear all"`, trigger slot contains ghost Button "Clear filters" | ghost button as trigger; clicking opens warning AlertDialog |

### With Body Content

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With body content | `title="Revoke API key?"`, `description="This key will immediately stop working."`, `confirmLabel="Revoke"`, default slot shows code block with API key preview | danger trigger button; AlertDialog includes custom body content between description and actions |

## 14. Approval And Adoption Notes

- Contract status: `seed contract`
- Approvers: pending
- Downstream adopters: list item delete actions, settings destructive actions,
  admin panels
- Future follow-up: consider adding `isLoading` prop for async confirm actions;
  consider controlled `open` prop for external state management
