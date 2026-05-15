<script lang="ts">
  import { onMount, tick, type Snippet } from "svelte";

  import { menuNavigableItems } from "./internal";
  import { resolveOverlayPosition } from "./overlay-position";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    MenuItem,
    OverlayPlacement,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    items?: MenuItem[];
    open?: boolean | null;
    defaultOpen?: boolean;
    placement?: OverlayPlacement;
    ariaLabel?: string | null;
    triggerAriaLabel?: string | null;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
    onAction?: ((value: string) => void) | undefined;
    trigger?: Snippet<[]>;
  }

  let {
    items = [],
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    placement = "bottom-start",
    ariaLabel = null,
    triggerAriaLabel = null,
    sizeRole = "chrome",
    size = null,
    density = null,
    onOpenChange = undefined,
    onAction = undefined,
    trigger,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let rootElement = $state<HTMLDivElement | null>(null);
  let triggerElement = $state<HTMLDivElement | null>(null);
  let overlayElement = $state<HTMLDivElement | null>(null);
  let itemElements = $state<Array<HTMLButtonElement | null>>([]);
  let uncontrolledOpen = $state(false);
  let highlightIndex = $state(0);
  let resolvedPlacement = $state<OverlayPlacement>("bottom-start");
  let overlayStyle = $state("");

  $effect.pre(() => {
    if (!rootElement) {
      uncontrolledOpen = defaultOpen;
      resolvedPlacement = placement;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(open !== null);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);
  const actionableItems = $derived(menuNavigableItems(items));

  $effect(() => {
    if (!isOpen) {
      return;
    }

    tick().then(() => {
      void updateOverlayPosition();
      itemElements[highlightIndex]?.focus();
    });
  });

  function setOpen(nextOpen: boolean): void {
    if (isControlled) {
      open = nextOpen;
    } else {
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

  function handleTriggerClick(event: MouseEvent): void {
    event.preventDefault();
    event.stopPropagation();
    setOpen(!isOpen);
  }

  function handleTriggerKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter" || event.key === " " || event.key === "ArrowDown") {
      event.preventDefault();
      event.stopPropagation();
      setOpen(true);
    }
  }

  async function updateOverlayPosition(): Promise<void> {
    if (!isOpen || !triggerElement) {
      return;
    }

    await tick();

    if (!overlayElement) {
      return;
    }

    const nextPosition = resolveOverlayPosition(
      triggerElement.getBoundingClientRect(),
      overlayElement.getBoundingClientRect(),
      placement,
    );

    resolvedPlacement = nextPosition.placement;
    overlayStyle = `top: ${nextPosition.top}px; left: ${nextPosition.left}px;`;
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!isOpen || !rootElement) {
        return;
      }

      if (!rootElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && isOpen) {
        event.preventDefault();
        setOpen(false);
      }
    }

    function handleViewportChange(): void {
      if (isOpen) {
        void updateOverlayPosition();
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);
    window.addEventListener("resize", handleViewportChange);
    window.addEventListener("scroll", handleViewportChange, true);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
      window.removeEventListener("resize", handleViewportChange);
      window.removeEventListener("scroll", handleViewportChange, true);
    };
  });
</script>

<div class="poodle-menu" bind:this={rootElement} data-size={resolvedSize} data-density={resolvedDensity}>
  <div
    bind:this={triggerElement}
    class="poodle-menu__trigger"
    role="button"
    tabindex="0"
    aria-expanded={isOpen ? "true" : "false"}
    aria-label={triggerAriaLabel ?? undefined}
    onclick={handleTriggerClick}
    onkeydown={handleTriggerKeydown}
  >
    {@render trigger?.()}
  </div>

  {#if isOpen}
    <div
      bind:this={overlayElement}
      class="poodle-menu__overlay"
      data-placement={resolvedPlacement}
      style={overlayStyle}
      role="menu"
      aria-label={ariaLabel ?? undefined}
    >
      {#each items as item, index (item.value)}
        {#if item.kind === "separator"}
          <div class="poodle-menu__separator" role="separator"></div>
        {:else}
          <button
            bind:this={itemElements[actionableItems.findIndex((candidate) => candidate.value === item.value)]}
            type="button"
            class="poodle-menu__item"
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
            <span class="poodle-menu__label">{item.label}</span>

            {#if item.checked}
              <span class="poodle-menu__meta" aria-hidden="true">✓</span>
            {:else if item.shortcutLabel}
              <span class="poodle-menu__meta" aria-hidden="true">{item.shortcutLabel}</span>
            {/if}
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .poodle-menu {
    position: relative;
    display: inline-flex;
  }

  .poodle-menu__trigger {
    display: inline-flex;
  }

  .poodle-menu__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-menu__overlay {
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

  .poodle-menu__item {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    width: 100%;
    min-height: var(--poodle-size-control-height);
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

  .poodle-menu__item:hover:not(:disabled),
  .poodle-menu__item:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    outline: none;
  }

  .poodle-menu__item[data-tone="danger"] {
    color: var(--poodle-color-status-danger);
  }

  .poodle-menu__item[data-tone="danger"]:hover:not(:disabled),
  .poodle-menu__item[data-tone="danger"]:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-status-danger) 14%, transparent);
  }

  .poodle-menu__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-menu__meta {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
  }

  .poodle-menu__separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  /* Size variants */
  .poodle-menu[data-size="xs"] .poodle-menu__item { min-height: 1.5rem; font-size: 0.6875rem; }
  .poodle-menu[data-size="xs"] .poodle-menu__meta { font-size: 0.5625rem; }
  .poodle-menu[data-size="sm"] .poodle-menu__item { min-height: 1.75rem; font-size: 0.75rem; }
  .poodle-menu[data-size="sm"] .poodle-menu__meta { font-size: 0.625rem; }
  .poodle-menu[data-size="lg"] .poodle-menu__item { min-height: 2.75rem; font-size: 0.9375rem; }
  .poodle-menu[data-size="lg"] .poodle-menu__meta { font-size: 0.75rem; }
  .poodle-menu[data-size="xl"] .poodle-menu__item { min-height: 3.25rem; font-size: 1rem; }
  .poodle-menu[data-size="xl"] .poodle-menu__meta { font-size: 0.8125rem; }

  /* Density variants */
  .poodle-menu[data-density="compact"] .poodle-menu__item { padding-inline: 0.375rem; }
  .poodle-menu[data-density="comfortable"] .poodle-menu__item { padding-inline: 0.75rem; }
</style>
