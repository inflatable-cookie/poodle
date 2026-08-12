<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/menu.css";
  import {
    layerContains,
    menuTransition,
    registerDismissLayer,
    type MenuEvent as MenuMachineEvent,
    type OverlaySurfaceGeometryChangeHandler,
  } from "@inflatable-cookie/poodle-core";
  import { tick, type Snippet } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import MenuSurface from "./MenuSurface.svelte";

  import type {
    ControlDensity,
    ControlSize,
    MenuItem,
    OverlayPlacement,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    items?: MenuItem[];
    open?: boolean | null;
    defaultOpen?: boolean;
    placement?: OverlayPlacement;
    ariaLabel?: string | null;
    triggerAriaLabel?: string | null;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    dismissOnOutsideInteract?: boolean;
    onOpenChange?: ((open: boolean) => void) | undefined;
    onAction?: ((value: string) => void) | undefined;
    onSurfaceGeometryChange?: OverlaySurfaceGeometryChangeHandler | undefined;
    trigger?: Snippet<[]>;
  }

  let {
    items = [],
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    placement = "bottom-start",
    ariaLabel = null,
    triggerAriaLabel = null,
    sizeRole = "chrome",
    size = null,
    density = null,
    dismissOnOutsideInteract = true,
    onOpenChange = undefined,
    onAction = undefined,
    onSurfaceGeometryChange = undefined,
    trigger,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let rootElement = $state<HTMLDivElement | null>(null);
  let triggerElement = $state<HTMLDivElement | null>(null);
  let overlayElement = $state<HTMLDivElement | null>(null);
  let surface = $state<{ focusFirstItem: () => void } | null>(null);
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);

  $effect.pre(() => {
    if (!seededDefaultOpen) {
      uncontrolledOpen = defaultOpen;
      seededDefaultOpen = true;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(open !== null);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);

  $effect(() => {
    if (!isOpen) {
      return;
    }

    tick().then(() => {
      surface?.focusFirstItem();
    });
  });

  function send(event: MenuMachineEvent): void {
    const result = menuTransition(isOpen ? "open" : "closed", {}, event);

    for (const effect of result.effects) {
      if (effect.type === "emitOpenChange") {
        if (!isControlled) {
          uncontrolledOpen = effect.open;
        } else {
          // Write back through the binding before notifying, so `bind:open`
          // works as it does on every other bindable Poodle component. A host
          // that wants to refuse the close re-asserts the value inside
          // `onOpenChange`, which lands last and renders no intermediate
          // state — covered by DialogControlled.svelte.test.ts.
          open = effect.open;
        }

        onOpenChange?.(effect.open);
      } else if (effect.type === "emitAction") {
        onAction?.(effect.value);
      }
      // focusFirstItem intent runs in the isOpen $effect above, after the
      // surface has rendered and been positioned.
    }
  }

  function handleTriggerClick(event: MouseEvent): void {
    event.preventDefault();
    event.stopPropagation();
    send({ type: "TOGGLE" });
  }

  function handleTriggerKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter" || event.key === " " || event.key === "ArrowDown") {
      event.preventDefault();
      event.stopPropagation();
      send({ type: "OPEN" });
    }
  }

  $effect(() => {
    if (!isOpen) {
      return;
    }

    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, overlayElement),
      dismissOnOutsideInteract,
      onDismiss: (reason) => send(reason === "escape" ? { type: "ESCAPE" } : { type: "OUTSIDE_INTERACT" }),
    });
  });

</script>

<div class="poodle-menu" bind:this={rootElement} data-size={resolvedSize} data-density={resolvedDensity}>
  <div
    bind:this={triggerElement}
    class="poodle-menu__trigger"
    role="button"
    tabindex="0"
    aria-expanded={isOpen ? "true" : "false"}
    aria-label={triggerAriaLabel ?? undefined}
    onclick={handleTriggerClick}
    onkeydown={handleTriggerKeydown}
  >
    {@render trigger?.()}
  </div>

  {#if isOpen}
    <MenuSurface
      bind:this={surface}
      bind:overlayElement
      items={items}
      ariaLabel={ariaLabel}
      size={resolvedSize}
      density={resolvedDensity}
      anchor={triggerElement}
      placement={placement}
      {onSurfaceGeometryChange}
      onAction={(value) => send({ type: "ACTION", value })}
    />
  {/if}
</div>
