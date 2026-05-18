<script lang="ts">
  import { onMount, tick, type Snippet } from "svelte";

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

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    onOpenChange?.(nextOpen);
  }

  function handleContextMenu(event: MouseEvent): void {
    event.preventDefault();
    uncontrolledAnchorPoint = { x: event.clientX, y: event.clientY };
    setOpen(true);
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
    setOpen(true);
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!isOpen || !rootElement) {
        return;
      }

      if (!overlayElement || !overlayElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && isOpen) {
        event.preventDefault();
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
    };
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
      onAction={(value) => {
        onAction?.(value);
        setOpen(false);
      }}
    />
  {/if}
</div>
