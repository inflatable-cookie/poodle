# Text Input

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `TextInput`
- Layer: `foundation`
- Summary: a single-line text entry control with explicit value, validation,
  focus, and submission semantics; supports prefix/suffix affixes, character
  counting, built-in async validation status indicators, and leading/trailing affordance slots
- In scope: plain text input, placeholder, prefix/suffix affixes, character
  count, validation state, submit/cancel behavior, controlled and uncontrolled
  value models
- In scope: multiline editing via `type="multiline"` or `rows > 1`
- In scope: search mode via `type="search"` — automatic leading search icon
  (when no leading slot is provided), optional clear button with `showClearButton`
  prop, and `clear` event
- In scope: slug mode via `type="slug"` — auto-normalised URL segment entry
  with source-driven generation, reserved-route protection, and the same
  validation event semantics as other typed inputs
- Out of scope: inline edit mode switching (see EditableLabel), number
  formatting (see NumberInput)

## 2. Anatomy

```text
[Root .text-input]  <div>
  ├── [Prefix .text-input__affix--prefix] (optional, when prefix prop set)
  ├── [Field .text-input__field]
  │   ├── [Leading Affordance .text-input__affordance] (optional, overlaid inside field)
  │   ├── [Input Control .text-input__control]  <input|textarea>
  │   ├── [Trailing Affordance .text-input__affordance] (optional, overlaid inside field)
  │   ├── [Clear Button .text-input__clear] (optional, search mode only)
  │   └── [Validation Indicator .text-input__validation-indicator] (optional, overlaid inside field)
  ├── [Suffix .text-input__affix--suffix] (optional, when suffix prop set)
  └── [Character Count .text-input__char-count] (optional, when showCharCount)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | field chrome container with flex layout | background, border, radius, shadow, focus ring |
| Prefix | no | non-editable text prefix with separator | text color, border, spacing |
| Field | yes | relative positioning layer for editable surface and overlaid chrome | internal spacing, overlay offsets |
| Leading Affordance | no | icon or adornment inside editable field leading edge | icon color, icon size |
| Input Control | yes | native input element (`<input>` single-line, `<textarea>` multiline) that owns its own padding | typography, text color, caret, resize |
| Trailing Affordance | no | icon or action inside editable field trailing edge | icon color, icon size |
| Clear Button | no | search clear action inside editable field trailing edge | icon color, hover state, focus ring |
| Validation Indicator | no | pending shared spinner or valid/invalid status icon overlaid inside field | status colors, icon size, motion |
| Suffix | no | non-editable text suffix with separator | text color, border, spacing |
| Character Count | no | live char count, optionally with max | code typography, status color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | none | yes | element id for label association |
| `value` | `string \| null` | `null` | no | host-owned value when supplied; `null` remains a valid controlled empty state |
| `defaultValue` | `string` | `""` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | hint text when empty |
| `name` | `string \| undefined` | `undefined` | no | form submission name |
| `autocomplete` | `string \| undefined` | `undefined` | no | native autocomplete attribute |
| `disabled` | `boolean` | `false` | no | disables editing and interaction |
| `readOnly` | `boolean` | `false` | no | allows selection without editing |
| `required` | `boolean` | `false` | no | native required attribute |
| `pattern` | `string \| undefined` | `undefined` | no | native pattern attribute |
| `spellcheck` | `boolean \| undefined` | `undefined` | no | native spellcheck attribute |
| `autocapitalize` | `string \| undefined` | `undefined` | no | native autocapitalize attribute |
| `autocorrect` | `"on" \| "off" \| undefined` | `undefined` | no | native autocorrection attribute |
| `enterKeyHint` | `"enter" \| "done" \| "go" \| "next" \| "previous" \| "search" \| "send" \| null` | `null` | no | native enterkeyhint attribute |
| `debounce` | `number \| null` | `null` | no | delays `onValueChange` while typing |
| `validate` | `InputValidator \| undefined` | `undefined` | no | optional validator function for sync or async validation |
| `validationContext` | `unknown` | `undefined` | no | app-owned context passed to `validate` |
| `validationKey` | `unknown` | `undefined` | no | optional stable value merged into validation context when validation must re-run after identity changes |
| `validationDebounce` | `number` | `300` | no | delay before validation runs while typing |
| `validateOnBlur` | `boolean` | `true` | no | whether blur triggers immediate validation |
| `showValidationStatus` | `boolean` | `true` | no | whether built-in validation status chrome is shown |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | visual and assistive validation state |
| `ariaLabel` | `string \| null` | `null` | no | required when no external label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `inputMode` | `"none" \| "search" \| "text" \| "tel" \| "url" \| "email" \| "numeric" \| "decimal" \| null` | `null` | no | virtual keyboard hint |
| `list` | `string \| null` | `null` | no | id of a `<datalist>` element to associate for native suggestion lists; passed directly to the native input as the `list` attribute |
| `type` | `string` | `"text"` | no | HTML input type attribute. When `"multiline"`, renders a `<textarea>`. When `"search"`, automatically renders a leading search icon (if no `leading` slot is provided), supports `showClearButton`, and emits `clear` event. When `"slug"`, behaves as semantic slug entry but renders a native text input for HTML compatibility |
| `rows` | `number \| null` | `null` | no | number of visible text rows; when greater than 1 and `type` is not explicitly set, auto-switches to multiline mode (equivalent to `type="multiline"`); textarea defaults to 4 rows when multiline and rows is null |
| `resize` | `"vertical" \| "horizontal" \| "both" \| "none"` | `"vertical"` | no | resize handle direction in multiline mode; only applies when the control renders as a `<textarea>` |
| `source` | `string \| null` | `null` | no | when `type="slug"`, source text used to auto-generate the slug until the user meaningfully edits the field |
| `showClearButton` | `boolean` | `true` | no | when `type="search"`, whether to show the clear button when the input has a value and is not disabled/readonly |
| `prefix` | `string \| null` | `null` | no | static text before input (e.g. "$") |
| `suffix` | `string \| null` | `null` | no | static text after input (e.g. "kg") |
| `maxLength` | `number \| null` | `null` | no | maximum character count |
| `showCharCount` | `boolean` | `false` | no | display live character counter |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `onValueChange` | `(value: string) => void` | `undefined` | no | callback fired when the value changes; respects `debounce` |
| `onValidationChange` | `(detail: { status: "idle" \| "validating" \| "valid" \| "invalid"; valid: boolean; message: string }) => void` | `undefined` | no | fired when built-in validation status changes |
| `onSubmit` | `(value: string) => void` | `undefined` | no | fires on Enter, or Cmd/Ctrl+Enter in multiline mode |
| `onCancel` | `() => void` | `undefined` | no | fires on Escape |
| `onClear` | `() => void` | `undefined` | no | fires when the built-in search clear button is used |
| `onKeyDown` | `(event: KeyboardEvent) => void` | `undefined` | no | native keydown passthrough |
| `onFocus` | `(event: FocusEvent) => void` | `undefined` | no | native focus passthrough |
| `onBlur` | `(event: FocusEvent) => void` | `undefined` | no | native blur passthrough |
| `leading` | `Snippet` | `undefined` | no | optional leading affordance snippet; replaces named `leading` slot |
| `trailing` | `Snippet` | `undefined` | no | optional trailing affordance snippet; replaces named `trailing` slot |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue` sets the initial value; component owns its own state
- do not mix controlled and uncontrolled modes simultaneously
- when `type="slug"`, `onValueChange` also fires for source-driven auto-generation
  so parent-controlled forms stay in sync

