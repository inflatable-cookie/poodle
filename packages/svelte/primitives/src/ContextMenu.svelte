<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { menuNavigableItems } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, MenuItem, SemanticControlSizeRole } from "./types";

  export let items: MenuItem[] = [];
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let anchorPoint: { x: number; y: number } | null = null;
  export let ariaLabel: string | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let size: ControlSize | null = null;
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
    action: { value: string };
  }>();

  const uiPresentation = getUiPresentation();

  let rootElement: HTMLDivElement | null = null;
  let overlayElement: HTMLDivElement | null = null;
  let itemElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledOpen = defaultOpen;
  let uncontrolledAnchorPoint = anchorPoint;
  let highlightIndex = 0;

  let adjustedPosition: { left: string; top: string } | null = null;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
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
  data-size={resolvedSize}
  data-density={resolvedDensity}
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
            disabled={item.disabled === true}
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
            <span>{item.label}</span>

            {#if item.checked}
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

  .context-menu__item {
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

  .context-menu__item:hover:not(:disabled),
  .context-menu__item:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    outline: none;
  }

  .context-menu__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .context-menu__meta {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
  }

  .context-menu__separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  /* Size variants */
  .context-menu[data-size="xs"] .context-menu__item {
    min-height: calc(var(--poodle-size-control-height) - 0.75rem);
    padding: calc(var(--poodle-space-control-y) - 0.0625rem) calc(var(--poodle-space-control-x) - 0.125rem);
    font-size: 0.75rem;
  }

  .context-menu[data-size="sm"] .context-menu__item {
    min-height: calc(var(--poodle-size-control-height) - 0.625rem);
  }

  .context-menu[data-size="lg"] .context-menu__item {
    min-height: calc(var(--poodle-size-control-height) + 0.125rem);
    font-size: 0.9375rem;
  }

  .context-menu[data-size="xl"] .context-menu__item {
    min-height: calc(var(--poodle-size-control-height) + 0.25rem);
    font-size: 1rem;
  }

  /* Density variants */
  .context-menu[data-density="compact"] .context-menu__item { padding-inline: 0.375rem; }
  .context-menu[data-density="comfortable"] .context-menu__item { padding-inline: 0.75rem; }
</style>
