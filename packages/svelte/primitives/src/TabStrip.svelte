<script context="module" lang="ts">
  let nextTabStripId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { findNextEnabledIndex, firstEnabledIndex } from "./internal";

  import type { Orientation, TabStripItem } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let items: TabStripItem[] = [];
  export let isReorderable = false;
  export let orientation: Orientation = "horizontal";
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    reorder: { items: string[] };
    close: { value: string };
  }>();

  const tabStripId = ++nextTabStripId;
  let tabElements: Array<HTMLButtonElement | null> = [];
  let renderedItems: TabStripItem[] = items;
  let uncontrolledValue = defaultValue;
  let focusIndex = 0;

  $: renderedItems = items;
  $: isControlled = value !== null;
  $: currentValue =
    (isControlled ? value : uncontrolledValue) ??
    renderedItems[firstEnabledIndex(renderedItems)]?.value ??
    null;
  $: selectedIndex = renderedItems.findIndex((item) => item.value === currentValue);
  $: if (selectedIndex >= 0) {
    focusIndex = selectedIndex;
  }

  function setValue(nextValue: string): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function requestReorder(index: number, direction: -1 | 1): void {
    if (!isReorderable) {
      return;
    }

    const nextIndex = index + direction;

    if (nextIndex < 0 || nextIndex >= renderedItems.length) {
      return;
    }

    const nextItems = [...renderedItems];
    const [moved] = nextItems.splice(index, 1);
    nextItems.splice(nextIndex, 0, moved);
    renderedItems = nextItems;
    focusIndex = nextIndex;
    dispatch("reorder", { items: nextItems.map((item) => item.value) });
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    const horizontal = orientation === "horizontal";

    if ((horizontal && event.key === "ArrowRight") || (!horizontal && event.key === "ArrowDown")) {
      if (isReorderable && event.altKey) {
        event.preventDefault();
        requestReorder(index, 1);
      } else {
        event.preventDefault();
        const nextIndex = findNextEnabledIndex(renderedItems, index, 1);
        focusIndex = nextIndex;
        tabElements[nextIndex]?.focus();
      }
    }

    if ((horizontal && event.key === "ArrowLeft") || (!horizontal && event.key === "ArrowUp")) {
      if (isReorderable && event.altKey) {
        event.preventDefault();
        requestReorder(index, -1);
      } else {
        event.preventDefault();
        const nextIndex = findNextEnabledIndex(renderedItems, index, -1);
        focusIndex = nextIndex;
        tabElements[nextIndex]?.focus();
      }
    }

    if (event.key === "Home") {
      event.preventDefault();
      const nextIndex = firstEnabledIndex(renderedItems);
      focusIndex = nextIndex;
      tabElements[nextIndex]?.focus();
    }

    if (event.key === "End") {
      event.preventDefault();
      const nextIndex = findNextEnabledIndex(renderedItems, 0, -1);
      focusIndex = nextIndex;
      tabElements[nextIndex]?.focus();
    }

    if (event.key === "Delete" && renderedItems[index]?.isClosable) {
      event.preventDefault();
      dispatch("close", { value: renderedItems[index].value });
    }
  }
</script>

<div class="tab-strip" data-orientation={orientation}>
  <div
    class="tab-strip__list"
    role="tablist"
    aria-label={ariaLabel ?? undefined}
    aria-orientation={orientation}
  >
    {#each renderedItems as item, index (item.value)}
      <div class="tab-strip__item" data-selected={currentValue === item.value}>
        <button
          bind:this={tabElements[index]}
          type="button"
          class="tab-strip__tab"
          disabled={item.isDisabled === true}
          id={`pug-tab-strip-${tabStripId}-${item.value}`}
          role="tab"
          tabindex={focusIndex === index ? 0 : -1}
          aria-selected={currentValue === item.value ? "true" : "false"}
          on:focus={() => (focusIndex = index)}
          on:click={() => setValue(item.value)}
          on:keydown={(event) => handleKeydown(event, index)}
        >
          <span class="tab-strip__label">{item.label}</span>
        </button>

        {#if item.isClosable}
          <button
            type="button"
            class="tab-strip__close"
            aria-label={`Close ${item.label}`}
            on:click={() => dispatch("close", { value: item.value })}
          >
            ×
          </button>
        {/if}
      </div>
    {/each}

    {#if $$slots.actions}
      <div class="tab-strip__actions">
        <slot name="actions" />
      </div>
    {/if}
  </div>
</div>

<style>
  .tab-strip {
    min-width: 0;
  }

  .tab-strip__list {
    display: flex;
    flex-wrap: nowrap;
    align-items: stretch;
    gap: 0.25rem;
    overflow: auto;
  }

  .tab-strip[data-orientation="vertical"] .tab-strip__list {
    flex-direction: column;
  }

  .tab-strip__item {
    display: inline-flex;
    align-items: center;
    gap: 0.125rem;
    min-width: 0;
    padding: 0.125rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 68%, transparent);
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 92%, transparent);
  }

  .tab-strip__item[data-selected="true"] {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 32%, var(--pug-color-border-subtle));
    background: color-mix(in srgb, var(--pug-color-accent-base) 14%, var(--pug-color-background-surface));
  }

  .tab-strip__tab,
  .tab-strip__close {
    border: 0;
    background: transparent;
    color: var(--pug-color-text-primary);
    font: inherit;
  }

  .tab-strip__tab {
    display: inline-flex;
    align-items: center;
    min-height: calc(var(--pug-size-control-height) - 0.375rem);
    padding: 0 0.625rem;
    cursor: pointer;
  }

  .tab-strip__close {
    width: 1.5rem;
    height: 1.5rem;
    border-radius: calc(var(--pug-radius-control) - 0.125rem);
    cursor: pointer;
  }

  .tab-strip__tab:focus-visible,
  .tab-strip__close:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .tab-strip__tab:disabled,
  .tab-strip__close:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .tab-strip__label {
    white-space: nowrap;
  }

  .tab-strip__actions {
    display: inline-flex;
    align-items: center;
    margin-left: auto;
  }
</style>