### Validation

- `validationState` remains available for caller-owned visual state
- when `validate` is provided, `TextInput` owns validation timing and calls
  `onValidationChange`
- built-in validation maps:
  - `idle` -> keep caller-provided `validationState`
  - `validating` -> `pending`
  - `valid` -> `valid`
  - `invalid` -> `invalid`
- `validationContext` is opaque and app-owned; use it when validation depends
  on sibling field values or existing record identifiers
- `validationKey` is merged into `validationContext` before validation runs;
  use it when the caller needs a stable revalidation trigger without replacing
  the whole context object

### Slug Mode

- `type="slug"` is a semantic mode, not a native HTML input type
- the rendered control uses `<input type="text">` with:
  - `autocapitalize="off"`
  - `spellcheck={false}`
  - `inputmode="text"`
- slug mode uses `var(--poodle-typography-code-family)` for both the editable
  value and any prefix affix so the full slug reads as one code-like unit
- slug mode sizes editable and affix text through
  `var(--poodle-typography-code-adjustmentRatio)` so monospace treatment can be
  globally tuned without per-component overrides
- input normalisation rules:
  - accents are removed
  - lowercase is enforced
  - spaces and underscores become hyphens
  - repeated hyphens collapse
  - leading and trailing hyphens are trimmed
- built-in validation rules:
  - valid slugs contain lowercase letters, numbers, and single hyphen separators
  - length must be between `2` and `maxLength` (default `100`)
  - reserved route-like slugs are rejected: `new`, `edit`, `delete`, `create`, `update`, `list`, `admin`, `api`, `auth`, `login`, `logout`, `register`, `settings`, `profile`, `dashboard`, `search`
