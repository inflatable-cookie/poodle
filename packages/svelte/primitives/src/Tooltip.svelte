<script context="module" lang="ts">
  let nextTooltipId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onDestroy, onMount, tick } from "svelte";

  import { resolveOverlayPosition } from "./overlay-position";
  import type { OverlayPlacement } from "./types";

  export let content: string;
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let delayMs = 300;
  export let placement: OverlayPlacement = "top";

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
  }>();

  const tooltipId = `poodle-tooltip-${++nextTooltipId}`;
  let timer: ReturnType<typeof setTimeout> | null = null;
  let uncontrolledOpen = defaultOpen;
  let triggerElement: HTMLSpanElement | null = null;
  let bubbleElement: HTMLSpanElement | null = null;
  let resolvedPlacement: OverlayPlacement = placement;
  let bubbleStyle = "";

  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;
  $: if (isOpen) {
    void updateTooltipPosition();
  }

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }

  function scheduleOpen(): void {
    clearTimer();
    timer = setTimeout(() => setOpen(true), delayMs);
  }

  function clearTimer(): void {
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
  }

  function dismiss(): void {
    clearTimer();
    setOpen(false);
  }

  async function updateTooltipPosition(): Promise<void> {
    if (!isOpen || !triggerElement) {
      return;
    }

    await tick();

    if (!bubbleElement) {
      return;
    }

    const nextPosition = resolveOverlayPosition(
      triggerElement.getBoundingClientRect(),
      bubbleElement.getBoundingClientRect(),
      placement,
    );

    resolvedPlacement = nextPosition.placement;
    bubbleStyle = `top: ${nextPosition.top}px; left: ${nextPosition.left}px;`;
  }

  function handleViewportChange(): void {
    if (isOpen) {
      void updateTooltipPosition();
    }
  }

  onMount(() => {
    window.addEventListener("resize", handleViewportChange);
    window.addEventListener("scroll", handleViewportChange, true);

    return () => {
      window.removeEventListener("resize", handleViewportChange);
      window.removeEventListener("scroll", handleViewportChange, true);
    };
  });

  onDestroy(() => clearTimer());
</script>

<span
  class="tooltip"
  role="presentation"
  on:mouseenter={scheduleOpen}
  on:mouseleave={dismiss}
  on:focusin={scheduleOpen}
  on:focusout={dismiss}
  on:keydown={(event) => {
    if (event.key === "Escape") {
      dismiss();
    }
  }}
>
  <span
    bind:this={triggerElement}
    class="tooltip__trigger"
    role="button"
    tabindex="0"
    aria-describedby={isOpen ? tooltipId : undefined}
  >
    <slot />
  </span>

  {#if isOpen}
    <span
      id={tooltipId}
      bind:this={bubbleElement}
      class="tooltip__bubble"
      data-placement={resolvedPlacement}
      style={bubbleStyle}
      role="tooltip"
    >
      {content}
    </span>
  {/if}
</span>

<style>
  .tooltip {
    position: relative;
    display: inline-flex;
  }

  .tooltip__trigger {
    display: inline-flex;
  }

  .tooltip__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .tooltip__bubble {
    position: fixed;
    z-index: var(--poodle-overlay-z-menu);
    max-width: 16rem;
    padding: 0.375rem 0.5rem;
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)
    );
    border-radius: var(
      --poodle-treatment-surface-elevated-radius,
      calc(var(--poodle-radius-control) - 0.125rem)
    );
    background: var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))
    );
    box-shadow: var(--poodle-treatment-surface-elevated-shadow, var(--poodle-elevation-overlay));
    color: var(--poodle-color-text-primary);
    font-size: 0.6875rem;
    line-height: 1.35;
    white-space: nowrap;
  }
</style>
