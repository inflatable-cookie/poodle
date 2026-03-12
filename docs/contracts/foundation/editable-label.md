# Editable Label

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `EditableLabel`
- Layer: `foundation`
- Summary: a read-mostly label that can transition into text editing for
  renaming or inline revision flows
- In scope: display mode, edit mode, commit/cancel semantics, optimistic label
  handling as an app concern
- Out of scope: rich inline editing, multiline editing, conflict resolution

## 2. Anatomy

```text
[Root]
  ├── [Display Label] (view mode)
  └── [Input Control] (edit mode)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | inline edit host | text color, spacing |
| Display Label | yes | read-mode label with edit affordance | text color, hover/focus treatment |
| Input Control | conditional | edit-mode single-line input | input tokens, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | none | yes | current committed label |
| `ariaLabel` | `string` | `"Edit label"` | no | edit affordance name |
| `isDisabled` | `boolean` | `false` | no | disables edit entry |
| `activationMode` | `"doubleClick" \| "enterOrSpace" \| "programmatic"` | `"doubleClick"` | no | primary edit-entry pattern |
| `selectOnFocus` | `boolean` | `true` | no | select all text when editing begins |
| `onCommit` | `(value: string) => void` | none | yes | commit callback |
| `onCancel` | `() => void` | none | no | cancel callback |
| `onEditStart` | `() => void` | none | no | edit-entry callback |

### Controlled And Uncontrolled

- controlled committed value
- transient editing state owned by the component unless a future explicit
  controlled-edit-mode API is added

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| view | default | static label with edit affordance |
| hover | pointer enters display label | subtle edit affordance cue |
| focus | display label or input focused | visible focus treatment |
| editing | edit mode entered | input control replaces display label |
| disabled | `isDisabled=true` | no edit activation |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| view -> editing | activation gesture or programmatic start | input opens with current value |
| editing -> view commit | `Enter`, blur, or explicit commit action | `onCommit` fires if value changed and valid |
| editing -> view cancel | `Escape` or explicit cancel | edit discarded and `onCancel` fires |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onEditStart` | edit mode begins | none | optional |
| `onCommit` | edit is confirmed | trimmed or accepted string | caller decides persistence |
| `onCancel` | edit is abandoned | none | optional |

## 6. Accessibility

### Semantics

- Role: button-like trigger in view mode, text input in edit mode
- Required attributes: accessible name for the edit trigger when visible text is
  insufficient
- Optional attributes: description relationship for rename guidance
- Labeling rules: assistive technology must be able to discover both the
  current label value and the edit action

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` or `Space` in view mode | enters edit mode when keyboard activation is enabled |
| text-input keys in edit mode | behave as single-line text editing |
| `Enter` in edit mode | commits edit |
| `Escape` in edit mode | cancels edit |
| `Tab` in edit mode | commits or exits according to documented blur behavior |

### Focus And Announcement

- focus entry: display label is keyboard-reachable when editing can be started
  from the keyboard
- focus transition: entering edit mode moves focus into the input and preserves
  context for assistive technology
- focus restoration: cancel or commit returns focus to the display label unless
  the parent flow intentionally redirects it
- live-region behavior: none by default
- GPUI-native accessibility mapping notes: GPUI must expose the display state as
  an actionable rename trigger and then swap to full text-input accessibility
  semantics in edit mode, including focus restoration and state announcement

## 7. Layout

### Sizing

- display label truncation is allowed when width is constrained
- edit input should preserve approximately the same footprint to avoid jarring
  layout shifts

### Composition

- parent expectations: tab labels, track labels, card titles, list rows
- child expectations: text only in the base contract
- resizing rules: edit input inherits available width from the display label

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Display Label | `semantic.typography.label.*` and `semantic.color.text.*` | label styling |
| Display Label | hover/focus token roles | edit affordance cue |
| Input Control | `TextInput` token baseline | edit mode styling |
| Focus treatment | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | focus |

## 9. Svelte Notes

- typically composes `TextInput` for edit mode and semantic text/button-like
  markup for view mode
- double-click activation may be offered, but keyboard activation must remain
  available when the control is intended to be editable

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::editable_label`
- GPUI implementation must preserve mode switching semantics, focus transfer,
  and suppression of global shortcuts while the inline text editor is active

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] view-mode and edit-mode semantics match
- [ ] commit/cancel behavior matches
- [ ] focus transfer and restoration match
- [ ] text-focused shortcut suppression matches during edit mode

### Tier 2: Visual Parity

- [ ] display-to-edit visual transition remains proportionally stable
- [ ] focus treatment uses the same semantic cues

### Tier 3: Implementation Freedom

- [ ] DOM swap vs GPUI entity-state swap stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| double-click ergonomics may vary by platform convention | pointer ergonomics are platform-sensitive | allowed | keep keyboard edit-entry parity strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Aura inline renames, Spark view/track/browser renames
- future follow-up: decide whether optimistic-value behavior deserves shared
  guidance at the composite layer

## Next Task

Treat `EditableLabel` as a stateful text-entry wrapper over `TextInput`, not as
its own fully separate editing model.
