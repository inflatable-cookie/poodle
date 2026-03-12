<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { SurfaceTabItem } from "./types";

  export let items: SurfaceTabItem[] = [];
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let isReorderable = true;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    reorder: { items: string[] };
    requestRename: { value: string };
    requestMove: { value: string };
    requestClose: { value: string };
    requestAdd: void;
  }>();

  let uncontrolledValue = defaultValue;
  let tabElements: Array<HTMLDivElement | null> = [];

  $: currentValue = value ?? uncontrolledValue ?? items[0]?.value ?? null;

  function focusTab(nextValue: string): void {
    const nextIndex = items.findIndex((item) => item.value === nextValue);
    tabElements[nextIndex]?.focus();
  }

  function setValue(nextValue: string): void {
    if (value === null) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function moveItem(targetValue: string, direction: -1 | 1): void {
    const currentIndex = items.findIndex((item) => item.value === targetValue);
    const nextIndex = currentIndex + direction;

    if (currentIndex === -1 || nextIndex < 0 || nextIndex >= items.length) {
      return;
    }

    const nextItems = [...items];
    const [moved] = nextItems.splice(currentIndex, 1);
    nextItems.splice(nextIndex, 0, moved);
    dispatch("reorder", { items: nextItems.map((item) => item.value) });
  }

  function handleKeydown(event: KeyboardEvent, item: SurfaceTabItem): void {
    if ((event.key === "ArrowRight" || event.key === "ArrowDown") && !event.altKey && !event.shiftKey) {
      event.preventDefault();
      const currentIndex = items.findIndex((entry) => entry.value === item.value);
      const nextItem = items[(currentIndex + 1) % items.length];
      if (nextItem) {
        setValue(nextItem.value);
        focusTab(nextItem.value);
      }
      return;
    }

    if ((event.key === "ArrowLeft" || event.key === "ArrowUp") && !event.altKey && !event.shiftKey) {
      event.preventDefault();
      const currentIndex = items.findIndex((entry) => entry.value === item.value);
      const nextItem = items[(currentIndex - 1 + items.length) % items.length];
      if (nextItem) {
        setValue(nextItem.value);
        focusTab(nextItem.value);
      }
      return;
    }

    if (event.key === "Home") {
      event.preventDefault();
      const nextItem = items[0];
      if (nextItem) {
        setValue(nextItem.value);
        focusTab(nextItem.value);
      }
      return;
    }

    if (event.key === "End") {
      event.preventDefault();
      const nextItem = items[items.length - 1];
      if (nextItem) {
        setValue(nextItem.value);
        focusTab(nextItem.value);
      }
      return;
    }

    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      setValue(item.value);
      return;
    }

    if (event.altKey && event.shiftKey && event.key === "ArrowLeft" && isReorderable) {
      event.preventDefault();
      moveItem(item.value, -1);
      return;
    }

    if (event.altKey && event.shiftKey && event.key === "ArrowRight" && isReorderable) {
      event.preventDefault();
      moveItem(item.value, 1);
      return;
    }
  }
</script>

<div class="surface-tabs" role="tablist" aria-label={ariaLabel ?? "Workspace surfaces"}>
  <div class="surface-tabs__items">
    {#each items as item, index (item.value)}
      <div
        bind:this={tabElements[index]}
        role="tab"
        class="surface-tabs__tab"
        class:surface-tabs__tab--active={currentValue === item.value}
        aria-selected={currentValue === item.value}
        tabindex={currentValue === item.value ? 0 : -1}
        on:click={() => setValue(item.value)}
        on:keydown={(event) => handleKeydown(event, item)}
      >
        <span>{item.label}</span>
        <span class="surface-tabs__actions">
          <button
            type="button"
            class="surface-tabs__icon-button"
            aria-label={`Rename ${item.label}`}
            on:click|stopPropagation={() => dispatch("requestRename", { value: item.value })}
          >
            ✎
          </button>
          <button
            type="button"
            class="surface-tabs__icon-button"
            aria-label={`Move ${item.label}`}
            on:click|stopPropagation={() => dispatch("requestMove", { value: item.value })}
          >
            ⇱
          </button>
          {#if item.isClosable}
            <button
              type="button"
              class="surface-tabs__icon-button"
              aria-label={`Close ${item.label}`}
              on:click|stopPropagation={() => dispatch("requestClose", { value: item.value })}
            >
              ×
            </button>
          {/if}
        </span>
      </div>
    {/each}
  </div>

  <button
    type="button"
    class="surface-tabs__add"
    aria-label="Add surface"
    on:click={() => dispatch("requestAdd")}
  >
    +
  </button>
</div>

<style>
  .surface-tabs {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--pug-space-inline-sm);
    align-items: center;
    padding: 8px var(--pug-space-panel-x);
    border-bottom: 1px solid var(--pug-color-border-subtle);
    background: color-mix(in srgb, var(--pug-color-background-panel) 92%, transparent);
  }

  .surface-tabs__items {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    align-items: center;
    min-width: 0;
  }

  .surface-tabs__tab {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: 10px;
    align-items: center;
    min-height: 32px;
    padding: 0 10px 0 12px;
    border: 1px solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 68%, transparent);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font: inherit;
    text-align: left;
  }

  .surface-tabs__tab--active {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 24%, var(--pug-color-border-subtle));
    background: color-mix(in srgb, var(--pug-color-accent-base) 20%, var(--pug-color-background-elevated));
    box-shadow: inset 0 1px 0 color-mix(in srgb, var(--pug-color-text-inverse) 10%, transparent);
  }

  .surface-tabs__actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .surface-tabs__icon-button,
  .surface-tabs__add {
    min-height: 28px;
    min-width: 28px;
    padding: 0;
    border: 0;
    border-radius: calc(var(--pug-radius-control) - 1px);
    background: transparent;
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    font: inherit;
  }

  .surface-tabs__add {
    border: 1px solid var(--pug-color-border-subtle);
    background: color-mix(in srgb, var(--pug-color-background-surface) 68%, transparent);
  }

  .surface-tabs__icon-button:hover,
  .surface-tabs__add:hover {
    background: color-mix(in srgb, var(--pug-color-background-surface) 84%, transparent);
    color: var(--pug-color-text-primary);
  }

  .surface-tabs__tab:focus-visible,
  .surface-tabs__icon-button:focus-visible,
  .surface-tabs__add:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 2px;
  }
</style>
