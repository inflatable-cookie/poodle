<script lang="ts">
  import { onDestroy, onMount, tick, type Snippet } from "svelte";

  import { getFocusableElements } from "./internal";
  import { portal } from "./portal";
  import IconButton from "./IconButton.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    open?: boolean | null | undefined;
    defaultOpen?: boolean;
    title?: string | null;
    description?: string | null;
    role?: "dialog" | "alertdialog";
    dismissOnEscape?: boolean;
    dismissOnBackdrop?: boolean;
    ariaLabel?: string | null;
    contentClassName?: string;
    contentStyle?: string;
    overlayClassName?: string;
    showCloseButton?: boolean;
    closeLabel?: string;
    width?: "sm" | "md" | "lg" | "xl" | "full";
    bare?: boolean;
    size?: ControlSize | null;
    closeButtonSize?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
    onRequestClose?: (() => void) | undefined;
    kind?: "dialog" | "alertdialog" | undefined;
    children?: Snippet<[]>;
    header?: Snippet<[]>;
    footer?: Snippet<[]>;
    actions?: Snippet<[]>;
  }

  let {
    open = $bindable<boolean | null | undefined>(undefined),
    defaultOpen = false,
    title = null,
    description = null,
    role = "dialog",
    dismissOnEscape = true,
    dismissOnBackdrop = true,
    ariaLabel = null,
    contentClassName = "",
    contentStyle = "",
    overlayClassName = "",
    showCloseButton = false,
    closeLabel = "Close dialog",
    width = "md",
    bare = false,
    size = null,
    closeButtonSize = null,
    sizeRole = "control",
    density = null,
    onOpenChange = undefined,
    onRequestClose = undefined,
    kind = undefined,
    children,
    header,
    footer,
    actions,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

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

  const effectiveRole = $derived(kind ?? role);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedCloseButtonSize = $derived(closeButtonSize ?? resolveSemanticControlSize($uiPresentation.sizeScale, "chrome"));
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

      if (typeof document !== "undefined") {
        bodyOverflow = document.body.style.overflow;
        document.body.style.overflow = "hidden";
      }
    }

    if (!isOpen && previousOpen) {
      if (typeof document !== "undefined" && bodyOverflow !== null) {
        document.body.style.overflow = bodyOverflow;
      }

      lastFocusedElement?.focus();
    }

    previousOpen = isOpen;
  });

  function setOpen(nextOpen: boolean): void {
    if (isControlled) {
      open = nextOpen;
    } else {
      uncontrolledOpen = nextOpen;
    }

    onOpenChange?.(nextOpen);
  }

  function requestClose(): void {
    onRequestClose?.();
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

  onDestroy(() => {
    if (bodyOverflow !== null) {
      document.body.style.overflow = bodyOverflow;
    }
  });
</script>

{#if isOpen}
  <div use:portal class="poodle-dialog" data-size={resolvedSize} data-density={resolvedDensity} data-width={width}>
    <button
      type="button"
      class={`poodle-dialog__backdrop ${overlayClassName}`}
      aria-label="Dismiss dialog backdrop"
      onclick={() => {
        if (dismissOnBackdrop) {
          requestClose();
        }
      }}
    ></button>
    <div
      bind:this={surfaceElement}
      class={`poodle-dialog__surface ${contentClassName}`}
      class:poodle-dialog__surface--bare={bare}
      style={contentStyle}
      role={effectiveRole}
      tabindex="-1"
      aria-label={title ? undefined : ariaLabel ?? undefined}
      aria-modal="true"
      onkeydown={trapFocus}
    >
      {#if bare}
        {#if showCloseButton}
          <div class="poodle-dialog__close poodle-dialog__close--overlay">
              <IconButton
                type="button"
                icon="x"
                ariaLabel={closeLabel}
                variant="ghost"
                sizeRole="chrome"
                size={resolvedCloseButtonSize}
                onClick={requestClose}
              />
            </div>
          {/if}
        {@render children?.()}
      {:else}
        {#if header || title || description || showCloseButton}
          <div class="poodle-dialog__header-row">
            {#if header}
              <div class="poodle-dialog__header">
                {@render header()}
              </div>
            {:else if title || description}
              <div class="poodle-dialog__header">
                {#if title}
                  <strong class="poodle-dialog__title">{title}</strong>
                {/if}

                {#if description}
                  <p>{description}</p>
                {/if}
              </div>
            {/if}

            {#if showCloseButton}
              <div class="poodle-dialog__close">
                <IconButton
                  type="button"
                  icon="x"
                  ariaLabel={closeLabel}
                  variant="ghost"
                  sizeRole="chrome"
                  size={resolvedCloseButtonSize}
                  onClick={requestClose}
                />
              </div>
            {/if}
          </div>
        {/if}

        <div class="poodle-dialog__body">
          {@render children?.()}
        </div>

        {#if footer}
          <div class="poodle-dialog__footer">
            {@render footer()}
          </div>
        {:else if actions}
          <div class="poodle-dialog__actions">
            {@render actions()}
          </div>
        {/if}
      {/if}
    </div>
  </div>
{/if}

<style>
  .poodle-dialog {
    position: fixed;
    inset: 0;
    z-index: var(--poodle-overlay-z-dialog);
    display: grid;
    place-items: center;
    padding: 2rem;
  }

  .poodle-dialog__backdrop {
    position: absolute;
    inset: 0;
    padding: 0;
    border: 0;
    background: var(--poodle-color-background-overlay);
    cursor: default;
  }

  .poodle-dialog__surface {
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

  .poodle-dialog__surface--bare {
    padding: 0;
  }

  /* Width presets */
  .poodle-dialog[data-width="sm"] .poodle-dialog__surface { width: min(24rem, 100%); }
  .poodle-dialog[data-width="lg"] .poodle-dialog__surface { width: min(48rem, 100%); }
  .poodle-dialog[data-width="xl"] .poodle-dialog__surface { width: min(64rem, 100%); }
  .poodle-dialog[data-width="full"] .poodle-dialog__surface { width: 100%; }

  .poodle-dialog__header-row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
    margin-bottom: 1.25rem;
  }

  .poodle-dialog__close {
    position: static;
    flex-shrink: 0;
    margin-right: -0.375rem;
  }

  .poodle-dialog__close--overlay {
    position: absolute;
    top: var(--poodle-space-panel-y);
    right: var(--poodle-space-panel-x);
    z-index: 1;
  }

  .poodle-dialog__header {
    display: grid;
    gap: 0.5rem;
    flex: 1 1 auto;
    min-width: 0;
  }

  .poodle-dialog__title {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    line-height: 1.2;
  }

  .poodle-dialog__header p {
    margin: 0;
    padding-top: 0.375rem;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .poodle-dialog__body {
    min-width: 0;
  }

  .poodle-dialog__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    justify-content: flex-end;
    margin-top: var(--poodle-space-stack-lg);
  }

  .poodle-dialog__footer {
    margin-top: var(--poodle-space-stack-lg);
  }

  /* Size variants */
  .poodle-dialog[data-size="xs"] .poodle-dialog__header strong {
    font-size: 0.8125rem;
  }

  .poodle-dialog[data-size="xs"] .poodle-dialog__header p {
    font-size: 0.6875rem;
  }

  .poodle-dialog[data-size="sm"] .poodle-dialog__header strong {
    font-size: 0.875rem;
  }

  .poodle-dialog[data-size="sm"] .poodle-dialog__header p {
    font-size: 0.75rem;
  }

  .poodle-dialog[data-size="lg"] .poodle-dialog__header strong {
    font-size: 1.0625rem;
  }

  .poodle-dialog[data-size="lg"] .poodle-dialog__header p {
    font-size: 0.875rem;
  }

  .poodle-dialog[data-size="xl"] .poodle-dialog__header strong {
    font-size: 1.125rem;
  }

  .poodle-dialog[data-size="xl"] .poodle-dialog__header p {
    font-size: 0.9375rem;
  }

  /* Density variants */
  .poodle-dialog[data-density="compact"] .poodle-dialog__surface { padding: 0.5rem 0.75rem; }
  .poodle-dialog[data-density="compact"] .poodle-dialog__surface--bare { padding: 0; }
  .poodle-dialog[data-density="compact"] .poodle-dialog__close--overlay {
    top: 0.5rem;
    right: 0.75rem;
  }
  .poodle-dialog[data-density="comfortable"] .poodle-dialog__surface { padding: 1rem 1.25rem; }
  .poodle-dialog[data-density="comfortable"] .poodle-dialog__surface--bare { padding: 0; }
  .poodle-dialog[data-density="comfortable"] .poodle-dialog__close--overlay {
    top: 1rem;
    right: 1.25rem;
  }
</style>
