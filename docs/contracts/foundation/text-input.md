# Text Input

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `TextInput`
- Layer: `foundation`
- Summary: a single-line text entry control with explicit value, validation,
  focus, and submission semantics
- In scope: plain text input, placeholder, leading/trailing affordance slots,
  validation state, submit/cancel behavior
- Out of scope: multiline editing, search-specific clear behavior, inline edit
  mode switching

## 2. Anatomy

```text
[Root]
  ├── [Leading Affordance] (optional)
  ├── [Input Control]
  ├── [Trailing Affordance] (optional)
  └── [Validation/Status Affordance] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | field chrome container | background, border, radius, focus ring |
| Leading Affordance | no | icon or adornment before text | icon color, inline gap |
| Input Control | yes | editable single-line text surface | typography, text color, caret |
| Trailing Affordance | no | icon or action after text | icon color, inline gap |
| Validation/Status Affordance | no | status icon or progress cue | status color, icon size |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled value |
| `defaultValue` | `string` | `""` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | hint when empty |
| `isDisabled` | `boolean` | `false` | no | disables editing |
| `isReadOnly` | `boolean` | `false` | no | allows selection without editing |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | visual and assistive state |
| `ariaLabel` | `string \| null` | `null` | no | required when no external label exists |
| `descriptionId` | `string \| null` | `null` | no | descriptive relationship |
| `errorMessageId` | `string \| null` | `null` | no | validation relationship |
| `leadingIcon` | `string \| null` | `null` | no | decorative or semantic adornment |
| `trailingIcon` | `string \| null` | `null` | no | decorative or semantic adornment |
| `onValueChange` | `(value: string) => void` | none | no | fires on user editing |
| `onSubmit` | `(value: string) => void` | none | no | fires on enter when submission is enabled |
| `onCancel` | `() => void` | none | no | fires on escape when cancel semantics are enabled |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`
- do not mix controlled and uncontrolled modes simultaneously

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral input chrome |
| hover | pointer enters | optional subtle boundary emphasis |
| focus | keyboard or pointer focus | visible active focus treatment |
| disabled | `isDisabled=true` | non-interactive muted field |
| readOnly | `isReadOnly=true` | selectable but not editable |
| invalid | `validationState="invalid"` | error emphasis and assistive invalid state |
| valid | `validationState="valid"` | optional success emphasis |
| pending | `validationState="pending"` | progress or validating treatment |

### Component States

State table is sufficient for the base primitive.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | user edits value | current string | IME-safe and grapheme-safe meaning required |
| `onSubmit` | enter confirms input | current string | optional behavior |
| `onCancel` | escape cancels editing or pending entry | none | optional behavior |
| `onFocus` | control receives focus | framework-native event | optional passthrough |
| `onBlur` | control loses focus | framework-native event | optional passthrough |

## 6. Accessibility

### Semantics

- Role: native single-line text input
- Required attributes: accessible name from external label or `ariaLabel`
- Optional attributes: description and error relationships, invalid state,
  readonly state
- Labeling rules: placeholder text never counts as the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| character input | inserts text |
| `Arrow Left/Right` | moves caret |
| `Home/End` | moves to start/end |
| `Shift+Arrow` | extends selection |
| platform copy/cut/paste/select-all shortcuts | operate on text selection |
| `Enter` | fires `onSubmit` when submit semantics are enabled |
| `Escape` | fires `onCancel` when cancel semantics are enabled |
| `Tab` | moves focus out of the control |

### Focus And Announcement

- focus entry: field receives visible focus treatment and caret
- focus exit: focus treatment clears; validation can be surfaced by parent
  systems on blur
- live-region behavior: validation announcement is usually parent-owned, but the
  field must expose invalid state relationships
- GPUI-native accessibility mapping notes: GPUI must expose role/control type,
  accessible name, value, readonly/disabled/invalid state, selection/caret
  behavior, and IME-safe text entry semantics through native accessibility APIs

## 7. Layout

### Sizing

- minimum height from shared control-size tokens
- width can shrink or stretch with parent constraints
- content remains single-line and horizontally scrolls internally if necessary

### Composition

- parent expectations: forms, toolbars, search shells, inline editors
- child expectations: optional icon or affordance adornments only
- resizing rules: adornments do not collapse text-edit area below usable width

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.color.background.panel` or control background roles | field fill |
| Root | `semantic.color.border.*` | field boundary and validation emphasis |
| Root | `semantic.radius.control` | field shape |
| Root | `semantic.size.control.height` | field height |
| Root | `semantic.space.control.*` | field padding |
| Input Control | `semantic.typography.body.*` | text styling |
| Input Control | `semantic.color.text.primary/secondary` | input and placeholder text |
| Affordances | `semantic.color.icon.*` and `semantic.icon.size.default` | adornment styling |
| Focus treatment | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | focus ring |
| Validation | `semantic.color.status.*` | state emphasis |

## 9. Svelte Notes

- should prefer native `<input type="text">` behavior
- browser autofill, IME, selection, and undo behavior should remain native
  where possible
- higher wrappers may add labels and validation messages, but the input
  contract owns its name/value/focus semantics

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::text_input`
- GPUI implementation must intentionally handle caret movement, selection,
  clipboard shortcuts, IME composition, and text-focused keybinding suppression
- while focused, application-global shortcuts should defer to the text control
  unless the contract explicitly defines an exception

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value/change semantics match
- [ ] caret navigation and selection semantics match
- [ ] accessible naming and invalid/readonly/disabled state exposure match
- [ ] submit/cancel behavior matches when enabled
- [ ] text-focused shortcut suppression matches

### Tier 2: Visual Parity

- [ ] control sizing, spacing, and focus treatment use the same token roles
- [ ] validation emphasis uses the same semantic color roles

### Tier 3: Implementation Freedom

- [ ] native browser input internals vs GPUI text system internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native text-caret visuals may differ | platform-native text rendering is acceptable | allowed | keep editing semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Aura search/edit controls, Spark search/edit controls,
  forms and shell inputs
- future follow-up: attach richer validation timing rules once field wrappers
  land in `g02.001`

## Next Task

Build `SearchField`, `EditableLabel`, and `NumberEntry` on top of `TextInput`
instead of redefining baseline editing semantics each time.
