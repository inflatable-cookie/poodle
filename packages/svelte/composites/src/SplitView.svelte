<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";

  import { CollapseToggle } from "@poodle/svelte-primitives";
  import type { CollapseDirection } from "@poodle/svelte-primitives";
  import { ResizeHandle } from "@poodle/svelte-primitives";
  import type { SplitOrientation } from "@poodle/svelte-primitives";

  export let orientation: SplitOrientation = "horizontal";
  export let ratio = 0.5;
  export let defaultRatio = 0.5;
  export let minPrimarySize: number | null = null;
  export let minSecondarySize: number | null = null;
  export let primarySize: number | null = null;
  export let secondarySize: number | null = null;
  export let primaryCollapsed = false;
  export let secondaryCollapsed = false;
  export let showCollapsePrimary = false;
  export let showCollapseSecondary = false;
  export let ariaLabel: string | null = null;
  export let disabled = false;

  const dispatch = createEventDispatcher<{
    ratioChange: { ratio: number };
    primaryCollapsedChange: { isCollapsed: boolean };
    secondaryCollapsedChange: { isCollapsed: boolean };
  }>();

  let container: HTMLDivElement | null = null;
  let uncontrolledRatio = defaultRatio;
  let dragMousePos = 0;

  // ── Derived ──────────────────────────────────────────────────────

  $: currentRatio = Math.min(0.95, Math.max(0.05, ratio ?? uncontrolledRatio));

  $: primaryFlex = primaryCollapsed
    ? "0 0 0"
    : primarySize != null
      ? `0 0 ${primarySize}px`
    : secondarySize != null
      ? "1 1 0"
    : secondaryCollapsed
      ? "1 1 0"
      : `0 0 ${currentRatio * 100}%`;

  $: secondaryFlex = secondaryCollapsed
    ? "0 0 0"
    : secondarySize != null
      ? `0 0 ${secondarySize}px`
    : primaryCollapsed || primarySize != null
      ? "1 1 0"
      : "1 1 0";

  $: primaryMinStyle = minPrimarySize != null && !primaryCollapsed
    ? `min-${orientation === "horizontal" ? "width" : "height"}: ${minPrimarySize}px`
    : "";

  $: secondaryMinStyle = minSecondarySize != null && !secondaryCollapsed
    ? `min-${orientation === "horizontal" ? "width" : "height"}: ${minSecondarySize}px`
    : "";

  $: hasToggles = showCollapsePrimary || showCollapseSecondary;
  $: beforeDirection = (orientation === "horizontal" ? "left" : "up") as CollapseDirection;
  $: afterDirection = (orientation === "horizontal" ? "right" : "down") as CollapseDirection;

  // ── Ratio management ─────────────────────────────────────────────

  function setRatio(nextRatio: number): void {
    const clamped = Math.min(0.95, Math.max(0.05, nextRatio));
    uncontrolledRatio = clamped;
    ratio = clamped;
    dispatch("ratioChange", { ratio: clamped });
  }

  // ── Resize via ResizeHandle events ──────────────────────────────

  function rawRatio(mousePos: number): number {
    if (!container) return currentRatio;
    const rect = container.getBoundingClientRect();
    const start = orientation === "horizontal" ? rect.left : rect.top;
    const total = orientation === "horizontal" ? rect.width : rect.height;
    if (total <= 0) return currentRatio;
    return (mousePos - start) / total;
  }

  function handleResizeStart(e: CustomEvent<{ position: number }>): void {
    dragMousePos = e.detail.position;

    // Uncollapse on drag start from collapsed position
    if (primaryCollapsed) {
      setRatio(0.05);
      primaryCollapsed = false;
      dispatch("primaryCollapsedChange", { isCollapsed: false });
    }
    if (secondaryCollapsed) {
      setRatio(0.95);
      secondaryCollapsed = false;
      dispatch("secondaryCollapsedChange", { isCollapsed: false });
    }
  }

  function handleResizeMove(e: CustomEvent<{ delta: number }>): void {
    if (!container) return;
    dragMousePos += e.detail.delta;
    const raw = rawRatio(dragMousePos);

    if (raw <= 0.02) {
      if (!primaryCollapsed) {
        primaryCollapsed = true;
        setRatio(0.5);
        dispatch("primaryCollapsedChange", { isCollapsed: true });
      }
    } else if (raw >= 0.98) {
      if (!secondaryCollapsed) {
        secondaryCollapsed = true;
        setRatio(0.5);
        dispatch("secondaryCollapsedChange", { isCollapsed: true });
      }
    } else {
      if (primaryCollapsed) {
        primaryCollapsed = false;
        dispatch("primaryCollapsedChange", { isCollapsed: false });
      }
      if (secondaryCollapsed) {
        secondaryCollapsed = false;
        dispatch("secondaryCollapsedChange", { isCollapsed: false });
      }
      setRatio(raw);
    }
  }

  function handleResizeStep(e: CustomEvent<{ delta: number }>): void {
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const total = orientation === "horizontal" ? rect.width : rect.height;
    if (total <= 0) return;
    setRatio(currentRatio + e.detail.delta / total);
  }

  // ── Collapse toggles ─────────────────────────────────────────────

  function toggleCollapsePrimary(e: CustomEvent<{ isCollapsed: boolean }>): void {
    primaryCollapsed = e.detail.isCollapsed;
    dispatch("primaryCollapsedChange", { isCollapsed: primaryCollapsed });
  }

  function toggleCollapseSecondary(e: CustomEvent<{ isCollapsed: boolean }>): void {
    secondaryCollapsed = e.detail.isCollapsed;
    dispatch("secondaryCollapsedChange", { isCollapsed: secondaryCollapsed });
  }
