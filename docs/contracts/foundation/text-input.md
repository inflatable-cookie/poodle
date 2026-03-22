# Text Input

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `TextInput`
- Layer: `foundation`
- Summary: a single-line text entry control with explicit value, validation,
  focus, and submission semantics; supports prefix/suffix affixes, character
  counting, and leading/trailing affordance slots
- In scope: plain text input, placeholder, prefix/suffix affixes, character
  count, validation state, submit/cancel behavior, controlled and uncontrolled
  value models
- Out of scope: multiline editing (see TextArea), search-specific clear behavior
  (see SearchField), inline edit mode switching (see EditableLabel), number
  formatting (see NumberEntry)

## 2. Anatomy

```text
[Root .text-input]  <div>
  ├── [Prefix .text-input__affix--prefix] (optional, when prefix prop set)
  ├── [Leading Affordance .text-input__affordance] (optional, via slot)
  ├── [Input Control .text-input__control]  <input>
  ├── [Character Count .text-input__char-count] (optional, when showCharCount)
  ├── [Trailing Affordance .text-input__affordance] (optional, via slot)
  └── [Suffix .text-input__affix--suffix] (optional, when suffix prop set)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | field chrome container with flex layout | background, border, radius, shadow, focus ring |
| Prefix | no | non-editable text prefix with separator | text color, border, spacing |
| Leading Affordance | no | icon or adornment before text | icon color, icon size |
| Input Control | yes | native single-line text input element | typography, text color, caret |
| Character Count | no | live char count, optionally with max | code typography, status color |
| Trailing Affordance | no | icon or action after text | icon color, icon size |
| Suffix | no | non-editable text suffix with separator | text color, border, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | none | yes | element id for label association |
| `value` | `string \| null` | `null` | no | controlled value; when non-null, component is controlled |
| `defaultValue` | `string` | `""` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | hint text when empty |
| `name` | `string \| undefined` | `undefined` | no | form submission name |
| `isDisabled` | `boolean` | `false` | no | disables editing and interaction |
| `isReadOnly` | `boolean` | `false` | no | allows selection without editing |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | visual and assistive validation state |
| `ariaLabel` | `string \| null` | `null` | no | required when no external label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `inputMode` | `string \| undefined` | `undefined` | no | virtual keyboard hint (e.g. "numeric") |
| `type` | `string` | `"text"` | no | HTML input type attribute |
| `prefix` | `string \| null` | `null` | no | static text before input (e.g. "$") |
| `suffix` | `string \| null` | `null` | no | static text after input (e.g. "kg") |
| `maxLength` | `number \| null` | `null` | no | maximum character count |
| `showCharCount` | `boolean` | `false` | no | display live character counter |

### Controlled And Uncontrolled

- controlled: `value` (non-null) plus `valueChange` event
- uncontrolled: `defaultValue` sets the initial value; component owns its own state
- do not mix controlled and uncontrolled modes simultaneously

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral input chrome with default border and background |
| hover | pointer enters root | no explicit hover style on root (delegated to focus) |
| focus | focus-within on root | border-color switches to focus border, background to focus fill, box-shadow to focus shadow |
| disabled | `isDisabled=true` | opacity reduced via `state-opacity-disabled`, interaction suppressed |
| readOnly | `isReadOnly=true` | selectable but not editable, no visual change beyond native behavior |
| invalid | `validationState="invalid"` | border-color switches to `status-danger` |
| valid | `validationState="valid"` | border-color switches to `status-success` |
| pending | `validationState="pending"` | border-color switches to `accent-base` |
| char-over | character count exceeds maxLength | char count text color switches to `status-danger` |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user edits value | `{ value: string }` | fires on each input change |
| `submit` | Enter key pressed | `{ value: string }` | fires current value |
| `cancel` | Escape key pressed | `void` | fires with no payload |
| `focus` | input receives focus | `FocusEvent` | native focus event passthrough |
| `blur` | input loses focus | `FocusEvent` | native blur event passthrough |

## 6. Accessibility

### Semantics

- Role: native `<input>` element
- `id`: from prop, used for external `<label for>` association
- `aria-label`: from ariaLabel prop; required when no external label exists
- `aria-describedby`: from describedBy prop
- `aria-invalid`: `"true"` when validationState is `"invalid"`
- `aria-readonly`: set when isReadOnly
- `disabled`: native disabled attribute when isDisabled
- `maxlength`: from maxLength prop
- `inputmode`: from inputMode prop
- Prefix/suffix: decorative, not announced; `user-select: none`
- Labeling rules: placeholder text never counts as the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| character input | inserts text |
| `Arrow Left/Right` | moves caret |
| `Home/End` | moves to start/end |
| `Shift+Arrow` | extends selection |
| platform copy/cut/paste/select-all shortcuts | operate on text selection |
| `Enter` | fires `submit` event with current value |
| `Escape` | fires `cancel` event |
| `Tab` | moves focus out of the control |

### Focus And Announcement

- focus entry: root receives visible focus treatment (border, background, and
  shadow transition); caret appears in input
- focus exit: focus treatment clears; validation can be surfaced by parent
  Field wrapper on blur
- live-region behavior: validation announcement is parent-owned (Field), but the
  input must expose invalid state via `aria-invalid`
- GPUI-native accessibility mapping notes: GPUI must expose role/control type,
  accessible name, value, readonly/disabled/invalid state, selection/caret
  behavior, and IME-safe text entry semantics through native accessibility APIs

## 7. Layout

### Sizing

- minimum height: `size-control-height` (from token)
- width: stretches with parent; `min-width: 0` on the input allows flex shrink
- content remains single-line and horizontally scrolls internally if necessary

### Composition

- parent expectations: Field wrappers, forms, toolbars, search shells, inline
  editors
- child expectations: optional icon affordances via slots, prefix/suffix text
- resizing rules: affordances and affixes do not collapse the text-edit area
  below usable width

## 8. Token Usage — Exact Values

### CSS Custom Properties (treatment system)

| Var | Default Value | Focus Value |
|-----|---------------|-------------|
| `--pug-text-input-radius` | `var(--pug-treatment-interactive-subtle-radius, var(--pug-radius-control))` | — |
| `--pug-text-input-fill` | `var(--pug-treatment-interactive-subtle-fill, var(--pug-color-background-surface))` | `var(--pug-text-input-fill-focus)` |
| `--pug-text-input-border` | `var(--pug-treatment-interactive-subtle-border, var(--pug-color-border-default))` | `var(--pug-text-input-border-focus)` |
| `--pug-text-input-shadow` | (none by default) | `var(--pug-text-input-shadow-focus)` fallback `0 0 0 var(--pug-border-width-focus) color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent)` |

### Root `.text-input`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--pug-space-inline-sm)` |
| `min-height` | `var(--pug-size-control-height)` |
| `padding` | `0 var(--pug-space-control-x)` |
| `border` | `0.0625rem solid var(--pug-text-input-border)` |
| `border-radius` | `var(--pug-text-input-radius)` |
| `background` | `var(--pug-text-input-fill)` |
| `color` | `var(--pug-color-text-primary)` |
| `box-shadow` | `var(--pug-text-input-shadow)` |
| `transition` | `border-color, box-shadow, background` all at `var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard)` |

