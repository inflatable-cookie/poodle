<script context="module" lang="ts">
  let nextRadioGroupId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { Orientation, RadioGroupOption } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let options: RadioGroupOption[] = [];
  export let orientation: Orientation = "vertical";
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;
  export let selectedColor: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  const generatedName = `poodle-radio-group-${++nextRadioGroupId}`;
  let uncontrolledValue = defaultValue;

  $: isControlled = value !== null;
  $: currentValue = isControlled ? value : uncontrolledValue;
  $: radioGroupStyles = selectedColor ? `--poodle-radio-selected-color: ${selectedColor}` : undefined;

  function handleChange(nextValue: string): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }
</script>

<div
  class="radio-group"
  data-orientation={orientation}
  data-disabled={disabled}
  role="radiogroup"
  aria-label={ariaLabel ?? undefined}
  aria-describedby={describedBy ?? undefined}
  style={radioGroupStyles}
>
  {#each options as option (option.value)}
    <label class="radio-group__option" data-disabled={disabled || option.disabled === true}>
      <input
        class="radio-group__control"
        type="radio"
        name={name ?? generatedName}
        value={option.value}
        checked={currentValue === option.value}
        disabled={disabled || option.disabled === true}
        on:change={() => handleChange(option.value)}
      />
      <span class="radio-group__indicator" aria-hidden="true">
        <span class="radio-group__dot"></span>
      </span>
      <span class="radio-group__label">{option.label}</span>
    </label>
  {/each}
</div>

<style>
  .radio-group {
    --poodle-radio-selected-color: var(--poodle-color-accent-base);
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .radio-group[data-orientation="horizontal"] {
    grid-auto-flow: column;
    grid-auto-columns: minmax(0, max-content);
    gap: var(--poodle-space-inline-md);
    align-items: center;
  }

  .radio-group__option {
    display: inline-grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
  }

  .radio-group__option[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .radio-group__control {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .radio-group__indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-default) + 0.125rem);
    height: calc(var(--poodle-size-icon-default) + 0.125rem);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-color-background-surface);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .radio-group__dot {
    width: calc(var(--poodle-size-icon-default) * 0.5);
    height: calc(var(--poodle-size-icon-default) * 0.5);
    border-radius: 999px;
    background: transparent;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .radio-group__control:checked + .radio-group__indicator {
    border-color: var(--poodle-radio-selected-color);
  }

  .radio-group__control:checked + .radio-group__indicator .radio-group__dot {
    background: var(--poodle-radio-selected-color);
  }

  .radio-group__control:focus-visible + .radio-group__indicator {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .radio-group__label {
    min-width: 0;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }
</style>
