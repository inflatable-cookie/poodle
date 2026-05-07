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
  let rootElement: HTMLSpanElement | null = null;
  let triggerElement: HTMLElement | null = null;
  let bubbleElement: HTMLSpanElement | null = null;
  let resolvedPlacement: OverlayPlacement = placement;
  let bubbleStyle = "";

  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;
  $: if (isOpen) {
    void updateTooltipPosition();
  }

  function setOpen(nextOpen: boolean): void {
    if (!nextOpen && triggerElement) {
      triggerElement.removeAttribute("aria-describedby");
    }

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

  function resolveAnchor(target: EventTarget | null): HTMLElement | null {
    if (!(target instanceof HTMLElement) || !rootElement) {
      return null;
    }

    return rootElement.contains(target) ? target : null;
  }

  function handlePointerEnter(event: PointerEvent): void {
    const anchor = resolveAnchor(event.target);
    if (!anchor) {
      return;
    }

    if (triggerElement && triggerElement !== anchor) {
      triggerElement.removeAttribute("aria-describedby");
    }
    triggerElement = anchor;
    scheduleOpen();
  }

  function handlePointerLeave(event: PointerEvent): void {
    if (!rootElement) {
      dismiss();
      return;
    }

    const nextTarget = event.relatedTarget;
    if (nextTarget instanceof Node && rootElement.contains(nextTarget)) {
      return;
    }

    dismiss();
  }

  function handleFocusIn(event: FocusEvent): void {
    const anchor = resolveAnchor(event.target);
    if (!anchor) {
      return;
    }

    if (triggerElement && triggerElement !== anchor) {
      triggerElement.removeAttribute("aria-describedby");
    }
    triggerElement = anchor;
    scheduleOpen();
  }

  function handleFocusOut(event: FocusEvent): void {
    if (!rootElement) {
      dismiss();
      return;
    }

    const nextTarget = event.relatedTarget;
    if (nextTarget instanceof Node && rootElement.contains(nextTarget)) {
      return;
    }

    dismiss();
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
    triggerElement.setAttribute("aria-describedby", tooltipId);
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

  onDestroy(() => {
    clearTimer();
    triggerElement?.removeAttribute("aria-describedby");
  });
</script>

<span
  bind:this={rootElement}
  class="poodle-tooltip"
  role="presentation"
  on:pointerenter={handlePointerEnter}
  on:pointerleave={handlePointerLeave}
  on:focusin={handleFocusIn}
  on:focusout={handleFocusOut}
  on:keydown={(event) => {
    if (event.key === "Escape") {
      dismiss();
    }
  }}
>
  <slot />

  {#if isOpen}
    <span
      id={tooltipId}
      bind:this={bubbleElement}
      class="poodle-tooltip__bubble"
      data-placement={resolvedPlacement}
      style={bubbleStyle}
      role="tooltip"
    >
      {content}
    </span>
  {/if}
</span>

<style>
  .poodle-tooltip {
    display: contents;
  }

  .poodle-tooltip__bubble {
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
    box-shadow:
      0 0.5rem 1.25rem rgba(0, 0, 0, 0.3),
      0 0.125rem 0.375rem rgba(0, 0, 0, 0.2);
    color: var(--poodle-color-text-primary);
    font-size: 0.6875rem;
    line-height: 1.35;
    white-space: nowrap;
  }
</style>
