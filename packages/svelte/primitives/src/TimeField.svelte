<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlSize, SemanticControlSizeRole } from "./types";

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

  const dispatch = createEventDispatcher<{
    valueChange: { value: string | null };
  }>();
  const uiPresentation = getUiPresentation();

  let uncontrolledValue = defaultValue;

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
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
  data-size={resolvedSize}
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
  .time-field {
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

  .time-field:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .time-field:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }
</style>
