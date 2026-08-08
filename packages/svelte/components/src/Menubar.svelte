<script module lang="ts">
  let nextMenubarId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-styles/menubar.css";
  import {
    menuListCanActivate,
    menuListNavigate,
    registerDismissLayer,
    layerContains,
  } from "@inflatable-cookie/poodle-headless";
  import { tick } from "svelte";

  import { findNextEnabledIndex, firstEnabledIndex, menuNavigableItems } from "./internal";
  import { anchored } from "./anchored";
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
  // Each menu anchors to its own trigger group, not to the whole bar.
  let groupElements = $state<Array<HTMLDivElement | null>>([]);
  let overlayElement = $state<HTMLDivElement | null>(null);
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
    if (actionableItems.length === 0) {
      return;
    }

    const nextIndex = menuListNavigate(actionableItems, highlightIndex, direction === 1 ? "next" : "prev");
    highlightIndex = nextIndex;
    menuItemElements[nextIndex]?.focus();
  }

  function activateItem(item: MenuItem): void {
    if (!menuListCanActivate(item)) {
      return;
    }

    onAction?.(item.value);
    setValue(null);
    triggerElements[focusIndex]?.focus();
  }

  $effect(() => {
    if (!currentValue) {
      return;
    }

    return registerDismissLayer({
      // The open menu is portalled out of the bar, so both are "inside".
      contains: (target) => layerContains(target, rootElement, overlayElement),
      dismissOnOutsideInteract: true,
      onDismiss: () => setValue(null),
    });
  });
</script>

<div bind:this={rootElement} class="poodle-menubar" data-size={resolvedSize} data-density={resolvedDensity}>
  <div class="poodle-menubar__list" role="menubar" aria-label={ariaLabel ?? undefined}>
    {#each items as item, index (item.value)}
      <div class="poodle-menubar__group" bind:this={groupElements[index]}>
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
            bind:this={overlayElement}
            use:anchored={{ anchor: groupElements[index], placement: "bottom-start", offset: 4 }}
            id={`poodle-menubar-menu-${menubarId}-${item.value}`}
            class="poodle-menubar__overlay"
            data-size={resolvedSize}
            data-density={resolvedDensity}
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

