<script lang="ts">
  import "@poodle/styles/drawer.css";
  import {
    getFocusableElements,
    modalTransition,
    registerDismissLayer,
    trapFocusKeydown,
    type ModalEvent,
  } from "@poodle/headless";
  import { onDestroy, tick, type Snippet } from "svelte";
  import { fade } from "svelte/transition";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, DrawerEdge, SemanticControlSizeRole } from "./types";

  interface Props {
    open?: boolean | null | undefined;
    defaultOpen?: boolean;
    edge?: DrawerEdge;
    modal?: boolean;
    title?: string | null;
    description?: string | null;
    dismissOnEscape?: boolean;
    dismissOnBackdrop?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
    onRequestClose?: (() => void) | undefined;
    children?: Snippet<[]>;
    actions?: Snippet<[]>;
  }

  let {
    open = $bindable<boolean | null | undefined>(undefined),
    defaultOpen = false,
    edge = "right",
    modal = true,
    title = null,
    description = null,
    dismissOnEscape = true,
    dismissOnBackdrop = true,
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
    onOpenChange = undefined,
    onRequestClose = undefined,
    children,
    actions,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const duration = 200;

  let surfaceElement = $state<HTMLDivElement | null>(null);
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);
  let lastFocusedElement = $state<HTMLElement | null>(null);
  let bodyOverflow = $state<string | null>(null);
  let previousOpen = $state(false);

  $effect.pre(() => {
    if (!seededDefaultOpen && open === undefined) {
      uncontrolledOpen = defaultOpen;
      seededDefaultOpen = true;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(open !== undefined);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);

  $effect(() => {
    if (isOpen && !previousOpen) {
      lastFocusedElement = document.activeElement as HTMLElement | null;
      tick().then(() => {
        const focusable = getFocusableElements(surfaceElement);
        focusable[0]?.focus() ?? surfaceElement?.focus();
      });
    }

    if (!isOpen && previousOpen) {
      lastFocusedElement?.focus();
    }

    previousOpen = isOpen;
  });

  $effect(() => {
    if (typeof document === "undefined") {
      return;
    }

    if (isOpen && modal) {
      if (bodyOverflow === null) {
        bodyOverflow = document.body.style.overflow;
        document.body.style.overflow = "hidden";
      }

      return;
    }

    if (bodyOverflow !== null) {
      document.body.style.overflow = bodyOverflow;
      bodyOverflow = null;
    }
  });

  /** Custom Svelte transition: slides from the configured edge. */
  function slideEdge(node: HTMLElement) {
    const axis = edge === "left" || edge === "right" ? "X" : "Y";
    const sign = edge === "right" || edge === "bottom" ? 1 : -1;

    return {
      duration,
      css: (_t: number, u: number) => `transform: translate${axis}(${u * sign * 100}%)`,
    };
  }

  function send(event: ModalEvent): void {
    const result = modalTransition(
      isOpen ? "open" : "closed",
      { dismissOnEscape, dismissOnBackdrop },
      event,
    );

    for (const effect of result.effects) {
      if (effect.type === "emitRequestClose") {
        onRequestClose?.();
      } else if (effect.type === "emitOpenChange") {
        if (!isControlled) {
          uncontrolledOpen = effect.open;
        }

        onOpenChange?.(effect.open);
      }
      // Focus save/restore and scroll-lock intents run in the isOpen edge
      // effects above, which see the actual open flip.
    }
  }

  function requestClose(): void {
    send({ type: "REQUEST_CLOSE" });
  }

  function trapFocus(event: KeyboardEvent): void {
    if (!modal) {
      return;
    }

    trapFocusKeydown(surfaceElement, event);
  }

  $effect(() => {
    if (!isOpen) {
      return;
    }

    return registerDismissLayer({
      contains: () => true,
      dismissOnOutsideInteract: false,
      onDismiss: () => send({ type: "ESCAPE" }),
    });
  });

  onDestroy(() => {
    if (typeof document !== "undefined" && bodyOverflow !== null) {
      document.body.style.overflow = bodyOverflow;
      bodyOverflow = null;
    }
  });
</script>

{#if isOpen}
  <div class="poodle-drawer" data-edge={edge} data-modal={modal} data-size={resolvedSize} data-density={resolvedDensity}>
    {#if modal}
      <button
        type="button"
        class="poodle-drawer__backdrop"
        aria-label="Dismiss drawer backdrop"
        transition:fade={{ duration }}
        onclick={() => send({ type: "BACKDROP_CLICK" })}
      ></button>
    {/if}

    <div
      bind:this={surfaceElement}
      class="poodle-drawer__surface"
      role="dialog"
      tabindex="-1"
      aria-modal={modal ? "true" : undefined}
      aria-label={title ? undefined : ariaLabel ?? undefined}
      transition:slideEdge
      onkeydown={trapFocus}
    >
      {#if title || description}
        <div class="poodle-drawer__header">
          {#if title}
            <strong>{title}</strong>
          {/if}

          {#if description}
            <p>{description}</p>
          {/if}
        </div>
      {/if}

      <div class="poodle-drawer__body">
        {@render children?.()}
      </div>

      {#if actions}
        <div class="poodle-drawer__actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  </div>
{/if}

