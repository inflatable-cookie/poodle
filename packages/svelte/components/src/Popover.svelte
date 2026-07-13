<script lang="ts">
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

<style>
  .poodle-popover {
    position: relative;
    display: inline-flex;
  }

  .poodle-popover[data-block="true"] {
    flex: 1 1 auto;
    display: flex;
    width: 100%;
    min-width: 0;
  }

  .poodle-popover__trigger {
    display: inline-flex;
  }

  .poodle-popover__trigger[data-block="true"] {
    display: flex;
    width: 100%;
    min-width: 0;
  }

  .poodle-popover__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-popover__trigger[data-disabled="true"] {
    cursor: not-allowed;
  }

  .poodle-popover__surface {
    position: absolute;
    z-index: var(--poodle-overlay-z-menu);
    min-width: var(--poodle-popover-surface-min-width, 14rem);
    max-width: var(--poodle-popover-surface-max-width, min(24rem, 90vw));
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-recipe-popover-surface-fill, var(--poodle-color-background-elevated));
    --poodle-surface: var(--poodle-color-background-elevated);
    box-shadow: var(--poodle-recipe-popover-surface-shadow, inset 0 0.0625rem 0 rgba(255, 255, 255, 0.08),
      0 0.625rem 1.5rem rgba(9, 13, 18, 0.22),
      0 0.125rem 0.375rem rgba(0, 0, 0, 0.15));
  }

  .poodle-popover__surface[data-surface-width="trigger"] {
    width: 100%;
    min-width: 100%;
    box-sizing: border-box;
  }

  .poodle-popover__surface[data-placement^="bottom"] {
    top: calc(100% + var(--poodle-popover-offset));
    left: 0;
  }

  .poodle-popover__surface[data-placement^="top"] {
    bottom: calc(100% + var(--poodle-popover-offset));
    left: 0;
  }

  .poodle-popover__surface[data-placement^="right"] {
    top: 0;
    left: calc(100% + var(--poodle-popover-offset));
  }

  .poodle-popover__surface[data-placement^="left"] {
    top: 0;
    right: calc(100% + var(--poodle-popover-offset));
  }

  .poodle-popover__surface[data-placement$="end"] {
    left: auto;
    right: 0;
  }
</style>
