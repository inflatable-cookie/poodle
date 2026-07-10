<script module lang="ts">
  let nextHoverCardId = 0;
</script>

<script lang="ts">
  import {
    hoverTransition,
    type HoverEvent as HoverMachineEvent,
    type HoverState,
  } from "@poodle/headless";
  import { onDestroy, tick, type Snippet } from "svelte";

  import { resolveOverlayPosition } from "./overlay-position";
  import type { OverlayPlacement } from "./types";

  interface Props {
    open?: boolean | null;
    defaultOpen?: boolean;
    openDelayMs?: number;
    closeDelayMs?: number;
    placement?: OverlayPlacement;
    ariaLabel?: string | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
    trigger?: Snippet<[]>;
    children?: Snippet<[]>;
  }

  let {
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    openDelayMs = 180,
    closeDelayMs = 120,
    placement = "top",
    ariaLabel = null,
    onOpenChange = undefined,
    trigger,
    children,
  }: Props = $props();

  const hoverCardId = `poodle-hover-card-${++nextHoverCardId}`;
  let openTimer: ReturnType<typeof setTimeout> | null = null;
  let closeTimer: ReturnType<typeof setTimeout> | null = null;
  let uncontrolledOpen = $state(false);
  let triggerElement = $state<HTMLSpanElement | null>(null);
  let surfaceElement = $state<HTMLSpanElement | null>(null);
  let surfaceStyle = $state("");

  $effect.pre(() => {
    if (!triggerElement) {
      uncontrolledOpen = defaultOpen;
    }
  });

  const isControlled = $derived(open !== null);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);

  let machineState: HoverState = "closed";

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    if (nextOpen) {
      surfaceStyle = "visibility: hidden;";
      tick().then(positionSurface);
    }

    onOpenChange?.(nextOpen);
  }

  function send(event: HoverMachineEvent): void {
    const result = hoverTransition(machineState, { openDelayMs, closeDelayMs }, event);
    machineState = result.state;

    for (const effect of result.effects) {
      if (effect.type === "clearTimer") {
        clearTimers();
      } else if (effect.type === "startTimer") {
        clearTimers();
        openTimer = setTimeout(() => send({ type: "TIMER_FIRE" }), effect.ms);
      } else if (effect.type === "emitOpenChange") {
        setOpen(effect.open);
      }
    }
  }

  function positionSurface(): void {
    if (!triggerElement || !surfaceElement) {
      return;
    }

    const position = resolveOverlayPosition(
      triggerElement.getBoundingClientRect(),
      surfaceElement.getBoundingClientRect(),
      placement,
      8,
    );

    surfaceStyle = `left: ${position.left}px; top: ${position.top}px;`;
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
    send({ type: "ENTER" });
  }

  function scheduleClose(): void {
    send({ type: "LEAVE" });
  }

  onDestroy(() => clearTimers());
</script>

<span
  class="poodle-hover-card"
  role="presentation"
  onmouseenter={scheduleOpen}
  onmouseleave={scheduleClose}
  onfocusin={scheduleOpen}
  onfocusout={scheduleClose}
  onkeydown={(event) => {
    if (event.key === "Escape") {
      clearTimers();
      setOpen(false);
    }
  }}
>
  <span
    bind:this={triggerElement}
    class="poodle-hover-card__trigger"
    role="button"
    tabindex="0"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? hoverCardId : undefined}
  >
    {@render trigger?.()}
  </span>

  {#if isOpen}
    <span
      bind:this={surfaceElement}
      id={hoverCardId}
      class="poodle-hover-card__surface"
      role="dialog"
      tabindex="-1"
      aria-label={ariaLabel ?? undefined}
      style={surfaceStyle}
      onmouseenter={clearTimers}
      onmouseleave={scheduleClose}
    >
      {@render children?.()}
    </span>
  {/if}
</span>

<style>
  .poodle-hover-card {
    position: relative;
    display: inline-flex;
  }

  .poodle-hover-card__trigger {
    display: inline-flex;
  }

  .poodle-hover-card__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-hover-card__surface {
    position: fixed;
    z-index: var(--poodle-overlay-z-menu);
    min-width: 14rem;
    max-width: min(22rem, 90vw);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)
    );
    border-radius: var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface));
    background: var(--poodle-recipe-hover-card-surface-fill, var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))
    ));
    --poodle-surface: var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))
    );
    box-shadow: var(--poodle-recipe-hover-card-surface-shadow, var(--poodle-treatment-surface-elevated-shadow, var(--poodle-elevation-overlay)));
  }
</style>
