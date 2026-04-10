# NumberInput

Status: active
Updated: 2026-04-09

## 1. Purpose

- Component name: `NumberInput`
- Layer: `foundation`

`NumberInput` is Poodle's single public numeric-entry component.

It covers both:

- numeric bindings for direct number editing
- string-form bindings for form workflows that need raw string values,
  validation, or prefix presentation

This replaces the old split between `NumberEntry` and `NumberInput`.

## Public Contract

- Component name: `NumberInput`
- Import: `@poodle/svelte-primitives`

## Core Props

- `value: number | string | null`
- `defaultValue: number | string | null`
- `min: number | string | null`
- `max: number | string | null`
- `step: number | string | null`
- `precision: number | null`
- `prefix: string | null`
- `validate: InputValidator | undefined`
- `validationContext: unknown`
- `validationState: ValidationState`
- `showSteppers: boolean`
- standard control props:
  `id`, `name`, `placeholder`, `disabled`, `readOnly`, `required`,
  `ariaLabel`, `describedBy`, `size`, `sizeRole`, `density`

## Events

- `valueChange`
- `validationChange`
- `submit`
- `increment`
- `decrement`
- `focus`
- `blur`

## Usage

```svelte
<script lang="ts">
  import { NumberInput } from "@poodle/svelte-primitives";

  let quantity: number | null = 1;
  let formYear = "2026";
</script>

<NumberInput bind:value={quantity} min={0} max={100} showSteppers />
<NumberInput bind:value={formYear} min={1900} max={2100} prefix="FY" />
```

## Notes

- numeric consumers receive `number | null`
- string-form consumers round-trip string values while still getting numeric
  input behavior
- steppers, clamping, and precision are shared across both modes

## 2. Accessibility

- root input: `role="spinbutton"`, `aria-valuenow`, `aria-valuemin`,
  `aria-valuemax`
- stepper buttons: `aria-label="Increment"` / `aria-label="Decrement"`
- disabled state: `aria-disabled="true"` on root input
- validation: `aria-invalid="true"` when validation state is error,
  `aria-describedby` linked to validation message
- keyboard: `ArrowUp` / `ArrowDown` to step, `Home` / `End` for min / max
  when supported

## Next Task

Keep all public docs, preview registry entries, and examples aligned to
`NumberInput` only. `NumberEntry` should not reappear as a public Svelte name.
