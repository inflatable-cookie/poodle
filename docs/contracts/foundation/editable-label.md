# Editable Label

> **Surface elevation**: EditableLabel is a surface consumer (72% moderate contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `EditableLabel`
- Layer: `foundation`
- Summary: a read-mostly label that can transition into text editing for
  renaming or inline revision flows, with configurable activation modes and
  commit/cancel semantics
- In scope: display mode, edit mode, commit/cancel semantics, activation mode
  configuration, select-on-focus behavior, disabled state
- Out of scope: rich inline editing, multiline editing, conflict resolution,
  optimistic update policy (app concern)

## 2. Anatomy

```text
[Root .editable-label]  <div>
  ├── [Display .editable-label__display]  <button> (view mode)
  └── [Input .editable-label__input]  <input> (edit mode)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | inline edit host container | min-width |
| Display | yes | read-mode label with edit affordance | typography, border, background, color, cursor |
| Input | conditional | edit-mode single-line input | typography, border, background, color, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | — | yes | current committed label |
| `ariaLabel` | `string` | `"Edit label"` | no | accessible name for the edit affordance |
| `isDisabled` | `boolean` | `false` | no | disables edit entry |
| `activationMode` | `"doubleClick" \| "enterOrSpace" \| "programmatic"` | `"doubleClick"` | no | primary edit-entry pattern |
| `selectOnFocus` | `boolean` | `true` | no | select all text when editing begins |

### Controlled And Uncontrolled

- controlled committed value: `value` prop is the source of truth
- transient editing state (isEditing, draftValue) owned internally by the component
- `inputElement` ref available internally for focus management

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| view | default | static label with subtle edit affordance on interaction |
| hover | pointer enters display label | subtle border and background hint |
| focus | display label focused via keyboard | subtle border and background hint |
| editing | edit mode entered | input replaces display with focus ring |
| disabled | `isDisabled=true` | no edit activation, reduced opacity |

### Component States

```text
[view] --activation--> [editing]
[editing] --Enter/blur--> [view] (commit fires if value changed)
[editing] --Escape--> [view] (cancel fires)
```

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| view -> editing | activation gesture or programmatic start | input opens with current value, optionally selected |
| editing -> view (commit) | `Enter` or blur | `commit` fires with trimmed value if changed |
| editing -> view (cancel) | `Escape` | edit discarded, `cancel` fires, original value restored |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `editStart` | edit mode begins | `void` | optional callback |
| `commit` | edit is confirmed | `{ value: string }` | fires only if value changed from original |
| `cancel` | edit is abandoned | `void` | optional callback |

## 6. Accessibility

### Semantics

- Role: button-like trigger in view mode (keyboard-reachable), text input in edit mode
- `aria-label`: from prop, applied to display trigger (required when visible text is insufficient)
- In edit mode: input receives focus and standard text-input accessibility
- Labeling rules: assistive technology must discover both current label value and edit action

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` in view mode | enters edit mode (when `activationMode` is `"enterOrSpace"`) |
| `Space` in view mode | enters edit mode (when `activationMode` is `"enterOrSpace"`) |
| double-click in view mode | enters edit mode (when `activationMode` is `"doubleClick"`) |
| text-input keys in edit mode | standard single-line text editing |
| `Enter` in edit mode | commits edit |
| `Escape` in edit mode | cancels edit, restores original value |
| `Tab` in edit mode | commits edit (blur-triggered commit) |

### Focus And Announcement

- focus entry: display label is keyboard-reachable
- focus transition: entering edit mode moves focus into input; `selectOnFocus` selects all text
- focus restoration: commit or cancel returns focus to the display label
- live-region behavior: none by default
- GPUI-native accessibility mapping notes: GPUI must expose display state as an actionable rename trigger and swap to full text-input accessibility in edit mode, including focus restoration

## 7. Layout

### Sizing

- Root: `min-width: 0` to allow truncation in constrained containers
- Display and input share the same footprint to avoid layout shift on mode transition
- Text truncation allowed on display label when width is constrained

### Composition

- parent expectations: tab labels, track labels, card titles, list rows, panel headers
- child expectations: text only in the base contract
- resizing rules: edit input inherits available width from the display label area

## 8. Token Usage — Exact Values

### Root `.editable-label`

| Property | Value |
|----------|-------|
| `min-width` | `0` |

### Display and Input — shared base

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `min-width` | `0` |
| `padding` | `0.375rem 0.5rem` |
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-primary)` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `var(--pug-typography-label-size)` |
| `font-weight` | `var(--pug-typography-label-weight)` |
| `line-height` | `var(--pug-typography-label-lineHeight)` |
| `text-align` | `left` |

### Display `.editable-label__display`

| Property | Value |
|----------|-------|
| `cursor` | `text` |

### Display — hover / focus (not disabled)

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--pug-color-border-default) 72%, transparent)` |
| `background` | `color-mix(in srgb, var(--pug-surface) 72%, var(--pug-color-background-elevated))` |
| `outline` | `none` |

