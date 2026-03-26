<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { menuNavigableItems } from "./internal";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";
  import Spinner from "./Spinner.svelte";

  import type {
    ButtonTone,
    ButtonVariant,
    ControlDensity,
    ControlSize,
    MenuItem,
    SemanticControlSizeRole,
  } from "./types";

  export let variant: ButtonVariant = "secondary";
  export let tone: ButtonTone = "default";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let type: HTMLButtonElement["type"] = "button";
  export let items: MenuItem[] = [];
  export let disabled = false;
  export let loading = false;
  export let ariaLabel: string | null = null;
  export let menuAriaLabel = "More actions";

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    action: { value: string };
  }>();

  let rootElement: HTMLDivElement | null = null;
  let toggleElement: HTMLButtonElement | null = null;
  let menuElement: HTMLDivElement | null = null;
  let itemElements: Array<HTMLButtonElement | null> = [];
  let menuOpen = false;
  let highlightIndex = 0;
  let menuPlacement: "bottom-start" | "top-start" = "bottom-start";
  let menuMaxHeight: string | null = null;
  const uiPresentation = getUiPresentation();

  $: isUnavailable = disabled || loading;
  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
  $: resolvedVisualSize = resolveSupportingVisualSize(resolvedSize);
  $: actionableItems = menuNavigableItems(items);
  $: if (menuOpen) {
    tick().then(async () => {
      syncMenuLayout();
      itemElements[highlightIndex]?.focus();
    });
  }

  function toggleMenu(): void {
    if (isUnavailable) return;
    menuOpen = !menuOpen;
    if (!menuOpen) highlightIndex = 0;
  }

  function closeMenu(): void {
    menuOpen = false;
    highlightIndex = 0;
    menuPlacement = "bottom-start";
    menuMaxHeight = null;
  }

  function getScrollContainer(element: HTMLElement | null): HTMLElement | null {
    let current = element?.parentElement ?? null;

    while (current) {
      const style = getComputedStyle(current);
      const overflowY = style.overflowY;
      if (
        (overflowY === "auto" || overflowY === "scroll" || overflowY === "overlay") &&
        current.scrollHeight > current.clientHeight
      ) {
        return current;
      }
      current = current.parentElement;
    }

    return null;
  }

  function syncMenuLayout(): void {
    if (!menuOpen || !rootElement || !menuElement) return;

    const rootRect = rootElement.getBoundingClientRect();
    const menuRect = menuElement.getBoundingClientRect();
    const scrollContainer = getScrollContainer(rootElement);
    const boundaryTop = scrollContainer?.getBoundingClientRect().top ?? 0;
    const boundaryBottom =
      scrollContainer?.getBoundingClientRect().bottom ?? window.innerHeight;
    const gutter = 6;
    const availableBelow = Math.max(0, boundaryBottom - rootRect.bottom - gutter);
    const availableAbove = Math.max(0, rootRect.top - boundaryTop - gutter);
    const shouldOpenUpward =
      availableBelow < menuRect.height && availableAbove > availableBelow;
    const availableSpace = shouldOpenUpward ? availableAbove : availableBelow;

    menuPlacement = shouldOpenUpward ? "top-start" : "bottom-start";
    menuMaxHeight = availableSpace > 0 ? `${Math.floor(availableSpace)}px` : null;
  }

  function moveHighlight(direction: 1 | -1): void {
    const count = actionableItems.length;
    if (count === 0) return;

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
    if (item.disabled || item.kind === "separator") return;
    dispatch("action", { value: item.value });
    closeMenu();
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!menuOpen || !rootElement) return;
      if (!rootElement.contains(event.target as Node)) {
        closeMenu();
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && menuOpen) {
        event.preventDefault();
        closeMenu();
        toggleElement?.focus();
      }
    }

    function handleBoundaryChange(): void {
      if (menuOpen) {
        syncMenuLayout();
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);
    window.addEventListener("resize", handleBoundaryChange);
    document.addEventListener("scroll", handleBoundaryChange, true);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
      window.removeEventListener("resize", handleBoundaryChange);
      document.removeEventListener("scroll", handleBoundaryChange, true);
    };
  });
