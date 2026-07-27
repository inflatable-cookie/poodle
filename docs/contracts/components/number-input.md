# NumberInput

Status: active
Updated: 2026-07-10

## 1. Purpose

- Component name: `NumberInput`
- Layer: `foundation`

`NumberInput` is Poodle's single public numeric-entry component.

It covers both:

- numeric bindings for direct number editing
- string-form bindings for form workflows that need raw string values,
  validation, or prefix/suffix presentation (currency, units)

This replaces the old split between `NumberEntry` and `NumberInput`.

## Public Contract

- Component name: `NumberInput`
- Import: `@poodle/svelte`

## Core Props

- `value: number | string | null | undefined`
- `defaultValue: number | string | null`
- `min: number | string | null`
- `max: number | string | null`
- `step: number | string | null`
- `precision: number | null`
- `prefix: string | null`
- `suffix: string | null`
- `validate: InputValidator | undefined`
- `validationContext: unknown`
- `validationState: ValidationState`
- `showSteppers: boolean`
- standard control props:
  `id`, `name`, `placeholder`, `disabled`, `readOnly`, `required`,
  `ariaLabel`, `describedBy`, `size`, `sizeRole`, `density`

### Behavior Machine

Behavior classification: machine-backed via core machinery

Value semantics from `@poodle/headless`: `parseNumberish` (numeric
coercion; empty and non-finite become null), `parseStep` (invalid or
non-positive steps fall back to 1), `clampNullable` (optional min/max
bounds), `validationStatusToState`. Increment/decrement, commit-on-blur,
and async validation orchestration stay adapter-side.

## Callbacks

- `onValueChange`
- `onValidationChange`
- `onSubmit`
- `onIncrement`
- `onDecrement`
- `onFocus`
- `onBlur`

## Usage

```svelte
<script lang="ts">
  import { NumberInput } from "@poodle/svelte";

  let quantity: number | null = 1;
  let formYear = "2026";
</script>

<NumberInput bind:value={quantity} min={0} max={100} showSteppers />
<NumberInput bind:value={formYear} min={1900} max={2100} prefix="FY" />
<NumberInput bind:value={weight} min={0} suffix="kg" />
```

## Notes

- numeric consumers receive `number | null`
- string-form consumers round-trip string values while still getting numeric
  input behavior
- steppers, clamping, and precision are shared across both modes
- leave `value` undefined to use uncontrolled mode seeded by `defaultValue`
- pass `value={null}` for a controlled empty state

## 2. Accessibility

- root input: `role="spinbutton"`, `aria-valuenow`, `aria-valuemin`,
  `aria-valuemax`
- root input: an accessible name is **required** — `aria-label` from
  `ariaLabel`, or an associated `<label>`. The component has no caption of its
  own, so without one the control is announced as "spin button" and its value,
  with nothing to say which quantity it holds. A `NumberInput` with no
  accessible name is invalid usage, not a permitted default.
- stepper buttons: `aria-label="Increment"` / `aria-label="Decrement"`
- disabled state: `aria-disabled="true"` on root input
- validation: `aria-invalid="true"` when validation state is error,
  `aria-describedby` linked to validation message
- keyboard: `ArrowUp` / `ArrowDown` to step, `Home` / `End` for min / max
  when supported

## Next Task

Keep all public docs, preview registry entries, and examples aligned to
`NumberInput` only. `NumberEntry` should not reappear as a public Svelte name.
