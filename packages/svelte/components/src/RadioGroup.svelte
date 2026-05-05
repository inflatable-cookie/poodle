<script context="module" lang="ts">
  let nextRadioGroupId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, Orientation, RadioGroupOption, SemanticControlSizeRole } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let options: RadioGroupOption[] = [];
  export let orientation: Orientation = "vertical";
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;
  export let selectedColor: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  const generatedName = `poodle-radio-group-${++nextRadioGroupId}`;
  const uiPresentation = getUiPresentation();
  let uncontrolledValue = defaultValue;

  $: isControlled = value !== null;
  $: currentValue = isControlled ? value : uncontrolledValue;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: radioGroupStyles = selectedColor ? `--poodle-radio-selected-color: ${selectedColor}` : undefined;

  function handleChange(nextValue: string): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }
</script>

<div
  class="poodle-radio-group"
  data-orientation={orientation}
  data-disabled={disabled}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  role="radiogroup"
  aria-label={ariaLabel ?? undefined}
  aria-describedby={describedBy ?? undefined}
  style={radioGroupStyles}
>
  {#each options as option (option.value)}
    <label class="poodle-radio-group__option" data-disabled={disabled || option.disabled === true}>
      <input
        class="poodle-radio-group__control"
        type="radio"
        name={name ?? generatedName}
        value={option.value}
        checked={currentValue === option.value}
        disabled={disabled || option.disabled === true}
        on:change={() => handleChange(option.value)}
      />
      <span class="poodle-radio-group__indicator" aria-hidden="true">
        <span class="poodle-radio-group__dot"></span>
      </span>
      <span class="poodle-radio-group__label">{option.label}</span>
    </label>
  {/each}
</div>

<style>
  .poodle-radio-group {
    --poodle-radio-selected-color: var(--poodle-color-accent-base);
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-radio-group[data-orientation="horizontal"] {
    grid-auto-flow: column;
    grid-auto-columns: minmax(0, max-content);
    gap: var(--poodle-space-inline-md);
    align-items: center;
  }

  .poodle-radio-group__option {
    position: relative;
    display: inline-grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
  }

  .poodle-radio-group__option[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-radio-group__control {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    padding: 0;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  .poodle-radio-group__indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-md) + 0.125rem);
    height: calc(var(--poodle-size-icon-md) + 0.125rem);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-color-background-surface);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-radio-group__dot {
    width: calc(var(--poodle-size-icon-md) * 0.5);
    height: calc(var(--poodle-size-icon-md) * 0.5);
    border-radius: 999px;
    background: transparent;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-radio-group__control:checked + .poodle-radio-group__indicator {
    border-color: var(--poodle-radio-selected-color);
  }

  .poodle-radio-group__control:checked + .poodle-radio-group__indicator .poodle-radio-group__dot {
    background: var(--poodle-radio-selected-color);
  }

  .poodle-radio-group__control:focus-visible + .poodle-radio-group__indicator {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-radio-group__label {
    min-width: 0;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  /* Density variants */
  .poodle-radio-group[data-density="compact"] {
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-radio-group[data-density="comfortable"] {
    gap: var(--poodle-space-stack-lg);
  }

  /* Size variants */
  .poodle-radio-group[data-size="xs"] .poodle-radio-group__indicator {
    width: calc(var(--poodle-size-icon-xs) + 0.25rem);
    height: calc(var(--poodle-size-icon-xs) + 0.25rem);
  }

  .poodle-radio-group[data-size="xs"] .poodle-radio-group__dot {
    width: 0.4rem;
    height: 0.4rem;
  }

  .poodle-radio-group[data-size="sm"] .poodle-radio-group__indicator {
    width: calc(var(--poodle-size-icon-sm) + 0.25rem);
    height: calc(var(--poodle-size-icon-sm) + 0.25rem);
  }

  .poodle-radio-group[data-size="sm"] .poodle-radio-group__dot {
    width: 0.45rem;
    height: 0.45rem;
  }

  .poodle-radio-group[data-size="lg"] .poodle-radio-group__indicator {
    width: calc(var(--poodle-size-icon-lg) + 0.125rem);
    height: calc(var(--poodle-size-icon-lg) + 0.125rem);
  }

  .poodle-radio-group[data-size="lg"] .poodle-radio-group__dot {
    width: 0.55rem;
    height: 0.55rem;
  }

  .poodle-radio-group[data-size="xl"] .poodle-radio-group__indicator {
    width: calc(var(--poodle-size-icon-xl) + 0.125rem);
    height: calc(var(--poodle-size-icon-xl) + 0.125rem);
  }

  .poodle-radio-group[data-size="xl"] .poodle-radio-group__dot {
    width: 0.6rem;
    height: 0.6rem;
  }
</style>