</script>

<div
  class="split-view"
  data-orientation={orientation}
  data-primary-collapsed={primaryCollapsed || undefined}
  data-secondary-collapsed={secondaryCollapsed || undefined}
  aria-label={ariaLabel ?? "Split view"}
  bind:this={container}
>
  <div
    class="split-view__pane split-view__pane--primary"
    style="flex: {primaryFlex}; overflow: hidden; {primaryMinStyle}"
  >
    {#if !primaryCollapsed}
      <slot name="primary" />
    {/if}
  </div>

  <div
    class="split-view__divider"
    data-orientation={orientation}
    data-disabled={disabled || undefined}
    data-has-toggles={hasToggles || undefined}
  >
    <ResizeHandle
      {orientation}
      disabled={disabled}
      ariaLabel="Resize"
      on:resizeStart={handleResizeStart}
      on:resizeMove={handleResizeMove}
      on:resizeStep={handleResizeStep}
    />

    {#if hasToggles}
      <div class="split-view__toggles">
        {#if showCollapsePrimary && !secondaryCollapsed}
          <CollapseToggle
            direction={beforeDirection}
            collapsed={primaryCollapsed}
            disabled={disabled}
            ariaLabel={primaryCollapsed ? "Expand primary" : "Collapse primary"}
            on:toggle={toggleCollapsePrimary}
          />
        {/if}
        {#if showCollapseSecondary && !primaryCollapsed}
          <CollapseToggle
            direction={afterDirection}
            collapsed={secondaryCollapsed}
            disabled={disabled}
            ariaLabel={secondaryCollapsed ? "Expand secondary" : "Collapse secondary"}
            on:toggle={toggleCollapseSecondary}
          />
        {/if}
      </div>
    {/if}
  </div>

  <div
    class="split-view__pane split-view__pane--secondary"
    style="flex: {secondaryFlex}; overflow: hidden; {secondaryMinStyle}"
  >
    {#if !secondaryCollapsed}
      <slot name="secondary" />
    {/if}
  </div>
</div>

<style>
  .split-view {
    display: flex;
    min-height: 0;
    min-width: 0;
    height: 100%;
    width: 100%;
  }

  .split-view[data-orientation="vertical"] {
    flex-direction: column;
  }

  .split-view__pane {
    min-width: 0;
    min-height: 0;
  }

  /* ── Divider ─────────────────────────────────────────────────── */

  .split-view__divider {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .split-view__divider[data-orientation="horizontal"] {
    width: 0.5rem;
    height: 100%;
  }

  .split-view__divider[data-orientation="vertical"] {
    height: 0.5rem;
    width: 100%;
  }

  /* ── Collapse toggles overlay ────────────────────────────────── */

  .split-view__toggles {
    position: absolute;
    z-index: 1;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    pointer-events: none;
  }

  .split-view__toggles :global(*) {
    pointer-events: auto;
  }

  .split-view__divider[data-orientation="horizontal"] .split-view__toggles {
    flex-direction: column;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .split-view__divider[data-orientation="vertical"] .split-view__toggles {
    flex-direction: row;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
</style>
