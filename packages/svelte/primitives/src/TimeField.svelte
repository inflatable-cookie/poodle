<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let id: string | null = null;
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let min: string | null = null;
  export let max: string | null = null;
  export let step = 60;
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string | null };
  }>();

  let uncontrolledValue = defaultValue;

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
  class="time-field"
  type="time"
  value={currentValue}
  {min}
  {max}
  {step}
  disabled={isDisabled}
  aria-label={ariaLabel ?? undefined}
  aria-describedby={describedBy ?? undefined}
  on:input={handleInput}
/>

<style>
  .time-field {
    min-height: var(--flint-size-control-height);
    padding: 0 var(--flint-space-control-x);
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: var(--flint-radius-control);
    background: var(--flint-color-background-surface);
    color: var(--flint-color-text-primary);
    font-family: var(--flint-typography-body-family);
    font-size: var(--flint-typography-body-size);
    line-height: var(--flint-typography-body-lineHeight);
  }

  .time-field:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .time-field:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }
</style>
