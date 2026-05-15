<script module lang="ts">
  let nextMenubarId = 0;
</script>

<script lang="ts">
  import { onMount, tick } from "svelte";

  import { findNextEnabledIndex, firstEnabledIndex, menuNavigableItems } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, MenubarItem, MenuItem, SemanticControlSizeRole } from "./types";

  interface Props {
    value?: string | null;
    defaultValue?: string | null;
    items?: MenubarItem[];
    ariaLabel?: string | null;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onValueChange?: ((value: string | null) => void) | undefined;
    onAction?: ((value: string) => void) | undefined;
  }

  let {
    value = $bindable<string | null>(null),
    defaultValue = null,
    items = [],
    ariaLabel = null,
    sizeRole = "chrome",
    size = null,
    density = null,
    onValueChange = undefined,
    onAction = undefined,
  }: Props = $props();

  const menubarId = ++nextMenubarId;
  const uiPresentation = getUiPresentation();

  let rootElement = $state<HTMLDivElement | null>(null);
  let triggerElements = $state<Array<HTMLButtonElement | null>>([]);
  let menuItemElements = $state<Array<HTMLButtonElement | null>>([]);
  let uncontrolledValue = $state<string | null>(null);
  let seededDefaultValue = $state(false);
  let focusIndex = $state(0);
  let highlightIndex = $state(0);
  let lastOpenValue = $state<string | null>(null);

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue;
      seededDefaultValue = true;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const currentValue = $derived(value ?? uncontrolledValue);
  const currentMenu = $derived(items.find((item) => item.value === currentValue) ?? null);
  const actionableItems = $derived(menuNavigableItems(currentMenu?.items ?? []));
  const selectedIndex = $derived(items.findIndex((item) => item.value === currentValue));

  $effect(() => {
    if (selectedIndex >= 0) {
      focusIndex = selectedIndex;
    } else if (firstEnabledIndex(items) >= 0 && focusIndex === 0) {
      focusIndex = firstEnabledIndex(items);
    }
  });

  $effect(() => {
    if (currentValue !== lastOpenValue) {
      highlightIndex = 0;
      lastOpenValue = currentValue;
    }
  });

  $effect(() => {
    if (currentValue && actionableItems.length > 0) {
    tick().then(() => menuItemElements[highlightIndex]?.focus());
    }
  });

  function setValue(nextValue: string | null): void {
    if (value === null) {
      uncontrolledValue = nextValue;
    } else {
      value = nextValue;
    }

    onValueChange?.(nextValue);
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

      if (!actionableItems[nextIndex]?.disabled) {
        highlightIndex = nextIndex;
        menuItemElements[nextIndex]?.focus();
        return;
      }
    }
  }

  function activateItem(item: MenuItem): void {
    if (item.disabled || item.kind === "separator") {
      return;
    }

    onAction?.(item.value);
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

<div bind:this={rootElement} class="poodle-menubar" data-size={resolvedSize} data-density={resolvedDensity}>
  <div class="poodle-menubar__list" role="menubar" aria-label={ariaLabel ?? undefined}>
    {#each items as item, index (item.value)}
      <div class="poodle-menubar__group">
        <button
          bind:this={triggerElements[index]}
          type="button"
          class="poodle-menubar__trigger"
          data-open={currentValue === item.value}
          disabled={item.disabled === true}
          role="menuitem"
          aria-haspopup="menu"
          aria-expanded={currentValue === item.value ? "true" : "false"}
          aria-controls={currentValue === item.value ? `poodle-menubar-menu-${menubarId}-${item.value}` : undefined}
          onfocus={() => (focusIndex = index)}
          onclick={() => setValue(currentValue === item.value ? null : item.value)}
          onmouseenter={() => {
            if (currentValue !== null && currentValue !== item.value && !item.disabled) {
              openMenuAtIndex(index);
            }
          }}
          onkeydown={(event) => {
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
            id={`poodle-menubar-menu-${menubarId}-${item.value}`}
            class="poodle-menubar__overlay"
            role="menu"
            aria-label={item.label}
          >
            {#each item.items as menuItem (menuItem.value)}
              {#if menuItem.kind === "separator"}
                <div class="poodle-menubar__separator" role="separator"></div>
              {:else}
                <button
                  bind:this={menuItemElements[actionableItems.findIndex((candidate) => candidate.value === menuItem.value)]}
                  type="button"
                  class="poodle-menubar__item"
                  disabled={menuItem.disabled === true}
                  role={menuItem.kind === "checkbox" || menuItem.kind === "radio" ? `menuitem${menuItem.kind}` : "menuitem"}
                  aria-checked={menuItem.kind === "checkbox" || menuItem.kind === "radio" ? (menuItem.checked ? "true" : "false") : undefined}
                  onclick={() => activateItem(menuItem)}
                  onkeydown={(event) => {
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
                  <span class="poodle-menubar__label">{menuItem.label}</span>

                  {#if menuItem.checked}
                    <span class="poodle-menubar__meta" aria-hidden="true">✓</span>
                  {:else if menuItem.shortcutLabel}
                    <span class="poodle-menubar__meta" aria-hidden="true">{menuItem.shortcutLabel}</span>
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
  .poodle-menubar {
    display: inline-flex;
    min-width: 0;
  }

  .poodle-menubar__list {
    display: inline-flex;
    gap: 0.125rem;
    padding: 0.1875rem;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent);
  }

  .poodle-menubar__group {
    position: relative;
    display: inline-flex;
  }

  .poodle-menubar__trigger {
    display: inline-flex;
    align-items: center;
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: 1;
  }

  .poodle-menubar__trigger[data-open="true"],
  .poodle-menubar__trigger:hover:not(:disabled),
  .poodle-menubar__trigger:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 14%, transparent);
    outline: none;
  }

  .poodle-menubar__overlay {
    position: absolute;
    top: calc(100% + 0.25rem);
    left: 0;
    z-index: var(--poodle-overlay-z-menu);
    min-width: 12rem;
    padding: 0.25rem;
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)
    );
    border-radius: var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface));
    background: var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))
    );
    box-shadow: var(--poodle-treatment-surface-elevated-shadow, var(--poodle-elevation-overlay));
  }

  .poodle-menubar__item {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    width: 100%;
    min-height: var(--poodle-size-control-height);
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-body-size);
    text-align: left;
  }

  .poodle-menubar__item:hover:not(:disabled),
  .poodle-menubar__item:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    outline: none;
  }

  .poodle-menubar__meta {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
  }

  .poodle-menubar__separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  .poodle-menubar__trigger:disabled,
  .poodle-menubar__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .poodle-menubar[data-size="xs"] .poodle-menubar__trigger {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.6875rem;
  }

  .poodle-menubar[data-size="xs"] .poodle-menubar__item {
    min-height: var(--poodle-size-control-height);
    font-size: 0.75rem;
  }

  .poodle-menubar[data-size="sm"] .poodle-menubar__trigger {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
  }

  .poodle-menubar[data-size="sm"] .poodle-menubar__item {
    min-height: var(--poodle-size-control-height);
  }

  .poodle-menubar[data-size="lg"] .poodle-menubar__trigger {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.875rem;
  }

  .poodle-menubar[data-size="lg"] .poodle-menubar__item {
    min-height: var(--poodle-size-control-height);
    font-size: 0.9375rem;
  }

  .poodle-menubar[data-size="xl"] .poodle-menubar__trigger {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.9375rem;
  }

  .poodle-menubar[data-size="xl"] .poodle-menubar__item {
    min-height: var(--poodle-size-control-height);
    font-size: 1rem;
  }

  /* Density variants */
  .poodle-menubar[data-density="compact"] .poodle-menubar__item { padding-inline: 0.5rem; }
  .poodle-menubar[data-density="comfortable"] .poodle-menubar__item { padding-inline: 0.75rem; }
</style>
