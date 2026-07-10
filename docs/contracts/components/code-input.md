# CodeInput

Status: detailed contract
Updated: 2026-07-10

> **Note:** This component replaces both `PinInput` and `TotpInput`. Use
> `mask: true` for PIN-style masked entry. Use `mask: false` (default) for
> visible code entry (OTP, verification codes, etc.).

## 1. Purpose

- Component name: `CodeInput`
- Layer: `foundation`
- Summary: a segmented code-entry input with one hidden real input and visual
  digit slots, designed for autofill, paste handling, and password-manager
  compatibility
- In scope: fixed-length digit entry, hidden real input, visual digit slots,
  one-time-code autocomplete, paste handling, completion signaling, Field
  integration, optional masking
- Out of scope: backup-code entry, auth-flow orchestration

## 2. Anatomy

```text
[Field]
  └── [Root .code-input] <div role="group">
        ├── [Hidden submission input] <input type="hidden">
        ├── [Real input .code-input__control] <input type="text">
        └── [Visual slot .code-input__slot]... (repeated `length`)
              (slot at index 2 carries `--split-after` for 6-digit codes →
               3+3 grouping gap)
```

## 3. Public Props

| Prop | Type | Default |
|------|------|---------|
| `id` | `string \| null` | `null` |
| `value` | `string \| null \| undefined` | `undefined` |
| `defaultValue` | `string` | `""` |
| `name` | `string` | `"code"` |
| `label` | `string` | `"Authenticator code"` |
| `hint` | `string \| null` | `null` |
| `error` | `string \| null` | `null` |
| `mask` | `boolean` | `false` |
| `numbersOnly` | `boolean` | `true` |
| `disabled` | `boolean` | `false` |
| `length` | `number` | `6` |
| `ariaLabel` | `string \| null` | `null` |
| `autocomplete` | `string` | `"one-time-code"` |
| `size` | `ControlSize \| null` | `null` |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` |
| `density` | `ControlDensity \| null` | `null` |
| `validationState` | `ValidationState` | `"none"` |

## 4. Callbacks

| Callback | Payload | When |
|------|---------|------|
| `onValueChange` | `string` | whenever the sanitized code changes |
| `onComplete` | `string` | when the code reaches `length` digits |

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Machine-backed via core machinery (g11 extraction sweep): sanitization,
caret-position clamping into the filled prefix, slot-click selection, and
insert-replacement math (overwrite + caret advance, length-capped) live in
`@poodle/headless` `code-input.ts`. Selection-range DOM calls and
requestAnimationFrame timing stay adapter-side.

## 5. Behavior

- By default the input sanitizes to digits only and clamps to `length`
- Set `numbersOnly={false}` to allow arbitrary text input up to `length`
- Leave `value` undefined to use uncontrolled mode seeded by `defaultValue`
- Pass `value=""` to use a controlled empty state
- A single real input owns:
  - browser autofill
  - one-time-code autocomplete
  - password-manager interaction
  - paste behavior
- Visual slots mirror the real input value
- When `mask` is true, filled slots display a bullet character instead of the
  digit
- Clicking any slot focuses the real input and moves the caret
- Clicking a filled slot selects that character so typing replaces it in place
- The component composes its own `Field` wrapper so callers can use `label`,
  `hint`, and `error` directly
- `validationState` accepts the full `ValidationState` union, but only the
  `invalid` case (or a non-null `error`) changes slot visuals; other states
  render with the default slot colors. An `error` string forces invalid styling

## 6. Accessibility

- Hidden input carries `aria-label` from the `ariaLabel` prop
- Real input uses `inputmode="numeric"` and `pattern="[0-9]*"` when
  `numbersOnly=true`; otherwise it falls back to plain text entry
- Visual slots are `aria-hidden="true"` -- only the real input is in the
  accessibility tree
- Focus ring appears on the active slot to indicate which digit is next
- Disabled state applies `aria-disabled` and prevents keyboard entry
- When `mask` is true, the real input uses `type="password"` for platform-
  native obscuring

## 7. Token Usage -- Exact Values

### Root `.code-input`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `width` | `max-content` |

Slots are square. The border color is `--code-slot-border`, derived from
`validationState` (see Validation state rows below).

| Property | Value |
|----------|-------|
| `width` | `2.25rem` |
| `height` | `2.25rem` |
| `padding` | `0` |
| `border` | `0.0625rem solid var(--code-slot-border, var(--poodle-color-border-default))` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `1rem` |
| `font-weight` | `600` |
| `line-height` | `1` |
| `text-align` | `center` |

### Slot — split-after (`.code-input__slot--split-after`, index 2 when `length === 6`)

| Property | Value |
|----------|-------|
| `margin-right` | `var(--poodle-space-inline-md)` |

This produces the 3+3 visual grouping for six-digit codes.

### Slot -- active (`.code-input__slot--active`, the slot at the caret position while focused)

| Property | Value |
|----------|-------|
| `border-color` | `var(--code-slot-focus, var(--poodle-color-accent-border))` |
| `box-shadow` | `0 0 0 var(--poodle-border-width-focus) var(--code-slot-focus-ring)` |

### Slot -- validation state (drives `--code-slot-border` / `--code-slot-focus` / `--code-slot-focus-ring`)

| validationState | `--code-slot-border` | `--code-slot-focus` | `--code-slot-focus-ring` |
|-----------------|----------------------|---------------------|---------------------------|
| not invalid (default) | `var(--poodle-color-border-default)` | `var(--poodle-color-accent-border)` | `color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)` |
| `"invalid"` (or `error` set) | `var(--poodle-color-status-danger)` | `var(--poodle-color-status-danger)` | `color-mix(in srgb, var(--poodle-color-status-danger) 24%, transparent)` |

### Slot -- disabled

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |
| `cursor` | `not-allowed` |

### Size adjustments

Slots are square (width == height) at every size.

| Size | slot width / height | slot font-size |
|------|---------------------|----------------|
| `xs` | `1.5rem` | `0.8125rem` |
| `sm` | `1.75rem` | `0.875rem` |
| `md` | `2.25rem` | `1rem` |
| `lg` | `2.75rem` | `1.125rem` |
| `xl` | `3.25rem` | `1.25rem` |

Density adjusts only the inter-slot gap: `compact` `0.25rem`, `default`
`var(--poodle-space-inline-sm)`, `comfortable` `var(--poodle-space-inline-md)`.

## 8. Specimen Definitions

### 6-digit Code

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| 6-digit code | `length={6}`, `ariaLabel="Verification code"` | Six empty slots; digits only; typing auto-advances; displays entered code on completion |

### 4-digit Masked

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| 4-digit masked | `length={4}`, `mask`, `ariaLabel="PIN"` | Four slots with password masking; entered characters display as dots |

### Alphanumeric

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Alphanumeric | `length={6}`, `numbersOnly={false}`, `ariaLabel="Recovery code"` | Six slots accepting letters and digits; clicking an earlier slot allows in-place replacement |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `length={6}`, `defaultValue="123"`, `disabled` | Six slots with first three pre-filled, reduced opacity, non-interactive |

### Invalid

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Invalid | `length={6}`, `value="999999"`, `validationState="invalid"`, `error="Invalid code"` | Six slots with danger border, error message below |

## 9. Parity Checklist

### Tier 1: Strict Parity

- [ ] fixed-length semantics match (length prop controls slot count)
- [ ] auto-advance behavior matches (digit entry moves to next slot)
- [ ] backspace-retreat behavior matches (empty slot backspace goes to previous)
- [ ] onComplete fires when all slots filled
- [ ] mask prop toggles obscured input
- [ ] group role with aria-label matches

### Tier 2: Visual Parity

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] slot is square (2.25rem x 2.25rem at md) match
- [ ] gap between slots (space-inline-sm) matches
- [ ] code-family font on slots matches
- [ ] focus ring (outline with focusRing color) matches
- [ ] disabled opacity matches

### Tier 3: Implementation Freedom

- [ ] internal input implementation (native input vs platform text field) stays internal
- [ ] paste distribution strategy is implementation-owned

## 10. Approval And Adoption Notes

- contract status: `detailed contract`
- replaces: `PinInput`, `TotpInput`
- downstream adopters: verification flows, 2FA entry, compact code-entry surfaces
