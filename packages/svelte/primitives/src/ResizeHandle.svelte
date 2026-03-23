<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";

  import type { SplitOrientation } from "./types";

  export let orientation: SplitOrientation = "horizontal";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let ariaValueNow: number | null = null;
  export let ariaValueMin = 0;
  export let ariaValueMax = 100;

  const dispatch = createEventDispatcher<{
    resizeStart: { position: number };
    resizeMove: { delta: number };
    resizeEnd: { position: number };
    resizeStep: { delta: number };
  }>();

  let isDragging = false;
  let lastPosition = 0;
  let isListening = false;

  function handlePointerDown(event: MouseEvent): void {
    if (isDisabled) return;
    event.preventDefault();
    isDragging = true;
    lastPosition = orientation === "horizontal" ? event.clientX : event.clientY;
    dispatch("resizeStart", { position: lastPosition });
    startListening();
  }

  function handlePointerMove(event: MouseEvent): void {
    if (!isDragging) return;
    const pos = orientation === "horizontal" ? event.clientX : event.clientY;
    const delta = pos - lastPosition;
    lastPosition = pos;
    dispatch("resizeMove", { delta });
  }

  function handlePointerUp(): void {
    if (!isDragging) return;
    isDragging = false;
    dispatch("resizeEnd", { position: lastPosition });
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
    if (isDisabled) return;
    const step = 8;
    const prevKey = orientation === "horizontal" ? "ArrowLeft" : "ArrowUp";
    const nextKey = orientation === "horizontal" ? "ArrowRight" : "ArrowDown";

    if (event.key === prevKey) {
      event.preventDefault();
      dispatch("resizeStep", { delta: -step });
    } else if (event.key === nextKey) {
      event.preventDefault();
      dispatch("resizeStep", { delta: step });
    } else if (event.key === "Home") {
      event.preventDefault();
      dispatch("resizeStep", { delta: -9999 });
    } else if (event.key === "End") {
      event.preventDefault();
      dispatch("resizeStep", { delta: 9999 });
    }
  }

  onDestroy(() => {
    stopListening();
  });
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex a11y_no_noninteractive_element_interactions -->
<div
  class="resize-handle"
  data-orientation={orientation}
  data-disabled={isDisabled || undefined}
  data-dragging={isDragging || undefined}
  role="separator"
  aria-orientation={orientation}
  aria-label={ariaLabel ?? "Resize"}
  aria-valuenow={ariaValueNow}
  aria-valuemin={ariaValueMin}
  aria-valuemax={ariaValueMax}
  tabindex={isDisabled ? -1 : 0}
  on:mousedown={handlePointerDown}
  on:keydown={handleKeydown}
>
  <span class="resize-handle__line" aria-hidden="true"></span>
</div>

<style>
  .resize-handle {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .resize-handle[data-orientation="horizontal"] {
    width: 0.5rem;
    height: 100%;
    cursor: col-resize;
  }

  .resize-handle[data-orientation="vertical"] {
    width: 100%;
    height: 0.5rem;
    cursor: row-resize;
  }

  .resize-handle[data-disabled] {
    cursor: default;
    opacity: 0.4;
  }

  .resize-handle__line {
    position: absolute;
    border-radius: 999rem;
    background: color-mix(in srgb, var(--poodle-color-border-default) 82%, transparent);
    transition: background 120ms ease;
  }

  .resize-handle[data-orientation="horizontal"] .resize-handle__line {
    width: 0.125rem;
    height: 100%;
  }

  .resize-handle[data-orientation="vertical"] .resize-handle__line {
    width: 100%;
    height: 0.125rem;
  }

  .resize-handle:hover .resize-handle__line,
  .resize-handle[data-dragging] .resize-handle__line {
    background: var(--poodle-color-accent-base);
  }

  .resize-handle:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }
</style>
