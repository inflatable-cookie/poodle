<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { getFocusableElements } from "./internal";

  import type { DialogKind } from "./types";

  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let title: string | null = null;
  export let description: string | null = null;
  export let kind: DialogKind = "dialog";
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

    if (typeof document !== "undefined") {
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
    if (event.key !== "Tab" || !surfaceElement) {
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
  <div class="dialog">
    <button
      type="button"
      class="dialog__backdrop"
      aria-label="Dismiss dialog backdrop"
      on:click={() => {
        if (dismissOnBackdrop) {
          requestClose();
        }
      }}
    ></button>
    <div
      bind:this={surfaceElement}
      class="dialog__surface"
      role={kind}
      tabindex="-1"
      aria-label={title ? undefined : ariaLabel ?? undefined}
      aria-modal="true"
      on:keydown={trapFocus}
    >
      {#if title || description}
        <div class="dialog__header">
          {#if title}
            <strong>{title}</strong>
          {/if}

          {#if description}
            <p>{description}</p>
          {/if}
        </div>
      {/if}

      <div class="dialog__body">
        <slot />
      </div>

      {#if $$slots.actions}
        <div class="dialog__actions">
          <slot name="actions" />
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .dialog {
    position: fixed;
    inset: 0;
    z-index: var(--flint-overlay-z-dialog);
    display: grid;
    place-items: center;
    padding: 2rem;
  }

  .dialog__backdrop {
    position: absolute;
    inset: 0;
    padding: 0;
    border: 0;
    background: var(--flint-color-background-overlay);
    cursor: default;
  }

  .dialog__surface {
    position: relative;
    z-index: 1;
    width: min(34rem, 100%);
    max-height: min(80vh, 42rem);
    overflow: auto;
    padding: var(--flint-space-panel-y) var(--flint-space-panel-x);
    border: 0.0625rem solid var(
      --flint-treatment-surface-elevated-border,
      color-mix(in srgb, var(--flint-color-border-default) 78%, transparent)
    );
    border-radius: var(--flint-treatment-surface-elevated-radius, var(--flint-radius-surface));
    background: var(
      --flint-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--flint-color-background-elevated) 98%, var(--flint-color-background-panel))
    );
    --flint-surface: var(
      --flint-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--flint-color-background-elevated) 98%, var(--flint-color-background-panel))
    );
    box-shadow: var(--flint-treatment-surface-elevated-shadow, var(--flint-elevation-dialog));
  }

  .dialog__header {
    display: grid;
    gap: 0.375rem;
    margin-bottom: var(--flint-space-stack-md);
  }

  .dialog__header strong {
    font-family: var(--flint-typography-heading-family);
    font-size: 1rem;
    line-height: 1.2;
  }

  .dialog__header p {
    margin: 0;
    color: var(--flint-color-text-secondary);
  }

  .dialog__body {
    min-width: 0;
  }

  .dialog__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--flint-space-inline-sm);
    justify-content: flex-end;
    margin-top: var(--flint-space-stack-lg);
  }
</style>
