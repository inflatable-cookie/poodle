<script context="module" lang="ts">
  let nextTooltipId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";

  import type { OverlayPlacement } from "./types";

  export let content: string;
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let delayMs = 300;
  export let placement: OverlayPlacement = "top";

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
  }>();

  const tooltipId = `pug-tooltip-${++nextTooltipId}`;
  let timer: ReturnType<typeof setTimeout> | null = null;
  let uncontrolledOpen = defaultOpen;

  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;

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
    class="tooltip__trigger"
    role="button"
    tabindex="0"
    aria-describedby={isOpen ? tooltipId : undefined}
  >
    <slot />
  </span>

  {#if isOpen}
    <span id={tooltipId} class="tooltip__bubble" data-placement={placement} role="tooltip">
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
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .tooltip__bubble {
    position: absolute;
    z-index: var(--pug-overlay-z-menu);
    max-width: 16rem;
    padding: 0.375rem 0.5rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent);
    border-radius: calc(var(--pug-radius-control) - 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel));
    box-shadow: var(--pug-elevation-overlay);
    color: var(--pug-color-text-primary);
    font-size: 0.6875rem;
    line-height: 1.35;
    white-space: normal;
  }

  .tooltip__bubble[data-placement^="top"] {
    bottom: calc(100% + 0.5rem);
    left: 50%;
    transform: translateX(-50%);
  }

  .tooltip__bubble[data-placement^="bottom"] {
    top: calc(100% + 0.5rem);
    left: 50%;
    transform: translateX(-50%);
  }

  .tooltip__bubble[data-placement^="left"] {
    top: 50%;
    right: calc(100% + 0.5rem);
    transform: translateY(-50%);
  }

  .tooltip__bubble[data-placement^="right"] {
    top: 50%;
    left: calc(100% + 0.5rem);
    transform: translateY(-50%);
  }

  .tooltip__bubble[data-placement$="start"] {
    left: 0;
    right: auto;
    transform: none;
  }

  .tooltip__bubble[data-placement$="end"] {
    right: 0;
    left: auto;
    transform: none;
  }
</style>
