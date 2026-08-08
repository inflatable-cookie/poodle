<script lang="ts">
  import "@inflatable-cookie/poodle-styles/split-view.css";
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
    SplitToggleVisibility,
  } from "./types";

  interface Props {
    orientation?: SplitOrientation;
    ratio?: number | undefined;
    defaultRatio?: number;
    minRatio?: number;
    maxRatio?: number;
    minPrimarySize?: number | null;
    minSecondarySize?: number | null;
    primarySize?: number | null;
    secondarySize?: number | null;
    primaryCollapsed?: boolean | undefined;
    secondaryCollapsed?: boolean | undefined;
    /** Takes zero space without being a collapse: no toggle, no collapsed
     * data attribute. For panes that are absent rather than user-collapsed. */
    primaryHidden?: boolean;
    /** See `primaryHidden`. */
    secondaryHidden?: boolean;
    primaryCollapsedSize?: number | null;
    secondaryCollapsedSize?: number | null;
    collapsePrimaryBelowSize?: number | null;
    collapseSecondaryBelowSize?: number | null;
    showCollapsePrimary?: boolean;
    showCollapseSecondary?: boolean;
    /** When the collapse toggles are visible. `"always"` keeps the pill on
     * screen; `"hover"` reveals it only while the pointer is on the seam (the
     * resize grab strip or the pill itself) or a toggle holds focus. A
     * collapsed pane's expand toggle stays visible in either mode — hiding it
     * would leave the pane unrecoverable. */
    toggleVisibility?: SplitToggleVisibility;
    /** Show the visible divider line. Default off: the pane borders already
     * read as the separator, and the resize handle's grab area is an overlay
     * that costs no layout space either way. */
    divider?: boolean;
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
    minRatio = 0.05,
    maxRatio = 0.95,
    minPrimarySize = null,
    minSecondarySize = null,
    primarySize = null,
    secondarySize = null,
    primaryCollapsed = $bindable<boolean | undefined>(undefined),
    secondaryCollapsed = $bindable<boolean | undefined>(undefined),
    primaryHidden = false,
    secondaryHidden = false,
    primaryCollapsedSize = null,
    secondaryCollapsedSize = null,
    collapsePrimaryBelowSize = null,
    collapseSecondaryBelowSize = null,
    showCollapsePrimary = false,
    showCollapseSecondary = false,
    toggleVisibility = "always",
    divider = false,
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
  let dragMousePos = 0;
  let dragContainerStart = 0;
  let dragContainerSize = 0;
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
  const resolvedMinRatio = $derived(Math.min(0.95, Math.max(0.05, minRatio)));
  const resolvedMaxRatio = $derived(
    Math.max(resolvedMinRatio, Math.min(0.95, maxRatio)),
  );
  const currentRatio = $derived(
    Math.min(
      resolvedMaxRatio,
      Math.max(
        resolvedMinRatio,
        hasControlledRatio ? (ratio ?? defaultRatio) : uncontrolledRatio,
      ),
    ),
  );
  const isPrimaryCollapsed = $derived(
    hasControlledPrimaryCollapsed ? primaryCollapsed === true : uncontrolledPrimaryCollapsed,
  );
  const isSecondaryCollapsed = $derived(
    hasControlledSecondaryCollapsed ? secondaryCollapsed === true : uncontrolledSecondaryCollapsed,
  );
  // Hidden panes take no space but are not collapses: they get no toggle and
  // no collapsed data attribute, so hover-reveal never pins a pill for a pane
  // nobody collapsed.
  const isPrimaryGone = $derived(isPrimaryCollapsed || primaryHidden);
  const isSecondaryGone = $derived(isSecondaryCollapsed || secondaryHidden);

  const isPrimaryRailed = $derived(isPrimaryCollapsed && primaryCollapsedSize != null);
  const isSecondaryRailed = $derived(
    isSecondaryCollapsed && secondaryCollapsedSize != null,
  );

  /**
   * The seam's position along the split axis, as a CSS length against the
   * split root. The toggle pill anchors to the root, never the divider box:
   * a collapsed or hidden sibling can leave the divider degenerate, and the
   * pill must stay centered on the seam regardless.
   */
  const seamPosition = $derived(
    isPrimaryGone
      ? primaryCollapsedSize != null
        ? `${primaryCollapsedSize}px`
        : "0px"
      : isSecondaryGone
        ? secondaryCollapsedSize != null
          ? `calc(100% - ${secondaryCollapsedSize}px)`
          : "100%"
        : primarySize != null
          ? `${primarySize}px`
          : `${currentRatio * 100}%`,
  );

  const primaryFlex = $derived(
    isPrimaryGone
      ? primaryCollapsedSize != null
        ? `0 0 ${primaryCollapsedSize}px`
        : "0 0 0"
      : primarySize != null
        ? `0 0 ${primarySize}px`
        : secondarySize != null || isSecondaryGone
          ? "1 1 0"
          : `0 0 ${currentRatio * 100}%`,
  );
  const secondaryFlex = $derived(
    isSecondaryGone
      ? secondaryCollapsedSize != null
        ? `0 0 ${secondaryCollapsedSize}px`
        : "0 0 0"
      : "1 1 0",
  );
  const primaryMinStyle = $derived(
    minPrimarySize != null && !isPrimaryGone
      ? `min-${orientation === "horizontal" ? "width" : "height"}: ${minPrimarySize}px`
      : "",
  );
  const secondaryMinStyle = $derived(
    minSecondarySize != null && !isSecondaryGone
      ? `min-${orientation === "horizontal" ? "width" : "height"}: ${minSecondarySize}px`
      : "",
  );
  const hasToggles = $derived(showCollapsePrimary || showCollapseSecondary);
  const beforeDirection = $derived((orientation === "horizontal" ? "left" : "up") as CollapseDirection);
  const afterDirection = $derived((orientation === "horizontal" ? "right" : "down") as CollapseDirection);

  function setRatio(nextRatio: number): void {
    const clamped = Math.min(
      resolvedMaxRatio,
      Math.max(resolvedMinRatio, nextRatio),
    );
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

  function measureContainer(): { start: number; size: number } | null {
    if (!container) return null;
    const rect = container.getBoundingClientRect();
    const start = orientation === "horizontal" ? rect.left : rect.top;
    const size = orientation === "horizontal" ? rect.width : rect.height;
    return size > 0 ? { start, size } : null;
  }

  function rawRatio(mousePos: number): number {
    if (dragContainerSize <= 0) return currentRatio;
    return (mousePos - dragContainerStart) / dragContainerSize;
  }

  const RAIL_EXPAND_HYSTERESIS_PX = 8;

  function handleResizeStart(position: number): void {
    dragMousePos = position;
    const measurement = measureContainer();
    dragContainerStart = measurement?.start ?? 0;
    dragContainerSize = measurement?.size ?? 0;

    // Legacy hidden-collapse panes re-open on drag start. Railed panes
    // (a collapsed size is configured) stay railed until the drag pulls
    // them past their collapse threshold in handleResizeMove.
    if (isPrimaryCollapsed && primaryCollapsedSize == null) {
      setRatio(0.05);
      setPrimaryCollapsed(false);
    }
    if (isSecondaryCollapsed && secondaryCollapsedSize == null) {
      setRatio(0.95);
      setSecondaryCollapsed(false);
    }
  }

  function handleResizeMove(delta: number): void {
    if (!container || dragContainerSize <= 0) return;
    dragMousePos += delta;
    const total = dragContainerSize;
    const raw = rawRatio(dragMousePos);

    // Rail-collapse lanes resolve collapse/expand from drag intent here,
    // preserve the last expanded ratio, and emit no ratio while railed.
    if (collapsePrimaryBelowSize != null) {
      const primaryPx = raw * total;
      if (isPrimaryCollapsed) {
        if (primaryPx > collapsePrimaryBelowSize + RAIL_EXPAND_HYSTERESIS_PX) {
          setPrimaryCollapsed(false);
          setRatio(raw);
        }
        return;
      }
      if (primaryPx < collapsePrimaryBelowSize) {
        setPrimaryCollapsed(true);
        return;
      }
    }

    if (collapseSecondaryBelowSize != null) {
      const secondaryPx = (1 - raw) * total;
      if (isSecondaryCollapsed) {
        if (secondaryPx > collapseSecondaryBelowSize + RAIL_EXPAND_HYSTERESIS_PX) {
          setSecondaryCollapsed(false);
          setRatio(raw);
        }
        return;
      }
      if (secondaryPx < collapseSecondaryBelowSize) {
        setSecondaryCollapsed(true);
        return;
      }
    }

    if (raw <= 0.02 && collapsePrimaryBelowSize == null) {
      if (!isPrimaryCollapsed) {
        setPrimaryCollapsed(true);
        setRatio(0.5);
      }
      return;
    }

    if (raw >= 0.98 && collapseSecondaryBelowSize == null) {
      if (!isSecondaryCollapsed) {
        setSecondaryCollapsed(true);
        setRatio(0.5);
      }
      return;
    }

    if (isPrimaryCollapsed && primaryCollapsedSize == null) {
      setPrimaryCollapsed(false);
    }
    if (isSecondaryCollapsed && secondaryCollapsedSize == null) {
      setSecondaryCollapsed(false);
    }
    setRatio(raw);
  }

  function handleResizeEnd(): void {
    dragContainerStart = 0;
    dragContainerSize = 0;
  }

  function handleResizeStep(delta: number): void {
    const measurement = measureContainer();
    if (!measurement) return;
    setRatio(currentRatio + delta / measurement.size);
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
  data-divider={divider ? "line" : undefined}
  data-toggle-visibility={toggleVisibility}
  data-disabled={disabled || undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  style={`--poodle-split-seam: ${seamPosition}`}
  aria-label={ariaLabel ?? "Split view"}
  bind:this={container}
>
  <div
    class="poodle-split-view__pane poodle-split-view__pane--primary"
    style="flex: {primaryFlex}; overflow: hidden; {primaryMinStyle}"
  >
    {#if !isPrimaryGone || isPrimaryRailed}
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
      onResizeEnd={handleResizeEnd}
      onResizeStep={handleResizeStep}
    />
  </div>

  <div
    class="poodle-split-view__pane poodle-split-view__pane--secondary"
    style="flex: {secondaryFlex}; overflow: hidden; {secondaryMinStyle}"
  >
    {#if !isSecondaryGone || isSecondaryRailed}
      {@render secondary?.()}
    {/if}
  </div>

  <!-- The pill anchors to the split root along a computed seam position, so
       it stays centered on the seam even when a pane's collapse leaves the
       divider box degenerate. -->
  {#if hasToggles}
    <div class="poodle-split-view__toggles">
      <!-- A collapsed pane's expand toggle must survive the other pane
           collapsing too; hiding both strands the pair with no way back. -->
      {#if showCollapsePrimary && (!isSecondaryCollapsed || isPrimaryCollapsed)}
        <CollapseToggle
          direction={beforeDirection}
          collapsed={isPrimaryCollapsed}
          {disabled}
          ariaLabel={isPrimaryCollapsed ? "Expand primary" : "Collapse primary"}
          onToggle={toggleCollapsePrimary}
        />
      {/if}
      {#if showCollapseSecondary && (!isPrimaryCollapsed || isSecondaryCollapsed)}
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
