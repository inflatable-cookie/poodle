<script context="module" lang="ts">
  let nextTabsId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { findNextEnabledIndex, firstEnabledIndex } from "./internal";

  import type { Orientation, TabActivationMode, TabDefinition } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let tabs: TabDefinition[] = [];
  export let orientation: Orientation = "horizontal";
  export let activationMode: TabActivationMode = "automatic";
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  const tabsId = ++nextTabsId;
  let tabElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledValue = defaultValue;
  let focusIndex = 0;

  $: selectedIndex = tabs.findIndex((tab) => tab.value === currentValue);
  $: if (selectedIndex >= 0) {
    focusIndex = selectedIndex;
  }
  $: isControlled = value !== null;
  $: currentValue =
    (isControlled ? value : uncontrolledValue) ??
    tabs[firstEnabledIndex(tabs)]?.value ??
    null;

  function setValue(nextValue: string): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function moveFocus(nextIndex: number): void {
    focusIndex = nextIndex;
    tabElements[nextIndex]?.focus();

    if (activationMode === "automatic") {
      const nextValue = tabs[nextIndex]?.value;

      if (nextValue) {
        setValue(nextValue);
      }
    }
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    const horizontal = orientation === "horizontal";

    if ((horizontal && event.key === "ArrowRight") || (!horizontal && event.key === "ArrowDown")) {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(tabs, index, 1));
    }

    if ((horizontal && event.key === "ArrowLeft") || (!horizontal && event.key === "ArrowUp")) {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(tabs, index, -1));
    }

    if (event.key === "Home") {
      event.preventDefault();
      moveFocus(firstEnabledIndex(tabs));
    }

    if (event.key === "End") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(tabs, 0, -1));
    }

    if (
      activationMode === "manual" &&
      (event.key === "Enter" || event.key === " ")
    ) {
      event.preventDefault();
      const nextValue = tabs[index]?.value;

      if (nextValue) {
        setValue(nextValue);
      }
    }
  }
</script>

<div class="tabs" data-orientation={orientation}>
  <div
    class="tabs__list"
    role="tablist"
    aria-label={ariaLabel ?? undefined}
    aria-orientation={orientation}
  >
    {#each tabs as tab, index (tab.value)}
      <button
        bind:this={tabElements[index]}
        type="button"
        class="tabs__tab"
        data-selected={currentValue === tab.value}
        disabled={tab.isDisabled === true}
        id={`pug-tab-${tabsId}-${tab.value}`}
        role="tab"
        tabindex={focusIndex === index ? 0 : -1}
        aria-selected={currentValue === tab.value ? "true" : "false"}
        aria-controls={`pug-tabpanel-${tabsId}-${tab.value}`}
        on:focus={() => (focusIndex = index)}
        on:click={() => setValue(tab.value)}
        on:keydown={(event) => handleKeydown(event, index)}
      >
        {tab.label}
      </button>
    {/each}
  </div>

  {#if currentValue}
    <div
      class="tabs__panel"
      id={`pug-tabpanel-${tabsId}-${currentValue}`}
      role="tabpanel"
      tabindex="0"
      aria-labelledby={`pug-tab-${tabsId}-${currentValue}`}
    >
      <slot activeValue={currentValue} />
    </div>
  {/if}
</div>

<style>
  .tabs {
    display: grid;
    gap: var(--pug-space-stack-md);
    min-width: 0;
  }

  .tabs[data-orientation="vertical"] {
    grid-template-columns: auto minmax(0, 1fr);
    align-items: start;
  }

  .tabs__list {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 0.25rem;
    padding-bottom: 0.25rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 82%, transparent);
  }

  .tabs[data-orientation="vertical"] .tabs__list {
    flex-direction: column;
    padding-bottom: 0;
    padding-right: 0.5rem;
    border-bottom: 0;
    border-right: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 82%, transparent);
  }

  .tabs__tab {
    min-height: calc(var(--pug-size-control-height) - 0.25rem);
    padding: 0 0.75rem;
    border: 0;
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
  }

  .tabs__tab[data-selected="true"] {
    background: color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent);
    color: var(--pug-color-text-primary);
  }

  .tabs__tab:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .tabs__tab:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .tabs__panel {
    min-width: 0;
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 74%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 96%, transparent);
  }
</style>
