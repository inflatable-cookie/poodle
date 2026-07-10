<script lang="ts">
  import {
    menuTransition,
    registerDismissLayer,
    type MenuEvent as MenuMachineEvent,
  } from "@poodle/headless";
  import { tick, type Snippet } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import MenuSurface from "./MenuSurface.svelte";

  import type { ControlDensity, ControlSize, MenuItem, SemanticControlSizeRole } from "./types";

  interface Props {
    items?: MenuItem[];
    open?: boolean | null;
    defaultOpen?: boolean;
    anchorPoint?: { x: number; y: number } | null;
    ariaLabel?: string | null;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
    onAction?: ((value: string) => void) | undefined;
    children?: Snippet<[]>;
  }

  let {
    items = [],
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    anchorPoint = null,
    ariaLabel = null,
    sizeRole = "chrome",
    size = null,
    density = null,
    onOpenChange = undefined,
    onAction = undefined,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let rootElement = $state<HTMLDivElement | null>(null);
  let overlayElement = $state<HTMLDivElement | null>(null);
  let surface = $state<{ focusFirstItem: () => void } | null>(null);
  let uncontrolledOpen = $state(false);
  let uncontrolledAnchorPoint = $state<{ x: number; y: number } | null>(null);
  let seededDefaults = $state(false);
  let adjustedPosition = $state<{ left: string; top: string } | null>(null);

  $effect.pre(() => {
    if (!seededDefaults) {
      uncontrolledOpen = defaultOpen;
      uncontrolledAnchorPoint = anchorPoint;
      seededDefaults = true;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(open !== null);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);
  const currentAnchorPoint = $derived(anchorPoint ?? uncontrolledAnchorPoint);
  const overlayStyle = $derived(
    adjustedPosition
      ? `left: ${adjustedPosition.left}; top: ${adjustedPosition.top};`
      : currentAnchorPoint
        ? `left: ${currentAnchorPoint.x}px; top: ${currentAnchorPoint.y}px; visibility: hidden;`
        : "",
  );

  $effect(() => {
    if (!isOpen) {
      return;
    }

    adjustedPosition = null;
    tick().then(() => {
      if (overlayElement && currentAnchorPoint) {
        const rect = overlayElement.getBoundingClientRect();
        const vw = window.innerWidth;
        const vh = window.innerHeight;
        const pad = 8;
        let x = currentAnchorPoint.x;
        let y = currentAnchorPoint.y;

        if (x + rect.width > vw - pad) {
          x = Math.max(pad, x - rect.width);
        }

        if (y + rect.height > vh - pad) {
          y = Math.max(pad, vh - rect.height - pad);
        }

        adjustedPosition = { left: `${x}px`, top: `${y}px` };
      }

      surface?.focusFirstItem();
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
      // focusFirstItem intent runs in the isOpen $effect above.
    }
  }

  function handleContextMenu(event: MouseEvent): void {
    event.preventDefault();
    uncontrolledAnchorPoint = { x: event.clientX, y: event.clientY };
    send({ type: "OPEN" });
  }

  function handleTriggerKeydown(event: KeyboardEvent): void {
    if (event.key !== "ContextMenu" && !(event.shiftKey && event.key === "F10")) {
      return;
    }

    event.preventDefault();
    const target = event.currentTarget;
    if (!(target instanceof HTMLElement)) {
      return;
    }

    const rect = target.getBoundingClientRect();
    uncontrolledAnchorPoint = { x: rect.left + 16, y: rect.top + 16 };
    send({ type: "OPEN" });
  }

  $effect(() => {
    if (!isOpen) {
      return;
    }

    return registerDismissLayer({
      // Only the overlay itself counts as inside; clicking the trigger area
      // closes, matching the previous document-listener behavior.
      contains: (target) => overlayElement?.contains(target) ?? false,
      dismissOnOutsideInteract: true,
      onDismiss: (reason) => send(reason === "escape" ? { type: "ESCAPE" } : { type: "OUTSIDE_INTERACT" }),
    });
  });
</script>

<div
  class="poodle-context-menu"
  bind:this={rootElement}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  role="button"
  tabindex="0"
  aria-haspopup="menu"
  oncontextmenu={handleContextMenu}
  onkeydown={handleTriggerKeydown}
>
  {@render children?.()}

  {#if isOpen && currentAnchorPoint}
    <MenuSurface
      bind:this={surface}
      bind:overlayElement
      items={items}
      ariaLabel={ariaLabel}
      size={resolvedSize}
      density={resolvedDensity}
      overlayStyle={overlayStyle}
      onAction={(value) => send({ type: "ACTION", value })}
    />
  {/if}
</div>
