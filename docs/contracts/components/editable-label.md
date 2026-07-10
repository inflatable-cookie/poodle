# Editable Label

> **Surface elevation**: EditableLabel is a surface consumer (72% moderate contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `EditableLabel`
- Layer: `foundation`
- Summary: a read-mostly label that can transition into text editing for
  renaming or inline revision flows, with configurable activation modes,
  commit/cancel semantics, empty state display, and flush variant
- In scope: display mode, edit mode, commit/cancel semantics, activation mode
  configuration, select-on-focus behavior, disabled state, empty state,
  edit icon, flush variant
- Out of scope: rich inline editing, multiline editing, conflict resolution,
  optimistic update policy (app concern)

## 2. Anatomy

```text
[Root .editable-label]  <div>
  ├── [Display .editable-label__display]  <button> (view mode)
  │     ├── [Text .editable-label__text]  <span>
  │     └── [Edit Icon .editable-label__icon]  <svg> (optional)
  └── [Input .editable-label__input]  <input> (edit mode)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | inline edit host container | min-width |
| Display | yes | read-mode label with edit affordance | typography, border, background, color, cursor |
| Text | yes | label text content | color (empty state: text-secondary, italic) |
| Edit Icon | no | pencil icon shown on hover/focus | color, opacity, size |
| Input | conditional | edit-mode single-line input | typography, border, background, color, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | — | yes | current committed label |
| `ariaLabel` | `string` | `"Edit label"` | no | accessible name for the edit affordance |
| `disabled` | `boolean` | `false` | no | disables edit entry |
| `activationMode` | `"doubleClick" \| "enterOrSpace" \| "programmatic"` | `"doubleClick"` | no | primary edit-entry pattern |
| `selectOnFocus` | `boolean` | `true` | no | select all text when editing begins |
| `variant` | `"default" \| "flush"` | `"default"` | no | default has padding/border; flush has no padding/border |
| `emptyText` | `string \| null` | `null` | no | italic placeholder text when value is empty |
| `placeholder` | `string \| null` | `null` | no | input placeholder during editing |
| `maxLength` | `number \| null` | `null` | no | maximum input length |
| `showEditIcon` | `boolean` | `false` | no | show pencil icon on hover/focus |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Controlled And Uncontrolled

- committed value is host-owned: `value` prop is the source of truth, and the
  host applies accepted edits from `onCommit`
- transient editing state (isEditing, draftValue) owned internally by the component
- `inputElement` ref available internally for focus management

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| view | default | static label with subtle edit affordance on interaction |
| hover | pointer enters display label | subtle border and background hint (default variant) |
| focus | display label focused via keyboard | focus ring (default variant) |
| editing | edit mode entered | input replaces display with focus ring |
| disabled | `disabled=true` | no edit activation, reduced opacity |
| empty | value is empty, `emptyText` set | italic secondary-color placeholder text |

### Component States

```text
[view] --activation--> [editing]
[editing] --Enter/blur--> [view] (commit fires)
[editing] --Escape--> [view] (cancel fires)
```

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| view -> editing | activation gesture or programmatic start | input opens with current value, optionally selected |
| editing -> view (commit) | `Enter` or blur | `commit` fires with trimmed value and previousValue |
| editing -> view (cancel) | `Escape` | edit discarded, `cancel` fires, original value restored |

### Behavior Machine

Behavior classification: adapter-owned interaction (g11.004 sweep)

View/edit mode toggle with commit/cancel keys. Extraction debt: edit-mode machine (shared with EditableList).

## 5. Callbacks

| Callback | When It Fires | Signature | Notes |
|----------|---------------|-----------|-------|
| `onEditStart` | edit mode begins | `() => void` | optional callback |
| `onCommit` | edit is confirmed | `(detail: { value: string; previousValue: string }) => void` | always fires on commit (host decides whether to apply) |
| `onCancel` | edit is abandoned | `() => void` | optional callback |

## 6. Accessibility

### Semantics

- Role: button-like trigger in view mode (keyboard-reachable), text input in edit mode
- `aria-label`: from prop, applied to display trigger (required when visible text is insufficient)
- In edit mode: input receives focus and standard text-input accessibility
- Edit icon: `aria-hidden="true"` (decorative)
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

## 7. Layout

### Sizing

- Root: `display: inline-flex; min-width: 0` to allow truncation in constrained containers
- Display and input share the same footprint to avoid layout shift on mode transition
- Text truncation allowed on display label when width is constrained

### Variants

#### Default variant

- Padding: `0.375rem 0.5rem`
- Border: `0.0625rem solid transparent` (visible on hover/focus)
- Border-radius: `var(--poodle-radius-control)`
- Hover/focus: subtle border and background hint

#### Flush variant

- Padding: `0`
- Border: none
- Background: transparent (no hover background change)
- Editing: bottom border only (`0.0625rem solid accent-focusRing`)

### Composition

- parent expectations: tab labels, track labels, card titles, list rows, panel headers, page headings
- child expectations: text only in the base contract
- resizing rules: edit input inherits available width from the display label area

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Value | Notes |
|-----------|-------|-------|
| `data-editing` | `{isEditing}` | reflects current editing state |
| `data-disabled` | `{disabled}` | reflects disabled state |
| `data-variant` | `{variant}` | `"default"` or `"flush"` |

### Root `.editable-label`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `min-width` | `0` |

### Display and Input — shared base

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `min-width` | `0` |
| `padding` | `0.375rem 0.5rem` |
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `line-height` | `var(--poodle-typography-label-lineHeight)` |
| `text-align` | `left` |

### Display `.editable-label__display`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `cursor` | `text` |

### Display — hover / focus (not disabled, default variant)

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 52%, transparent)` |
| `outline` | `none` |

### Display — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

### Input `.editable-label__input` (editing state)

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-focusRing)` |
| `background` | `var(--poodle-color-background-surface)` |
| `outline` | `none` |
| `box-shadow` | `0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)` |

### Display — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Empty state text

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-style` | `italic` |

