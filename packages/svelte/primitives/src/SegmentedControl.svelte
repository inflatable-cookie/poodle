<script context="module" lang="ts">
  let nextSegmentedControlId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { SegmentedControlOption } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let options: SegmentedControlOption[] = [];
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let name: string | undefined = undefined;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  const generatedName = `pug-segmented-control-${++nextSegmentedControlId}`;
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

<div class="segmented-control" role="radiogroup" aria-label={ariaLabel ?? undefined}>
  {#each options as option (option.value)}
    <label class="segmented-control__segment" data-selected={currentValue === option.value}>
      <input
        class="segmented-control__control"
        type="radio"
        name={name ?? generatedName}
        value={option.value}
        checked={currentValue === option.value}
        disabled={isDisabled || option.isDisabled === true}
        aria-label={option.ariaLabel ?? undefined}
        on:change={() => handleChange(option.value)}
      />
      <span class="segmented-control__label">{option.label}</span>
    </label>
  {/each}
</div>

<style>
  .segmented-control {
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: minmax(0, 1fr);
    gap: 0.125rem;
    padding: 0.125rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 84%, transparent);
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 82%, transparent);
  }

  .segmented-control__segment {
    position: relative;
    display: grid;
    min-width: 0;
    cursor: pointer;
  }

  .segmented-control__control {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .segmented-control__label {
    display: block;
    min-width: 0;
    min-height: calc(var(--pug-size-control-height) - 0.25rem);
    padding: 0 0.75rem;
    border-radius: calc(var(--pug-radius-control) - 0.125rem);
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: calc(var(--pug-size-control-height) - 0.25rem);
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    transition:
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .segmented-control__segment[data-selected="true"] .segmented-control__label {
    background: var(--pug-color-accent-base);
    color: var(--pug-color-text-inverse);
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent);
  }

  .segmented-control__control:focus-visible + .segmented-control__label {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .segmented-control__control:disabled + .segmented-control__label {
    opacity: var(--pug-state-opacity-disabled);
    cursor: not-allowed;
  }
</style>
