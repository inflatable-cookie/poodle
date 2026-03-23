# Text Area

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `TextArea`
- Layer: `foundation`
- Summary: a multiline text entry control for longer text content with explicit
  value, validation, and chord-based submission semantics
- In scope: multiline editing, line breaks, vertical resize, scroll behavior,
  validation state, Cmd/Ctrl+Enter submit, Escape cancel, controlled and
  uncontrolled value models
- Out of scope: rich text formatting, markdown rendering, code editor features
  (syntax highlighting, line numbers), auto-grow behavior

## 2. Anatomy

```text
[Root .text-area]  <div>
  └── [Text Area Control .text-area__control]  <textarea>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | field chrome container with grid layout | background, border, radius, shadow, focus ring |
| Text Area Control | yes | native multiline editing surface | typography, text color, padding, resize |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | none | yes | element id for label association |
| `value` | `string \| null` | `null` | no | controlled value; when non-null, component is controlled |
| `defaultValue` | `string` | `""` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | hint text when empty |
| `rows` | `number` | `4` | no | initial visible line count |
| `name` | `string \| undefined` | `undefined` | no | form submission name |
| `isDisabled` | `boolean` | `false` | no | disables editing and interaction |
| `isReadOnly` | `boolean` | `false` | no | allows selection without editing |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | visual and assistive validation state |
| `ariaLabel` | `string \| null` | `null` | no | required when no external label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |

### Controlled And Uncontrolled

- controlled: `value` (non-null) plus `valueChange` event
- uncontrolled: `defaultValue` sets the initial value; component owns its own state
- do not mix controlled and uncontrolled modes simultaneously

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral multiline field chrome |
| focus | focus-within on root | border-color switches to focus border, background to focus fill, box-shadow to focus shadow |
| disabled | `isDisabled=true` | opacity reduced via `state-opacity-disabled`, interaction suppressed |
| readOnly | `isReadOnly=true` | selectable but not editable |
| invalid | `validationState="invalid"` | border-color switches to `status-danger` |
| valid | `validationState="valid"` | border-color switches to `status-success` |
| pending | `validationState="pending"` | border-color switches to `accent-base` |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user edits content | `{ value: string }` | fires on each input change; multiline-safe |
| `submit` | Cmd/Ctrl+Enter pressed | `{ value: string }` | chord-based, NOT plain Enter (Enter inserts newline) |
| `cancel` | Escape pressed | `void` | fires with no payload |
| `focus` | textarea receives focus | `FocusEvent` | native focus event passthrough |
| `blur` | textarea loses focus | `FocusEvent` | native blur event passthrough |

## 6. Accessibility

### Semantics

- Role: native `<textarea>` element
- `id`: from prop, used for external `<label for>` association
- `aria-label`: from ariaLabel prop; required when no external label exists
- `aria-describedby`: from describedBy prop
- `aria-invalid`: `"true"` when validationState is `"invalid"`
- `readonly`: native readonly attribute set when isReadOnly (note: `aria-readonly` is NOT explicitly set; the native `readonly` attribute is used instead)
- `disabled`: native disabled attribute when isDisabled
- `rows`: from rows prop, sets initial visible height
- Labeling rules: placeholder text never counts as the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| character input | inserts text |
| `Enter` | inserts line break (default multiline behavior) |
| `Arrow keys` | moves caret within text |
| `Shift+Arrow` | extends selection |
| platform copy/cut/paste/select-all shortcuts | operate on multiline selection |
| `Cmd/Ctrl+Enter` | fires `submit` event with current value |
| `Escape` | fires `cancel` event |
| `Tab` | moves focus out of the control |

### Focus And Announcement

- focus entry: root receives visible focus treatment (border, background, and
  shadow transition); insertion point appears in textarea
- focus exit: focus treatment clears; validation can be surfaced by parent
  Field wrapper on blur
- live-region behavior: validation announcement is parent-owned (Field), but the
  textarea must expose invalid state via `aria-invalid`
- GPUI-native accessibility mapping notes: multiline role, value exposure,
  caret/selection movement, vertical scrolling, and IME composition must be
  surfaced through native accessibility APIs

## 7. Layout

### Sizing

- minimum height: determined by `rows` prop (default 4 lines via `min-height: calc(1lh * rows)`)
- no explicit min-height from control-height token (unlike TextInput)
- width: stretches with parent
- vertical resize: `resize: vertical` allows user resizing

### Composition

- parent expectations: Field wrappers, forms, notes panels, comments,
  descriptions, dialog content
- child expectations: none (no slots or child elements)
- resizing rules: may be fixed-height or user-resizable; internal vertical
  scrolling occurs when content exceeds visible height

## 8. Token Usage — Exact Values

### CSS Custom Properties (treatment system)

| Var | Default Value | Focus Value |
|-----|---------------|-------------|
| `--poodle-text-area-radius` | `var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control))` | — |
| `--poodle-text-area-fill` | `var(--poodle-treatment-interactive-subtle-fill, var(--poodle-color-background-surface))` | `var(--poodle-text-area-fill-focus)` |
| `--poodle-text-area-border` | `var(--poodle-treatment-interactive-subtle-border, var(--poodle-color-border-default))` | `var(--poodle-text-area-border-focus)` |
| `--poodle-text-area-shadow` | (none by default) | `var(--poodle-text-area-shadow-focus)` fallback `0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)` |

### Root `.text-area`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `min-height` | `0` |
| `border` | `0.0625rem solid var(--poodle-text-area-border)` |
| `border-radius` | `var(--poodle-text-area-radius)` |
| `background` | `var(--poodle-text-area-fill)` |
| `color` | `var(--poodle-color-text-primary)` |
| `box-shadow` | `var(--poodle-text-area-shadow)` |
| `transition` | `border-color, box-shadow, background` all at `var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Root focus-within

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-text-area-border-focus)` |
| `background` | `var(--poodle-text-area-fill-focus)` |
| `box-shadow` | `var(--poodle-text-area-shadow-focus)` fallback `0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)` |

### Root validation states

| State | `border-color` |
|-------|----------------|
| `invalid` | `var(--poodle-color-status-danger)` |
| `valid` | `var(--poodle-color-status-success)` |
| `pending` | `var(--poodle-color-accent-base)` |

### Root disabled (`:has(:disabled)`)

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Text Area Control `.text-area__control`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `width` | `100%` |
| `min-height` | `calc(1lh * 4)` (based on rows prop, default 4) |
| `padding` | `var(--poodle-space-control-y) var(--poodle-space-control-x)` |
| `border` | `0` |
| `resize` | `vertical` |
| `background` | `transparent` |
| `color` | `inherit` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |
| `outline` | `0` |

### Text Area Control `::placeholder`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

## 9. Svelte Notes

- Uses native `<textarea>` element inside a styled `<div>` wrapper with grid
  layout
- CSS custom properties (`--poodle-text-area-*`) enable treatment-level theming;
  same treatment token chain as TextInput but with `text-area` prefix
- `data-validation-state` data attribute drives validation border-color via CSS
  attribute selectors
- Controlled mode: when `value` prop is non-null, the textarea value is bound
  to it; `valueChange` must be handled to update
- Uncontrolled mode: internal state initialized from `defaultValue`
- Browser line wrapping, selection, IME, and undo behavior remain native
- Submit is chord-based (`Cmd/Ctrl+Enter`), not plain Enter — this is the key
  behavioral difference from TextInput
- `rows` prop sets the initial `min-height` via `calc(1lh * rows)`
- Root uses `display: grid` instead of flex (unlike TextInput) because the
  textarea is the sole child and grid enables clean sizing

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::text_area`
- GPUI implementation must intentionally support multiline caret movement,
  selection, vertical scrolling, IME composition, and text-focused shortcut
  suppression
