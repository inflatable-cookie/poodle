<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { getFocusableElements } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, DrawerEdge, SemanticControlSizeRole } from "./types";

  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let edge: DrawerEdge = "right";
  export let modal = true;
  export let title: string | null = null;
  export let description: string | null = null;
  export let dismissOnEscape = true;
  export let dismissOnBackdrop = true;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
    requestClose: void;
  }>();

  const uiPresentation = getUiPresentation();

  let surfaceElement: HTMLDivElement | null = null;
  let uncontrolledOpen = defaultOpen;
  let lastFocusedElement: HTMLElement | null = null;
  let bodyOverflow: string | null = null;
  let previousOpen = false;

  // Animation state: "closed" | "entering" | "open" | "exiting"
  let animState: "closed" | "entering" | "open" | "exiting" = defaultOpen ? "open" : "closed";

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;

  $: if (isOpen && !previousOpen) {
    // Opening
    animState = "entering";
    requestAnimationFrame(() => {
      animState = "open";
    });

    lastFocusedElement = document.activeElement as HTMLElement | null;
    tick().then(() => {
      const focusable = getFocusableElements(surfaceElement);
      focusable[0]?.focus() ?? surfaceElement?.focus();
    });

    if (typeof document !== "undefined" && modal) {
      bodyOverflow = document.body.style.overflow;
      document.body.style.overflow = "hidden";
    }
  }

  $: if (!isOpen && previousOpen) {
    // Closing
    animState = "exiting";

    if (typeof document !== "undefined" && bodyOverflow !== null) {
      document.body.style.overflow = bodyOverflow;
    }

    lastFocusedElement?.focus();
  }

  $: previousOpen = isOpen;

  $: visible = animState !== "closed";

  function onTransitionEnd(): void {
    if (animState === "exiting") {
      animState = "closed";
    }
  }

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }

  function requestClose(): void {
    dispatch("requestClose");
    setOpen(false);
  }

  function trapFocus(event: KeyboardEvent): void {
    if (!modal || event.key !== "Tab" || !surfaceElement) {
      return;
    }

    const focusable = getFocusableElements(surfaceElement);

    if (focusable.length === 0) {
      event.preventDefault();
      surfaceElement.focus();
      return;
    }

    const first = focusable[0];
    const last = focusable[focusable.length - 1];

    if (event.shiftKey && document.activeElement === first) {
      event.preventDefault();
      last.focus();
    }

    if (!event.shiftKey && document.activeElement === last) {
      event.preventDefault();
      first.focus();
    }
  }

  onMount(() => {
    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && isOpen && dismissOnEscape) {
        event.preventDefault();
        requestClose();
      }
    }

    document.addEventListener("keydown", handleKeydown);

    return () => {
      document.removeEventListener("keydown", handleKeydown);

      if (bodyOverflow !== null) {
        document.body.style.overflow = bodyOverflow;
      }
    };
  });
</script>

