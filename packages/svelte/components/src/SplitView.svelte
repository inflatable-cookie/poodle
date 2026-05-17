<script lang="ts">
  import type { Snippet } from "svelte";

  import { default as CollapseToggle } from "./CollapseToggle.svelte";
  import { default as ResizeHandle } from "./ResizeHandle.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    CollapseDirection,
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    SplitOrientation,
  } from "./types";

  interface Props {
    orientation?: SplitOrientation;
    ratio?: number | undefined;
    defaultRatio?: number;
    minPrimarySize?: number | null;
    minSecondarySize?: number | null;
    primarySize?: number | null;
    secondarySize?: number | null;
    primaryCollapsed?: boolean | undefined;
    secondaryCollapsed?: boolean | undefined;
    showCollapsePrimary?: boolean;
    showCollapseSecondary?: boolean;
    ariaLabel?: string | null;
    disabled?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onRatioChange?: ((ratio: number) => void) | null;
    onPrimaryCollapsedChange?: ((isCollapsed: boolean) => void) | null;
    onSecondaryCollapsedChange?: ((isCollapsed: boolean) => void) | null;
    primary?: Snippet<[]>;
    secondary?: Snippet<[]>;
  }

  let {
    orientation = "horizontal",
    ratio = $bindable<number | undefined>(undefined),
    defaultRatio = 0.5,
    minPrimarySize = null,
    minSecondarySize = null,
    primarySize = null,
    secondarySize = null,
    primaryCollapsed = $bindable<boolean | undefined>(undefined),
    secondaryCollapsed = $bindable<boolean | undefined>(undefined),
    showCollapsePrimary = false,
    showCollapseSecondary = false,
    ariaLabel = null,
    disabled = false,
    size = null,
    sizeRole = "chrome",
    density = null,
    onRatioChange = null,
    onPrimaryCollapsedChange = null,
    onSecondaryCollapsedChange = null,
    primary,
    secondary,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let container = $state<HTMLDivElement | null>(null);
  let uncontrolledRatio = $state(0.5);
  let uncontrolledPrimaryCollapsed = $state(false);
  let uncontrolledSecondaryCollapsed = $state(false);
  let dragMousePos = $state(0);
  let seededDefaultRatio = $state(false);

  $effect.pre(() => {
    if (!seededDefaultRatio && ratio === undefined) {
      uncontrolledRatio = defaultRatio;
      seededDefaultRatio = true;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasControlledRatio = $derived(ratio !== undefined);
  const hasControlledPrimaryCollapsed = $derived(primaryCollapsed !== undefined);
  const hasControlledSecondaryCollapsed = $derived(secondaryCollapsed !== undefined);
  const currentRatio = $derived(
    Math.min(0.95, Math.max(0.05, hasControlledRatio ? (ratio ?? defaultRatio) : uncontrolledRatio)),
  );
  const isPrimaryCollapsed = $derived(
    hasControlledPrimaryCollapsed ? primaryCollapsed === true : uncontrolledPrimaryCollapsed,
  );
  const isSecondaryCollapsed = $derived(
    hasControlledSecondaryCollapsed ? secondaryCollapsed === true : uncontrolledSecondaryCollapsed,
  );

  const primaryFlex = $derived(
    isPrimaryCollapsed
      ? "0 0 0"
      : primarySize != null
        ? `0 0 ${primarySize}px`
        : secondarySize != null
          ? "1 1 0"
          : isSecondaryCollapsed
            ? "1 1 0"
            : `0 0 ${currentRatio * 100}%`,
  );
  const secondaryFlex = $derived(
    isSecondaryCollapsed
      ? "0 0 0"
      : secondarySize != null
        ? `0 0 ${secondarySize}px`
        : isPrimaryCollapsed || primarySize != null
          ? "1 1 0"
          : "1 1 0",
  );
  const primaryMinStyle = $derived(
    minPrimarySize != null && !isPrimaryCollapsed
      ? `min-${orientation === "horizontal" ? "width" : "height"}: ${minPrimarySize}px`
      : "",
  );
  const secondaryMinStyle = $derived(
    minSecondarySize != null && !isSecondaryCollapsed
      ? `min-${orientation === "horizontal" ? "width" : "height"}: ${minSecondarySize}px`
      : "",
  );
  const hasToggles = $derived(showCollapsePrimary || showCollapseSecondary);
  const beforeDirection = $derived((orientation === "horizontal" ? "left" : "up") as CollapseDirection);
  const afterDirection = $derived((orientation === "horizontal" ? "right" : "down") as CollapseDirection);

  function setRatio(nextRatio: number): void {
    const clamped = Math.min(0.95, Math.max(0.05, nextRatio));
    if (hasControlledRatio) {
      ratio = clamped;
    } else {
      uncontrolledRatio = clamped;
    }

    onRatioChange?.(clamped);
  }

  function setPrimaryCollapsed(nextCollapsed: boolean): void {
    if (hasControlledPrimaryCollapsed) {
      primaryCollapsed = nextCollapsed;
    } else {
      uncontrolledPrimaryCollapsed = nextCollapsed;
    }

    onPrimaryCollapsedChange?.(nextCollapsed);
  }

  function setSecondaryCollapsed(nextCollapsed: boolean): void {
    if (hasControlledSecondaryCollapsed) {
      secondaryCollapsed = nextCollapsed;
    } else {
      uncontrolledSecondaryCollapsed = nextCollapsed;
    }

    onSecondaryCollapsedChange?.(nextCollapsed);
  }

  function rawRatio(mousePos: number): number {
    if (!container) return currentRatio;
    const rect = container.getBoundingClientRect();
    const start = orientation === "horizontal" ? rect.left : rect.top;
    const total = orientation === "horizontal" ? rect.width : rect.height;
    if (total <= 0) return currentRatio;
    return (mousePos - start) / total;
  }

  function handleResizeStart(position: number): void {
    dragMousePos = position;

    if (isPrimaryCollapsed) {
      setRatio(0.05);
      setPrimaryCollapsed(false);
    }
    if (isSecondaryCollapsed) {
      setRatio(0.95);
      setSecondaryCollapsed(false);
    }
  }

  function handleResizeMove(delta: number): void {
    if (!container) return;
    dragMousePos += delta;
    const raw = rawRatio(dragMousePos);

    if (raw <= 0.02) {
      if (!isPrimaryCollapsed) {
        setPrimaryCollapsed(true);
        setRatio(0.5);
      }
      return;
    }

    if (raw >= 0.98) {
      if (!isSecondaryCollapsed) {
        setSecondaryCollapsed(true);
        setRatio(0.5);
      }
      return;
    }

    if (isPrimaryCollapsed) {
      setPrimaryCollapsed(false);
    }
    if (isSecondaryCollapsed) {
      setSecondaryCollapsed(false);
    }
    setRatio(raw);
  }

  function handleResizeStep(delta: number): void {
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const total = orientation === "horizontal" ? rect.width : rect.height;
    if (total <= 0) return;
    setRatio(currentRatio + delta / total);
  }

  function toggleCollapsePrimary(nextCollapsed: boolean): void {
    setPrimaryCollapsed(nextCollapsed);
  }

  function toggleCollapseSecondary(nextCollapsed: boolean): void {
    setSecondaryCollapsed(nextCollapsed);
  }
</script>

<div
  class="poodle-split-view"
  data-orientation={orientation}
  data-primary-collapsed={isPrimaryCollapsed || undefined}
  data-secondary-collapsed={isSecondaryCollapsed || undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  aria-label={ariaLabel ?? "Split view"}
  bind:this={container}
>
  <div
    class="poodle-split-view__pane poodle-split-view__pane--primary"
    style="flex: {primaryFlex}; overflow: hidden; {primaryMinStyle}"
  >
    {#if !isPrimaryCollapsed}
      {@render primary?.()}
    {/if}
  </div>

  <div
    class="poodle-split-view__divider"
    data-orientation={orientation}
    data-disabled={disabled || undefined}
    data-has-toggles={hasToggles || undefined}
  >
    <ResizeHandle
      {orientation}
      {disabled}
      ariaLabel="Resize"
      onResizeStart={handleResizeStart}
      onResizeMove={handleResizeMove}
      onResizeStep={handleResizeStep}
    />

    {#if hasToggles}
      <div class="poodle-split-view__toggles">
        {#if showCollapsePrimary && !isSecondaryCollapsed}
          <CollapseToggle
            direction={beforeDirection}
            collapsed={isPrimaryCollapsed}
            {disabled}
            ariaLabel={isPrimaryCollapsed ? "Expand primary" : "Collapse primary"}
            onToggle={toggleCollapsePrimary}
          />
        {/if}
        {#if showCollapseSecondary && !isPrimaryCollapsed}
          <CollapseToggle
            direction={afterDirection}
            collapsed={isSecondaryCollapsed}
            {disabled}
            ariaLabel={isSecondaryCollapsed ? "Expand secondary" : "Collapse secondary"}
            onToggle={toggleCollapseSecondary}
          />
        {/if}
      </div>
    {/if}
  </div>

  <div
    class="poodle-split-view__pane poodle-split-view__pane--secondary"
    style="flex: {secondaryFlex}; overflow: hidden; {secondaryMinStyle}"
  >
    {#if !isSecondaryCollapsed}
      {@render secondary?.()}
    {/if}
  </div>
</div>

<style>
  .poodle-split-view {
    display: flex;
    min-height: 0;
    min-width: 0;
    height: 100%;
    width: 100%;
  }

  .poodle-split-view[data-orientation="vertical"] {
    flex-direction: column;
  }

  .poodle-split-view__pane {
    min-width: 0;
    min-height: 0;
  }

  .poodle-split-view__divider {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .poodle-split-view__divider[data-orientation="horizontal"] {
    width: 0.5rem;
    height: 100%;
  }

  .poodle-split-view__divider[data-orientation="vertical"] {
    width: 100%;
    height: 0.5rem;
  }

  .poodle-split-view__toggles {
    position: absolute;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.125rem;
    padding: 0.125rem;
    border-radius: var(--poodle-radius-pill);
    background: color-mix(
      in srgb,
      var(--poodle-color-background-panel) 92%,
      var(--poodle-color-background-elevated)
    );
    box-shadow: 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-default) 70%, transparent);
  }

  .poodle-split-view__divider[data-orientation="horizontal"] .poodle-split-view__toggles {
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    flex-direction: column;
  }

  .poodle-split-view__divider[data-orientation="vertical"] .poodle-split-view__toggles {
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    flex-direction: row;
  }

  .poodle-split-view__divider[data-disabled] .poodle-split-view__toggles {
    opacity: var(--poodle-state-opacity-disabled);
  }
</style>