- when `prefix` is present, built-in validation and custom `validate` receive
  the combined candidate `prefix + value`
- when `source` is present, the control auto-generates from `source` until the
  user edits the field; if the user clears the slug entirely, auto-generation
  resumes on later source changes
- custom `validate` still runs after built-in slug checks and receives the
  combined slug candidate plus merged validation context

### Multiline Mode

- multiline mode is entered when `type="multiline"` or when `rows` is set to a
  value greater than 1 with the default `type="text"`
- the rendered control switches from `<input>` to `<textarea>`
- multiline root loses the fixed `min-height` from the single-line control; the
  textarea's natural height (driven by `rows`) replaces it
- `rows` defaults to `4` when multiline and no rows value is provided
- `resize` controls the native resize handle: `"vertical"` (default), `"horizontal"`,
  `"both"`, or `"none"`
- character count, when shown, is overlaid in the bottom-right of the textarea
  at `0.375rem` from the bottom and `0.5rem` from the right edge
- char-count overlay adds extra bottom padding to the control equal to the
  character count element height so typed text never runs under the counter
- all other behaviour (validation, affixes, affordances, focus ring, disabled
  state, slug and search modes) is incompatible with multiline and should not
  be combined with it

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral input chrome with default border and background |
| hover | pointer enters root | no explicit hover style on root (delegated to focus) |
| focus | focus-within on root | border-color switches to focus border, background to focus fill, box-shadow to focus shadow |
| disabled | `disabled=true` | opacity reduced via `state-opacity-disabled`, interaction suppressed |
| readOnly | `readOnly=true` | selectable but not editable, no visual change beyond native behavior |
| invalid | `validationState="invalid"` | border-color switches to `status-danger` |
| valid | `validationState="valid"` | border-color switches to `status-success` |
| pending | `validationState="pending"` | border-color switches to `accent-base` |
| pending-indicator | `validationState="pending"` | trailing spinner shows in accent color |
| valid-indicator | `validationState="valid"` | trailing tick icon shows in success color |
| invalid-indicator | `validationState="invalid"` | trailing cross icon shows in danger color |
| char-over | character count exceeds maxLength | char count text color switches to `status-danger` |

The built-in trailing indicators (`pending-indicator`, `valid-indicator`, `invalid-indicator`) only render when `showValidationStatus` is `true` **and** the effective validation state is not `none`; when `showValidationStatus` is `false` the validation border-color states still apply but no indicator icon/spinner is shown.

### Behavior Machine

Behavior classification: machine-backed via core machinery

Value semantics from `@poodle/headless`: `slugify` / `isValidSlugFormat`
(slug mode normalization and format validation) and
`validationStatusToState` (validation status to visual state mapping).
Debounce, async validation orchestration, and submit/cancel/clear event
plumbing stay adapter-side.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | user edits value | `string` | fires on each input change; respects `debounce` |
| `onValidationChange` | built-in validation status changes | `{ status: "idle" \| "validating" \| "valid" \| "invalid"; valid: boolean; message: string }` | only fires when `validate` is provided |
| `onSubmit` | Enter key pressed | `string` | fires current value |
| `onCancel` | Escape key pressed | none | fires with no payload |
| `onClear` | built-in search clear button pressed | none | only used in search mode |
| `onFocus` | input receives focus | `FocusEvent` | native focus passthrough |
| `onBlur` | input loses focus | `FocusEvent` | native blur passthrough |
| `onKeyDown` | keydown on the control | `KeyboardEvent` | native keydown passthrough |

