<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";
  import { fade } from "svelte/transition";

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
  const duration = 200;

  let surfaceElement: HTMLDivElement | null = null;
  let uncontrolledOpen = defaultOpen;
  let lastFocusedElement: HTMLElement | null = null;
  let bodyOverflow: string | null = null;
  let previousOpen = false;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;

  $: if (isOpen && !previousOpen) {
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
    if (typeof document !== "undefined" && bodyOverflow !== null) {
      document.body.style.overflow = bodyOverflow;
      bodyOverflow = null;
    }

    lastFocusedElement?.focus();
  }
  $: previousOpen = isOpen;

  /** Custom Svelte transition: slides from the configured edge. */
  function slideEdge(node: HTMLElement) {
    const axis = edge === "left" || edge === "right" ? "X" : "Y";
    const sign = edge === "right" || edge === "bottom" ? 1 : -1;

    return {
      duration,
      css: (t: number, u: number) =>
        `transform: translate${axis}(${u * sign * 100}%)`,
    };
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

{#if isOpen}
  <div class="poodle-drawer" data-edge={edge} data-modal={modal} data-size={resolvedSize} data-density={resolvedDensity}>
    {#if modal}
      <button
        type="button"
        class="poodle-drawer__backdrop"
        aria-label="Dismiss drawer backdrop"
        transition:fade={{ duration }}
        on:click={() => {
          if (dismissOnBackdrop) {
            requestClose();
          }
        }}
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
      on:keydown={trapFocus}
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
        <slot />
      </div>

      {#if $$slots.actions}
        <div class="poodle-drawer__actions">
          <slot name="actions" />
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
    background: var(--poodle-color-background-overlay);
    pointer-events: auto;
    cursor: default;
  }

  .poodle-drawer__surface {
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
  }

  .poodle-drawer[data-edge="top"] .poodle-drawer__surface,
  .poodle-drawer[data-edge="bottom"] .poodle-drawer__surface {
    width: 100vw;
    height: min(24rem, 100vh);
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
    color: var(--poodle-color-text-secondary);
  }

  .poodle-drawer__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    justify-content: flex-end;
    margin-top: var(--poodle-space-stack-md);
  }

  /* Size variants */
  .poodle-drawer[data-size="xs"] .poodle-drawer__header strong { font-size: 0.8125rem; }
  .poodle-drawer[data-size="xs"] .poodle-drawer__header p { font-size: 0.75rem; }
  .poodle-drawer[data-size="sm"] .poodle-drawer__header strong { font-size: 0.875rem; }
  .poodle-drawer[data-size="lg"] .poodle-drawer__header strong { font-size: 1.0625rem; }
  .poodle-drawer[data-size="xl"] .poodle-drawer__header strong { font-size: 1.125rem; }

  /* Density variants */
  .poodle-drawer[data-density="compact"] .poodle-drawer__surface { padding: 0.5rem 0.75rem; }
  .poodle-drawer[data-density="comfortable"] .poodle-drawer__surface { padding: 1rem 1.25rem; }
</style>
