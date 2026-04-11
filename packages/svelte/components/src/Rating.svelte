<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let value: number | null = null;
  export let defaultValue: number | null = null;
  export let max = 5;
  export let allowClear = false;
  export let disabled = false;
  export let ariaLabel: string | null = null;

  const uiPresentation = getUiPresentation();

  const dispatch = createEventDispatcher<{
    valueChange: { value: number | null };
  }>();

  let itemElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledValue = defaultValue;
  let focusIndex = (defaultValue ?? 1) - 1;
  let hoverIndex = -1;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: currentValue = value ?? uncontrolledValue;
  $: itemCount = Math.max(1, Math.floor(max));
  $: if (currentValue !== null) {
    focusIndex = Math.max(0, Math.min(itemCount - 1, currentValue - 1));
  }

  // When hovering, show filled up to hover position; otherwise show filled up to value
  $: displayValue = hoverIndex >= 0 ? hoverIndex + 1 : (currentValue ?? 0);

  function setValue(nextValue: number | null): void {
    if (value === null) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function selectIndex(index: number): void {
    const nextValue = index + 1;

    if (allowClear && currentValue === nextValue) {
      setValue(null);
      return;
    }

    setValue(nextValue);
  }

  function moveFocus(nextIndex: number): void {
    focusIndex = Math.max(0, Math.min(itemCount - 1, nextIndex));
    itemElements[focusIndex]?.focus();
  }
</script>

<div
  class="rating"
  role="radiogroup"
  tabindex="-1"
  aria-label={ariaLabel ?? undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  on:mouseleave={() => (hoverIndex = -1)}
>
  {#each Array.from({ length: itemCount }, (_, index) => index) as index}
    <button
      bind:this={itemElements[index]}
      type="button"
      class="rating__item"
      data-filled={displayValue >= index + 1}
      data-hovered={hoverIndex === index}
      disabled={disabled}
      role="radio"
      aria-checked={currentValue === index + 1 ? "true" : "false"}
      aria-label={`${index + 1} of ${itemCount}`}
      tabindex={focusIndex === index ? 0 : -1}
      on:mouseenter={() => { if (!disabled) hoverIndex = index; }}
      on:focus={() => moveFocus(index)}
      on:click={() => selectIndex(index)}
      on:keydown={(event) => {
        if (event.key === "ArrowRight" || event.key === "ArrowUp") {
          event.preventDefault();
          moveFocus(index + 1);
        }

        if (event.key === "ArrowLeft" || event.key === "ArrowDown") {
          event.preventDefault();
          moveFocus(index - 1);
        }

        if (event.key === "Home") {
          event.preventDefault();
          moveFocus(0);
        }

        if (event.key === "End") {
          event.preventDefault();
          moveFocus(itemCount - 1);
        }

        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          selectIndex(index);
        }
      }}
    >
      <span class="rating__glyph" aria-hidden="true"><Icon name="star" size={resolvedSize} /></span>
    </button>
  {/each}
</div>

<style>
  .rating {
    display: inline-flex;
    align-items: center;
    gap: 0.125rem;
  }

  .rating__item {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-md) + 0.75rem);
    height: calc(var(--poodle-size-icon-md) + 0.75rem);
    padding: 0;
    border: 0;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: color-mix(in srgb, var(--poodle-color-text-secondary) 48%, transparent);
    cursor: pointer;
    font: inherit;
    transition: color 120ms ease, filter 120ms ease;
  }

  .rating__item[data-filled="true"] {
    color: var(--poodle-color-accent-base);
  }

  .rating__item[data-hovered="true"] {
    filter: drop-shadow(0 0 0.375rem color-mix(in srgb, var(--poodle-color-accent-base) 52%, transparent));
  }

  .rating__item:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .rating__glyph {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .rating__glyph :global(svg) {
    width: 1.125em;
    height: 1.125em;
    stroke-width: 2;
  }

  .rating__item[data-filled="true"] .rating__glyph :global(svg) {
    fill: currentColor;
    stroke-width: 1.5;
  }

  .rating__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .rating[data-size="xs"] .rating__item { width: calc(var(--poodle-size-icon-xs) + 0.75rem); height: calc(var(--poodle-size-icon-xs) + 0.75rem); }
  .rating[data-size="xs"] .rating__glyph { font-size: 0.75rem; }
  .rating[data-size="sm"] .rating__item { width: calc(var(--poodle-size-icon-sm) + 0.75rem); height: calc(var(--poodle-size-icon-sm) + 0.75rem); }
  .rating[data-size="sm"] .rating__glyph { font-size: 0.875rem; }
  .rating[data-size="lg"] .rating__item { width: calc(var(--poodle-size-icon-lg) + 0.75rem); height: calc(var(--poodle-size-icon-lg) + 0.75rem); }
  .rating[data-size="lg"] .rating__glyph { font-size: 1.125rem; }
  .rating[data-size="xl"] .rating__item { width: calc(var(--poodle-size-icon-xl) + 0.75rem); height: calc(var(--poodle-size-icon-xl) + 0.75rem); }
  .rating[data-size="xl"] .rating__glyph { font-size: 1.25rem; }

  /* Density variants */
  .rating[data-density="compact"] { gap: 0.0625rem; }
  .rating[data-density="comfortable"] { gap: 0.25rem; }
</style>
