<script lang="ts">
  import "@poodle/styles/menu-surface.css";
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

