<script module lang="ts">
  let nextTooltipId = 0;
</script>

<script lang="ts">
  import "@poodle/styles/tooltip.css";
  import {
    hoverTransition,
    type HoverEvent as HoverMachineEvent,
    type HoverState,
  } from "@poodle/headless";
  import { onDestroy, onMount, tick, type Snippet } from "svelte";

  import { resolveOverlayPosition } from "./overlay-position";
  import type { OverlayPlacement } from "./types";

  interface Props {
    content: string;
    open?: boolean | null;
    defaultOpen?: boolean;
    delayMs?: number;
    placement?: OverlayPlacement;
    onOpenChange?: ((open: boolean) => void) | undefined;
    children?: Snippet<[]>;
  }

  let {
    content,
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    delayMs = 300,
    placement = "top",
    onOpenChange = undefined,
    children,
  }: Props = $props();

  const tooltipId = `poodle-tooltip-${++nextTooltipId}`;
  let timer: ReturnType<typeof setTimeout> | null = null;
  let uncontrolledOpen = $state(false);
  let rootElement = $state<HTMLSpanElement | null>(null);
  let triggerElement = $state<HTMLElement | null>(null);
  let bubbleElement = $state<HTMLSpanElement | null>(null);
  let resolvedPlacement = $state<OverlayPlacement>("top");
  let bubbleStyle = $state("");
  let seededDefaults = $state(false);

  $effect.pre(() => {
    if (!seededDefaults) {
      uncontrolledOpen = defaultOpen;
      resolvedPlacement = placement;
      seededDefaults = true;
    }
  });

  const isControlled = $derived(open !== null);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);

  $effect(() => {
    if (isOpen) {
      void updateTooltipPosition();
    }
  });

  let machineState: HoverState = "closed";

  function send(event: HoverMachineEvent): void {
    const result = hoverTransition(machineState, { openDelayMs: delayMs, closeDelayMs: 0 }, event);
    machineState = result.state;

    for (const effect of result.effects) {
      if (effect.type === "clearTimer") {
        clearTimer();
      } else if (effect.type === "startTimer") {
        clearTimer();
        timer = setTimeout(() => send({ type: "TIMER_FIRE" }), effect.ms);
      } else if (effect.type === "emitOpenChange") {
        if (!effect.open && triggerElement) {
          triggerElement.removeAttribute("aria-describedby");
        }

        if (!isControlled) {
          uncontrolledOpen = effect.open;
        }

        onOpenChange?.(effect.open);
      }
    }
  }

  function scheduleOpen(): void {
    send({ type: "ENTER" });
  }

  function clearTimer(): void {
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
  }

  function dismiss(): void {
    send({ type: "DISMISS" });
  }

  function getDefaultAnchor(): HTMLElement | null {
    if (!rootElement) {
      return null;
    }

    return rootElement.firstElementChild instanceof HTMLElement
      ? rootElement.firstElementChild
      : null;
  }

  function resolveAnchor(target: EventTarget | null): HTMLElement | null {
    if (!rootElement) {
      return null;
    }

    if (!(target instanceof HTMLElement)) {
      return getDefaultAnchor();
    }

    if (target === rootElement) {
      return getDefaultAnchor();
    }

    return rootElement.contains(target) ? target : getDefaultAnchor();
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
    await tick();

    const renderedRect = bubbleElement.getBoundingClientRect();
    const correctedTop = nextPosition.top + (nextPosition.top - renderedRect.top);
    const correctedLeft = nextPosition.left + (nextPosition.left - renderedRect.left);
    if (Math.abs(correctedTop - nextPosition.top) > 0.5
      || Math.abs(correctedLeft - nextPosition.left) > 0.5) {
      bubbleStyle = `top: ${correctedTop}px; left: ${correctedLeft}px;`;
    }
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
  onpointerenter={handlePointerEnter}
  onpointerleave={handlePointerLeave}
  onfocusin={handleFocusIn}
  onfocusout={handleFocusOut}
  onkeydown={(event) => {
    if (event.key === "Escape") {
      dismiss();
    }
  }}
>
  {@render children?.()}

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
