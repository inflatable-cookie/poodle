<script lang="ts">
  import "@poodle/styles/resize-handle.css";
  import { resizeAxisPosition, resizeDragDelta, resizeKeydownStep } from "@poodle/headless";
  import type { SplitOrientation } from "./types";

  let {
    orientation = "horizontal",
    disabled = false,
    ariaLabel = null,
    ariaValueNow = null,
    ariaValueMin = 0,
    ariaValueMax = 100,
    onResizeStart = null,
    onResizeMove = null,
    onResizeEnd = null,
    onResizeStep = null,
  }: {
    orientation?: SplitOrientation;
    disabled?: boolean;
    ariaLabel?: string | null;
    ariaValueNow?: number | null;
    ariaValueMin?: number;
    ariaValueMax?: number;
    onResizeStart?: ((position: number) => void) | null;
    onResizeMove?: ((delta: number) => void) | null;
    onResizeEnd?: ((position: number) => void) | null;
    onResizeStep?: ((delta: number) => void) | null;
  } = $props();

  let isDragging = $state(false);
  let lastPosition = 0;
  let pendingPosition = 0;
  let resizeFrame: number | null = null;
  let isListening = false;

  function handlePointerDown(event: MouseEvent): void {
    if (disabled) return;
    event.preventDefault();
    isDragging = true;
    lastPosition = resizeAxisPosition(orientation, event.clientX, event.clientY);
    pendingPosition = lastPosition;
    onResizeStart?.(lastPosition);
    startListening();
  }

  function handlePointerMove(event: MouseEvent): void {
    if (!isDragging) return;
    pendingPosition = resizeAxisPosition(orientation, event.clientX, event.clientY);
    if (resizeFrame !== null) return;
    resizeFrame = requestAnimationFrame(flushPointerMove);
  }

  function flushPointerMove(): void {
    if (resizeFrame !== null) {
      cancelAnimationFrame(resizeFrame);
      resizeFrame = null;
    }
    if (!isDragging || pendingPosition === lastPosition) return;
    const move = resizeDragDelta(lastPosition, pendingPosition);
    lastPosition = move.position;
    onResizeMove?.(move.delta);
  }

  function handlePointerUp(event: MouseEvent): void {
    if (!isDragging) return;
    pendingPosition = resizeAxisPosition(orientation, event.clientX, event.clientY);
    flushPointerMove();
    isDragging = false;
    onResizeEnd?.(lastPosition);
    stopListening();
  }

  function startListening(): void {
    if (isListening || typeof window === "undefined") return;
    window.addEventListener("mousemove", handlePointerMove);
    window.addEventListener("mouseup", handlePointerUp);
    isListening = true;
  }

  function stopListening(): void {
    if (!isListening || typeof window === "undefined") return;
    window.removeEventListener("mousemove", handlePointerMove);
    window.removeEventListener("mouseup", handlePointerUp);
    if (resizeFrame !== null) {
      cancelAnimationFrame(resizeFrame);
      resizeFrame = null;
    }
    isListening = false;
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (disabled) return;

    const step = resizeKeydownStep(event.key, orientation);

    if (step === null) return;

    event.preventDefault();
    onResizeStep?.(step);
  }

  $effect(() => {
    return () => {
      stopListening();
    };
  });
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  class="poodle-resize-handle"
  data-orientation={orientation}
  data-disabled={disabled || undefined}
  data-dragging={isDragging || undefined}
  role="separator"
  aria-orientation={orientation}
  aria-label={ariaLabel ?? "Resize"}
  aria-valuenow={ariaValueNow ?? ariaValueMin}
  aria-valuemin={ariaValueMin}
  aria-valuemax={ariaValueMax}
  tabindex={disabled ? -1 : 0}
  onmousedown={handlePointerDown}
  onkeydown={handleKeydown}
>
  <span class="poodle-resize-handle__hit" aria-hidden="true"></span>
  <span class="poodle-resize-handle__line" aria-hidden="true"></span>
</div>
