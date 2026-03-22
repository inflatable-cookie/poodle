<script context="module" lang="ts">
  let nextNavigationMenuId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import { findNextEnabledIndex, firstEnabledIndex } from "./internal";

  import type { NavigationMenuItem } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let items: NavigationMenuItem[] = [];
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string | null };
  }>();

  const menuId = ++nextNavigationMenuId;
  let rootElement: HTMLDivElement | null = null;
  let triggerElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledValue = defaultValue;
  let focusIndex = 0;

  $: currentValue = value ?? uncontrolledValue;
  $: currentItem = items.find((item) => item.value === currentValue) ?? null;
  $: selectedIndex = items.findIndex((item) => item.value === currentValue);
  $: if (selectedIndex >= 0) {
    focusIndex = selectedIndex;
  } else if (firstEnabledIndex(items) >= 0 && focusIndex === 0) {
    focusIndex = firstEnabledIndex(items);
  }

  function setValue(nextValue: string | null): void {
    if (value === null) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function toggleValue(nextValue: string): void {
    setValue(currentValue === nextValue ? null : nextValue);
  }

  function moveFocus(nextIndex: number): void {
    focusIndex = nextIndex;
    triggerElements[nextIndex]?.focus();
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    if (event.key === "ArrowRight") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(items, index, 1));
    }

    if (event.key === "ArrowLeft") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(items, index, -1));
    }

    if (event.key === "Home") {
      event.preventDefault();
      moveFocus(firstEnabledIndex(items));
    }

    if (event.key === "End") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(items, 0, -1));
    }

    if (event.key === "ArrowDown" || event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      const nextValue = items[index]?.value;

      if (nextValue) {
        setValue(nextValue);
      }
    }

    if (event.key === "Escape") {
      event.preventDefault();
      setValue(null);
    }
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!currentValue || !rootElement) {
        return;
      }

      if (!rootElement.contains(event.target as Node)) {
        setValue(null);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && currentValue) {
        event.preventDefault();
        setValue(null);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
    };
  });
</script>

<div class="navigation-menu" bind:this={rootElement}>
  <nav
    class="navigation-menu__list"
    aria-label={ariaLabel ?? undefined}
  >
    {#each items as item, index (item.value)}
      <button
        bind:this={triggerElements[index]}
        type="button"
        class="navigation-menu__trigger"
        data-open={currentValue === item.value}
        disabled={item.isDisabled === true}
        id={`pug-navigation-menu-trigger-${menuId}-${item.value}`}
        aria-expanded={currentValue === item.value ? "true" : "false"}
        aria-controls={currentValue === item.value ? `pug-navigation-menu-panel-${menuId}-${item.value}` : undefined}
        on:focus={() => (focusIndex = index)}
        on:click={() => toggleValue(item.value)}
        on:keydown={(event) => handleKeydown(event, index)}
      >
        <span class="navigation-menu__label">{item.label}</span>
      </button>
    {/each}
  </nav>

  {#if currentItem}
    <div
      class="navigation-menu__viewport"
      id={`pug-navigation-menu-panel-${menuId}-${currentItem.value}`}
      aria-labelledby={`pug-navigation-menu-trigger-${menuId}-${currentItem.value}`}
    >
      <slot activeValue={currentItem.value} activeItem={currentItem} />
    </div>
  {/if}
</div>

<style>
  .navigation-menu {
    display: grid;
    gap: var(--pug-space-stack-md);
    min-width: 0;
  }

  .navigation-menu__list {
    display: inline-flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
    align-items: center;
  }

  .navigation-menu__trigger {
    display: inline-flex;
    align-items: center;
    gap: var(--pug-space-inline-sm);
    min-height: calc(var(--pug-size-control-height) - 0.125rem);
    padding: 0 var(--pug-space-control-x);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent);
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 88%, transparent);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
  }

  .navigation-menu__trigger[data-open="true"] {
    background: color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent);
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 42%, var(--pug-color-border-default));
  }

  .navigation-menu__trigger:hover:not(:disabled),
  .navigation-menu__trigger:focus-visible {
    background: color-mix(in srgb, var(--pug-color-accent-base) 12%, transparent);
    outline: none;
  }

  .navigation-menu__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .navigation-menu__viewport {
    min-width: 0;
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 74%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 96%, transparent);
    box-shadow: var(--pug-elevation-overlay);
  }
</style>
