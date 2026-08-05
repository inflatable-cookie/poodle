<script lang="ts">
  import "@poodle/styles/split-button.css";
  import { registerDismissLayer, layerContains } from "@poodle/headless";
  import { tick, type Snippet } from "svelte";

  import { menuNavigableItems } from "./internal.ts";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation.ts";
  import { anchored } from "./anchored.ts";
  import { default as Spinner } from "./Spinner.svelte";

  import type {
    ButtonTone,
    ButtonVariant,
    ControlDensity,
    ControlSize,
    MenuItem,
    SemanticControlSizeRole,
  } from "./types.ts";

  interface Props {
    variant?: ButtonVariant;
    tone?: ButtonTone;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    type?: HTMLButtonElement["type"];
    items?: MenuItem[];
    disabled?: boolean;
    loading?: boolean;
    ariaLabel?: string | null;
    menuAriaLabel?: string;
    onClick?: ((event: MouseEvent) => void) | undefined;
    onAction?: ((value: string) => void) | undefined;
    children?: Snippet<[]>;
  }

  let {
    variant = "secondary",
    tone = "default",
    size = null,
    sizeRole = "control",
    density = null,
    type = "button",
    items = [],
    disabled = false,
    loading = false,
    ariaLabel = null,
    menuAriaLabel = "More actions",
    onClick = undefined,
    onAction = undefined,
    children,
  }: Props = $props();

  let rootElement = $state<HTMLDivElement | null>(null);
  let toggleElement = $state<HTMLButtonElement | null>(null);
  let menuElement = $state<HTMLDivElement | null>(null);
  let itemElements = $state<Array<HTMLButtonElement | null>>([]);
  let menuOpen = $state(false);
  let highlightIndex = $state(0);
  let menuPlacement = $state<"bottom-start" | "top-start">("bottom-start");
  let menuMaxHeight = $state<string | null>(null);
  const uiPresentation = getUiPresentation();

  const isUnavailable = $derived(disabled || loading);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedVisualSize = $derived(resolveSupportingVisualSize(resolvedSize));
  const actionableItems = $derived(menuNavigableItems(items));

  $effect(() => {
    if (!menuOpen) {
      return;
    }

    tick().then(() => {
      syncMenuHeight();
      itemElements[highlightIndex]?.focus();
    });
  });

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

  /** Cap the menu at the room available on whichever side it opened, so a long
   * list scrolls inside the surface instead of running off the viewport. */
  function syncMenuHeight(): void {
    if (!menuOpen || !rootElement) return;

    const rootRect = rootElement.getBoundingClientRect();
    const gutter = 6;
    const available =
      menuPlacement === "top-start"
        ? rootRect.top - gutter
        : window.innerHeight - rootRect.bottom - gutter;

    menuMaxHeight = available > 0 ? `${Math.floor(available)}px` : null;
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
    onAction?.(item.value);
    closeMenu();
  }

  $effect(() => {
    if (!menuOpen) {
      return;
    }

    return registerDismissLayer({
      // The menu is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, menuElement),
      dismissOnOutsideInteract: true,
      onDismiss: (reason) => {
        closeMenu();

        if (reason === "escape") {
          toggleElement?.focus();
        }
      },
    });
  });

</script>

<div class="poodle-split-button" data-variant={variant} data-tone={tone !== "default" ? tone : undefined} data-size={resolvedSize} data-density={resolvedDensity} bind:this={rootElement}>
  <button
    {type}
    class="poodle-split-button__primary"
    disabled={isUnavailable}
    aria-label={ariaLabel ?? undefined}
    aria-busy={loading ? "true" : undefined}
    onclick={(event) => onClick?.(event)}
  >
    {#if loading}
      <span class="poodle-split-button__spinner" aria-hidden="true">
        <Spinner variant="ring" size={resolvedVisualSize} tone="current" />
      </span>
    {/if}
    <span class="poodle-split-button__label">
      {@render children?.()}
    </span>
  </button>

  <div class="poodle-split-button__divider" aria-hidden="true"></div>

  <button
    type="button"
    class="poodle-split-button__toggle"
    bind:this={toggleElement}
    disabled={isUnavailable}
    aria-haspopup="true"
    aria-expanded={menuOpen ? "true" : "false"}
    aria-label={menuAriaLabel}
    onclick={toggleMenu}
    onkeydown={(event) => {
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
    <svg class="poodle-split-button__chevron" viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <path d="M4 6l4 4 4-4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </button>

  {#if menuOpen}
    <div
      bind:this={menuElement}
      use:anchored={{
        anchor: rootElement,
        placement: "bottom-start",
        offset: 6,
        onPlacement: (next) => {
          menuPlacement = next.startsWith("top") ? "top-start" : "bottom-start";
          syncMenuHeight();
        },
      }}
      class="poodle-split-button__menu"
      data-placement={menuPlacement}
      role="menu"
      aria-label={menuAriaLabel}
      style:max-height={menuMaxHeight ?? undefined}
    >
      {#each items as item (item.value)}
        {#if item.kind === "separator"}
          <div class="poodle-split-button__separator" role="separator"></div>
        {:else}
          <button
            bind:this={itemElements[actionableItems.findIndex((c) => c.value === item.value)]}
            type="button"
            class="poodle-split-button__item"
            disabled={item.disabled === true}
            role="menuitem"
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
            <span class="poodle-split-button__item-label">{item.label}</span>
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>