- Enter must insert a newline; submit requires Cmd/Ctrl+Enter chord
- while focused, application-global shortcuts should defer to the text control
  unless the contract explicitly defines an exception (Cmd/Ctrl+Enter for
  submit, Escape for cancel)
- treatment radius fallback: use treatment token if set, else radius-control
- `1lh` unit for min-height: GPUI must compute equivalent based on line-height
  and row count

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] multiline editing semantics match (Enter inserts newline)
- [ ] Cmd/Ctrl+Enter fires submit (not plain Enter)
- [ ] Escape fires cancel
- [ ] value/change semantics match (controlled and uncontrolled)
- [ ] accessible naming and invalid/readonly/disabled state exposure match
- [ ] text-focused shortcut suppression matches

### Tier 2: Visual Parity

- [ ] control sizing (min-height from rows, padding) uses the same token roles
- [ ] validation emphasis uses the same semantic color roles (danger, success, accent)
- [ ] focus treatment matches (border, background, box-shadow transitions)
- [ ] disabled opacity matches (state-opacity-disabled)
- [ ] typography matches (body family, size, line-height)
- [ ] resize: vertical behavior present in both runtimes

### Tier 3: Implementation Freedom

- [ ] native browser textarea internals vs GPUI multiline editor internals stay internal
- [ ] transition timing is platform-owned
- [ ] native scrollbar visuals may differ
- [ ] treatment token fallback chain (CSS var fallback vs Rust conditional)

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native scrollbar visuals may differ | platform-native scrolling visuals are acceptable | allowed | keep editing and accessibility semantics strict |
| CSS transition timing | GPUI may not support CSS-style transitions | allowed | match where possible |
| treatment radius fallback chain | CSS var fallback vs Rust conditional | allowed | same visual result required |
| `1lh` unit support | GPUI must compute equivalent from line-height and rows | allowed | verify visual height parity |
| resize handle appearance | platform-native resize affordance is acceptable | allowed | ensure resize: vertical behavior exists |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `placeholder="Write a note…"`, `ariaLabel="Note"` | Empty multiline field with placeholder text; typing emits `valueChange` and displays character count below |

### With Initial Value

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With initial value | `defaultValue="A brief description about yourself."`, `rows={3}`, `ariaLabel="Biography"` | Multiline field pre-filled with text, 3 visible rows |

### Read-only

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Read-only | `defaultValue="This content cannot be modified by the user."`, `rows={2}`, `isReadOnly` | Field displays text that can be selected but not edited |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `placeholder="Disabled"`, `isDisabled` | Field with reduced opacity, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: Field-wrapped form textareas, notes fields, descriptions,
  comments, long-form content editors
- future follow-up: code editor and rich text editor primitives are separate
  families and should not extend this contract
