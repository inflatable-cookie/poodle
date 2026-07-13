<script lang="ts">
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

<style>
  .poodle-drawer {
    position: fixed;
    inset: 0;
    z-index: var(--poodle-overlay-z-dialog);
    display: flex;
    pointer-events: none;
  }

  .poodle-drawer[data-edge="left"] { justify-content: flex-start; }
  .poodle-drawer[data-edge="right"] { justify-content: flex-end; }
  .poodle-drawer[data-edge="top"] { align-items: flex-start; }
  .poodle-drawer[data-edge="bottom"] { align-items: flex-end; }

  .poodle-drawer__backdrop {
    position: absolute;
    inset: 0;
    padding: 0;
    border: 0;
    background: var(--poodle-recipe-drawer-backdrop-fill, var(--poodle-color-background-overlay));
    pointer-events: auto;
    cursor: default;
  }

  .poodle-drawer__surface {
    position: relative;
    z-index: 1;
    pointer-events: auto;
    width: min(28rem, 100vw);
    height: 100dvh;
    overflow: auto;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0 solid color-mix(in srgb, var(--poodle-color-border-default) 78%, transparent);
    border-radius: 0;
    background: var(--poodle-recipe-drawer-surface-fill, color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel)));
    --poodle-surface: color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel));
    box-shadow: var(--poodle-recipe-drawer-surface-shadow, var(--poodle-elevation-dialog));
  }

  .poodle-drawer[data-edge="left"] .poodle-drawer__surface {
    border-inline-end-width: 0.0625rem;
  }

  .poodle-drawer[data-edge="right"] .poodle-drawer__surface {
    border-inline-start-width: 0.0625rem;
  }

  .poodle-drawer[data-edge="top"] .poodle-drawer__surface {
    border-block-end-width: 0.0625rem;
  }

  .poodle-drawer[data-edge="bottom"] .poodle-drawer__surface {
    border-block-start-width: 0.0625rem;
  }

  .poodle-drawer[data-edge="top"] .poodle-drawer__surface,
  .poodle-drawer[data-edge="bottom"] .poodle-drawer__surface {
    width: 100vw;
    height: min(24rem, 100dvh);
  }

  .poodle-drawer__header {
    display: grid;
    gap: 0.375rem;
    margin-bottom: var(--poodle-space-stack-md);
  }

  .poodle-drawer__header strong {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    line-height: 1.2;
  }

  .poodle-drawer__header p {
    margin: 0;
    color: var(--poodle-recipe-drawer-header-text, var(--poodle-color-text-secondary));
  }

  .poodle-drawer__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    justify-content: flex-end;
    margin-top: var(--poodle-space-stack-md);
  }
</style>