## 6. Accessibility

### Semantics

- Role: native `<input>` element
- `id`: from prop, used for external `<label for>` association
- `aria-label`: from ariaLabel prop; required when no external label exists
- `aria-describedby`: from describedBy prop
- `aria-invalid`: `"true"` when validationState is `"invalid"`
- `readonly`: native readonly attribute set when readOnly (note: `aria-readonly` is NOT explicitly set; the native `readonly` attribute is used instead)
- `disabled`: native disabled attribute when disabled
- `required`: native required attribute when required
- `pattern`: native pattern attribute when provided
- `autocomplete`: native autocomplete attribute when provided
- `autocorrect`: native autocorrection attribute when provided
- `maxlength`: from maxLength prop
- `inputmode`: from inputMode prop
- `list`: from list prop; associates a native datalist for browser-provided suggestions
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
| `Enter` | calls `onSubmit` with the current value |
| `Escape` | calls `onCancel` |
| `Tab` | moves focus out of the control |

### Focus And Announcement

- focus entry: root receives visible focus treatment (border, background, and
  shadow transition); caret appears in input
- focus exit: focus treatment clears; validation can be surfaced by parent
  Field wrapper on blur
- live-region behavior: validation announcement is parent-owned (Field), but the
  input must expose invalid state via `aria-invalid`
- validation indicator is decorative chrome only and must remain `aria-hidden`
- GPUI-native accessibility mapping notes: GPUI must expose role/control type,
  accessible name, value, readonly/disabled/invalid state, selection/caret
  behavior, and IME-safe text entry semantics through native accessibility APIs

## 7. Layout

### Sizing

- minimum height: `size-control-height` (from token) — single-line only
- multiline mode: `min-height: auto`; natural height is driven by `rows` (default 4);
  minimum textarea height is `calc(1lh * 4)` (four line-heights at the current
  font size)
- width: stretches with parent; `min-width: 0` on the input allows flex shrink
- single-line content scrolls horizontally if it exceeds the available width
- multiline content wraps and scrolls vertically

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
| `--poodle-text-input-radius` | `var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control))` | — |
| `--poodle-text-input-fill` | `var(--poodle-treatment-interactive-subtle-fill, var(--poodle-color-background-surface))` | `var(--poodle-text-input-fill-focus)` |
| `--poodle-text-input-border` | `var(--poodle-treatment-interactive-subtle-border, var(--poodle-color-border-default))` | `var(--poodle-text-input-border-focus)` |
| `--poodle-text-input-shadow` | (none by default) | `var(--poodle-text-input-shadow-focus)` fallback `0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)` |

### Root `.text-input`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-text-input-border)` |
| `border-radius` | `var(--poodle-text-input-radius)` |
| `background` | `var(--poodle-text-input-fill)` |
| `color` | `var(--poodle-color-text-primary)` |
| `box-shadow` | `var(--poodle-text-input-shadow)` |
| `transition` | `border-color, box-shadow, background` all at `var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Root focus-within

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-text-input-border-focus)` |
| `background` | `var(--poodle-text-input-fill-focus)` |
| `box-shadow` | `var(--poodle-text-input-shadow-focus)` fallback `0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)` |

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

### Input Control `.text-input__control`

| Property | Value |
|----------|-------|
| `display` | `block` |
| `width` | `100%` |
| `height` | `calc(var(--poodle-size-control-height) - (var(--poodle-border-width-default) * 2))` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `inherit` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |
| `outline` | `0` |

### Field `.text-input__field`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `flex` |
| `flex` | `1` |
| `min-width` | `0` |
| `align-items` | `stretch` |

### Input Control `::placeholder`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `opacity` | `var(--poodle-state-opacity-muted)` |

