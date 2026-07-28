<script module lang="ts">
  let nextDialogId = 0;
</script>

<script lang="ts">
  import "@poodle/styles/dialog.css";
  import {
    getFocusableElements,
    modalTransition,
    registerDismissLayer,
    trapFocusKeydown,
    type ModalEvent,
  } from "@poodle/headless";
  import { onDestroy, tick, type Snippet } from "svelte";
  import { portal } from "./portal";
  import { default as IconButton } from "./IconButton.svelte";
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
    overlayStyle?: string;
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
    overlayStyle = "",
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
  // A titled dialog takes its accessible name from the rendered title via
  // aria-labelledby; ariaLabel is the fallback only when there is no title.
  // A custom `header` snippet replaces the title element, so there is nothing
  // to point at — fall back to ariaLabel there too.
  const titleId = `poodle-dialog-title-${nextDialogId++}`;
  const labelledBy = $derived(!header && title ? titleId : undefined);
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
        } else {
          // Write back through the binding before notifying, so a host that
          // vetoes inside `onOpenChange` sets the final value last and no
          // intermediate state is ever rendered.
          open = effect.open;
        }

        onOpenChange?.(effect.open);
      }
      // Focus save/restore and body scroll lock intents are executed by the
      // isOpen edge $effect above, which sees the actual open flip.
    }
  }

  function requestClose(): void {
    send({ type: "REQUEST_CLOSE" });
  }

  function trapFocus(event: KeyboardEvent): void {
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
      style={overlayStyle}
      aria-label="Dismiss dialog backdrop"
      onclick={() => send({ type: "BACKDROP_CLICK" })}
    ></button>
    <div
      bind:this={surfaceElement}
      class={`poodle-dialog__surface ${contentClassName}`}
      class:poodle-dialog__surface--bare={bare}
      style={contentStyle}
      role={effectiveRole}
      tabindex="-1"
      aria-labelledby={labelledBy}
      aria-label={labelledBy ? undefined : ariaLabel ?? undefined}
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
                  <strong id={titleId} class="poodle-dialog__title">{title}</strong>
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

