<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { PanelTabItem } from "./types";

  export let items: PanelTabItem[] = [];
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let isReorderable = true;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    reorder: { items: string[] };
    requestClose: { value: string };
    requestContextMenu: { value: string };
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

  function handleKeydown(event: KeyboardEvent, item: PanelTabItem): void {
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

    if (isReorderable && event.altKey && event.shiftKey && event.key === "ArrowLeft") {
      event.preventDefault();
      moveItem(item.value, -1);
      return;
    }

    if (isReorderable && event.altKey && event.shiftKey && event.key === "ArrowRight") {
      event.preventDefault();
      moveItem(item.value, 1);
      return;
    }

    if (item.isClosable && event.key === "Delete") {
      event.preventDefault();
      dispatch("requestClose", { value: item.value });
    }
  }
</script>

<div class="panel-tabs" role="tablist" aria-label={ariaLabel ?? "Panel tabs"}>
  {#each items as item, index (item.value)}
    <div
      bind:this={tabElements[index]}
      role="tab"
      class="panel-tabs__tab"
      class:panel-tabs__tab--active={currentValue === item.value}
      aria-selected={currentValue === item.value}
      tabindex={currentValue === item.value ? 0 : -1}
      on:click={() => setValue(item.value)}
      on:keydown={(event) => handleKeydown(event, item)}
    >
      <span class="panel-tabs__copy">
        {#if item.icon}
          <span aria-hidden="true">{item.icon}</span>
        {/if}
        <span>{item.label}</span>
      </span>
      <span class="panel-tabs__actions">
        <button
          type="button"
          class="panel-tabs__icon-button"
          aria-label={`Open panel actions for ${item.label}`}
          on:click|stopPropagation={() => dispatch("requestContextMenu", { value: item.value })}
        >
          ⋯
        </button>
        {#if item.isClosable}
          <button
            type="button"
            class="panel-tabs__icon-button"
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

<style>
  .panel-tabs {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    align-items: center;
  }

  .panel-tabs__tab {
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

  .panel-tabs__tab--active {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 24%, var(--pug-color-border-subtle));
    background: color-mix(in srgb, var(--pug-color-accent-base) 20%, var(--pug-color-background-elevated));
    box-shadow: inset 0 1px 0 color-mix(in srgb, var(--pug-color-text-inverse) 10%, transparent);
  }

  .panel-tabs__tab:focus-visible,
  .panel-tabs__icon-button:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 2px;
  }

  .panel-tabs__copy,
  .panel-tabs__actions {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .panel-tabs__icon-button {
    width: 22px;
    height: 22px;
    padding: 0;
    border: 0;
    border-radius: calc(var(--pug-radius-control) - 1px);
    background: transparent;
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    font: inherit;
  }

  .panel-tabs__icon-button:hover {
    background: color-mix(in srgb, var(--pug-color-background-surface) 84%, transparent);
    color: var(--pug-color-text-primary);
  }
</style>
