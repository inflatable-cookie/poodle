<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/popover.css";
  import {
    createInstanceId,
    getFocusableElements,
    layerContains,
    popoverParts,
    popoverTransition,
    registerDismissLayer,
    type PopoverContext,
    type PopoverEvent,
    type OverlaySurfaceGeometryChangeHandler,
  } from "@inflatable-cookie/poodle-core";
  import { tick, type Snippet } from "svelte";

  import { anchored } from "./anchored";
  import type { OverlayPlacement, PopoverInitialFocus } from "./types";

  interface Props {
    open?: boolean | null;
    defaultOpen?: boolean;
    placement?: OverlayPlacement;
    offset?: number;
    dismissOnOutsideInteract?: boolean;
    initialFocus?: PopoverInitialFocus;
    ariaLabel?: string | null;
    block?: boolean;
    triggerIsInteractive?: boolean;
    disabled?: boolean;
    surfaceWidth?: "content" | "trigger";
    surfaceMinWidth?: string | null;
    surfaceMaxWidth?: string | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
    onSurfaceGeometryChange?: OverlaySurfaceGeometryChangeHandler | undefined;
    trigger?: Snippet<[]>;
    children?: Snippet<[]>;
  }

  let {
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    placement = "bottom-start",
    offset = 8,
    dismissOnOutsideInteract = true,
    initialFocus = "first-focusable",
    ariaLabel = null,
    block = false,
    triggerIsInteractive = false,
    disabled = false,
    surfaceWidth = "content",
    surfaceMinWidth = null,
    surfaceMaxWidth = null,
    onOpenChange = undefined,
    onSurfaceGeometryChange = undefined,
    trigger,
    children,
  }: Props = $props();

  const popoverId = createInstanceId("popover");
  let rootElement = $state<HTMLDivElement | null>(null);
  let triggerElement = $state<HTMLDivElement | null>(null);
  let surfaceElement = $state<HTMLDivElement | null>(null);
  let uncontrolledOpen = $state(false);
  let previousOpen = $state(false);
  let seededDefaultOpen = $state(false);

  $effect.pre(() => {
    if (!seededDefaultOpen) {
      uncontrolledOpen = defaultOpen;
      seededDefaultOpen = true;
    }
  });

  const isControlled = $derived(open !== null);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);
  // Null until the surface is measured; the anchored action reports back
  // whichever candidate survived collision resolution.
  let placementFromAnchor = $state<OverlayPlacement | null>(null);
  const resolvedPlacement = $derived(isOpen ? (placementFromAnchor ?? placement) : placement);

  $effect(() => {
    if (!(isOpen && !previousOpen)) {
      previousOpen = isOpen;
      return;
    }

    tick().then(() => {
      if (!surfaceElement) {
        return;
      }

      if (initialFocus === "content") {
        surfaceElement.focus();
        return;
      }

      if (initialFocus === "first-focusable") {
        getFocusableElements(surfaceElement)[0]?.focus();
      }
    });

    previousOpen = isOpen;
  });

  const machineContext = $derived<PopoverContext>({
    disabled,
    dismissOnOutsideInteract,
    initialFocus,
  });

  const parts = $derived(
    popoverParts(isOpen ? "open" : "closed", machineContext, {
      surfaceId: popoverId,
      ariaLabel,
      block,
      triggerIsInteractive,
      placement: resolvedPlacement,
      surfaceWidth,
    }),
  );

  function send(event: PopoverEvent): void {
    const result = popoverTransition(isOpen ? "open" : "closed", machineContext, event);

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitOpenChange": {
          if (isControlled) {
            open = effect.open;
          } else {
            uncontrolledOpen = effect.open;
          }

          onOpenChange?.(effect.open);
          break;
        }
        case "restoreTriggerFocus": {
          triggerElement?.focus();
          break;
        }
        case "focusOnOpen": {
          // Executed by the isOpen $effect above, which waits for the surface
          // to render before applying the initialFocus strategy.
          break;
        }
      }
    }
  }

  $effect(() => {
    if (!isOpen) {
      return;
    }

    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, surfaceElement),
      dismissOnOutsideInteract,
      onDismiss: (reason) => send(reason === "escape" ? { type: "ESCAPE" } : { type: "OUTSIDE_INTERACT" }),
    });
  });
</script>

<div {...parts.root} class="poodle-popover" bind:this={rootElement}>
  <div
    bind:this={triggerElement}
    {...parts.trigger}
    class="poodle-popover__trigger"
    onclick={() => send({ type: "TOGGLE" })}
    onkeydown={triggerIsInteractive ? undefined : (event) => {
      if (disabled) {
        return;
      }

      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        send({ type: "TOGGLE" });
      }
    }}
  >
    {@render trigger?.()}
  </div>

  {#if isOpen}
    <div
      bind:this={surfaceElement}
      use:anchored={{
        anchor: rootElement,
        placement,
        offset,
        matchWidth: surfaceWidth === "trigger",
        onPlacement: (next) => (placementFromAnchor = next),
        onSurfaceGeometryChange,
      }}
      {...parts.surface}
      class="poodle-popover__surface"
      style={[
        surfaceMinWidth ? `--poodle-popover-surface-min-width: ${surfaceMinWidth}` : "",
        surfaceMaxWidth ? `--poodle-popover-surface-max-width: ${surfaceMaxWidth}` : "",
      ].filter(Boolean).join("; ")}
    >
      {@render children?.()}
    </div>
  {/if}
</div>
