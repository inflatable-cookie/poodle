<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let id: string | null = null;
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let min: string | null = null;
  export let max: string | null = null;
  export let step = 60;
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string | null };
  }>();
  const uiPresentation = getUiPresentation();

  let uncontrolledValue = defaultValue;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: currentValue = value ?? uncontrolledValue ?? "";

  function handleInput(event: Event): void {
    const nextValue = (event.currentTarget as HTMLInputElement).value || null;

    if (value === null) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }
</script>

<input
  id={id ?? undefined}
  class="time-input"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  type="time"
  value={currentValue}
  {min}
  {max}
  {step}
  disabled={disabled}
  aria-label={ariaLabel ?? undefined}
  aria-describedby={describedBy ?? undefined}
  on:input={handleInput}
/>

<style>
  .time-input {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .time-input:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .time-input:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .time-input[data-size="xs"] {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.75rem;
  }

  .time-input[data-size="sm"] {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
  }

  .time-input[data-size="lg"] {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.9375rem;
  }

  .time-input[data-size="xl"] {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 1rem;
  }

  /* Density variants */
  .time-input[data-density="compact"] { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .time-input[data-density="comfortable"] { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>
