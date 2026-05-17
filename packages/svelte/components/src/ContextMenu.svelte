<script lang="ts">
  import { onMount, tick, type Snippet } from "svelte";

  import { menuNavigableItems } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, MenuItem, SemanticControlSizeRole } from "./types";

  interface Props {
    items?: MenuItem[];
    open?: boolean | null;
    defaultOpen?: boolean;
    anchorPoint?: { x: number; y: number } | null;
    ariaLabel?: string | null;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
    onAction?: ((value: string) => void) | undefined;
    children?: Snippet<[]>;
  }

  let {
    items = [],
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    anchorPoint = null,
    ariaLabel = null,
    sizeRole = "chrome",
    size = null,
    density = null,
    onOpenChange = undefined,
    onAction = undefined,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let rootElement = $state<HTMLDivElement | null>(null);
  let overlayElement = $state<HTMLDivElement | null>(null);
  let itemElements = $state<Array<HTMLButtonElement | null>>([]);
  let uncontrolledOpen = $state(false);
  let uncontrolledAnchorPoint = $state<{ x: number; y: number } | null>(null);
  let seededDefaults = $state(false);
  let highlightIndex = $state(0);
  let adjustedPosition = $state<{ left: string; top: string } | null>(null);

  $effect.pre(() => {
    if (!seededDefaults) {
      uncontrolledOpen = defaultOpen;
      uncontrolledAnchorPoint = anchorPoint;
      seededDefaults = true;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(open !== null);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);
  const currentAnchorPoint = $derived(anchorPoint ?? uncontrolledAnchorPoint);
  const actionableItems = $derived(menuNavigableItems(items));

  $effect(() => {
    if (!isOpen) {
      return;
    }

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
  });

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    if (!nextOpen) {
      highlightIndex = 0;
    }

    onOpenChange?.(nextOpen);
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

    onAction?.(item.value);
    setOpen(false);
  }

  function handleContextMenu(event: MouseEvent): void {
    event.preventDefault();
    uncontrolledAnchorPoint = { x: event.clientX, y: event.clientY };
    setOpen(true);
  }

  function handleTriggerKeydown(event: KeyboardEvent): void {
    if (event.key !== "ContextMenu" && !(event.shiftKey && event.key === "F10")) {
      return;
    }

    event.preventDefault();
    const target = event.currentTarget;
    if (!(target instanceof HTMLElement)) {
      return;
    }

    const rect = target.getBoundingClientRect();
    uncontrolledAnchorPoint = { x: rect.left + 16, y: rect.top + 16 };
    setOpen(true);
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
  class="poodle-context-menu"
  bind:this={rootElement}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  role="button"
  tabindex="0"
  aria-haspopup="menu"
  oncontextmenu={handleContextMenu}
  onkeydown={handleTriggerKeydown}
>
  {@render children?.()}

  {#if isOpen && currentAnchorPoint}
    <div
      bind:this={overlayElement}
      class="poodle-context-menu__overlay"
      role="menu"
      aria-label={ariaLabel ?? undefined}
      style={adjustedPosition
        ? `left: ${adjustedPosition.left}; top: ${adjustedPosition.top};`
        : `left: ${currentAnchorPoint.x}px; top: ${currentAnchorPoint.y}px; visibility: hidden;`}
    >
      {#each items as item (item.value)}
        {#if item.kind === "separator"}
          <div class="poodle-context-menu__separator" role="separator"></div>
        {:else}
          <button
            bind:this={itemElements[actionableItems.findIndex((candidate) => candidate.value === item.value)]}
            type="button"
            class="poodle-context-menu__item"
            disabled={item.disabled === true}
            role={item.kind === "checkbox" || item.kind === "radio" ? `menuitem${item.kind}` : "menuitem"}
            aria-checked={item.kind === "checkbox" || item.kind === "radio" ? (item.checked ? "true" : "false") : undefined}
            onclick={() => activateItem(item)}
            onkeydown={(event) => {
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
              <span class="poodle-context-menu__meta" aria-hidden="true">✓</span>
            {:else if item.shortcutLabel}
              <span class="poodle-context-menu__meta" aria-hidden="true">{item.shortcutLabel}</span>
            {/if}
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .poodle-context-menu__overlay {
    --poodle-context-menu-overlay-padding: 0.25rem;
    --poodle-context-menu-item-min-height: 2rem;
    --poodle-context-menu-item-padding-y-base: 0.375rem;
    --poodle-context-menu-item-padding-x-base: 0.5rem;
    --poodle-context-menu-item-padding-y-adjust: 0rem;
    --poodle-context-menu-item-padding-x-adjust: 0rem;
    --poodle-context-menu-separator-margin: 0.25rem;
    position: fixed;
    z-index: var(--poodle-overlay-z-menu);
    min-width: 14rem;
    padding: var(--poodle-context-menu-overlay-padding);
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

  .poodle-context-menu__item {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    width: 100%;
    min-height: var(--poodle-context-menu-item-min-height);
    padding:
      calc(var(--poodle-context-menu-item-padding-y-base) + var(--poodle-context-menu-item-padding-y-adjust))
      calc(var(--poodle-context-menu-item-padding-x-base) + var(--poodle-context-menu-item-padding-x-adjust));
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-body-size);
    text-align: left;
  }

  .poodle-context-menu__item:hover:not(:disabled),
  .poodle-context-menu__item:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    outline: none;
  }

  .poodle-context-menu__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-context-menu__meta {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
  }

  .poodle-context-menu__separator {
    width: 100%;
    height: 0.0625rem;
    margin: var(--poodle-context-menu-separator-margin) 0;
    background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  .poodle-context-menu[data-size="xs"] .poodle-context-menu__item {
    --poodle-context-menu-item-min-height: 1.5rem;
    --poodle-context-menu-item-padding-y-base: 0.25rem;
    --poodle-context-menu-item-padding-x-base: 0.375rem;
    font-size: 0.75rem;
  }

  .poodle-context-menu[data-size="sm"] .poodle-context-menu__item {
    --poodle-context-menu-item-min-height: 1.75rem;
    --poodle-context-menu-item-padding-y-base: 0.3125rem;
    --poodle-context-menu-item-padding-x-base: 0.4375rem;
    font-size: 0.8125rem;
  }

  .poodle-context-menu[data-size="md"] .poodle-context-menu__item {
    --poodle-context-menu-item-min-height: 2rem;
    --poodle-context-menu-item-padding-y-base: 0.375rem;
    --poodle-context-menu-item-padding-x-base: 0.5rem;
    font-size: 0.875rem;
  }

  .poodle-context-menu[data-size="lg"] .poodle-context-menu__item {
    --poodle-context-menu-item-min-height: 2.25rem;
    --poodle-context-menu-item-padding-y-base: 0.4375rem;
    --poodle-context-menu-item-padding-x-base: 0.5625rem;
    font-size: 0.9375rem;
  }

  .poodle-context-menu[data-size="xl"] .poodle-context-menu__item {
    --poodle-context-menu-item-min-height: 2.5rem;
    --poodle-context-menu-item-padding-y-base: 0.5rem;
    --poodle-context-menu-item-padding-x-base: 0.625rem;
    font-size: 1rem;
  }

  .poodle-context-menu[data-density="compact"] .poodle-context-menu__overlay {
    --poodle-context-menu-overlay-padding: 0.1875rem;
    --poodle-context-menu-item-padding-y-adjust: -0.125rem;
    --poodle-context-menu-item-padding-x-adjust: -0.125rem;
    --poodle-context-menu-separator-margin: 0.1875rem;
  }

  .poodle-context-menu[data-density="default"] .poodle-context-menu__overlay {
    --poodle-context-menu-overlay-padding: 0.25rem;
    --poodle-context-menu-item-padding-y-adjust: 0rem;
    --poodle-context-menu-item-padding-x-adjust: 0rem;
    --poodle-context-menu-separator-margin: 0.25rem;
  }

  .poodle-context-menu[data-density="comfortable"] .poodle-context-menu__overlay {
    --poodle-context-menu-overlay-padding: 0.3125rem;
    --poodle-context-menu-item-padding-y-adjust: 0.125rem;
    --poodle-context-menu-item-padding-x-adjust: 0.125rem;
    --poodle-context-menu-separator-margin: 0.3125rem;
  }
</style>