### Affordance `.text-input__affordance`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `50%` |
| `transform` | `translateY(-50%)` |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `color` | `var(--poodle-color-icon-muted)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `var(--poodle-icon-size-default)` |
| `pointer-events` | `none` |

### Validation Indicator `.text-input__validation-indicator`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `50%` |
| `transform` | `translateY(-50%)` |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `color` | `var(--poodle-color-icon-muted)` |
| `pointer-events` | `none` |

### Validation Indicator state colors

| State | `color` |
|-------|---------|
| `pending` | `var(--poodle-color-accent-base)` |
| `valid` | `var(--poodle-color-status-success)` |
| `invalid` | `var(--poodle-color-status-danger)` |

### Spinner

Pending validation uses the shared [`Spinner`](./spinner.md) primitive with
`variant="ring"`, `sizeRole="chrome"`, and `tone="current"` inside the validation
indicator slot.

### Affix `.text-input__affix`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `align-self` | `stretch` |
| `padding-inline` | `0.625rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `opacity` | `var(--poodle-state-opacity-muted)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `white-space` | `nowrap` |
| `user-select` | `none` |

Affixes use a single flat `padding-inline: 0.625rem` (no separate margin); horizontal separation from the input comes from the separator border below.

### Prefix `.text-input__affix--prefix`

| Property | Value |
|----------|-------|
| `border-right` | `0.0625rem solid var(--poodle-color-border-default)` |

### Suffix `.text-input__affix--suffix`

| Property | Value |
|----------|-------|
| `border-left` | `0.0625rem solid var(--poodle-color-border-default)` |

### Character Count `.text-input__char-count`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-muted)` |
| `font` | `var(--poodle-typography-code-xs)` (shorthand: family/size/line-height — resolves to `0.6875rem` code) |
| `pointer-events` | `none` |

In single-line mode the counter sits inline; in multiline mode it is absolutely positioned (`inset-inline-end: 0.5rem`, `inset-block-end: 0.375rem`) over the bottom-right of the textarea.

### Character Count over limit `.text-input__char-count--over`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-status-danger)` |

### Size adjustments

| Size | min-height | padding | font-size |
|------|------------|---------|-----------|
| `xs` | `calc(control-height - 0.5rem)` | `0 calc(space-control-x - 0.125rem)` | `0.75rem` |
| `sm` | `calc(control-height - 0.375rem)` | `0 calc(space-control-x - 0.0625rem)` | `typography-body-size` |
| `md` | `control-height` | `0 space-control-x` | `typography-body-size` |
| `lg` | `calc(control-height + 0.375rem)` | `0 calc(space-control-x + 0.125rem)` | `0.9375rem` |
| `xl` | `calc(control-height + 0.5rem)` | `0 calc(space-control-x + 0.1875rem)` | `1rem` |

### Density adjustments

Density shifts the control's inline and block padding via adjustment vars (`default` adds zero). The block adjustment changes the effective vertical text inset on the control (`padding-block: calc(text-input-padding-block + density-block-adjust)`), which is an intentional exception to the size/density orthogonality rule for this control.

| Density | `--poodle-text-input-density-inline-adjust` | `--poodle-text-input-density-block-adjust` |
|---------|---------------------------------------------|--------------------------------------------|
| `compact` | `-0.125rem` | `-0.0625rem` |
| `default` | `0` | `0` |
| `comfortable` | `0.125rem` | `0.0625rem` |

### Adornment padding reservation

The input control reserves inline padding for leading/trailing adornments so text never runs under overlaid chrome. The reserved padding is computed from the adornment counts, not fixed:

- **Start padding** (`--poodle-text-input-control-padding-start`): when a leading affordance is present, `calc(padding-inline + icon-size-default + adornment-gap * 1.5)`; otherwise `padding-inline`.
- **End padding** (`--poodle-text-input-control-padding-end`): with `n` trailing adornments (trailing affordance + clear button + validation indicator), `calc(padding-inline + n * icon-size-default + n * adornment-gap)`; `padding-inline` when `n` is `0`.

## 9. Svelte Notes

- Uses native `<input>` element inside a styled `<div>` wrapper
- CSS custom properties (`--poodle-text-input-*`) enable treatment-level theming
- Treatment token chain: `--poodle-treatment-interactive-subtle-radius` falls back
  to `--poodle-radius-control`
