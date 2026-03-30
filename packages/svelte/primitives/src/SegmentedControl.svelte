<script context="module" lang="ts">
  let nextSegmentedControlId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SegmentedControlOption,
    SemanticControlSizeRole,
  } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let options: SegmentedControlOption[] = [];
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let name: string | undefined = undefined;
  export let equalWidth = true;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  const generatedName = `poodle-segmented-control-${++nextSegmentedControlId}`;
  let uncontrolledValue = defaultValue;
  const uiPresentation = getUiPresentation();

  $: isControlled = value !== null;
  $: currentValue = isControlled ? value : uncontrolledValue;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  function handleChange(nextValue: string): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }
</script>

<div
  class="segmented-control"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-equal-width={equalWidth}
  role="radiogroup"
  aria-label={ariaLabel ?? undefined}
>
  {#each options as option (option.value)}
    <label class="segmented-control__segment" data-selected={currentValue === option.value} title={option.title ?? undefined}>
      <input
        class="segmented-control__control"
        type="radio"
        name={name ?? generatedName}
        value={option.value}
        checked={currentValue === option.value}
        disabled={disabled || option.disabled === true}
        aria-label={option.ariaLabel ?? undefined}
        on:change={() => handleChange(option.value)}
      />
      <span class="segmented-control__label">{option.label}</span>
    </label>
  {/each}
</div>

<style>
  .segmented-control {
    --poodle-segmented-control-height: var(--poodle-size-control-height);
    --poodle-segmented-control-x: var(--poodle-space-control-x);
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: minmax(0, 1fr);
    gap: 0.125rem;
    padding: 0.125rem;
    border: 0.0625rem solid var(
      --poodle-treatment-interactive-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 84%, transparent)
    );
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
    background: var(
      --poodle-treatment-interactive-fill,
      color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary))
    );
    box-shadow: var(--poodle-treatment-interactive-shadow, none);
  }

  .segmented-control[data-equal-width="false"] {
    grid-auto-columns: max-content;
    justify-content: start;
  }

  .segmented-control[data-size="xs"] {
    --poodle-segmented-control-height: 1.5rem;
  }

  .segmented-control[data-size="sm"] {
    --poodle-segmented-control-height: 1.75rem;
  }

  .segmented-control[data-size="md"] {
    --poodle-segmented-control-height: 2.25rem;
  }

  .segmented-control[data-size="lg"] {
    --poodle-segmented-control-height: 2.75rem;
  }

  .segmented-control[data-size="xl"] {
    --poodle-segmented-control-height: 3.25rem;
  }

  .segmented-control[data-density="compact"] {
    --poodle-segmented-control-x: 0.5rem;
  }

  .segmented-control[data-density="default"] {
    --poodle-segmented-control-x: 0.75rem;
  }

  .segmented-control[data-density="comfortable"] {
    --poodle-segmented-control-x: 1rem;
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
    min-height: calc(var(--poodle-segmented-control-height) - 0.25rem);
    padding: 0 var(--poodle-segmented-control-x);
    border-radius: calc(var(--poodle-treatment-interactive-radius, var(--poodle-radius-control)) - 0.125rem);
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: calc(var(--poodle-segmented-control-height) - 0.25rem);
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .segmented-control__segment[data-selected="true"] .segmented-control__label {
    background: var(--poodle-color-accent-base);
    color: var(--poodle-color-text-inverse);
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent);
  }

  .segmented-control__control:focus-visible + .segmented-control__label {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .segmented-control__control:disabled + .segmented-control__label {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }
</style>
