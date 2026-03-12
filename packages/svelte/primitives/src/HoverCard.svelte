<script context="module" lang="ts">
  let nextHoverCardId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";

  import type { OverlayPlacement } from "./types";

  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let openDelayMs = 180;
  export let closeDelayMs = 120;
  export let placement: OverlayPlacement = "top";
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
  }>();

  const hoverCardId = `pug-hover-card-${++nextHoverCardId}`;
  let openTimer: ReturnType<typeof setTimeout> | null = null;
  let closeTimer: ReturnType<typeof setTimeout> | null = null;
  let uncontrolledOpen = defaultOpen;

  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }

  function clearTimers(): void {
    if (openTimer) {
      clearTimeout(openTimer);
      openTimer = null;
    }

    if (closeTimer) {
      clearTimeout(closeTimer);
      closeTimer = null;
    }
  }

  function scheduleOpen(): void {
    clearTimers();
    openTimer = setTimeout(() => setOpen(true), openDelayMs);
  }

  function scheduleClose(): void {
    clearTimers();
    closeTimer = setTimeout(() => setOpen(false), closeDelayMs);
  }

  onDestroy(() => clearTimers());
</script>

<span
  class="hover-card"
  role="presentation"
  on:mouseenter={scheduleOpen}
  on:mouseleave={scheduleClose}
  on:focusin={scheduleOpen}
  on:focusout={scheduleClose}
  on:keydown={(event) => {
    if (event.key === "Escape") {
      clearTimers();
      setOpen(false);
    }
  }}
>
  <span
    class="hover-card__trigger"
    role="button"
    tabindex="0"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? hoverCardId : undefined}
  >
    <slot name="trigger" />
  </span>

  {#if isOpen}
    <span
      id={hoverCardId}
      class="hover-card__surface"
      data-placement={placement}
      role="dialog"
      tabindex="-1"
      aria-label={ariaLabel ?? undefined}
      on:mouseenter={clearTimers}
      on:mouseleave={scheduleClose}
    >
      <slot />
    </span>
  {/if}
</span>

<style>
  .hover-card {
    position: relative;
    display: inline-flex;
  }

  .hover-card__trigger {
    display: inline-flex;
  }

  .hover-card__trigger:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .hover-card__surface {
    position: absolute;
    z-index: var(--pug-overlay-z-menu);
    min-width: 14rem;
    max-width: min(22rem, 90vw);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel));
    box-shadow: var(--pug-elevation-overlay);
  }

  .hover-card__surface[data-placement^="top"] {
    bottom: calc(100% + 0.5rem);
    left: 50%;
    transform: translateX(-50%);
  }

  .hover-card__surface[data-placement^="bottom"] {
    top: calc(100% + 0.5rem);
    left: 50%;
    transform: translateX(-50%);
  }

  .hover-card__surface[data-placement^="left"] {
    top: 50%;
    right: calc(100% + 0.5rem);
    transform: translateY(-50%);
  }

  .hover-card__surface[data-placement^="right"] {
    top: 50%;
    left: calc(100% + 0.5rem);
    transform: translateY(-50%);
  }

  .hover-card__surface[data-placement$="start"] {
    left: 0;
    right: auto;
    transform: none;
  }

  .hover-card__surface[data-placement$="end"] {
    right: 0;
    left: auto;
    transform: none;
  }
</style>
