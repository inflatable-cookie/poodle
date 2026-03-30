# TotpInput

Status: detailed contract
Updated: 2026-03-28

## 1. Purpose

- Component name: `TotpInput`
- Layer: `foundation`
- Summary: a one-time-code input with one hidden real input and visual digit
  slots, designed for better autofill, paste handling, and password-manager
  compatibility than a multi-cell pin control
- In scope: fixed-length digit entry, hidden real input, visual digit slots,
  one-time-code autocomplete, paste handling, completion signaling, Field
  integration
- Out of scope: backup-code entry, auth-flow orchestration, multi-cell real
  input focus choreography (see `PinInput`)

## 2. Anatomy

```text
[Field]
  └── [Root .totp-input] <div role="group">
        ├── [Hidden submission input] <input type="hidden">
        ├── [Real input .totp-input__control] <input type="text">
        └── [Visual slot .totp-input__slot]... (repeated `length`)
```

## 3. Public Props

| Prop | Type | Default |
|------|------|---------|
| `id` | `string \| null` | `null` |
| `value` | `string \| null` | `null` |
| `defaultValue` | `string` | `""` |
| `name` | `string` | `"code"` |
| `label` | `string` | `"Authenticator code"` |
| `hint` | `string \| null` | `null` |
| `error` | `string \| null` | `null` |
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
- Clicking any slot focuses the real input and moves the caret
- The component composes its own `Field` wrapper so callers can use `label`,
  `hint`, and `error` directly

## 6. Accessibility

- Hidden input carries `autocomplete="one-time-code"`, `inputmode="numeric"`,
  and `aria-label` from the `ariaLabel` prop
- Visual slots are `aria-hidden="true"` — only the real input is in the
  accessibility tree
- Focus ring appears on the active slot to indicate which digit is next
- Disabled state applies `aria-disabled` and prevents keyboard entry

## 7. Relationship To `PinInput`

- `PinInput` uses multiple real cell inputs
- `TotpInput` intentionally uses one hidden real input plus visual slots
- Use `TotpInput` for verification-code and one-time-code workflows
- Use `PinInput` for generic multi-cell pin entry where per-cell native input
  behavior is the desired model
