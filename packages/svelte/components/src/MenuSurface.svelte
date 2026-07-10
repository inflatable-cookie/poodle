<script lang="ts">
  import { menuListCanActivate, menuListNavigate, menuNavigableItems } from "@poodle/headless";

  import type { ControlDensity, ControlSize, MenuItem, OverlayPlacement } from "./types";

  interface Props {
    items?: MenuItem[];
    ariaLabel?: string | null;
    size?: ControlSize;
    density?: ControlDensity;
    overlayStyle?: string;
    placement?: OverlayPlacement | null;
    overlayElement?: HTMLDivElement | null;
    onAction?: ((value: string) => void) | undefined;
  }

  let {
    items = [],
    ariaLabel = null,
    size = "md",
    density = "default",
    overlayStyle = "",
    placement = null,
    overlayElement = $bindable<HTMLDivElement | null>(null),
    onAction = undefined,
  }: Props = $props();

  let itemElements = $state<Array<HTMLButtonElement | null>>([]);
  let highlightIndex = $state(0);

  const actionableItems = $derived(menuNavigableItems(items));

  $effect(() => {
    const count = actionableItems.length;
    if (count === 0) {
      highlightIndex = 0;
      return;
    }

    if (highlightIndex >= count || actionableItems[highlightIndex]?.disabled) {
      highlightIndex = menuListNavigate(actionableItems, 0, "first");
    }
  });

  function focusIndex(index: number): void {
    highlightIndex = index;
    itemElements[index]?.focus();
  }

  export function focusFirstItem(): void {
    if (actionableItems.length === 0) {
      return;
    }

    focusIndex(menuListNavigate(actionableItems, highlightIndex, "first"));
  }

  export function moveHighlight(direction: 1 | -1): void {
    if (actionableItems.length === 0) {
      return;
    }

    focusIndex(menuListNavigate(actionableItems, highlightIndex, direction === 1 ? "next" : "prev"));
  }

  export function moveToBoundary(boundary: "start" | "end"): void {
    if (actionableItems.length === 0) {
      return;
    }

    focusIndex(menuListNavigate(actionableItems, highlightIndex, boundary === "start" ? "first" : "last"));
  }

  function activateItem(item: MenuItem): void {
    if (!menuListCanActivate(item)) {
      return;
    }

    onAction?.(item.value);
  }
</script>

<div
  bind:this={overlayElement}
  class="poodle-menu-surface"
  data-size={size}
  data-density={density}
  data-placement={placement ?? undefined}
  style={overlayStyle}
  role="menu"
  aria-label={ariaLabel ?? undefined}
