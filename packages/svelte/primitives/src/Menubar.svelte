<script context="module" lang="ts">
  let nextMenubarId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { findNextEnabledIndex, firstEnabledIndex, menuNavigableItems } from "./internal";

  import type { MenubarItem, MenuItem } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let items: MenubarItem[] = [];
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string | null };
    action: { value: string };
  }>();

  const menubarId = ++nextMenubarId;
  let rootElement: HTMLDivElement | null = null;
  let triggerElements: Array<HTMLButtonElement | null> = [];
  let menuItemElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledValue = defaultValue;
  let focusIndex = 0;
  let highlightIndex = 0;
  let lastOpenValue: string | null = null;

  $: currentValue = value ?? uncontrolledValue;
  $: currentMenu = items.find((item) => item.value === currentValue) ?? null;
  $: actionableItems = menuNavigableItems(currentMenu?.items ?? []);
  $: selectedIndex = items.findIndex((item) => item.value === currentValue);
  $: if (selectedIndex >= 0) {
    focusIndex = selectedIndex;
  } else if (firstEnabledIndex(items) >= 0 && focusIndex === 0) {
    focusIndex = firstEnabledIndex(items);
  }
  $: if (currentValue !== lastOpenValue) {
    highlightIndex = 0;
    lastOpenValue = currentValue;
  }
  $: if (currentValue && actionableItems.length > 0) {
    tick().then(() => menuItemElements[highlightIndex]?.focus());
  }

  function setValue(nextValue: string | null): void {
    if (value === null) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function moveTriggerFocus(nextIndex: number): void {
    focusIndex = nextIndex;
    triggerElements[nextIndex]?.focus();
  }

  function openMenuAtIndex(index: number): void {
    const nextValue = items[index]?.value;

    if (!nextValue) {
      return;
    }

    focusIndex = index;
    setValue(nextValue);
  }

  function moveMenuHighlight(direction: 1 | -1): void {
    const count = actionableItems.length;

    if (count === 0) {
      return;
    }

    let nextIndex = highlightIndex;

    for (let step = 0; step < count; step += 1) {
      nextIndex = (nextIndex + direction + count) % count;

      if (!actionableItems[nextIndex]?.isDisabled) {
        highlightIndex = nextIndex;
        menuItemElements[nextIndex]?.focus();
        return;
      }
    }
  }

  function activateItem(item: MenuItem): void {
    if (item.isDisabled || item.kind === "separator") {
      return;
    }

    dispatch("action", { value: item.value });
    setValue(null);
    triggerElements[focusIndex]?.focus();
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

<div bind:this={rootElement} class="menubar">
  <div class="menubar__list" role="menubar" aria-label={ariaLabel ?? undefined}>
    {#each items as item, index (item.value)}
      <div class="menubar__group">
        <button
          bind:this={triggerElements[index]}
          type="button"
          class="menubar__trigger"
          data-open={currentValue === item.value}
          disabled={item.isDisabled === true}
          role="menuitem"
          aria-haspopup="menu"
          aria-expanded={currentValue === item.value ? "true" : "false"}
          aria-controls={currentValue === item.value ? `flint-menubar-menu-${menubarId}-${item.value}` : undefined}
          on:focus={() => (focusIndex = index)}
          on:click={() => setValue(currentValue === item.value ? null : item.value)}
          on:keydown={(event) => {
            if (event.key === "ArrowRight") {
              event.preventDefault();
              moveTriggerFocus(findNextEnabledIndex(items, index, 1));
            }

            if (event.key === "ArrowLeft") {
              event.preventDefault();
              moveTriggerFocus(findNextEnabledIndex(items, index, -1));
            }

            if (event.key === "Home") {
              event.preventDefault();
              moveTriggerFocus(firstEnabledIndex(items));
            }

            if (event.key === "End") {
              event.preventDefault();
              moveTriggerFocus(findNextEnabledIndex(items, 0, -1));
            }

            if (event.key === "ArrowDown" || event.key === "Enter" || event.key === " ") {
              event.preventDefault();
              openMenuAtIndex(index);
            }

            if (event.key === "Escape") {
              event.preventDefault();
              setValue(null);
            }
          }}
        >
          {item.label}
        </button>

        {#if currentValue === item.value}
          <div
            id={`flint-menubar-menu-${menubarId}-${item.value}`}
            class="menubar__overlay"
            role="menu"
            aria-label={item.label}
          >
            {#each item.items as menuItem (menuItem.value)}
              {#if menuItem.kind === "separator"}
                <div class="menubar__separator" role="separator"></div>
              {:else}
                <button
                  bind:this={menuItemElements[actionableItems.findIndex((candidate) => candidate.value === menuItem.value)]}
                  type="button"
                  class="menubar__item"
                  disabled={menuItem.isDisabled === true}
                  role={menuItem.kind === "checkbox" || menuItem.kind === "radio" ? `menuitem${menuItem.kind}` : "menuitem"}
                  aria-checked={menuItem.kind === "checkbox" || menuItem.kind === "radio" ? (menuItem.isChecked ? "true" : "false") : undefined}
                  on:click={() => activateItem(menuItem)}
                  on:keydown={(event) => {
                    if (event.key === "ArrowDown") {
                      event.preventDefault();
                      moveMenuHighlight(1);
                    }

                    if (event.key === "ArrowUp") {
                      event.preventDefault();
                      moveMenuHighlight(-1);
                    }

                    if (event.key === "Home") {
                      event.preventDefault();
                      highlightIndex = 0;
                      menuItemElements[0]?.focus();
                    }

                    if (event.key === "End") {
                      event.preventDefault();
                      highlightIndex = actionableItems.length - 1;
                      menuItemElements[actionableItems.length - 1]?.focus();
                    }

                    if (event.key === "ArrowRight") {
                      event.preventDefault();
                      const nextIndex = findNextEnabledIndex(items, index, 1);
                      openMenuAtIndex(nextIndex);
                    }

                    if (event.key === "ArrowLeft") {
                      event.preventDefault();
                      const nextIndex = findNextEnabledIndex(items, index, -1);
                      openMenuAtIndex(nextIndex);
                    }

                    if (event.key === "Escape") {
                      event.preventDefault();
                      setValue(null);
                      triggerElements[index]?.focus();
                    }

                    if (event.key === "Enter" || event.key === " ") {
                      event.preventDefault();
                      activateItem(menuItem);
                    }
                  }}
                >
                  <span class="menubar__label">{menuItem.label}</span>

                  {#if menuItem.isChecked}
                    <span class="menubar__meta" aria-hidden="true">✓</span>
                  {:else if menuItem.shortcutLabel}
                    <span class="menubar__meta" aria-hidden="true">{menuItem.shortcutLabel}</span>
                  {/if}
                </button>
              {/if}
            {/each}
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .menubar {
    display: inline-flex;
    min-width: 0;
  }

  .menubar__list {
    display: inline-flex;
    gap: 0.125rem;
    padding: 0.1875rem;
    border: 0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 72%, transparent);
    border-radius: var(--flint-radius-surface);
    background: color-mix(in srgb, var(--flint-color-background-panel) 96%, transparent);
  }

  .menubar__group {
    position: relative;
    display: inline-flex;
  }

  .menubar__trigger {
    display: inline-flex;
    align-items: center;
    min-height: calc(var(--flint-size-control-height) - 0.25rem);
    padding: 0 var(--flint-space-control-x);
    border: 0;
    border-radius: var(--flint-radius-control);
    background: transparent;
    color: var(--flint-color-text-primary);
    cursor: pointer;
    font-family: var(--flint-typography-label-family);
    font-size: var(--flint-typography-label-size);
    font-weight: var(--flint-typography-label-weight);
    line-height: 1;
  }

  .menubar__trigger[data-open="true"],
  .menubar__trigger:hover:not(:disabled),
  .menubar__trigger:focus-visible {
    background: color-mix(in srgb, var(--flint-color-accent-base) 14%, transparent);
    outline: none;
  }

  .menubar__overlay {
    position: absolute;
    top: calc(100% + 0.25rem);
    left: 0;
    z-index: var(--flint-overlay-z-menu);
    min-width: 12rem;
    padding: 0.25rem;
    border: 0.0625rem solid var(
      --flint-treatment-surface-elevated-border,
      color-mix(in srgb, var(--flint-color-border-default) 72%, transparent)
    );
    border-radius: var(--flint-treatment-surface-elevated-radius, var(--flint-radius-surface));
    background: var(
      --flint-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--flint-color-background-elevated) 98%, var(--flint-color-background-panel))
    );
    box-shadow: var(--flint-treatment-surface-elevated-shadow, var(--flint-elevation-overlay));
  }

  .menubar__item {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    width: 100%;
    min-height: calc(var(--flint-size-control-height) - 0.25rem);
    padding: var(--flint-space-control-y) var(--flint-space-control-x);
    border: 0;
    border-radius: calc(var(--flint-radius-control) - 0.125rem);
    background: transparent;
    color: var(--flint-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: var(--flint-typography-body-size);
    text-align: left;
  }

  .menubar__item:hover:not(:disabled),
  .menubar__item:focus-visible {
    background: color-mix(in srgb, var(--flint-color-accent-base) 16%, transparent);
    outline: none;
  }

  .menubar__meta {
    color: var(--flint-color-text-secondary);
    font-family: var(--flint-typography-code-family);
    font-size: 0.6875rem;
  }

  .menubar__separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--flint-color-border-subtle) 72%, transparent);
  }

  .menubar__trigger:disabled,
  .menubar__item:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }
</style>
