# Editable Label

> **Surface elevation**: EditableLabel is a surface consumer (72% moderate contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-09-02

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
| `value` | `string` | — | yes | host-owned committed label; live edits never write this |
| `ariaLabel` | `string` | resolved | no | omitted name resolves from visible `value`, then `emptyText`, then `"Edit label"`; display and editor receive the same name |
| `disabled` | `boolean` | `false` | no | disables edit entry |
| `activationMode` | `"doubleClick" \| "enterOrSpace" \| "programmatic"` | `"doubleClick"` | no | primary edit-entry pattern |
| `selectOnFocus` | `boolean` | `true` | no | select all text when editing begins |
| `variant` | `"default" \| "flush"` | `"default"` | no | default has padding/border; flush has no padding/border |
| `emptyText` | `string \| null` | `null` | no | italic placeholder text when value is empty |
| `placeholder` | `string \| null` | `null` | no | input placeholder during editing |
| `maxLength` | `number \| null` | `null` | no | maximum length in Unicode scalar values, not UTF-16 code units. HTML `maxlength` is not the authority |
| `showEditIcon` | `boolean` | `false` | no | show pencil icon on hover/focus |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Controlled And Uncontrolled

- `value` is the committed string. The host applies accepted edits from
  `onCommit`. There is no public live draft, controlled draft, or pending API.
- Web owns `isEditing` and the session draft internally.
- Native projects committed `value`, optional `draft_value`, and `is_editing`.
  Live edits update `draft_value` only; they never overwrite committed `value`.
- A host echo of the just-committed value does not restart or cancel a session.
  An external replacement of committed `value` while editing returns to view on
  the new committed value with no commit.
- Disablement during edit cancels. Teardown/unmount itself emits neither commit
  nor cancel.

### Methods

Web runtimes expose exactly three imperative methods. No public `commit()`,
element getter, controlled draft, or second handle type.

Svelte exports these component-instance methods:

```ts
export function focus(): void;
export function startEditing(): void;
export function cancelEditing(): void;
```

React exports `EditableLabelHandle` from the component module and package root,
and exports `EditableLabel` through
`forwardRef<EditableLabelHandle, EditableLabelProps>`:

```ts
export interface EditableLabelHandle {
  focus(): void;
  startEditing(): void;
  cancelEditing(): void;
}
```

- `focus()` targets the display control in view mode and the live input in edit
  mode.
- `startEditing()` ignores `activationMode` but is inert when disabled or
  already editing.
- `cancelEditing()` is inert outside edit mode and otherwise follows the Escape
  law, including display-focus restoration.

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
[view] --activation / startEditing()--> [editing]
[editing] --Enter--> [view] (commit, restore display focus)
[editing] --Tab / pointer blur / window blur--> [view] (commit, no restore)
[editing] --Escape / cancelEditing()--> [view] (cancel, restore display focus)
[editing] --external value replacement--> [view] (no commit)
[editing] --disablement--> [view] (cancel)
[editing] --teardown--> (neither commit nor cancel)
```

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| view -> editing | matching activation gesture or `startEditing()` | input opens seeded from committed `value`; `selectOnFocus=true` selects all, false places the caret at the end |
| editing -> view (commit) | `Enter`, Tab, pointer blur, or window blur | `onCommit` fires with portable-trimmed draft and `previousValue`; unchanged commits still emit |
| editing -> view (cancel) | `Escape` or `cancelEditing()` | edit discarded, `onCancel` fires, committed `value` unchanged |
| editing -> view (replace) | host replaces committed `value` with a different string | return to view on the new value; no commit |
| editing -> view (disable) | `disabled` becomes true | cancel law; no commit |
| teardown | unmount while editing | neither commit nor cancel |

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Machine-backed: paired `editLabelTransition` in `@inflatable-cookie/poodle-core`
and `poodle_headless::edit`. View/editing, start-edit guards (disabled or
already editing), portable set-**T** trim, Unicode-scalar `maxLength`, commit
payload `{ value, previousValue }` (unchanged commits still emit), cancel,
external replacement, disablement, and teardown. Gesture vs `startEditing()`
gating lives in the adapter. Focus, select-on-focus, and display-focus
restoration are effect intents the adapter executes. Commit/cancel stay
state-guarded, so a blur after Escape or Enter cannot double-emit.

Portable trim set **T** is Unicode White_Space plus U+FEFF: U+0009–U+000D,
U+0020, U+0085, U+00A0, U+1680, U+2000–U+200A, U+2028, U+2029, U+202F,
U+205F, U+3000, and U+FEFF. Drop the longest prefix and suffix in **T**.
Interior characters stay. U+200B is not trimmed. Paired machines must not call
ECMAScript `String.trim` or Rust `str::trim` as the authority.

## 5. Callbacks

| Callback | When It Fires | Signature | Notes |
|----------|---------------|-----------|-------|
| `onEditStart` | edit mode begins | `() => void` | optional callback |
| `onCommit` | edit is confirmed | `(detail: { value: string; previousValue: string }) => void` | always fires on commit, including unchanged values; cancel never emits a commit |
| `onCancel` | edit is abandoned | `() => void` | optional callback |

## 6. Accessibility

### Semantics

- Role: button-like trigger in view mode (keyboard-reachable), text input in edit mode
- `aria-label`: the resolved accessible name, applied to both the display
  trigger and edit input so the name survives the mode transition. Omitted
  `ariaLabel` resolves from visible `value`, then `emptyText`, then
  `"Edit label"`
- In edit mode: input receives focus and standard text-input accessibility
- Edit icon: `aria-hidden="true"` (decorative)
- Labeling rules: assistive technology must discover both current label value and edit action

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` in view mode | enters edit mode when `activationMode` is `"doubleClick"` or `"enterOrSpace"`; ignored when `"programmatic"` |
| pointer double-click in view mode | enters edit mode when `activationMode` is `"doubleClick"` |
| pointer single-click in view mode | enters edit mode when `activationMode` is `"enterOrSpace"` |
| any gesture in `"programmatic"` view mode | stays in view; only `startEditing()` / native wrapper starts an edit |
| text-input keys in edit mode | standard single-line text editing via the shared TextInput insertion/composition/selection rules |
| `Enter` in edit mode | commits, prevents an ancestor form submit, and restores display focus |
| `Escape` in edit mode | cancels, restores the committed value, and restores display focus |
| `Tab` in edit mode | commits once through blur; display is not refocused; focus advances |

### Focus And Announcement

- focus entry: display label is keyboard-reachable
- focus transition: entering edit mode moves focus into input; `selectOnFocus` selects all text
- focus restoration: Enter, Escape, and `cancelEditing()` return focus to the
  display label. Tab, pointer blur, and window blur commit once without
  restoring. Teardown emits neither commit nor cancel and does not restore.
- live-region behavior: none by default. No new focus manager.

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
- Input rendered as a standard `<input type="text">` in edit mode; do not set
  HTML `maxlength` — scalar clamp is the authority
- Internal state: `isEditing` boolean, session `draftValue`, display and input
  refs. No public draft.
- Exported instance methods: `focus()`, `startEditing()`, `cancelEditing()`
- On edit start: set `isEditing = true`, seed draft from committed `value`,
  then tick and focus input
- `selectOnFocus`: when true, `input.select()` after focus; when false, place
  the caret at the end
- Commit logic: portable set-T trim, fire `onCommit` with value and previousValue
- Cancel logic: discard draft, fire `onCancel`, restore display focus
- Pointer/window blur and Tab commit once without restoring display focus
- Unmount while editing emits neither callback
- `activationMode="doubleClick"`: pointer double-click or Enter/Space
- `activationMode="enterOrSpace"`: pointer single-click or Enter/Space
- `activationMode="programmatic"`: no gesture; `startEditing()` still works
  unless disabled or already editing
- Replaces former `InlineEditableField` composite (merged)
- `data-size` attribute on root reflects the resolved size for CSS variant styling
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::editable_label`
- Spec struct: `EditableLabelSpec` in primitives crate
- Native spec projects committed `value`, optional `draft_value`, `is_editing`,
  and session selection. Live edits paint `draft_value`; `value` and
  `onCommit.previousValue` retain the committed snapshot
- GPUI must preserve mode-switching semantics, focus transfer, and suppression of global shortcuts while inline text editor is active
- Display-to-input swap is an entity-state transition rather than DOM swap
- Focus restoration after Enter/Escape/`cancelEditing` must return to the display element; Tab/pointer/window blur commit without restoring
- Live editing reuses the shared headless TextInput insertion, composition, and
  selection transitions without composing TextInput's visual component.
  `selectOnFocus=true` selects all; false places the caret at the end.
  `max_length` is a Unicode-scalar budget in those transitions.
- Session wrapper stamps `with_id` on the node. Host retains `{key}`,
  `{key}-draft`, `{key}-editing`, and caret between rebuilds. Live keystrokes
  never write the committed value key.
- `enterOrSpace` uses `on_activate` (pointer click plus synthesized
  Enter/Space). `doubleClick` uses `on_double_activate` for pointer
  double-click and `on_edit_key` on the display node for Enter/Space; it must
  not set `on_activate`. `programmatic` installs neither gesture.
- Enter/Escape/`cancelEditing` restore display focus through
  `request_focus` on the next view-mode paint. Tab, pointer blur, and window
  blur commit without restoring. After drop, the host must not apply a blur
  commit; machine `TEARDOWN` returns to view with no effects.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value, activationMode, selectOnFocus, disabled mean the same thing
- [ ] commit/cancel/editStart events match (commit includes previousValue; unchanged commits still emit)
- [ ] Enter commits and restores display focus; Escape cancels and restores it
- [ ] Tab, pointer blur, and window blur commit once without restoring
- [ ] teardown emits neither commit nor cancel
- [ ] emptyText and showEditIcon behavior match
- [ ] variant flush/default behavior match
- [ ] `doubleClick`, `enterOrSpace`, and `programmatic` share one activation boundary
- [ ] portable set-T trim and Unicode-scalar `maxLength` match across machines

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
| Jetstream has no commit or cancel events | both are keyboard (Enter, Escape) or blur, and the runtime raises neither | accepted, tracked | g12.017 |
| double-click ergonomics may vary by platform convention | pointer ergonomics are platform-sensitive | allowed | keep keyboard edit-entry parity strict |
| DOM swap vs GPUI entity-state swap | rendering architecture differs | allowed | same visual and semantic result required |
| color-mix formula rendering | GPUI may approximate color-mix | allowed | match visual result as closely as possible |

## 12a. Jetstream Notes

- `EditableLabel::from_spec(spec, theme).on_edit_start(...)`.
- Already editing, nothing restarts: a host that resets its draft on edit-start
  would otherwise lose what had been typed.

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