- `data-validation-state` data attribute drives validation border-color via CSS
  attribute selectors
- non-`none` validation states also render a built-in trailing indicator:
  shared spinner for `pending`, `check` for `valid`, `x` for `invalid`
- Controlled mode: when `value` is host-owned, `onValueChange` must update it
- Uncontrolled mode: internal state initialized from `defaultValue`
- Browser autofill, IME, selection, and undo behavior remain native
- `prefix` and `suffix` props render static text with separator borders;
  these are not editable and have `user-select: none`
- Character count renders `{currentLength}/{maxLength}` when both
  `showCharCount` and `maxLength` are set, or `{currentLength}` when only
  `showCharCount` is set
- Emits `data-size` on root element reflecting the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::text_input`
- GPUI implementation must intentionally handle caret movement, selection,
  clipboard shortcuts, IME composition, and text-focused keybinding suppression
- while focused, application-global shortcuts should defer to the text control
  unless the contract explicitly defines an exception (Enter for submit, Escape
  for cancel)
- prefix/suffix affixes should be rendered as static text with separator
  treatment but excluded from the editable value
- character count must be computed from the same value source as the input
- treatment radius fallback: use treatment token if set, else radius-control

## 11a. Jetstream Notes

- `TextInput::from_spec(spec, theme).on_clear(...)`.
- The clear button is the only part of a field a pointer can reach, so it is the
  only wired event. It renders for a `search` field with a value, and a
  disabled or read-only field does not clear.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value/change semantics match (controlled and uncontrolled)
- [ ] caret navigation and selection semantics match
- [ ] accessible naming and invalid/readonly/disabled state exposure match
- [ ] Enter calls `onSubmit`; Escape calls `onCancel`
- [ ] text-focused shortcut suppression matches
- [ ] maxLength enforcement matches
- [ ] prefix/suffix are non-editable and visually separated

### Tier 2: Visual Parity

- [ ] control sizing (min-height, padding, gap) uses the same token roles
- [ ] validation emphasis uses the same semantic color roles (danger, success, accent)
- [ ] built-in validation indicator semantics match (spinner, success icon, danger icon)
- [ ] focus treatment matches (border, background, box-shadow transitions)
- [ ] affix separator border uses the same color (`border-default` solid, `0.0625rem`)
- [ ] character count typography matches (`typography-code-xs` shorthand, color `text-muted`)
- [ ] character count over-limit color matches (status-danger)
- [ ] disabled opacity matches (state-opacity-disabled)
- [ ] all five sizes visually match (height, padding, font-size per size table)

### Tier 3: Implementation Freedom

- [ ] native browser input internals vs GPUI text system internals stay internal
- [ ] transition timing is platform-owned
- [ ] treatment token fallback chain (CSS var fallback vs Rust conditional)

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Jetstream raises no typing or key events | the runtime delivers pointer events only, so `onValueChange`, `onKeyDown`, `onSubmit` and `onCancel` have no route; the host owns the editor and feeds the value back through the spec | accepted, tracked | g12.017 |
| native text-caret visuals may differ | platform-native text rendering is acceptable | allowed | keep editing semantics strict |
| CSS transition timing | GPUI may not support CSS-style transitions | allowed | match where possible |
| treatment radius fallback chain | CSS var fallback vs Rust conditional | allowed | same visual result required |
| affix separator border color | GPUI must achieve same visual result by any means | allowed | verify visual parity (separator is `border-default` solid) |

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

### Pending validation

One text input showing async validation in progress:

| Label | Placeholder | Validation | Notes |
|-------|-------------|------------|-------|
| Workspace | acme-admin | pending | shows built-in trailing spinner and parent pending message |

### Disabled

One disabled text input:

| Label | Value | Props |
|-------|-------|-------|
| API key | sk-••••••••1234 | `disabled: true` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: EditableLabel, NumberInput,
  Field-wrapped form inputs, Aura search/edit controls, Spark search/edit controls
  (search behavior is now built into TextInput via type="search")
- future follow-up: attach richer validation timing rules once Field wrappers
  are fully adopted
