<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { menuNavigableItems } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlSize, MenuItem, OverlayPlacement, SemanticControlSizeRole } from "./types";

  export let items: MenuItem[] = [];
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placement: OverlayPlacement = "bottom-start";
  export let ariaLabel: string | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let size: ControlSize | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
    action: { value: string };
  }>();

  const uiPresentation = getUiPresentation();

  let rootElement: HTMLDivElement | null = null;
  let itemElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledOpen = defaultOpen;
  let highlightIndex = 0;

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;
  $: actionableItems = menuNavigableItems(items);
  $: if (isOpen) {
    tick().then(() => itemElements[highlightIndex]?.focus());
  }

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    if (!nextOpen) {
      highlightIndex = 0;
    }

    dispatch("openChange", { open: nextOpen });
  }

  function moveHighlight(direction: 1 | -1): void {
    const count = actionableItems.length;

    if (count === 0) {
      return;
    }

    let nextIndex = highlightIndex;

    for (let step = 0; step < count; step += 1) {
      nextIndex = (nextIndex + direction + count) % count;

      if (!actionableItems[nextIndex]?.disabled) {
        highlightIndex = nextIndex;
        itemElements[nextIndex]?.focus();
        return;
      }
    }
  }

  function activateItem(item: MenuItem): void {
    if (item.disabled || item.kind === "separator") {
      return;
    }

    dispatch("action", { value: item.value });
    setOpen(false);
  }

  function handleTriggerClick(event: MouseEvent): void {
    event.preventDefault();
    event.stopPropagation();
    setOpen(!isOpen);
  }

  function handleTriggerKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter" || event.key === " " || event.key === "ArrowDown") {
      event.preventDefault();
      event.stopPropagation();
      setOpen(true);
    }
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!isOpen || !rootElement) {
        return;
      }

      if (!rootElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && isOpen) {
        event.preventDefault();
        setOpen(false);
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

<div class="menu" bind:this={rootElement} data-size={resolvedSize}>
  <div
    class="menu__trigger"
    role="button"
    tabindex="0"
    aria-expanded={isOpen ? "true" : "false"}
    on:click={handleTriggerClick}
    on:keydown={handleTriggerKeydown}
  >
    <slot name="trigger" />
  </div>

  {#if isOpen}
    <div class="menu__overlay" data-placement={placement} role="menu" aria-label={ariaLabel ?? undefined}>
      {#each items as item, index (item.value)}
        {#if item.kind === "separator"}
          <div class="menu__separator" role="separator"></div>
        {:else}
          <button
            bind:this={itemElements[actionableItems.findIndex((candidate) => candidate.value === item.value)]}
            type="button"
            class="menu__item"
            disabled={item.disabled === true}
            data-kind={item.kind ?? "action"}
            role={item.kind === "checkbox" || item.kind === "radio" ? `menuitem${item.kind}` : "menuitem"}
            aria-checked={item.kind === "checkbox" || item.kind === "radio" ? (item.checked ? "true" : "false") : undefined}
            on:click={() => activateItem(item)}
            on:keydown={(event) => {
              if (event.key === "ArrowDown") {
                event.preventDefault();
                moveHighlight(1);
              }

              if (event.key === "ArrowUp") {
                event.preventDefault();
                moveHighlight(-1);
              }

              if (event.key === "Home") {
                event.preventDefault();
                highlightIndex = 0;
                itemElements[0]?.focus();
              }

              if (event.key === "End") {
                event.preventDefault();
                highlightIndex = actionableItems.length - 1;
                itemElements[actionableItems.length - 1]?.focus();
              }

              if (event.key === "Enter" || event.key === " ") {
                event.preventDefault();
                activateItem(item);
              }
            }}
          >
            <span class="menu__label">{item.label}</span>

            {#if item.checked}
              <span class="menu__meta" aria-hidden="true">✓</span>
            {:else if item.shortcutLabel}
              <span class="menu__meta" aria-hidden="true">{item.shortcutLabel}</span>
            {/if}
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .menu {
    position: relative;
    display: inline-flex;
  }

  .menu__trigger {
    display: inline-flex;
  }

  .menu__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .menu__overlay {
    position: absolute;
    z-index: var(--poodle-overlay-z-menu);
    min-width: 14rem;
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

  .menu__overlay[data-placement^="bottom"] {
    top: calc(100% + 0.375rem);
    left: 0;
  }

  .menu__overlay[data-placement^="top"] {
    bottom: calc(100% + 0.375rem);
    left: 0;
  }

  .menu__overlay[data-placement$="end"] {
    left: auto;
    right: 0;
  }

  .menu__item {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    width: 100%;
    min-height: calc(var(--poodle-size-control-height) - 0.25rem);
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

  .menu__item:hover:not(:disabled),
  .menu__item:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    outline: none;
  }

  .menu__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .menu__meta {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
  }

  .menu__separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }
</style>
