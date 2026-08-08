<script module lang="ts">
  let nextHoverCardId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-styles/hover-card.css";
  import {
    hoverTransition,
    type HoverEvent as HoverMachineEvent,
    type HoverState,
  } from "@inflatable-cookie/poodle-headless";
  import { onDestroy, type Snippet } from "svelte";

  import { anchored } from "./anchored";
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
    } else {
      // Write back through the binding before notifying, so `bind:open` works
      // as it does on every other bindable Poodle component. A host that wants
      // to refuse the close re-asserts the value inside `onOpenChange`, which
      // lands last and renders no intermediate state.
      open = nextOpen;
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
      use:anchored={{ anchor: triggerElement, placement, offset: 8 }}
      id={hoverCardId}
      class="poodle-hover-card__surface"
      role="dialog"
      tabindex="-1"
      aria-label={ariaLabel ?? undefined}
      onmouseenter={clearTimers}
      onmouseleave={scheduleClose}
    >
      {@render children?.()}
    </span>
  {/if}
</span>

