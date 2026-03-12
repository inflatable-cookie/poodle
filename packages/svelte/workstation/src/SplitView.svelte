<script lang="ts">
  import { pxToRem } from "@pug/svelte-tokens";
  import { createEventDispatcher } from "svelte";
  import { onDestroy } from "svelte";

  import type { SplitOrientation } from "./types";

  export let orientation: SplitOrientation = "horizontal";
  export let ratio = 0.5;
  export let defaultRatio = 0.5;
  export let minPrimarySize: number | null = null;
  export let minSecondarySize: number | null = null;
  export let isPrimaryCollapsed = false;
  export let isSecondaryCollapsed = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    ratioChange: { ratio: number };
  }>();

  let uncontrolledRatio = defaultRatio;
  let container: HTMLDivElement | null = null;
  let isDragging = false;
  let isListening = false;

  $: currentRatio = Math.min(0.9, Math.max(0.1, ratio ?? uncontrolledRatio));
  $: primarySize = `${Math.round(currentRatio * 1000) / 10}%`;
  $: secondarySize = `${Math.round((1 - currentRatio) * 1000) / 10}%`;
  $: primaryMinSize = pxToRem(minPrimarySize ?? 0);
  $: secondaryMinSize = pxToRem(minSecondarySize ?? 0);

  function setRatio(nextRatio: number): void {
    const clamped = Math.min(0.9, Math.max(0.1, nextRatio));
    uncontrolledRatio = clamped;
    dispatch("ratioChange", { ratio: clamped });
  }

  function startListening(): void {
    if (isListening || typeof window === "undefined") {
      return;
    }

    window.addEventListener("mousemove", handlePointerMove);
    window.addEventListener("mouseup", stopDragging);
    isListening = true;
  }

  function stopListening(): void {
    if (!isListening || typeof window === "undefined") {
      return;
    }

    window.removeEventListener("mousemove", handlePointerMove);
    window.removeEventListener("mouseup", stopDragging);
    isListening = false;
  }

  function handlePointerMove(event: MouseEvent): void {
    if (!isDragging || !container) {
      return;
    }

    const bounds = container.getBoundingClientRect();
    const size = orientation === "horizontal" ? bounds.width : bounds.height;
    const offset = orientation === "horizontal" ? event.clientX - bounds.left : event.clientY - bounds.top;
    if (size <= 0) {
      return;
    }

    setRatio(offset / size);
  }

  function stopDragging(): void {
    isDragging = false;
    stopListening();
  }

  function startDragging(event: MouseEvent): void {
    event.preventDefault();
    isDragging = true;
    startListening();
  }

  function handleKeydown(event: KeyboardEvent): void {
    const step = 0.04;

    if ((orientation === "horizontal" && event.key === "ArrowLeft") || (orientation === "vertical" && event.key === "ArrowUp")) {
      event.preventDefault();
      setRatio(currentRatio - step);
      return;
    }

    if ((orientation === "horizontal" && event.key === "ArrowRight") || (orientation === "vertical" && event.key === "ArrowDown")) {
      event.preventDefault();
      setRatio(currentRatio + step);
      return;
    }

    if (event.key === "Home") {
      event.preventDefault();
      setRatio(0.2);
      return;
    }

    if (event.key === "End") {
      event.preventDefault();
      setRatio(0.8);
    }
  }

  onDestroy(() => {
    stopListening();
  });
</script>

<div
  class="split-view"
  data-orientation={orientation}
  data-primary-collapsed={isPrimaryCollapsed}
  data-secondary-collapsed={isSecondaryCollapsed}
  aria-label={ariaLabel ?? "Split view"}
  bind:this={container}
  style={`--pug-split-primary-size: ${primarySize}; --pug-split-secondary-size: ${secondarySize}; --pug-split-primary-min: ${primaryMinSize}; --pug-split-secondary-min: ${secondaryMinSize};`}
>
  <div class="split-view__pane split-view__pane--primary">
    <slot name="primary" />
  </div>
  <button
    type="button"
    class="split-view__divider"
    aria-label={`${ariaLabel ?? "Resize split view"} (${Math.round(currentRatio * 100)}%)`}
    on:mousedown={startDragging}
    on:keydown={handleKeydown}
  >
    <span aria-hidden="true"></span>
  </button>
  <div class="split-view__pane split-view__pane--secondary">
    <slot name="secondary" />
  </div>
</div>

<style>
  .split-view {
    display: grid;
    min-height: 0;
    min-width: 0;
  }

  .split-view[data-orientation="horizontal"] {
    grid-template-columns:
      minmax(var(--pug-split-primary-min), var(--pug-split-primary-size))
      auto
      minmax(var(--pug-split-secondary-min), var(--pug-split-secondary-size));
  }

  .split-view[data-orientation="vertical"] {
    grid-template-rows:
      minmax(var(--pug-split-primary-min), var(--pug-split-primary-size))
      auto
      minmax(var(--pug-split-secondary-min), var(--pug-split-secondary-size));
  }

  .split-view__pane {
    min-width: 0;
    min-height: 0;
  }

  .split-view__divider {
    position: relative;
    padding: 0;
    border: 0;
    background: transparent;
    cursor: col-resize;
  }

  .split-view[data-orientation="vertical"] .split-view__divider {
    cursor: row-resize;
  }

  .split-view[data-orientation="horizontal"] .split-view__divider {
    width: 0.625rem;
  }

  .split-view[data-orientation="vertical"] .split-view__divider {
    height: 0.625rem;
  }

  .split-view__divider span {
    position: absolute;
    inset: 0;
    margin: auto;
    border-radius: 999rem;
    background: color-mix(in srgb, var(--pug-color-border-default) 82%, transparent);
  }

  .split-view[data-orientation="horizontal"] .split-view__divider span {
    width: 0.125rem;
    height: 100%;
  }

  .split-view[data-orientation="vertical"] .split-view__divider span {
    width: 100%;
    height: 0.125rem;
  }

  .split-view__divider:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>
