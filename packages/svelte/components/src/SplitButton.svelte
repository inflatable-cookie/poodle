<script lang="ts">
  import "./split-button.css";
  import { registerDismissLayer } from "@poodle/headless";
  import { onMount, tick, type Snippet } from "svelte";

  import { menuNavigableItems } from "./internal";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";
  import { default as Spinner } from "./Spinner.svelte";

  import type {
    ButtonTone,
    ButtonVariant,
    ControlDensity,
    ControlSize,
    MenuItem,
    SemanticControlSizeRole,
  } from "./types";

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
      syncMenuLayout();
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
    onAction?.(item.value);
    closeMenu();
  }

  $effect(() => {
    if (!menuOpen) {
      return;
    }

    return registerDismissLayer({
      contains: (target) => rootElement?.contains(target) ?? false,
      dismissOnOutsideInteract: true,
      onDismiss: (reason) => {
        closeMenu();

        if (reason === "escape") {
          toggleElement?.focus();
        }
      },
    });
  });

  onMount(() => {
    function handleBoundaryChange(): void {
      if (menuOpen) {
        syncMenuLayout();
      }
    }

    window.addEventListener("resize", handleBoundaryChange);
    document.addEventListener("scroll", handleBoundaryChange, true);

    return () => {
      window.removeEventListener("resize", handleBoundaryChange);
      document.removeEventListener("scroll", handleBoundaryChange, true);
    };
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

