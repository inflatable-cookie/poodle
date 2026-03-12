<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { getFocusableElements } from "./internal";

  import type { DrawerEdge } from "./types";

  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let edge: DrawerEdge = "right";
  export let isModal = true;
  export let title: string | null = null;
  export let description: string | null = null;
  export let dismissOnEscape = true;
  export let dismissOnBackdrop = true;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
    requestClose: void;
  }>();

  let surfaceElement: HTMLDivElement | null = null;
  let uncontrolledOpen = defaultOpen;
  let lastFocusedElement: HTMLElement | null = null;
  let bodyOverflow: string | null = null;
  let previousOpen = false;

  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;
  $: if (isOpen && !previousOpen) {
    lastFocusedElement = document.activeElement as HTMLElement | null;
    tick().then(() => {
      const focusable = getFocusableElements(surfaceElement);
      focusable[0]?.focus() ?? surfaceElement?.focus();
    });

    if (typeof document !== "undefined" && isModal) {
      bodyOverflow = document.body.style.overflow;
      document.body.style.overflow = "hidden";
    }
  }
  $: if (!isOpen && previousOpen) {
    if (typeof document !== "undefined" && bodyOverflow !== null) {
      document.body.style.overflow = bodyOverflow;
    }

    lastFocusedElement?.focus();
  }
  $: previousOpen = isOpen;

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
    if (!isModal || event.key !== "Tab" || !surfaceElement) {
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
  <div class="drawer" data-edge={edge} data-modal={isModal}>
    {#if isModal}
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
      aria-modal={isModal ? "true" : undefined}
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
    z-index: var(--pug-overlay-z-dialog);
    display: flex;
    pointer-events: none;
  }

  .drawer[data-edge="left"] {
    justify-content: flex-start;
  }

  .drawer[data-edge="right"] {
    justify-content: flex-end;
  }

  .drawer[data-edge="top"] {
    align-items: flex-start;
  }

  .drawer[data-edge="bottom"] {
    align-items: flex-end;
  }

  .drawer__backdrop {
    position: absolute;
    inset: 0;
    padding: 0;
    border: 0;
    background: var(--pug-color-background-overlay);
    pointer-events: auto;
    cursor: default;
  }

  .drawer__surface {
    position: relative;
    z-index: 1;
    pointer-events: auto;
    width: min(28rem, 100vw);
    height: 100vh;
    overflow: auto;
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 78%, transparent);
    border-radius: 0;
    background: color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel));
    box-shadow: var(--pug-elevation-dialog);
  }

  .drawer[data-edge="top"] .drawer__surface,
  .drawer[data-edge="bottom"] .drawer__surface {
    width: 100vw;
    height: min(24rem, 100vh);
  }

  .drawer__header {
    display: grid;
    gap: 0.375rem;
    margin-bottom: var(--pug-space-stack-md);
  }

  .drawer__header strong {
    font-family: var(--pug-typography-heading-family);
    font-size: 1rem;
    line-height: 1.2;
  }

  .drawer__header p {
    margin: 0;
    color: var(--pug-color-text-secondary);
  }

  .drawer__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
    justify-content: flex-end;
    margin-top: var(--pug-space-stack-md);
  }
</style>
