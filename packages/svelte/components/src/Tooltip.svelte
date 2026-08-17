<script module lang="ts">
  let nextTooltipId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/tooltip.css";
  import {
    hoverTransition,
    type HoverEvent as HoverMachineEvent,
    type HoverState,
  } from "@inflatable-cookie/poodle-core";
  import { onDestroy, type Snippet } from "svelte";

  import { anchored } from "./anchored";
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

  let anchorResolved = $state(false);

  // The anchor is normally established by hover/focus. A tooltip shown through
  // `open`/`defaultOpen` never ran ENTER, so the machine sits in "closed" and
  // the first-child anchor stays unresolved — the bubble renders but nothing
  // is announced and Escape is inert. Resolve the default anchor once so a
  // forced-open surface behaves exactly like a hovered one.
  $effect(() => {
    if (!isOpen || anchorResolved) {
      return;
    }

    const anchor = getDefaultAnchor();
    if (anchor) {
      triggerElement = anchor;
      anchorResolved = true;
    }
  });

  $effect(() => {
    if (isOpen && triggerElement) {
      // Announced only while shown: a stale describedby outlives the bubble.
      triggerElement.setAttribute("aria-describedby", tooltipId);
    } else if (triggerElement) {
      triggerElement.removeAttribute("aria-describedby");
    }
  });

  let machineState: HoverState = "closed";
  let syncedOpen = false;

  // Keep the machine level with the surface. A tooltip shown or hidden through
  // `open`/`defaultOpen` never ran ENTER or LEAVE, so without this sync the
  // machine holds a stale state in both directions: DISMISS from "closed" is
  // inert, so Escape leaves a forced-open tooltip stuck with no close reported;
  // and after a controlled true -> false the machine stays "open", so the next
  // ENTER takes the already-open branch and hover or focus can never reopen it.
  // SET_OPEN emits no change of its own, so syncing never echoes back to a host.
  $effect(() => {
    if (isOpen === syncedOpen) {
      return;
    }

    syncedOpen = isOpen;
    send({ type: "SET_OPEN", open: isOpen });
  });

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
        } else {
          // Write back through the binding before notifying, so `bind:open`
          // works as it does on every other bindable Poodle component. A host
          // that wants to refuse the close re-asserts the value inside
          // `onOpenChange`, which lands last and renders no intermediate
          // state — covered by DialogControlled.svelte.test.ts.
          open = effect.open;
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
      use:anchored={{
        anchor: triggerElement,
        placement,
        onPlacement: (next) => (resolvedPlacement = next),
      }}
      class="poodle-tooltip__bubble"
      data-placement={resolvedPlacement}
      role="tooltip"
    >
      {content}
    </span>
  {/if}
</span>
