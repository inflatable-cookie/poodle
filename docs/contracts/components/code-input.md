# CodeInput

Status: detailed contract
Updated: 2026-03-30

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
```

## 3. Public Props

| Prop | Type | Default |
|------|------|---------|
| `id` | `string \| null` | `null` |
| `value` | `string \| null` | `null` |
| `defaultValue` | `string` | `""` |
| `name` | `string` | `"code"` |
| `label` | `string` | `"Verification code"` |
| `hint` | `string \| null` | `null` |
| `error` | `string \| null` | `null` |
| `mask` | `boolean` | `false` |
| `disabled` | `boolean` | `false` |
| `length` | `number` | `6` |
| `ariaLabel` | `string \| null` | `null` |
| `autocomplete` | `string` | `"one-time-code"` |
| `size` | `ControlSize \| null` | `null` |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` |
| `density` | `ControlDensity \| null` | `null` |
| `validationState` | `ValidationState` | `"none"` |

## 4. Events

| Event | Payload | When |
|------|---------|------|
| `valueChange` | `{ value: string }` | whenever the sanitized code changes |
| `complete` | `{ value: string }` | when the code reaches `length` digits |

## 5. Behavior

- The input sanitizes to digits only and clamps to `length`
- A single real input owns:
  - browser autofill
  - one-time-code autocomplete
  - password-manager interaction
  - paste behavior
- Visual slots mirror the real input value
- When `mask` is true, filled slots display a bullet character instead of the
  digit
- Clicking any slot focuses the real input and moves the caret
- The component composes its own `Field` wrapper so callers can use `label`,
  `hint`, and `error` directly

## 6. Accessibility

- Hidden input carries `autocomplete="one-time-code"`, `inputmode="numeric"`,
  and `aria-label` from the `ariaLabel` prop
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
| `gap` | `0.375rem` |

### Slot `.code-input__slot`

| Property | Value |
|----------|-------|
| `width` | `2.25rem` |
| `height` | `2.5rem` |
| `padding` | `0` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `1rem` |
| `line-height` | `1` |
| `text-align` | `center` |

### Slot -- focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Slot -- disabled

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Size adjustments

| Size | slot width | slot height | slot font-size |
|------|------------|-------------|----------------|
| `xs` | `calc(control-height - 0.5rem)` | `calc(control-height - 0.25rem)` | `0.8125rem` |
| `sm` | `calc(control-height - 0.25rem)` | `control-height` | `0.875rem` |
| `md` | `2.25rem` | `2.5rem` | `1rem` |
| `lg` | `calc(control-height + 0.25rem)` | `calc(control-height + 0.5rem)` | `1.125rem` |
| `xl` | `calc(control-height + 0.5rem)` | `calc(control-height + 0.75rem)` | `1.25rem` |

## 8. Specimen Definitions

### 6-digit Code

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| 6-digit code | `length={6}`, `ariaLabel="Verification code"` | Six empty slots; typing auto-advances; displays entered code on completion |

### 4-digit Masked

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| 4-digit masked | `length={4}`, `mask`, `ariaLabel="PIN"` | Four slots with password masking; entered characters display as dots |

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
- [ ] complete event fires when all slots filled
- [ ] mask prop toggles obscured input
- [ ] group role with aria-label matches

### Tier 2: Visual Parity

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] slot width (2.25rem) and height (2.5rem) match
- [ ] gap between slots (0.375rem) matches
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
