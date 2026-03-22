<script context="module" lang="ts">
  let nextHoverCardId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onDestroy, tick } from "svelte";

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
  let triggerElement: HTMLSpanElement | null = null;
  let surfaceElement: HTMLSpanElement | null = null;
  let surfaceStyle = "";

  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    if (nextOpen) {
      surfaceStyle = "visibility: hidden;";
      tick().then(positionSurface);
    }

    dispatch("openChange", { open: nextOpen });
  }

  function positionSurface(): void {
    if (!triggerElement || !surfaceElement) {
      return;
    }

    const trigger = triggerElement.getBoundingClientRect();
    const surface = surfaceElement.getBoundingClientRect();
    const pad = 8;
    const gap = 8;
    const vw = window.innerWidth;
    const vh = window.innerHeight;

    let x: number;
    let y: number;

    const isVertical = placement.startsWith("top") || placement.startsWith("bottom");
    const isHorizontal = placement.startsWith("left") || placement.startsWith("right");

    if (isVertical) {
      if (placement.endsWith("start")) {
        x = trigger.left;
      } else if (placement.endsWith("end")) {
        x = trigger.right - surface.width;
      } else {
        x = trigger.left + trigger.width / 2 - surface.width / 2;
      }

      if (placement.startsWith("top")) {
        y = trigger.top - surface.height - gap;
      } else {
        y = trigger.bottom + gap;
      }
    } else if (isHorizontal) {
      if (placement.startsWith("left")) {
        x = trigger.left - surface.width - gap;
      } else {
        x = trigger.right + gap;
      }

      y = trigger.top + trigger.height / 2 - surface.height / 2;
    } else {
      x = trigger.left + trigger.width / 2 - surface.width / 2;
      y = trigger.top - surface.height - gap;
    }

    if (x + surface.width > vw - pad) {
      x = vw - pad - surface.width;
    }

    if (x < pad) {
      x = pad;
    }

    if (y + surface.height > vh - pad) {
      y = vh - pad - surface.height;
    }

    if (y < pad) {
      y = pad;
    }

    surfaceStyle = `left: ${x}px; top: ${y}px;`;
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
    bind:this={triggerElement}
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
      bind:this={surfaceElement}
      id={hoverCardId}
      class="hover-card__surface"
      role="dialog"
      tabindex="-1"
      aria-label={ariaLabel ?? undefined}
      style={surfaceStyle}
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
    position: fixed;
    z-index: var(--pug-overlay-z-menu);
    min-width: 14rem;
    max-width: min(22rem, 90vw);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid var(
      --pug-treatment-surface-elevated-border,
      color-mix(in srgb, var(--pug-color-border-default) 72%, transparent)
    );
    border-radius: var(--pug-treatment-surface-elevated-radius, var(--pug-radius-surface));
    background: var(
      --pug-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))
    );
    --pug-surface: var(
      --pug-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))
    );
    box-shadow: var(--pug-treatment-surface-elevated-shadow, var(--pug-elevation-overlay));
  }
</style>
