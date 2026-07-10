<script lang="ts">
  import {
    menuTransition,
    registerDismissLayer,
    type MenuEvent as MenuMachineEvent,
  } from "@poodle/headless";
  import { onMount, tick, type Snippet } from "svelte";

  import { resolveOverlayPosition } from "./overlay-position";
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
    onOpenChange?: ((open: boolean) => void) | undefined;
    onAction?: ((value: string) => void) | undefined;
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
    onOpenChange = undefined,
    onAction = undefined,
    trigger,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let rootElement = $state<HTMLDivElement | null>(null);
  let triggerElement = $state<HTMLDivElement | null>(null);
  let overlayElement = $state<HTMLDivElement | null>(null);
  let surface = $state<{ focusFirstItem: () => void } | null>(null);
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);
  let resolvedPlacement = $state<OverlayPlacement>("bottom-start");
  let overlayStyle = $state("");

  $effect.pre(() => {
    if (!seededDefaultOpen) {
      uncontrolledOpen = defaultOpen;
      resolvedPlacement = placement;
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
      void updateOverlayPosition().then(() => {
        surface?.focusFirstItem();
      });
    });
  });

  function send(event: MenuMachineEvent): void {
    const result = menuTransition(isOpen ? "open" : "closed", {}, event);

    for (const effect of result.effects) {
      if (effect.type === "emitOpenChange") {
        if (!isControlled) {
          uncontrolledOpen = effect.open;
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

  async function updateOverlayPosition(): Promise<void> {
    if (!isOpen || !triggerElement) {
      return;
    }

    await tick();

    if (!overlayElement) {
      return;
    }

    const nextPosition = resolveOverlayPosition(
      triggerElement.getBoundingClientRect(),
      overlayElement.getBoundingClientRect(),
      placement,
    );

    resolvedPlacement = nextPosition.placement;
    overlayStyle = `top: ${nextPosition.top}px; left: ${nextPosition.left}px;`;
  }

  $effect(() => {
    if (!isOpen) {
      return;
    }

    return registerDismissLayer({
      contains: (target) => rootElement?.contains(target) ?? false,
      dismissOnOutsideInteract: true,
      onDismiss: (reason) => send(reason === "escape" ? { type: "ESCAPE" } : { type: "OUTSIDE_INTERACT" }),
    });
  });

  onMount(() => {
    function handleViewportChange(): void {
      if (isOpen) {
        void updateOverlayPosition();
      }
    }

    window.addEventListener("resize", handleViewportChange);
    window.addEventListener("scroll", handleViewportChange, true);

    return () => {
      window.removeEventListener("resize", handleViewportChange);
      window.removeEventListener("scroll", handleViewportChange, true);
    };
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
      placement={resolvedPlacement}
      overlayStyle={overlayStyle}
      onAction={(value) => send({ type: "ACTION", value })}
    />
  {/if}
</div>

<style>
  .poodle-menu {
    position: relative;
    display: inline-flex;
  }

  .poodle-menu__trigger {
    display: inline-flex;
  }

  .poodle-menu__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>
