<script lang="ts">
  import "@poodle/styles/menu-surface.css";
  import {
    menuItemHasSubmenu,
    menuListCanActivate,
    menuListNavigate,
    menuNavigableItems,
    type AnchorTarget,
    type OverlaySurfaceGeometryChangeHandler,
  } from "@poodle/headless";
  import { tick } from "svelte";

  import MenuSurface from "./MenuSurface.svelte";
  import { anchored } from "./anchored";
  import { surfaceGeometry } from "./surface-geometry";

  import type { ControlDensity, ControlSize, MenuItem, OverlayPlacement } from "./types";

  interface Props {
    items?: MenuItem[];
    ariaLabel?: string | null;
    size?: ControlSize;
    density?: ControlDensity;
    /** Anchor for a root surface. Null for a nested flyout, which is
     * positioned against its own row inside the already-portalled parent. */
    anchor?: AnchorTarget | null;
    offset?: number;
    placement?: OverlayPlacement | null;
    overlayElement?: HTMLDivElement | null;
    /** True for a submenu flyout nested inside a parent surface. */
    nested?: boolean;
    /** Nested flyout that ran out of room on the right and opens leftward. */
    flipped?: boolean;
    onAction?: ((value: string) => void) | undefined;
    onSurfaceGeometryChange?: OverlaySurfaceGeometryChangeHandler | undefined;
    /** Nested surfaces: request the parent to close this flyout (ArrowLeft). */
    onRequestClose?: (() => void) | undefined;
  }

  let {
    items = [],
    ariaLabel = null,
    size = "md",
    density = "default",
    anchor = null,
    offset = 6,
    placement = null,
    overlayElement = $bindable<HTMLDivElement | null>(null),
    nested = false,
    flipped = false,
    onAction = undefined,
    onSurfaceGeometryChange = undefined,
    onRequestClose = undefined,
  }: Props = $props();

  // `placement` is the request; this is what survived collision resolution.
  let resolvedPlacement = $state<OverlayPlacement | null>(null);
  const displayPlacement = $derived(anchor ? resolvedPlacement : placement);

  let itemElements = $state<Array<HTMLButtonElement | null>>([]);
  let highlightIndex = $state(0);
  let openSubmenuValue = $state<string | null>(null);
  let submenuSurface = $state<{ focusFirstItem: () => void } | null>(null);
  let submenuElement = $state<HTMLDivElement | null>(null);
  let submenuFlippedValue = $state<string | null>(null);

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

  // Flip an open flyout to the parent's left edge when it would overflow
  // the right viewport edge.
  $effect(() => {
    if (!openSubmenuValue || !submenuElement) {
      submenuFlippedValue = null;
      return;
    }

    const rect = submenuElement.getBoundingClientRect();
    if (rect.right > window.innerWidth - 8) {
      submenuFlippedValue = openSubmenuValue;
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

  function openSubmenu(item: MenuItem, focusFirst: boolean): void {
    if (item.disabled || !menuItemHasSubmenu(item)) {
      return;
    }

    openSubmenuValue = item.value;

    if (focusFirst) {
      tick().then(() => {
        submenuSurface?.focusFirstItem();
      });
    }
  }

  function closeSubmenu(refocusParent: boolean): void {
    if (openSubmenuValue === null) {
      return;
    }

    const parentValue = openSubmenuValue;
    openSubmenuValue = null;

    if (refocusParent) {
      const parentIndex = actionableItems.findIndex((candidate) => candidate.value === parentValue);
      if (parentIndex >= 0) {
        focusIndex(parentIndex);
      }
    }
  }

  function activateItem(item: MenuItem): void {
    if (!menuListCanActivate(item)) {
      return;
    }

    if (menuItemHasSubmenu(item)) {
      if (openSubmenuValue === item.value) {
        closeSubmenu(true);
      } else {
        openSubmenu(item, false);
      }
      return;
    }

    onAction?.(item.value);
  }

  function handleItemPointerEnter(item: MenuItem): void {
    if (menuItemHasSubmenu(item) && !item.disabled) {
      openSubmenu(item, false);
    } else if (openSubmenuValue !== null) {
      openSubmenuValue = null;
    }
  }
</script>

<div
  bind:this={overlayElement}
  use:anchored={{
    anchor,
    placement: placement ?? "bottom-start",
    offset,
    onPlacement: (next) => (resolvedPlacement = next),
    onSurfaceGeometryChange: nested ? undefined : onSurfaceGeometryChange,
  }}
  use:surfaceGeometry={{
    onSurfaceGeometryChange: nested ? onSurfaceGeometryChange : undefined,
    placement: displayPlacement,
  }}
  class="poodle-menu-surface"
  class:poodle-menu-surface--submenu={nested}
  class:poodle-menu-surface--flipped={flipped}
  data-size={size}
  data-density={density}
  data-placement={displayPlacement ?? undefined}
  role="menu"
  aria-label={ariaLabel ?? undefined}
>
  {#each items as item (item.value)}
    {#if item.kind === "separator"}
      <div class="poodle-menu-surface__separator" role="separator"></div>
    {:else}
      {@const hasSubmenu = menuItemHasSubmenu(item)}
      <div
        class="poodle-menu-surface__item-anchor"
        class:poodle-menu-surface__item-anchor--submenu-open={hasSubmenu && openSubmenuValue === item.value}
      >
        <button
          bind:this={itemElements[actionableItems.findIndex((candidate) => candidate.value === item.value)]}
          type="button"
          class="poodle-menu-surface__item"
          disabled={item.disabled === true}
          data-kind={hasSubmenu ? "submenu" : (item.kind ?? "action")}
          data-tone={item.tone ?? "default"}
          role={item.kind === "checkbox" || item.kind === "radio" ? `menuitem${item.kind}` : "menuitem"}
          aria-checked={item.kind === "checkbox" || item.kind === "radio" ? (item.checked ? "true" : "false") : undefined}
          aria-haspopup={hasSubmenu ? "menu" : undefined}
          aria-expanded={hasSubmenu ? (openSubmenuValue === item.value ? "true" : "false") : undefined}
          onclick={() => activateItem(item)}
          onpointerenter={() => handleItemPointerEnter(item)}
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

            if (event.key === "ArrowRight" && hasSubmenu) {
              event.preventDefault();
              event.stopPropagation();
              openSubmenu(item, true);
            }

            if (event.key === "ArrowLeft" && nested) {
              event.preventDefault();
              event.stopPropagation();
              onRequestClose?.();
            }

            if (event.key === "Enter" || event.key === " ") {
              event.preventDefault();
              if (hasSubmenu) {
                openSubmenu(item, true);
              } else {
                activateItem(item);
              }
            }
          }}
        >
          <span class="poodle-menu-surface__label">{item.label}</span>

          {#if hasSubmenu}
            <span class="poodle-menu-surface__submenu-indicator" aria-hidden="true">›</span>
          {:else if item.checked}
            <span class="poodle-menu-surface__meta" aria-hidden="true">✓</span>
          {:else if item.shortcutLabel}
            <span class="poodle-menu-surface__meta" aria-hidden="true">{item.shortcutLabel}</span>
          {/if}
        </button>

        {#if hasSubmenu && openSubmenuValue === item.value}
          <MenuSurface
            bind:this={submenuSurface}
            bind:overlayElement={submenuElement}
            items={item.children ?? []}
            ariaLabel={item.label}
            size={size}
            density={density}
            nested={true}
            flipped={submenuFlippedValue === item.value}
            {onSurfaceGeometryChange}
            onAction={onAction}
            onRequestClose={() => closeSubmenu(true)}
          />
        {/if}
      </div>
    {/if}
  {/each}
</div>
