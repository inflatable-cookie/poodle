<script module lang="ts">
  let nextDialogId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/dialog.css";
  import {
    getFocusableElements,
    modalTransition,
    registerDismissLayer,
    trapFocusKeydown,
    type ModalEvent,
  } from "@inflatable-cookie/poodle-core";
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
    initialFocus?: "auto" | "none" | string;
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
    initialFocus = "auto",
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
  // Handle for the deferred close-edge focus restore, so it can be cancelled.
  let pendingRestore: ReturnType<typeof setTimeout> | null = null;

  function cancelPendingRestore(): void {
    if (pendingRestore !== null) {
      clearTimeout(pendingRestore);
      pendingRestore = null;
    }
  }
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
      // A restore queued by the previous close must not land after a reopen.
      cancelPendingRestore();
      lastFocusedElement = document.activeElement as HTMLElement | null;
      tick().then(() => {
        // Already-focused guard (b1a4a5e7): never steal focus when something
        // inside the surface is already focused (e.g. TextInput `autofocus` or
        // a consumer effect). Runs before any initialFocus resolution.
        if (!surfaceElement || surfaceElement.contains(document.activeElement)) {
          return;
        }
        focusInitialElement();
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

      // Defer the focus restore one macrotask: a pending keyboard event
      // (e.g. the Enter keyup that just submitted the dialog) must dispatch
      // before the trigger regains focus, or it re-activates the trigger
      // button and reopens the dialog.
      //
      // The handle is kept so the restore can be cancelled. Without that it
      // outlives the Dialog: it fires after unmount, and it overwrites focus
      // the application placed deliberately in the same macrotask.
      const target = lastFocusedElement;
      lastFocusedElement = null;
      if (target !== null) {
        cancelPendingRestore();
        pendingRestore = setTimeout(() => {
          pendingRestore = null;
          // Only restore into a focus vacuum. The deferral exists to stop
          // focus falling to `body` when the surface goes away — not to win a
          // race against an application that has already placed focus. If
          // something outside the closing surface holds focus, the restore is
          // abandoned.
          const active = document.activeElement;
          const vacuum =
            active === null ||
            active === document.body ||
            (surfaceElement?.contains(active) ?? false);
          if (vacuum) {
            target.focus();
          }
        }, 0);
      }
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

  /**
   * Resolve where focus lands on the open edge, per the `initialFocus` prop.
   * The already-focused guard has already run (see the open-edge $effect), so
   * the active element is outside the surface here.
   *
   * - "none": focus nothing; the surface still traps focus.
   * - a CSS selector string: resolved within the surface; an unmatched
   *   selector falls back to "auto" behaviour rather than throwing.
   * - "auto" (default): first focusable in the content region
   *   (`.poodle-dialog__body`), skipping header chrome such as the close
   *   button; the surface itself when the body has no focusable element (and
   *   always in `bare` mode, where no body region exists).
   */
  function focusInitialElement(): void {
    const surface = surfaceElement;
    if (!surface || initialFocus === "none") {
      return;
    }

    if (initialFocus !== "auto") {
      const target = surface.querySelector<HTMLElement>(initialFocus);
      if (target) {
        target.focus();
        return;
      }
    }

    const body = surface.querySelector<HTMLElement>(".poodle-dialog__body");
    const focusable = getFocusableElements(body ?? surface);
    const target = focusable[0] ?? surface;
    target.focus();
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
    cancelPendingRestore();
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