### Root focus-within

| Property | Value |
|----------|-------|
| `border-color` | `var(--pug-text-input-border-focus)` |
| `background` | `var(--pug-text-input-fill-focus)` |
| `box-shadow` | `var(--pug-text-input-shadow-focus)` fallback `0 0 0 var(--pug-border-width-focus) color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent)` |

### Root validation states

| State | `border-color` |
|-------|----------------|
| `invalid` | `var(--pug-color-status-danger)` |
| `valid` | `var(--pug-color-status-success)` |
| `pending` | `var(--pug-color-accent-base)` |

### Root disabled (`:has(:disabled)`)

| Property | Value |
|----------|-------|
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Input Control `.text-input__control`

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `min-width` | `0` |
| `width` | `100%` |
| `height` | `calc(var(--pug-size-control-height) - (var(--pug-border-width-default) * 2))` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `inherit` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `var(--pug-typography-body-size)` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |
| `outline` | `0` |

### Input Control `::placeholder`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |

### Affordance `.text-input__affordance`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `color` | `var(--pug-color-icon-muted)` |
| `font-family` | `var(--pug-typography-code-family)` |
| `font-size` | `var(--pug-icon-size-default)` |

### Affix `.text-input__affix`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `var(--pug-typography-body-size)` |
| `white-space` | `nowrap` |
| `user-select` | `none` |

### Prefix `.text-input__affix--prefix`

