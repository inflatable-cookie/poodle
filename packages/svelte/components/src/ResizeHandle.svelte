<script lang="ts">
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
  let lastPosition = $state(0);
  let isListening = $state(false);

  function handlePointerDown(event: MouseEvent): void {
    if (disabled) return;
    event.preventDefault();
    isDragging = true;
    lastPosition = orientation === "horizontal" ? event.clientX : event.clientY;
    onResizeStart?.(lastPosition);
    startListening();
  }

  function handlePointerMove(event: MouseEvent): void {
    if (!isDragging) return;
    const pos = orientation === "horizontal" ? event.clientX : event.clientY;
    const delta = pos - lastPosition;
    lastPosition = pos;
    onResizeMove?.(delta);
  }

  function handlePointerUp(): void {
    if (!isDragging) return;
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
    isListening = false;
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (disabled) return;
    const step = 8;
    const prevKey = orientation === "horizontal" ? "ArrowLeft" : "ArrowUp";
    const nextKey = orientation === "horizontal" ? "ArrowRight" : "ArrowDown";

    if (event.key === prevKey) {
      event.preventDefault();
      onResizeStep?.(-step);
    } else if (event.key === nextKey) {
      event.preventDefault();
      onResizeStep?.(step);
    } else if (event.key === "Home") {
      event.preventDefault();
      onResizeStep?.(-9999);
    } else if (event.key === "End") {
      event.preventDefault();
      onResizeStep?.(9999);
    }
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
  aria-valuenow={ariaValueNow}
  aria-valuemin={ariaValueMin}
  aria-valuemax={ariaValueMax}
  tabindex={disabled ? -1 : 0}
  onmousedown={handlePointerDown}
  onkeydown={handleKeydown}
>
  <span class="poodle-resize-handle__line" aria-hidden="true"></span>
</div>

<style>
  .poodle-resize-handle {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .poodle-resize-handle[data-orientation="horizontal"] {
    width: 0.5rem;
    height: 100%;
    cursor: col-resize;
  }

  .poodle-resize-handle[data-orientation="vertical"] {
    width: 100%;
    height: 0.5rem;
    cursor: row-resize;
  }

  .poodle-resize-handle[data-disabled] {
    cursor: default;
    opacity: 0.4;
  }

  .poodle-resize-handle__line {
    position: absolute;
    border-radius: 999rem;
    background: color-mix(in srgb, var(--poodle-color-border-default) 82%, transparent);
    transition: background 120ms ease;
  }

  .poodle-resize-handle[data-orientation="horizontal"] .poodle-resize-handle__line {
    width: 0.125rem;
    height: 100%;
  }

  .poodle-resize-handle[data-orientation="vertical"] .poodle-resize-handle__line {
    width: 100%;
    height: 0.125rem;
  }

  .poodle-resize-handle:not([data-disabled]):hover .poodle-resize-handle__line,
  .poodle-resize-handle[data-dragging] .poodle-resize-handle__line {
    background: var(--poodle-color-accent-base);
  }

  .poodle-resize-handle:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }
</style>
