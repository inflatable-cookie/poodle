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
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  const generatedName = `pug-radio-group-${++nextRadioGroupId}`;
  let uncontrolledValue = defaultValue;

  $: isControlled = value !== null;
  $: currentValue = isControlled ? value : uncontrolledValue;

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
  data-disabled={isDisabled}
  role="radiogroup"
  aria-label={ariaLabel ?? undefined}
  aria-describedby={describedBy ?? undefined}
>
  {#each options as option (option.value)}
    <label class="radio-group__option" data-disabled={isDisabled || option.isDisabled === true}>
      <input
        class="radio-group__control"
        type="radio"
        name={name ?? generatedName}
        value={option.value}
        checked={currentValue === option.value}
        disabled={isDisabled || option.isDisabled === true}
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
    display: grid;
    gap: var(--pug-space-stack-sm);
  }

  .radio-group[data-orientation="horizontal"] {
    grid-auto-flow: column;
    grid-auto-columns: minmax(0, max-content);
    gap: var(--pug-space-inline-md);
    align-items: center;
  }

  .radio-group__option {
    display: inline-grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: center;
    gap: var(--pug-space-inline-sm);
    min-width: 0;
    color: var(--pug-color-text-primary);
    cursor: pointer;
  }

  .radio-group__option[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
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
    width: calc(var(--pug-size-icon-default) + 0.125rem);
    height: calc(var(--pug-size-icon-default) + 0.125rem);
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: 999px;
    background: var(--pug-color-background-surface);
    transition:
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .radio-group__dot {
    width: calc(var(--pug-size-icon-default) * 0.5);
    height: calc(var(--pug-size-icon-default) * 0.5);
    border-radius: 999px;
    background: transparent;
    transition: background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .radio-group__control:checked + .radio-group__indicator {
    border-color: var(--pug-color-accent-base);
  }

  .radio-group__control:checked + .radio-group__indicator .radio-group__dot {
    background: var(--pug-color-accent-base);
  }

  .radio-group__control:focus-visible + .radio-group__indicator {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .radio-group__label {
    min-width: 0;
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    font-weight: var(--pug-typography-label-weight);
    line-height: var(--pug-typography-label-lineHeight);
  }
</style>