{#if visible}
  <div
    class="drawer"
    data-edge={edge}
    data-modal={modal}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-anim={animState}
    on:transitionend={onTransitionEnd}
  >
    {#if modal}
      <button
        type="button"
        class="drawer__backdrop"
        aria-label="Dismiss drawer backdrop"
        on:click={() => {
          if (dismissOnBackdrop) {
            requestClose();
          }
        }}
      ></button>
    {/if}

    <div
      bind:this={surfaceElement}
      class="drawer__surface"
      role="dialog"
      tabindex="-1"
      aria-modal={modal ? "true" : undefined}
      aria-label={title ? undefined : ariaLabel ?? undefined}
      on:keydown={trapFocus}
    >
      {#if title || description}
        <div class="drawer__header">
          {#if title}
            <strong>{title}</strong>
          {/if}

          {#if description}
            <p>{description}</p>
          {/if}
        </div>
      {/if}

      <div class="drawer__body">
        <slot />
      </div>

      {#if $$slots.actions}
        <div class="drawer__actions">
          <slot name="actions" />
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .drawer {
    position: fixed;
    inset: 0;
    z-index: var(--poodle-overlay-z-dialog);
    display: flex;
    pointer-events: none;
  }

  .drawer[data-edge="left"] { justify-content: flex-start; }
  .drawer[data-edge="right"] { justify-content: flex-end; }
  .drawer[data-edge="top"] { align-items: flex-start; }
  .drawer[data-edge="bottom"] { align-items: flex-end; }

  /* Backdrop */
  .drawer__backdrop {
    position: absolute;
    inset: 0;
    padding: 0;
    border: 0;
    background: var(--poodle-color-background-overlay);
    pointer-events: auto;
    cursor: default;
    opacity: 0;
    transition: opacity var(--poodle-motion-duration-overlay, 200ms) var(--poodle-motion-easing-standard, ease);
  }

  .drawer[data-anim="open"] .drawer__backdrop {
    opacity: 1;
  }

  .drawer[data-anim="exiting"] .drawer__backdrop {
    opacity: 0;
  }

  /* Surface */
  .drawer__surface {
    position: relative;
    z-index: 1;
    pointer-events: auto;
    width: min(28rem, 100vw);
    height: 100vh;
    overflow: auto;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-default) 78%, transparent)
    );
    border-radius: 0;
    background: var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))
    );
    --poodle-surface: var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))
    );
    box-shadow: var(--poodle-treatment-surface-elevated-shadow, var(--poodle-elevation-dialog));
    transition: transform var(--poodle-motion-duration-overlay, 200ms) var(--poodle-motion-easing-standard, ease);
  }

  /* Slide transforms per edge — entering state (off-screen) */
  .drawer[data-edge="right"] .drawer__surface { transform: translateX(100%); }
  .drawer[data-edge="left"] .drawer__surface { transform: translateX(-100%); }
  .drawer[data-edge="top"] .drawer__surface { transform: translateY(-100%); }
  .drawer[data-edge="bottom"] .drawer__surface { transform: translateY(100%); }

  /* Open state — on-screen */
  .drawer[data-anim="open"] .drawer__surface {
    transform: translate(0, 0);
  }

  /* Exiting — slide back out */
  .drawer[data-anim="exiting"][data-edge="right"] .drawer__surface { transform: translateX(100%); }
  .drawer[data-anim="exiting"][data-edge="left"] .drawer__surface { transform: translateX(-100%); }
  .drawer[data-anim="exiting"][data-edge="top"] .drawer__surface { transform: translateY(-100%); }
  .drawer[data-anim="exiting"][data-edge="bottom"] .drawer__surface { transform: translateY(100%); }

  /* Top/bottom drawers: full width, limited height */
  .drawer[data-edge="top"] .drawer__surface,
  .drawer[data-edge="bottom"] .drawer__surface {
    width: 100vw;
    height: min(24rem, 100vh);
  }

  .drawer__header {
    display: grid;
    gap: 0.375rem;
    margin-bottom: var(--poodle-space-stack-md);
  }

  .drawer__header strong {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    line-height: 1.2;
  }

  .drawer__header p {
    margin: 0;
    color: var(--poodle-color-text-secondary);
  }

  .drawer__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    justify-content: flex-end;
    margin-top: var(--poodle-space-stack-md);
  }

  /* Size variants */
  .drawer[data-size="xs"] .drawer__header strong { font-size: 0.8125rem; }
  .drawer[data-size="xs"] .drawer__header p { font-size: 0.75rem; }
  .drawer[data-size="sm"] .drawer__header strong { font-size: 0.875rem; }
  .drawer[data-size="lg"] .drawer__header strong { font-size: 1.0625rem; }
  .drawer[data-size="xl"] .drawer__header strong { font-size: 1.125rem; }

  /* Density variants */
  .drawer[data-density="compact"] .drawer__surface { padding: 0.5rem 0.75rem; }
  .drawer[data-density="comfortable"] .drawer__surface { padding: 1rem 1.25rem; }

  /* Reduced motion */
  @media (prefers-reduced-motion: reduce) {
    .drawer__backdrop,
    .drawer__surface {
      transition-duration: 0ms;
    }
  }
</style>
