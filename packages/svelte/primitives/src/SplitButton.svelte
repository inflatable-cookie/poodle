<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { menuNavigableItems } from "./internal";

  import type { ButtonTone, ButtonVariant, ControlSize, MenuItem } from "./types";

  export let variant: ButtonVariant = "secondary";
  export let tone: ButtonTone = "default";
  export let size: ControlSize = "md";
  export let items: MenuItem[] = [];
  export let isDisabled = false;
  export let isLoading = false;
  export let ariaLabel: string | null = null;
  export let menuAriaLabel = "More actions";

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    action: { value: string };
  }>();

  let rootElement: HTMLDivElement | null = null;
  let itemElements: Array<HTMLButtonElement | null> = [];
  let menuOpen = false;
  let highlightIndex = 0;

  $: isUnavailable = isDisabled || isLoading;
  $: actionableItems = menuNavigableItems(items);
  $: if (menuOpen) {
    tick().then(() => itemElements[highlightIndex]?.focus());
  }

  function toggleMenu(): void {
    if (isUnavailable) return;
    menuOpen = !menuOpen;
    if (!menuOpen) highlightIndex = 0;
  }

  function closeMenu(): void {
    menuOpen = false;
    highlightIndex = 0;
  }

  function moveHighlight(direction: 1 | -1): void {
    const count = actionableItems.length;
    if (count === 0) return;

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
    if (item.isDisabled || item.kind === "separator") return;
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

<div class="split-button" data-variant={variant} data-tone={tone !== "default" ? tone : undefined} data-size={size} bind:this={rootElement}>
  <button
    type="button"
    class="split-button__primary"
    disabled={isUnavailable}
    aria-label={ariaLabel ?? undefined}
    aria-busy={isLoading ? "true" : undefined}
    on:click={(event) => dispatch("click", event)}
  >
    {#if isLoading}
      <span class="split-button__spinner" aria-hidden="true"></span>
    {/if}
    <span class="split-button__label">
      <slot />
    </span>
  </button>

  <div class="split-button__divider" aria-hidden="true"></div>

  <button
    type="button"
    class="split-button__toggle"
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
    <div class="split-button__menu" role="menu" aria-label={menuAriaLabel}>
      {#each items as item (item.value)}
        {#if item.kind === "separator"}
          <div class="split-button__separator" role="separator"></div>
        {:else}
          <button
            bind:this={itemElements[actionableItems.findIndex((c) => c.value === item.value)]}
            type="button"
            class="split-button__item"
            disabled={item.isDisabled === true}
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
    --pug-split-fill: var(
      --pug-treatment-interactive-fill,
      var(--pug-color-background-surface)
    );
    --pug-split-fill-hover: color-mix(
      in srgb,
      var(--pug-split-fill) 84%,
      var(--pug-color-background-elevated)
    );
    --pug-split-border: var(
      --pug-treatment-interactive-border,
      var(--pug-color-border-default)
    );
    --pug-split-shadow: var(
      --pug-treatment-interactive-shadow,
      none
    );
    --pug-split-text: var(--pug-color-text-primary);
    position: relative;
    display: inline-flex;
    align-items: stretch;
    width: fit-content;
    border-radius: var(--pug-treatment-interactive-radius, var(--pug-radius-control));
  }

  .split-button[data-variant="primary"] {
    --pug-split-fill: var(
      --pug-treatment-interactive-primary-fill,
      var(--pug-color-accent-base)
    );
    --pug-split-border: var(
      --pug-treatment-interactive-primary-border,
      color-mix(in srgb, var(--pug-color-accent-base) 84%, black)
    );
    --pug-split-shadow: var(
      --pug-treatment-interactive-primary-shadow,
      none
    );
    --pug-split-text: var(
      --pug-treatment-interactive-primary-text,
      var(--pug-color-text-inverse)
    );
  }

  .split-button[data-variant="ghost"] {
    --pug-split-fill: color-mix(in srgb, var(--pug-color-background-surface) 42%, transparent);
    --pug-split-border: color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent);
  }

  .split-button[data-tone="danger"] {
    --pug-split-fill: color-mix(in srgb, var(--pug-color-status-danger) 16%, var(--pug-color-background-surface));
    --pug-split-border: color-mix(in srgb, var(--pug-color-status-danger) 46%, var(--pug-color-border-default));
    --pug-split-text: var(--pug-color-text-primary);
  }

  .split-button[data-variant="primary"][data-tone="danger"] {
    --pug-split-fill: var(--pug-color-status-danger);
    --pug-split-border: color-mix(in srgb, var(--pug-color-status-danger) 84%, black);
    --pug-split-text: var(--pug-color-text-inverse);
  }

  .split-button[data-variant="ghost"][data-tone="danger"] {
    --pug-split-fill: transparent;
    --pug-split-border: transparent;
    --pug-split-text: var(--pug-color-status-danger);
  }

  .split-button__primary,
  .split-button__toggle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: var(--pug-size-control-height);
    border: 0.0625rem solid var(--pug-split-border);
    background: var(--pug-split-fill);
    box-shadow: var(--pug-split-shadow);
    color: var(--pug-split-text);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    font-weight: var(--pug-typography-label-weight);
    letter-spacing: 0.01em;
    line-height: 1;
    transition:
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .split-button[data-size="sm"] .split-button__primary,
  .split-button[data-size="sm"] .split-button__toggle {
    height: calc(var(--pug-size-control-height) - 0.375rem);
    font-size: 0.75rem;
  }

  .split-button[data-size="lg"] .split-button__primary,
  .split-button[data-size="lg"] .split-button__toggle {
    height: calc(var(--pug-size-control-height) + 0.375rem);
    font-size: 0.875rem;
  }

  .split-button__primary {
    gap: var(--pug-space-inline-sm);
    min-width: 4rem;
    padding: 0 var(--pug-space-control-x);
    border-right: 0;
    border-radius: var(--pug-treatment-interactive-radius, var(--pug-radius-control)) 0 0 var(--pug-treatment-interactive-radius, var(--pug-radius-control));
  }

  .split-button__primary:hover:not(:disabled) {
    background: var(--pug-split-fill-hover);
  }

  .split-button__primary:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
    z-index: 1;
  }

  .split-button__primary:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .split-button__toggle {
    width: 2rem;
    padding: 0;
    border-left: 0;
    border-radius: 0 var(--pug-treatment-interactive-radius, var(--pug-radius-control)) var(--pug-treatment-interactive-radius, var(--pug-radius-control)) 0;
  }

  .split-button__toggle:hover:not(:disabled) {
    background: var(--pug-split-fill-hover);
  }

  .split-button__toggle:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
    z-index: 1;
  }

  .split-button__toggle:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .split-button__divider {
    width: 0.0625rem;
    background: color-mix(in srgb, var(--pug-split-text) 22%, transparent);
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
    width: 0.75rem;
    height: 0.75rem;
    border: 0.125rem solid color-mix(in srgb, currentColor 24%, transparent);
    border-top-color: currentColor;
    border-radius: 999px;
    animation: split-button-spinner 0.8s linear infinite;
  }

  @keyframes split-button-spinner {
    to {
      transform: rotate(360deg);
    }
  }

  .split-button__menu {
    position: absolute;
    top: calc(100% + 0.375rem);
    left: 0;
    z-index: var(--pug-overlay-z-menu);
    min-width: 12rem;
    padding: 0.25rem;
    border: 0.0625rem solid var(
      --pug-treatment-surface-elevated-border,
      color-mix(in srgb, var(--pug-color-border-default) 72%, transparent)
    );
    border-radius: var(--pug-treatment-surface-elevated-radius, var(--pug-radius-surface));
    background: var(
      --pug-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))
    );
    box-shadow: var(--pug-treatment-surface-elevated-shadow, var(--pug-elevation-overlay));
  }

  .split-button__item {
    display: flex;
    align-items: center;
    width: 100%;
    min-height: calc(var(--pug-size-control-height) - 0.25rem);
    padding: var(--pug-space-control-y) var(--pug-space-control-x);
    border: 0;
    border-radius: calc(var(--pug-radius-control) - 0.125rem);
    background: transparent;
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: var(--pug-typography-label-size);
    text-align: left;
  }

  .split-button__item:hover:not(:disabled),
  .split-button__item:focus-visible {
    background: color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent);
    outline: none;
  }

  .split-button__item:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .split-button__separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent);
  }
</style>
