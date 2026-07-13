<script lang="ts">
  import "@poodle/styles/popover.css";
  import {
    createInstanceId,
    getFocusableElements,
    popoverParts,
    popoverTransition,
    registerDismissLayer,
    type PopoverContext,
    type PopoverEvent,
  } from "@poodle/headless";
  import { tick, type Snippet } from "svelte";

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
    disabled?: boolean;
    surfaceWidth?: "content" | "trigger";
    surfaceMinWidth?: string | null;
    surfaceMaxWidth?: string | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
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
    disabled = false,
    surfaceWidth = "content",
    surfaceMinWidth = null,
    surfaceMaxWidth = null,
    onOpenChange = undefined,
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
      placement,
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
      contains: (target) => rootElement?.contains(target) ?? false,
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
    onkeydown={(event) => {
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
      {...parts.surface}
      class="poodle-popover__surface"
      style={[
        `--poodle-popover-offset: ${offset}px`,
        surfaceMinWidth ? `--poodle-popover-surface-min-width: ${surfaceMinWidth}` : "",
        surfaceMaxWidth ? `--poodle-popover-surface-max-width: ${surfaceMaxWidth}` : "",
      ].filter(Boolean).join("; ")}
    >
      {@render children?.()}
    </div>
  {/if}
</div>

