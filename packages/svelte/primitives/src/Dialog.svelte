<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { getFocusableElements } from "./internal";
  import { portal } from "./portal";
  import IconButton from "./IconButton.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  /** Controlled open state. Pass `null` for uncontrolled mode. */
  export let open: boolean | null = null;
  /** Initial open state for uncontrolled mode. */
  export let defaultOpen = false;
  /** Optional title rendered in the built-in header. Ignored when `header` slot is used. */
  export let title: string | null = null;
  /** Optional description below the title. Ignored when `header` slot is used. */
  export let description: string | null = null;
  /** ARIA role. Defaults to "dialog". AlertDialog sets "alertdialog". */
  export let role: "dialog" | "alertdialog" = "dialog";
  /** Whether Escape key closes the dialog. */
  export let dismissOnEscape = true;
  /** Whether clicking the backdrop closes the dialog. */
  export let dismissOnBackdrop = true;
  /** Accessible label when no visible title is provided. */
  export let ariaLabel: string | null = null;
  /** Custom CSS class applied to the surface element. */
  export let contentClassName = "";
  /** Custom inline style applied to the surface element. */
  export let contentStyle = "";
  /** Custom CSS class applied to the backdrop element. */
  export let overlayClassName = "";
  /** Show a close button in the top-right corner. */
  export let showCloseButton = false;
  /** Accessible label for the close button. */
  export let closeLabel = "Close dialog";
  /** Surface width preset. Defaults to "md". */
  export let width: "sm" | "md" | "lg" | "xl" | "full" = "md";
  /** When true, the surface has no internal padding or structure — consumers control all layout. */
  export let bare = false;
  /** Explicit size override. */
  export let size: ControlSize | null = null;
  /** Semantic size role. */
  export let sizeRole: SemanticControlSizeRole = "control";
  /** Explicit density override. */
  export let density: ControlDensity | null = null;

  // Legacy compat: accept `kind` as alias for `role`
  /** @deprecated Use `role` instead. */
  export let kind: "dialog" | "alertdialog" | undefined = undefined;

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

  $: effectiveRole = kind ?? role;
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
  <div use:portal class="dialog" data-size={resolvedSize} data-density={resolvedDensity} data-width={width}>
    <button
      type="button"
      class={`dialog__backdrop ${overlayClassName}`}
      aria-label="Dismiss dialog backdrop"
      on:click={() => {
        if (dismissOnBackdrop) {
          requestClose();
        }
      }}
    ></button>
    <div
      bind:this={surfaceElement}
      class={`dialog__surface ${contentClassName}`}
      class:dialog__surface--bare={bare}
      style={contentStyle}
      role={effectiveRole}
      tabindex="-1"
      aria-label={title ? undefined : ariaLabel ?? undefined}
      aria-modal="true"
      on:keydown={trapFocus}
    >
      {#if bare}
        {#if showCloseButton}
          <div class="dialog__close dialog__close--overlay">
            <IconButton
              type="button"
              icon="x"
              ariaLabel={closeLabel}
              variant="ghost"
              sizeRole="chrome"
              size={resolvedSize}
              on:click={requestClose}
            />
          </div>
        {/if}
        <slot />
      {:else}
        {#if $$slots.header || title || description || showCloseButton}
          <div class="dialog__header-row">
            {#if $$slots.header}
              <div class="dialog__header">
                <slot name="header" />
              </div>
            {:else if title || description}
              <div class="dialog__header">
                {#if title}
                  <strong>{title}</strong>
                {/if}

                {#if description}
                  <p>{description}</p>
                {/if}
              </div>
            {/if}

            {#if showCloseButton}
              <div class="dialog__close">
                <IconButton
                  type="button"
                  icon="x"
                  ariaLabel={closeLabel}
                  variant="ghost"
                  sizeRole="chrome"
                  size={resolvedSize}
                  on:click={requestClose}
                />
              </div>
            {/if}
          </div>
        {/if}

        <div class="dialog__body">
          <slot />
        </div>

        {#if $$slots.footer}
          <div class="dialog__footer">
            <slot name="footer" />
          </div>
        {:else if $$slots.actions}
          <div class="dialog__actions">
            <slot name="actions" />
          </div>
        {/if}
      {/if}
    </div>
  </div>
{/if}

<style>
  .dialog {
    position: fixed;
    inset: 0;
    z-index: var(--poodle-overlay-z-dialog);
    display: grid;
    place-items: center;
    padding: 2rem;
  }

  .dialog__backdrop {
    position: absolute;
    inset: 0;
    padding: 0;
    border: 0;
    background: var(--poodle-color-background-overlay);
    cursor: default;
  }

  .dialog__surface {
    position: relative;
    z-index: 1;
    width: min(34rem, 100%);
    max-height: min(80vh, 42rem);
    overflow: auto;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-default) 78%, transparent)
    );
    border-radius: var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface));
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

  .dialog__surface--bare {
    padding: 0;
  }

  /* Width presets */
  .dialog[data-width="sm"] .dialog__surface { width: min(24rem, 100%); }
  .dialog[data-width="lg"] .dialog__surface { width: min(48rem, 100%); }
  .dialog[data-width="xl"] .dialog__surface { width: min(64rem, 100%); }
  .dialog[data-width="full"] .dialog__surface { width: 100%; }

  .dialog__header-row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
    margin-bottom: 1.25rem;
  }

  .dialog__close {
    position: static;
    flex-shrink: 0;
    margin-top: -0.5rem;
    margin-right: -0.375rem;
  }

  .dialog__close--overlay {
    position: absolute;
    top: var(--poodle-space-panel-y);
    right: var(--poodle-space-panel-x);
    z-index: 1;
  }

  .dialog__header {
    display: grid;
    gap: 0.5rem;
    flex: 1 1 auto;
    min-width: 0;
  }

  .dialog__header strong {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    line-height: 1.2;
  }

  .dialog__header p {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .dialog__body {
    min-width: 0;
  }

  .dialog__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    justify-content: flex-end;
    margin-top: var(--poodle-space-stack-lg);
  }

  .dialog__footer {
    margin-top: var(--poodle-space-stack-lg);
  }

  /* Size variants */
  .dialog[data-size="xs"] .dialog__header strong {
    font-size: 0.8125rem;
  }

  .dialog[data-size="xs"] .dialog__header p {
    font-size: 0.6875rem;
  }

  .dialog[data-size="sm"] .dialog__header strong {
    font-size: 0.875rem;
  }

  .dialog[data-size="sm"] .dialog__header p {
    font-size: 0.75rem;
  }

  .dialog[data-size="lg"] .dialog__header strong {
    font-size: 1.0625rem;
  }

  .dialog[data-size="lg"] .dialog__header p {
    font-size: 0.875rem;
  }

  .dialog[data-size="xl"] .dialog__header strong {
    font-size: 1.125rem;
  }

  .dialog[data-size="xl"] .dialog__header p {
    font-size: 0.9375rem;
  }

  /* Density variants */
  .dialog[data-density="compact"] .dialog__surface { padding: 0.5rem 0.75rem; }
  .dialog[data-density="compact"] .dialog__surface--bare { padding: 0; }
  .dialog[data-density="compact"] .dialog__close--overlay {
    top: 0.5rem;
    right: 0.75rem;
  }
  .dialog[data-density="comfortable"] .dialog__surface { padding: 1rem 1.25rem; }
  .dialog[data-density="comfortable"] .dialog__surface--bare { padding: 0; }
  .dialog[data-density="comfortable"] .dialog__close--overlay {
    top: 1rem;
    right: 1.25rem;
  }
</style>