</script>

<div class="split-button" data-variant={variant} data-tone={tone !== "default" ? tone : undefined} data-size={resolvedSize} data-density={resolvedDensity} bind:this={rootElement}>
  <button
    {type}
    class="split-button__primary"
    disabled={isUnavailable}
    aria-label={ariaLabel ?? undefined}
    aria-busy={loading ? "true" : undefined}
    on:click={(event) => dispatch("click", event)}
  >
    {#if loading}
      <span class="split-button__spinner" aria-hidden="true">
        <Spinner variant="ring" size={resolvedVisualSize} tone="current" />
      </span>
    {/if}
    <span class="split-button__label">
      <slot />
    </span>
  </button>

  <div class="split-button__divider" aria-hidden="true"></div>

  <button
    type="button"
    class="split-button__toggle"
    bind:this={toggleElement}
    disabled={isUnavailable}
    aria-haspopup="true"
    aria-expanded={menuOpen ? "true" : "false"}
    aria-label={menuAriaLabel}
    on:click={toggleMenu}
    on:keydown={(event) => {
      if (event.key === "ArrowDown") {
        event.preventDefault();
        if (!menuOpen) {
          menuOpen = true;
        } else {
          moveHighlight(1);
        }
      }
      if (event.key === "ArrowUp") {
        event.preventDefault();
        if (menuOpen) moveHighlight(-1);
      }
    }}
  >
    <svg class="split-button__chevron" viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <path d="M4 6l4 4 4-4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </button>

  {#if menuOpen}
    <div
      bind:this={menuElement}
      class="split-button__menu"
      data-placement={menuPlacement}
      role="menu"
      aria-label={menuAriaLabel}
      style:max-height={menuMaxHeight ?? undefined}
    >
      {#each items as item (item.value)}
        {#if item.kind === "separator"}
          <div class="split-button__separator" role="separator"></div>
        {:else}
          <button
            bind:this={itemElements[actionableItems.findIndex((c) => c.value === item.value)]}
            type="button"
            class="split-button__item"
            disabled={item.disabled === true}
            role="menuitem"
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
            <span class="split-button__item-label">{item.label}</span>
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .split-button {
    --poodle-split-fill: var(
      --poodle-treatment-interactive-fill,
      var(--poodle-color-background-surface)
    );
    --poodle-split-fill-hover: color-mix(
      in srgb,
      var(--poodle-split-fill) 84%,
      var(--poodle-color-background-elevated)
    );
    --poodle-split-border: var(
      --poodle-treatment-interactive-border,
      var(--poodle-color-border-default)
    );
    --poodle-split-shadow: var(
      --poodle-treatment-interactive-shadow,
      none
    );
    --poodle-split-text: var(--poodle-color-text-primary);
    position: relative;
    display: inline-flex;
    align-items: stretch;
    width: fit-content;
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
  }

  .split-button[data-variant="primary"] {
    --poodle-split-fill: var(
      --poodle-treatment-interactive-primary-fill,
      var(--poodle-color-accent-base)
    );
    --poodle-split-border: var(
      --poodle-treatment-interactive-primary-border,
      color-mix(in srgb, var(--poodle-color-accent-base) 84%, black)
    );
    --poodle-split-shadow: var(
      --poodle-treatment-interactive-primary-shadow,
      none
    );
    --poodle-split-text: var(
      --poodle-treatment-interactive-primary-text,
      var(--poodle-color-text-inverse)
    );
  }

  .split-button[data-variant="ghost"] {
    --poodle-split-fill: color-mix(in srgb, var(--poodle-color-background-surface) 42%, transparent);
    --poodle-split-border: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  .split-button[data-tone="danger"] {
    --poodle-split-fill: color-mix(in srgb, var(--poodle-color-status-danger) 16%, var(--poodle-color-background-surface));
    --poodle-split-border: color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default));
    --poodle-split-text: var(--poodle-color-text-primary);
  }

  .split-button[data-variant="primary"][data-tone="danger"] {
    --poodle-split-fill: var(--poodle-color-status-danger);
    --poodle-split-border: color-mix(in srgb, var(--poodle-color-status-danger) 84%, black);
    --poodle-split-text: var(--poodle-color-text-inverse);
  }

  .split-button[data-variant="ghost"][data-tone="danger"] {
    --poodle-split-fill: transparent;
    --poodle-split-border: transparent;
    --poodle-split-text: var(--poodle-color-status-danger);
  }

  .split-button__primary,
  .split-button__toggle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: var(--poodle-size-control-height);
    border: 0.0625rem solid var(--poodle-split-border);
    background: var(--poodle-split-fill);
    box-shadow: var(--poodle-split-shadow);
    color: var(--poodle-split-text);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    letter-spacing: 0.01em;
    line-height: 1;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .split-button[data-size="sm"] .split-button__primary,
  .split-button[data-size="sm"] .split-button__toggle {
    height: calc(var(--poodle-size-control-height) - 0.375rem);
    font-size: 0.75rem;
  }

  .split-button[data-size="xs"] .split-button__primary,
  .split-button[data-size="xs"] .split-button__toggle {
    height: calc(var(--poodle-size-control-height) - 0.5rem);
    font-size: 0.6875rem;
  }

  .split-button[data-size="lg"] .split-button__primary,
  .split-button[data-size="lg"] .split-button__toggle {
    height: calc(var(--poodle-size-control-height) + 0.375rem);
    font-size: 0.875rem;
  }

  .split-button[data-size="xl"] .split-button__primary,
  .split-button[data-size="xl"] .split-button__toggle {
    height: calc(var(--poodle-size-control-height) + 0.5rem);
    font-size: 0.9375rem;
  }

  .split-button__primary {
    gap: var(--poodle-space-inline-sm);
    min-width: 4rem;
    padding: 0 var(--poodle-space-control-x);
    border-right: 0;
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control)) 0 0 var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
  }

  .split-button__primary:hover:not(:disabled) {
    background: var(--poodle-split-fill-hover);
  }

  .split-button__primary:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    z-index: 1;
  }

  .split-button__primary:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .split-button__toggle {
    width: 2rem;
    padding: 0;
    border-left: 0;
    border-radius: 0 var(--poodle-treatment-interactive-radius, var(--poodle-radius-control)) var(--poodle-treatment-interactive-radius, var(--poodle-radius-control)) 0;
  }

  .split-button__toggle:hover:not(:disabled) {
    background: var(--poodle-split-fill-hover);
  }

  .split-button__toggle:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    z-index: 1;
  }

  .split-button__toggle:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .split-button__divider {
    width: 0.0625rem;
    background: color-mix(in srgb, var(--poodle-split-text) 22%, transparent);
    align-self: center;
    height: 60%;
  }

  .split-button__chevron {
    width: 0.75rem;
    height: 0.75rem;
  }

  .split-button__label {
    min-width: 0;
    white-space: nowrap;
  }

  .split-button__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .split-button__menu {
    position: absolute;
    top: calc(100% + 0.375rem);
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
    overflow-y: auto;
  }

  .split-button__menu[data-placement="top-start"] {
    top: auto;
    bottom: calc(100% + 0.375rem);
  }

  .split-button__item {
    display: flex;
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
    font-size: var(--poodle-typography-label-size);
    text-align: left;
  }

  .split-button__item:hover:not(:disabled),
  .split-button__item:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    outline: none;
  }

  .split-button__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .split-button__separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  /* Density variants */
  .split-button[data-density="compact"] { gap: 0; }
  .split-button[data-density="comfortable"] { gap: 0.125rem; }
</style>
