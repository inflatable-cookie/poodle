<script lang="ts">
  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  const uiPresentation = getUiPresentation();

  let {
    size = null,
    sizeRole = "control",
    density = null,
    value = $bindable<number | null | undefined>(undefined),
    defaultValue = null,
    max = 5,
    allowClear = false,
    disabled = false,
    ariaLabel = null,
    onValueChange = undefined,
  }: {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    value?: number | null | undefined;
    defaultValue?: number | null;
    max?: number;
    allowClear?: boolean;
    disabled?: boolean;
    ariaLabel?: string | null;
    onValueChange?: ((value: number | null) => void) | undefined;
  } = $props();

  let itemElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledValue = $state<number | null>(null);
  let focusIndex = $state(0);
  let hoverIndex = $state(-1);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const currentValue = $derived(value === undefined ? uncontrolledValue : value);
  const itemCount = $derived(Math.max(1, Math.floor(max)));

  $effect(() => {
    if (value === undefined && uncontrolledValue === null && defaultValue !== null) {
      uncontrolledValue = defaultValue;
    }
  });

  $effect(() => {
    if (currentValue !== null) {
      focusIndex = Math.max(0, Math.min(itemCount - 1, currentValue - 1));
    }
  });

  // When hovering, show filled up to hover position; otherwise show filled up to value
  const displayValue = $derived(hoverIndex >= 0 ? hoverIndex + 1 : (currentValue ?? 0));

  function setValue(nextValue: number | null): void {
    if (value === undefined) {
      uncontrolledValue = nextValue;
    }

    value = nextValue;
    onValueChange?.(nextValue);
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
  class="poodle-rating"
  role="radiogroup"
  tabindex="-1"
  aria-label={ariaLabel ?? undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  onmouseleave={() => (hoverIndex = -1)}
>
  {#each Array.from({ length: itemCount }, (_, index) => index) as index}
    <button
      bind:this={itemElements[index]}
      type="button"
      class="poodle-rating__item"
      data-filled={displayValue >= index + 1}
      data-hovered={hoverIndex === index}
      disabled={disabled}
      role="radio"
      aria-checked={currentValue === index + 1 ? "true" : "false"}
      aria-label={`${index + 1} of ${itemCount}`}
      tabindex={focusIndex === index ? 0 : -1}
      onmouseenter={() => { if (!disabled) hoverIndex = index; }}
      onfocus={() => moveFocus(index)}
      onclick={() => selectIndex(index)}
      onkeydown={(event) => {
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
      <span class="poodle-rating__glyph" aria-hidden="true"><Icon name="star" size={resolvedSize} /></span>
    </button>
  {/each}
</div>

<style>
  .poodle-rating {
    display: inline-flex;
    align-items: center;
    gap: 0.125rem;
  }

  .poodle-rating__item {
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

  .poodle-rating__item[data-filled="true"] {
    color: var(--poodle-color-accent-base);
  }

  .poodle-rating__item[data-hovered="true"] {
    filter: drop-shadow(0 0 0.375rem color-mix(in srgb, var(--poodle-color-accent-base) 52%, transparent));
  }

  .poodle-rating__item:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-rating__glyph {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .poodle-rating__glyph :global(svg) {
    width: 1.125em;
    height: 1.125em;
    stroke-width: 2;
  }

  .poodle-rating__item[data-filled="true"] .poodle-rating__glyph :global(svg) {
    fill: currentColor;
    stroke-width: 1.5;
  }

  .poodle-rating__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .poodle-rating[data-size="xs"] .poodle-rating__item { width: calc(var(--poodle-size-icon-xs) + 0.75rem); height: calc(var(--poodle-size-icon-xs) + 0.75rem); }
  .poodle-rating[data-size="xs"] .poodle-rating__glyph { font-size: 0.75rem; }
  .poodle-rating[data-size="sm"] .poodle-rating__item { width: calc(var(--poodle-size-icon-sm) + 0.75rem); height: calc(var(--poodle-size-icon-sm) + 0.75rem); }
  .poodle-rating[data-size="sm"] .poodle-rating__glyph { font-size: 0.875rem; }
  .poodle-rating[data-size="lg"] .poodle-rating__item { width: calc(var(--poodle-size-icon-lg) + 0.75rem); height: calc(var(--poodle-size-icon-lg) + 0.75rem); }
  .poodle-rating[data-size="lg"] .poodle-rating__glyph { font-size: 1.125rem; }
  .poodle-rating[data-size="xl"] .poodle-rating__item { width: calc(var(--poodle-size-icon-xl) + 0.75rem); height: calc(var(--poodle-size-icon-xl) + 0.75rem); }
  .poodle-rating[data-size="xl"] .poodle-rating__glyph { font-size: 1.25rem; }

  /* Density variants */
  .poodle-rating[data-density="compact"] { gap: 0.0625rem; }
  .poodle-rating[data-density="comfortable"] { gap: 0.25rem; }
</style>
