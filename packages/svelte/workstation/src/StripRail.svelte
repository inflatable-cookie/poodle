<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { Icon } from "@pug/svelte-primitives";

  import type { StripEdge, StripMode, StripItem } from "./types";

  export let edge: StripEdge = "left";
  export let mode: StripMode = "icon";
  export let items: StripItem[] = [];
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let isCollapsed = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    collapsedChange: { isCollapsed: boolean };
  }>();

  let uncontrolledValue = defaultValue;

  $: currentValue = value ?? uncontrolledValue;
  $: orientation = edge === "top" || edge === "bottom" ? "horizontal" : "vertical";

  $: groupedItems = (() => {
    const groups: { group: string | null; items: StripItem[] }[] = [];
    let currentGroup: string | null = null;
    let currentItems: StripItem[] = [];

    for (const item of items) {
      const g = item.group ?? null;
      if (g !== currentGroup && currentItems.length > 0) {
        groups.push({ group: currentGroup, items: currentItems });
        currentItems = [];
      }
      currentGroup = g;
      currentItems.push(item);
    }
    if (currentItems.length > 0) {
      groups.push({ group: currentGroup, items: currentItems });
    }
    return groups;
  })();

  function selectItem(itemId: string): void {
    uncontrolledValue = itemId;
    dispatch("valueChange", { value: itemId });
  }

  function handleKeydown(event: KeyboardEvent): void {
    const prevKey = orientation === "vertical" ? "ArrowUp" : "ArrowLeft";
    const nextKey = orientation === "vertical" ? "ArrowDown" : "ArrowRight";

    if (event.key === prevKey || event.key === nextKey) {
      event.preventDefault();
      const enabledItems = items.filter((i) => !i.isDisabled);
      const idx = enabledItems.findIndex((i) => i.id === currentValue);
      const next = event.key === nextKey
        ? (idx + 1) % enabledItems.length
        : (idx - 1 + enabledItems.length) % enabledItems.length;
      selectItem(enabledItems[next].id);
      const btn = document.querySelector(`[data-strip-item="${enabledItems[next].id}"]`) as HTMLElement | null;
      btn?.focus();
      return;
    }

    if (event.key === "Home") {
      event.preventDefault();
      const first = items.find((i) => !i.isDisabled);
      if (first) {
        selectItem(first.id);
        const btn = document.querySelector(`[data-strip-item="${first.id}"]`) as HTMLElement | null;
        btn?.focus();
      }
      return;
    }

    if (event.key === "End") {
      event.preventDefault();
      const last = [...items].reverse().find((i) => !i.isDisabled);
      if (last) {
        selectItem(last.id);
        const btn = document.querySelector(`[data-strip-item="${last.id}"]`) as HTMLElement | null;
        btn?.focus();
      }
    }
  }
</script>

<nav
  class="strip-rail"
  data-edge={edge}
  data-orientation={orientation}
  data-mode={mode}
  data-collapsed={isCollapsed || undefined}
  aria-label={ariaLabel ?? `${edge} strip`}
  aria-orientation={orientation === "vertical" ? "vertical" : "horizontal"}
  role="tablist"
