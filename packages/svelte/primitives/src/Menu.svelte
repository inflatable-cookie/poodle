<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { menuNavigableItems } from "./internal";

  import type { MenuItem, OverlayPlacement } from "./types";

  export let items: MenuItem[] = [];
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placement: OverlayPlacement = "bottom-start";
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
    action: { value: string };
  }>();

  let rootElement: HTMLDivElement | null = null;
  let itemElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledOpen = defaultOpen;
  let highlightIndex = 0;

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

      if (!actionableItems[nextIndex]?.isDisabled) {
        highlightIndex = nextIndex;
        itemElements[nextIndex]?.focus();
        return;
      }
    }
  }

  function activateItem(item: MenuItem): void {
    if (item.isDisabled || item.kind === "separator") {
      return;
    }

    dispatch("action", { value: item.value });
    setOpen(false);
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

<div class="menu" bind:this={rootElement}>
  <div
    class="menu__trigger"
    role="button"
    tabindex="0"
    aria-expanded={isOpen ? "true" : "false"}
    on:click={() => setOpen(!isOpen)}
    on:keydown={(event) => {
      if (event.key === "Enter" || event.key === " " || event.key === "ArrowDown") {
        event.preventDefault();
        setOpen(true);
      }
    }}
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
            disabled={item.isDisabled === true}
            data-kind={item.kind ?? "action"}
            role={item.kind === "checkbox" || item.kind === "radio" ? `menuitem${item.kind}` : "menuitem"}
            aria-checked={item.kind === "checkbox" || item.kind === "radio" ? (item.isChecked ? "true" : "false") : undefined}
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

            {#if item.isChecked}
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
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .menu__overlay {
    position: absolute;
    z-index: var(--pug-overlay-z-menu);
    min-width: 14rem;
    padding: 0.25rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel));
    box-shadow: var(--pug-elevation-overlay);
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
    min-height: 2rem;
    padding: 0.375rem 0.5rem;
    border: 0;
    border-radius: calc(var(--pug-radius-control) - 0.125rem);
    background: transparent;
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font: inherit;
    text-align: left;
  }

  .menu__item:hover:not(:disabled),
  .menu__item:focus-visible {
    background: color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent);
    outline: none;
  }

  .menu__item:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .menu__meta {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-code-family);
    font-size: 0.6875rem;
  }

  .menu__separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent);
  }
</style>