### Edit icon `.editable-label__icon`

| Property | Value |
|----------|-------|
| `width` | `0.75rem` |
| `height` | `0.75rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `opacity` | `0` (visible on hover/focus) |

### Flush variant overrides

| Property | Value |
|----------|-------|
| Display/Input `padding` | `0` |
| Display/Input `border` | `none` |
| Display/Input `border-radius` | `0` |
| Display hover/focus `background` | `transparent` |
| Input `border-bottom` | `0.0625rem solid var(--poodle-color-accent-focusRing)` |
| Input `box-shadow` | `none` |

### Size adjustments

| Size | padding | font-size |
|------|---------|-----------|
| `xs` | `calc(space-control-y - 0.125rem) calc(space-control-x - 0.125rem)` | `0.75rem` |
| `sm` | `calc(space-control-y - 0.0625rem) calc(space-control-x - 0.0625rem)` | _(base)_ |
| `md` | `0.375rem 0.5rem` | _(base)_ |
| `lg` | `calc(space-control-y + 0.0625rem) calc(space-control-x + 0.125rem)` | `0.9375rem` |
| `xl` | `calc(space-control-y + 0.125rem) calc(space-control-x + 0.1875rem)` | `1rem` |

## 9. Svelte Notes

- Display rendered as a `<button>` element for keyboard reachability in view mode
- Input rendered as a standard `<input type="text">` in edit mode
- Internal state: `isEditing` boolean, `draftValue` string, `inputElement` ref
- On edit start: set `isEditing = true`, `draftValue = value`, then tick and focus input
- `selectOnFocus`: when true, `inputElement.select()` after focus
- Commit logic: trim draftValue, fire `commit` with value and previousValue
- Cancel logic: restore draftValue to original value, fire `cancel`
- Blur on input triggers commit (not cancel)
- `activationMode="doubleClick"`: `dblclick` on display enters edit mode
- `activationMode="enterOrSpace"`: `click`, `Enter`, or `Space` on display enters edit mode
- `activationMode="programmatic"`: no built-in activation gesture; parent must control entry
- Replaces former `InlineEditableField` composite (merged)
- `data-size` attribute on root reflects the resolved size for CSS variant styling
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::editable_label`
- Spec struct: `EditableLabelSpec` in primitives crate
- GPUI must preserve mode-switching semantics, focus transfer, and suppression of global shortcuts while inline text editor is active
- Display-to-input swap is an entity-state transition rather than DOM swap
- Focus restoration after commit/cancel must return to the display element

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value, activationMode, selectOnFocus, disabled mean the same thing
- [ ] commit/cancel/editStart events match (commit includes previousValue)
- [ ] Enter commits, Escape cancels in edit mode
- [ ] blur triggers commit (not cancel)
- [ ] focus transfer and restoration match
- [ ] emptyText and showEditIcon behavior match
- [ ] variant flush/default behavior match

### Tier 2: Visual Parity

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] display and input share same padding (0.375rem 0.5rem) and typography
- [ ] hover/focus border and background match
- [ ] editing border-color (accent-focusRing) matches
- [ ] editing box-shadow matches
- [ ] disabled opacity matches
- [ ] empty state italic text-secondary matches
- [ ] edit icon size, color, and hover reveal match
- [ ] flush variant: no padding/border, bottom-border-only editing

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

## 13. Specimen Definitions

### Double-click to edit (default)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Double-click to edit | `value="My project title"`, `ariaLabel="Project title"`, default activationMode | Static label; double-clicking enters edit mode with input and focus ring |

### Click to edit with icon

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Click to edit with icon | `value="My project title"`, `activationMode="enterOrSpace"`, `showEditIcon` | Static label with pencil icon on hover; click enters edit mode |

### Empty state

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty state | `value=""`, `activationMode="enterOrSpace"`, `emptyText="Add a description…"` | Italic secondary-color placeholder text; click enters edit mode |

### Flush variant

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Flush variant | `value="Inline heading"`, `variant="flush"`, `activationMode="enterOrSpace"`, `showEditIcon` | No padding or border; text sits inline; editing shows bottom border only |

### With max length

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With max length | `value="Short text"`, `activationMode="enterOrSpace"`, `maxLength={20}`, `placeholder="Enter text…"` | Standard label; editing enforces character limit |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `value="Read-only value"`, `disabled` | Static label with reduced opacity, no edit activation possible |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: Aura inline renames, Spark view/track/browser renames,
  tab labels, card titles, page headings
- future follow-up: decide whether optimistic-value behavior deserves shared
  guidance at the composite layer
- migration note: `InlineEditableField` composite has been merged into this
  primitive; consumers should migrate to `EditableLabel` with
  `activationMode="enterOrSpace"` and `showEditIcon` for equivalent behavior