### Input `.editable-label__input` (editing state)

| Property | Value |
|----------|-------|
| `border-color` | `var(--pug-color-accent-focusRing)` |
| `background` | `var(--pug-color-background-surface)` |
| `outline` | `none` |
| `box-shadow` | `0 0 0 var(--pug-border-width-focus) color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent)` |

### Display — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

## 9. Svelte Notes

- Display rendered as a `<button>` element for keyboard reachability in view mode
- Input rendered as a standard `<input type="text">` in edit mode
- Internal state: `isEditing` boolean, `draftValue` string, `inputElement` ref
- On edit start: set `isEditing = true`, `draftValue = value`, then tick and focus input
- `selectOnFocus`: when true, `inputElement.select()` after focus
- Commit logic: trim draftValue, fire `commit` only if trimmed value differs from original
- Cancel logic: restore draftValue to original value, fire `cancel`
- Blur on input triggers commit (not cancel)
- `activationMode="doubleClick"`: `dblclick` on display enters edit mode
- `activationMode="enterOrSpace"`: `click`, `Enter`, or `Space` on display enters edit mode
- `activationMode="programmatic"`: no built-in activation gesture; parent must control entry

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::editable_label`
- Spec struct: `EditableLabelSpec` in primitives crate
- GPUI must preserve mode-switching semantics, focus transfer, and suppression of global shortcuts while inline text editor is active
- Display-to-input swap is an entity-state transition rather than DOM swap
- Focus restoration after commit/cancel must return to the display element
- The color-mix formulas for hover/focus should be replicated as closely as possible

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value, activationMode, selectOnFocus, isDisabled mean the same thing
- [ ] commit/cancel/editStart events match
- [ ] Enter commits, Escape cancels in edit mode
- [ ] blur triggers commit (not cancel)
- [ ] focus transfer and restoration match
- [ ] text-focused shortcut suppression matches during edit mode

### Tier 2: Visual Parity

- [ ] display and input share same padding (0.375rem 0.5rem) and typography
- [ ] hover/focus border-color (72% border-default mix) matches
- [ ] hover/focus background (84% background-surface mix) matches
- [ ] editing border-color (accent-focusRing) matches
- [ ] editing box-shadow (border-width-focus, 28% accent mix) matches
- [ ] disabled opacity matches
- [ ] display cursor (text) matches

### Tier 3: Implementation Freedom

- [ ] DOM swap vs GPUI entity-state swap stays internal
- [ ] double-click ergonomics may vary by platform convention
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| double-click ergonomics may vary by platform convention | pointer ergonomics are platform-sensitive | allowed | keep keyboard edit-entry parity strict |
| DOM swap vs GPUI entity-state swap | rendering architecture differs | allowed | same visual and semantic result required |
| color-mix formula rendering | GPUI may approximate color-mix | allowed | match visual result as closely as possible |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: Aura inline renames, Spark view/track/browser renames,
  tab labels, card titles
- future follow-up: decide whether optimistic-value behavior deserves shared
  guidance at the composite layer
