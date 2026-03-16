<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { menuNavigableItems } from "./internal";

  import type { MenuItem } from "./types";

  export let items: MenuItem[] = [];
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let anchorPoint: { x: number; y: number } | null = null;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
    action: { value: string };
  }>();

  let rootElement: HTMLDivElement | null = null;
  let overlayElement: HTMLDivElement | null = null;
  let itemElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledOpen = defaultOpen;
  let uncontrolledAnchorPoint = anchorPoint;
  let highlightIndex = 0;

  let adjustedPosition: { left: string; top: string } | null = null;

  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;
  $: currentAnchorPoint = anchorPoint ?? uncontrolledAnchorPoint;
  $: actionableItems = menuNavigableItems(items);
  $: if (isOpen) {
    adjustedPosition = null;
    tick().then(() => {
      if (overlayElement && currentAnchorPoint) {
        const rect = overlayElement.getBoundingClientRect();
        const vw = window.innerWidth;
        const vh = window.innerHeight;
        const pad = 8;
        let x = currentAnchorPoint.x;
        let y = currentAnchorPoint.y;

        if (x + rect.width > vw - pad) {
          x = Math.max(pad, x - rect.width);
        }

        if (y + rect.height > vh - pad) {
          y = Math.max(pad, vh - rect.height - pad);
        }

        adjustedPosition = { left: `${x}px`, top: `${y}px` };
      }

      itemElements[highlightIndex]?.focus();
    });
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

      if (!overlayElement || !overlayElement.contains(event.target as Node)) {
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

<div
  class="context-menu"
  bind:this={rootElement}
  role="button"
  tabindex="0"
  aria-haspopup="menu"
  on:contextmenu={(event) => {
    event.preventDefault();
    uncontrolledAnchorPoint = { x: event.clientX, y: event.clientY };
    setOpen(true);
  }}
  on:keydown={(event) => {
    if (event.key === "ContextMenu" || (event.shiftKey && event.key === "F10")) {
      event.preventDefault();
      const target = event.currentTarget as HTMLElement;
      const rect = target.getBoundingClientRect();
      uncontrolledAnchorPoint = { x: rect.left + 16, y: rect.top + 16 };
      setOpen(true);
    }
  }}
>
  <slot />

  {#if isOpen && currentAnchorPoint}
    <div
      bind:this={overlayElement}
      class="context-menu__overlay"
      role="menu"
      aria-label={ariaLabel ?? undefined}
      style={adjustedPosition
        ? `left: ${adjustedPosition.left}; top: ${adjustedPosition.top};`
        : `left: ${currentAnchorPoint.x}px; top: ${currentAnchorPoint.y}px; visibility: hidden;`}
    >
      {#each items as item (item.value)}
        {#if item.kind === "separator"}
          <div class="context-menu__separator" role="separator"></div>
        {:else}
          <button
            bind:this={itemElements[actionableItems.findIndex((candidate) => candidate.value === item.value)]}
            type="button"
            class="context-menu__item"
            disabled={item.isDisabled === true}
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
            <span>{item.label}</span>

            {#if item.isChecked}
              <span class="context-menu__meta" aria-hidden="true">✓</span>
            {:else if item.shortcutLabel}
              <span class="context-menu__meta" aria-hidden="true">{item.shortcutLabel}</span>
            {/if}
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .context-menu {
    position: relative;
    min-width: 0;
  }

  .context-menu__overlay {
    position: fixed;
    z-index: var(--pug-overlay-z-menu);
    min-width: 14rem;
    padding: 0.25rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel));
    box-shadow: var(--pug-elevation-overlay);
  }

  .context-menu__item {
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
    font-size: 0.875rem;
    text-align: left;
  }

  .context-menu__item:hover:not(:disabled),
  .context-menu__item:focus-visible {
    background: color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent);
    outline: none;
  }

  .context-menu__item:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .context-menu__meta {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-code-family);
    font-size: 0.6875rem;
  }

  .context-menu__separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent);
  }
</style>