>
  {#each items as item (item.value)}
    {#if item.kind === "separator"}
      <div class="poodle-menu-surface__separator" role="separator"></div>
    {:else}
      <button
        bind:this={itemElements[actionableItems.findIndex((candidate) => candidate.value === item.value)]}
        type="button"
        class="poodle-menu-surface__item"
        disabled={item.disabled === true}
        data-kind={item.kind ?? "action"}
        data-tone={item.tone ?? "default"}
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
            moveToBoundary("start");
          }

          if (event.key === "End") {
            event.preventDefault();
            moveToBoundary("end");
          }

          if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            activateItem(item);
          }
        }}
      >
        <span class="poodle-menu-surface__label">{item.label}</span>

        {#if item.checked}
          <span class="poodle-menu-surface__meta" aria-hidden="true">✓</span>
        {:else if item.shortcutLabel}
          <span class="poodle-menu-surface__meta" aria-hidden="true">{item.shortcutLabel}</span>
        {/if}
      </button>
    {/if}
  {/each}
</div>

<style>
  .poodle-menu-surface {
    --poodle-menu-overlay-padding: 0.25rem;
    --poodle-menu-item-min-height: 2rem;
    --poodle-menu-item-padding-y-base: 0.375rem;
    --poodle-menu-item-padding-x-base: 0.5rem;
    --poodle-menu-item-padding-y-adjust: 0rem;
    --poodle-menu-item-padding-x-adjust: 0rem;
    --poodle-menu-separator-margin: 0.25rem;
    position: fixed;
    z-index: var(--poodle-overlay-z-menu);
    min-width: 14rem;
    padding: var(--poodle-menu-overlay-padding);
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

  .poodle-menu-surface__item {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    width: 100%;
    min-height: var(--poodle-menu-item-min-height);
    padding:
      calc(var(--poodle-menu-item-padding-y-base) + var(--poodle-menu-item-padding-y-adjust))
      calc(var(--poodle-menu-item-padding-x-base) + var(--poodle-menu-item-padding-x-adjust));
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-body-size);
    text-align: left;
  }

  .poodle-menu-surface__item:hover:not(:disabled),
  .poodle-menu-surface__item:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    outline: none;
  }

  .poodle-menu-surface__item[data-tone="danger"] {
    color: var(--poodle-color-status-danger);
  }

  .poodle-menu-surface__item[data-tone="danger"]:hover:not(:disabled),
  .poodle-menu-surface__item[data-tone="danger"]:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-status-danger) 14%, transparent);
  }

  .poodle-menu-surface__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-menu-surface__meta {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
  }

  .poodle-menu-surface__separator {
    width: 100%;
    height: 0.0625rem;
    margin: var(--poodle-menu-separator-margin) 0;
    background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  .poodle-menu-surface[data-size="xs"] .poodle-menu-surface__item {
    --poodle-menu-item-min-height: 1.5rem;
    --poodle-menu-item-padding-y-base: 0.25rem;
    --poodle-menu-item-padding-x-base: 0.375rem;
    font-size: 0.6875rem;
  }

  .poodle-menu-surface[data-size="xs"] .poodle-menu-surface__meta { font-size: 0.5625rem; }

  .poodle-menu-surface[data-size="sm"] .poodle-menu-surface__item {
    --poodle-menu-item-min-height: 1.75rem;
    --poodle-menu-item-padding-y-base: 0.3125rem;
    --poodle-menu-item-padding-x-base: 0.4375rem;
    font-size: 0.75rem;
  }

  .poodle-menu-surface[data-size="sm"] .poodle-menu-surface__meta { font-size: 0.625rem; }

  .poodle-menu-surface[data-size="md"] .poodle-menu-surface__item {
    --poodle-menu-item-min-height: 2rem;
    --poodle-menu-item-padding-y-base: 0.375rem;
    --poodle-menu-item-padding-x-base: 0.5rem;
    font-size: 0.875rem;
  }

  .poodle-menu-surface[data-size="lg"] .poodle-menu-surface__item {
    --poodle-menu-item-min-height: 2.375rem;
    --poodle-menu-item-padding-y-base: 0.375rem;
    --poodle-menu-item-padding-x-base: 0.5625rem;
    font-size: 0.9375rem;
  }

  .poodle-menu-surface[data-size="lg"] .poodle-menu-surface__meta { font-size: 0.75rem; }

  .poodle-menu-surface[data-size="xl"] .poodle-menu-surface__item {
    --poodle-menu-item-min-height: 2.75rem;
    --poodle-menu-item-padding-y-base: 0.4375rem;
    --poodle-menu-item-padding-x-base: 0.625rem;
    font-size: 1rem;
  }

  .poodle-menu-surface[data-size="xl"] .poodle-menu-surface__meta { font-size: 0.8125rem; }

  .poodle-menu-surface[data-density="compact"] {
    --poodle-menu-overlay-padding: 0.1875rem;
    --poodle-menu-item-padding-y-adjust: -0.125rem;
    --poodle-menu-item-padding-x-adjust: -0.125rem;
    --poodle-menu-separator-margin: 0.1875rem;
  }

  .poodle-menu-surface[data-density="default"] {
    --poodle-menu-overlay-padding: 0.25rem;
    --poodle-menu-item-padding-y-adjust: 0rem;
    --poodle-menu-item-padding-x-adjust: 0rem;
    --poodle-menu-separator-margin: 0.25rem;
  }

  .poodle-menu-surface[data-density="comfortable"] {
    --poodle-menu-overlay-padding: 0.3125rem;
    --poodle-menu-item-padding-y-adjust: 0.125rem;
    --poodle-menu-item-padding-x-adjust: 0.125rem;
    --poodle-menu-separator-margin: 0.3125rem;
  }
</style>