| Property | Value |
|----------|-------|
| `padding-right` | `var(--pug-space-inline-sm)` |
| `border-right` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 52%, transparent)` |
| `margin-right` | `var(--pug-space-inline-sm)` |

### Suffix `.text-input__affix--suffix`

| Property | Value |
|----------|-------|
| `padding-left` | `var(--pug-space-inline-sm)` |
| `border-left` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 52%, transparent)` |
| `margin-left` | `var(--pug-space-inline-sm)` |

### Character Count `.text-input__char-count`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-code-family)` |
| `font-size` | `0.6875rem` |
| `white-space` | `nowrap` |

### Character Count over limit `.text-input__char-count--over`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-status-danger)` |

## 9. Svelte Notes

- Uses native `<input>` element inside a styled `<div>` wrapper
- CSS custom properties (`--pug-text-input-*`) enable treatment-level theming
- Treatment token chain: `--pug-treatment-interactive-subtle-radius` falls back
  to `--pug-radius-control`
- `data-validation` data attribute drives validation border-color via CSS
  attribute selectors
- Controlled mode: when `value` prop is non-null, the input value is bound to
  it; `valueChange` must be handled to update
- Uncontrolled mode: internal state initialized from `defaultValue`
- Browser autofill, IME, selection, and undo behavior remain native
- `prefix` and `suffix` props render static text with separator borders;
  these are not editable and have `user-select: none`
- Character count renders `{currentLength}/{maxLength}` when both
  `showCharCount` and `maxLength` are set, or `{currentLength}` when only
  `showCharCount` is set

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::text_input`
- GPUI implementation must intentionally handle caret movement, selection,
  clipboard shortcuts, IME composition, and text-focused keybinding suppression
- while focused, application-global shortcuts should defer to the text control
  unless the contract explicitly defines an exception (Enter for submit, Escape
  for cancel)
- prefix/suffix affixes should be rendered as static text with separator
  treatment but excluded from the editable value
- character count must be computed from the same value source as the input
- treatment radius fallback: use treatment token if set, else radius-control

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value/change semantics match (controlled and uncontrolled)
- [ ] caret navigation and selection semantics match
- [ ] accessible naming and invalid/readonly/disabled state exposure match
- [ ] Enter fires submit event; Escape fires cancel event
- [ ] text-focused shortcut suppression matches
- [ ] maxLength enforcement matches
- [ ] prefix/suffix are non-editable and visually separated

### Tier 2: Visual Parity

- [ ] control sizing (min-height, padding, gap) uses the same token roles
- [ ] validation emphasis uses the same semantic color roles (danger, success, accent)
- [ ] focus treatment matches (border, background, box-shadow transitions)
- [ ] affix separator border uses the same color-mix formula (border-subtle 52%)
- [ ] character count typography matches (code family, 0.6875rem)
- [ ] character count over-limit color matches (status-danger)
- [ ] disabled opacity matches (state-opacity-disabled)

### Tier 3: Implementation Freedom

- [ ] native browser input internals vs GPUI text system internals stay internal
- [ ] transition timing is platform-owned
- [ ] treatment token fallback chain (CSS var fallback vs Rust conditional)

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native text-caret visuals may differ | platform-native text rendering is acceptable | allowed | keep editing semantics strict |
| CSS transition timing | GPUI may not support CSS-style transitions | allowed | match where possible |
| treatment radius fallback chain | CSS var fallback vs Rust conditional | allowed | same visual result required |
| color-mix formulas for affix separators | GPUI must achieve same visual result by any means | allowed | verify visual parity |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

One basic text input with label and help text:

| Label | Placeholder | Help text | Notes |
|-------|-------------|-----------|-------|
| Name | Jane Doe | Enter your full name | empty, interactive |

### With validation

One text input showing validation:

| Label | Placeholder | Validation | Notes |
|-------|-------------|------------|-------|
| Email | you@example.com | invalid when missing `@`, valid otherwise | shows error message "Please enter a valid email address" when invalid |

### Disabled

One disabled text input:

| Label | Value | Props |
|-------|-------|-------|
| API key | sk-••••••••1234 | `isDisabled: true` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: SearchField (composition), EditableLabel, NumberEntry,
  Field-wrapped form inputs, Aura search/edit controls, Spark search/edit controls
- future follow-up: attach richer validation timing rules once Field wrappers
  are fully adopted
