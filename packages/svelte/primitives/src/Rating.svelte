<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlSize, SemanticControlSizeRole } from "./types";

  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
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

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: currentValue = value ?? uncontrolledValue;
  $: itemCount = Math.max(1, Math.floor(max));
  $: if (currentValue !== null) {
    focusIndex = Math.max(0, Math.min(itemCount - 1, currentValue - 1));
  }

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

<div class="rating" role="radiogroup" aria-label={ariaLabel ?? undefined} data-size={resolvedSize}>
  {#each Array.from({ length: itemCount }, (_, index) => index) as index}
    <button
      bind:this={itemElements[index]}
      type="button"
      class="rating__item"
      data-filled={currentValue !== null && currentValue >= index + 1}
      disabled={disabled}
      role="radio"
      aria-checked={currentValue === index + 1 ? "true" : "false"}
      aria-label={`${index + 1} of ${itemCount}`}
      tabindex={focusIndex === index ? 0 : -1}
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
      <span class="rating__glyph" aria-hidden="true"><Icon name="star" /></span>
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
    width: calc(var(--poodle-size-icon-default) + 0.75rem);
    height: calc(var(--poodle-size-icon-default) + 0.75rem);
    border: 0;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: color-mix(in srgb, var(--poodle-color-text-secondary) 78%, transparent);
    cursor: pointer;
    font: inherit;
  }

  .rating__item[data-filled="true"] {
    color: color-mix(in srgb, var(--poodle-color-accent-base) 84%, var(--poodle-color-text-primary));
  }

  .rating__item:hover:not(:disabled),
  .rating__item:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 14%, transparent);
    outline: none;
  }

  .rating__glyph {
    font-size: 1rem;
    line-height: 1;
  }

  .rating__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }
</style>