>
  {#if !isCollapsed}
    {#each groupedItems as group, gi}
      {#if gi > 0}
        <span class="strip-rail__separator" aria-hidden="true"></span>
      {/if}
      {#each group.items as item (item.id)}
        <button
          type="button"
          class="strip-rail__item"
          data-strip-item={item.id}
          data-active={currentValue === item.id || undefined}
          disabled={item.isDisabled}
          role="tab"
          aria-selected={currentValue === item.id}
          aria-label={item.label}
          title={item.label}
          tabindex={currentValue === item.id ? 0 : -1}
          on:click={() => selectItem(item.id)}
          on:keydown={handleKeydown}
        >
          <Icon name={item.icon} size="sm" />
          {#if mode === "mixed"}
            <span class="strip-rail__label">{item.label}</span>
          {/if}
          {#if item.badge}
            <span class="strip-rail__badge">{item.badge}</span>
          {/if}
        </button>
      {/each}
    {/each}
  {/if}
  <div class="strip-rail__footer">
    <slot name="footer" />
  </div>
</nav>

<style>
  .strip-rail {
    display: flex;
    gap: var(--pug-space-inline-xs, 0.125rem);
    background: var(--pug-color-surface-sunken);
    border-color: var(--pug-color-border-default);
    overflow: hidden;
  }

  .strip-rail[data-orientation="vertical"] {
    flex-direction: column;
    width: 2.75rem;
    min-height: 0;
    border-right-width: 1px;
    border-right-style: solid;
    padding: var(--pug-space-inline-sm, 0.375rem) 0;
    align-items: center;
  }

  .strip-rail[data-orientation="horizontal"] {
    flex-direction: row;
    height: 2.25rem;
    min-width: 0;
    border-bottom-width: 1px;
    border-bottom-style: solid;
    padding: 0 var(--pug-space-inline-sm, 0.375rem);
    align-items: center;
  }

  .strip-rail[data-edge="right"] {
    border-right-width: 0;
    border-left-width: 1px;
    border-left-style: solid;
  }

  .strip-rail[data-edge="bottom"] {
    border-bottom-width: 0;
    border-top-width: 1px;
    border-top-style: solid;
  }

  .strip-rail[data-collapsed] {
    width: 0;
    height: 0;
    padding: 0;
    border-width: 0;
    overflow: hidden;
  }

  .strip-rail__item {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--pug-space-inline-xs, 0.25rem);
    padding: var(--pug-space-inline-xs, 0.25rem);
    border: 0;
    border-radius: var(--pug-radius-sm, 0.25rem);
    background: transparent;
    color: var(--pug-color-text-muted);
    cursor: pointer;
    position: relative;
    font-size: 0;
    line-height: 1;
  }

  .strip-rail__item:hover {
    background: var(--pug-color-surface-hover);
    color: var(--pug-color-text-default);
  }

  .strip-rail__item[data-active] {
    background: var(--pug-color-surface-active);
    color: var(--pug-color-accent-base);
  }

  .strip-rail__item:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .strip-rail__item:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .strip-rail__label {
    font-size: var(--pug-font-size-xs, 0.6875rem);
    white-space: nowrap;
  }

  .strip-rail__badge {
    position: absolute;
    top: 0;
    right: 0;
    min-width: 0.75rem;
    height: 0.75rem;
    padding: 0 0.1875rem;
    font-size: 0.5rem;
    line-height: 0.75rem;
    text-align: center;
    border-radius: 999rem;
    background: var(--pug-color-status-error-base, #ef4444);
    color: #fff;
  }

  .strip-rail__separator {
    display: block;
    background: var(--pug-color-border-default);
    flex-shrink: 0;
  }

  .strip-rail[data-orientation="vertical"] .strip-rail__separator {
    width: 1.5rem;
    height: 1px;
    margin: var(--pug-space-inline-xs, 0.125rem) 0;
  }

  .strip-rail[data-orientation="horizontal"] .strip-rail__separator {
    width: 1px;
    height: 1rem;
    margin: 0 var(--pug-space-inline-xs, 0.125rem);
  }

  .strip-rail__footer {
    margin-top: auto;
  }

  .strip-rail[data-orientation="horizontal"] .strip-rail__footer {
    margin-top: 0;
    margin-left: auto;
  }

  .strip-rail[data-mode="mixed"] {
    width: auto;
    min-width: 8rem;
  }

  .strip-rail[data-mode="mixed"] .strip-rail__item {
    font-size: var(--pug-font-size-xs, 0.6875rem);
    justify-content: flex-start;
    padding: var(--pug-space-inline-xs, 0.25rem) var(--pug-space-inline-sm, 0.5rem);
  }
</style>
