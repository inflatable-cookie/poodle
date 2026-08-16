import "@inflatable-cookie/poodle-core/styles/split-view.css";

import { useRef, useState, type CSSProperties, type ReactNode } from "react";

import { CollapseToggle } from "./CollapseToggle";
import { ResizeHandle } from "./ResizeHandle";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  CollapseDirection,
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
  SplitOrientation,
  SplitToggleVisibility,
} from "./types";

export interface SplitViewProps {
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
  /** The pane takes zero space without being a collapse: no toggle, no collapsed
   * data attribute — for panes that are absent, not user-collapsed. */
  primaryHidden?: boolean;
  secondaryHidden?: boolean;
  primaryCollapsedSize?: number | null;
  secondaryCollapsedSize?: number | null;
  collapsePrimaryBelowSize?: number | null;
  collapseSecondaryBelowSize?: number | null;
  showCollapsePrimary?: boolean;
  showCollapseSecondary?: boolean;
  /** When the collapse toggles are visible. `"always"` keeps the pill on
   * screen; `"hover"` reveals it only while the pointer is on the seam (the
   * resize grab strip or the pill itself) or a toggle holds focus. A collapsed
   * pane's expand toggle stays visible in either mode — hiding it would leave
   * the pane unrecoverable. */
  toggleVisibility?: SplitToggleVisibility;
  ariaLabel?: string | null;
  disabled?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onRatioChange?: ((ratio: number) => void) | null;
  onPrimaryCollapsedChange?: ((isCollapsed: boolean) => void) | null;
  onSecondaryCollapsedChange?: ((isCollapsed: boolean) => void) | null;
  primary?: ReactNode;
  secondary?: ReactNode;
}

const RAIL_EXPAND_HYSTERESIS_PX = 8;

export function SplitView({
  orientation = "horizontal",
  ratio,
  defaultRatio = 0.5,
  minRatio = 0.05,
  maxRatio = 0.95,
  minPrimarySize = null,
  minSecondarySize = null,
  primarySize = null,
  secondarySize = null,
  primaryCollapsed,
  secondaryCollapsed,
  primaryHidden = false,
  secondaryHidden = false,
  primaryCollapsedSize = null,
  secondaryCollapsedSize = null,
  collapsePrimaryBelowSize = null,
  collapseSecondaryBelowSize = null,
  showCollapsePrimary = false,
  showCollapseSecondary = false,
  toggleVisibility = "always",
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
}: SplitViewProps) {
  const uiPresentation = useUiPresentation();

  const containerRef = useRef<HTMLDivElement | null>(null);
  const dragMousePos = useRef(0);
  const [uncontrolledRatio, setUncontrolledRatio] = useState(ratio === undefined ? defaultRatio : 0.5);
  const [uncontrolledPrimaryCollapsed, setUncontrolledPrimaryCollapsed] = useState(false);
  const [uncontrolledSecondaryCollapsed, setUncontrolledSecondaryCollapsed] = useState(false);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasControlledRatio = ratio !== undefined;
  const hasControlledPrimaryCollapsed = primaryCollapsed !== undefined;
  const hasControlledSecondaryCollapsed = secondaryCollapsed !== undefined;
  const resolvedMinRatio = Math.min(0.95, Math.max(0.05, minRatio));
  const resolvedMaxRatio = Math.max(resolvedMinRatio, Math.min(0.95, maxRatio));
  const currentRatio = Math.min(
    resolvedMaxRatio,
    Math.max(resolvedMinRatio, hasControlledRatio ? (ratio ?? defaultRatio) : uncontrolledRatio),
  );
  const isPrimaryCollapsed = hasControlledPrimaryCollapsed ? primaryCollapsed === true : uncontrolledPrimaryCollapsed;
  const isSecondaryCollapsed = hasControlledSecondaryCollapsed
    ? secondaryCollapsed === true
    : uncontrolledSecondaryCollapsed;
  // Hidden panes take no space but are not collapses: they get no toggle and
  // no collapsed data attribute, so hover-reveal never pins a pill for a pane
  // nobody collapsed.
  const isPrimaryGone = isPrimaryCollapsed || primaryHidden;
  const isSecondaryGone = isSecondaryCollapsed || secondaryHidden;

  const isPrimaryRailed = isPrimaryCollapsed && primaryCollapsedSize != null;
  const isSecondaryRailed = isSecondaryCollapsed && secondaryCollapsedSize != null;

  // Refs mirror the latest collapse/ratio state for drag handlers, which
  // run outside the render cycle.
  const stateRef = useRef({ currentRatio, isPrimaryCollapsed, isSecondaryCollapsed });
  stateRef.current = { currentRatio, isPrimaryCollapsed, isSecondaryCollapsed };

  const primaryFlex = isPrimaryGone
    ? primaryCollapsedSize != null
      ? `0 0 ${primaryCollapsedSize}px`
      : "0 0 0"
    : primarySize != null
      ? `0 0 ${primarySize}px`
      : secondarySize != null || isSecondaryGone
        ? "1 1 0"
        : `0 0 ${currentRatio * 100}%`;
  const secondaryFlex = isSecondaryGone
    ? secondaryCollapsedSize != null
      ? `0 0 ${secondaryCollapsedSize}px`
      : "0 0 0"
    : "1 1 0";
  const minSizeProperty = orientation === "horizontal" ? "minWidth" : "minHeight";
  const primaryStyle: CSSProperties = {
    flex: primaryFlex,
    overflow: "hidden",
    ...(minPrimarySize != null && !isPrimaryGone ? { [minSizeProperty]: `${minPrimarySize}px` } : {}),
  };
  const secondaryStyle: CSSProperties = {
    flex: secondaryFlex,
    overflow: "hidden",
    ...(minSecondarySize != null && !isSecondaryGone ? { [minSizeProperty]: `${minSecondarySize}px` } : {}),
  };
  const hasToggles = showCollapsePrimary || showCollapseSecondary;
  const beforeDirection = (orientation === "horizontal" ? "left" : "up") as CollapseDirection;
  const afterDirection = (orientation === "horizontal" ? "right" : "down") as CollapseDirection;

  function setRatio(nextRatio: number): void {
    const clamped = Math.min(resolvedMaxRatio, Math.max(resolvedMinRatio, nextRatio));
    if (!hasControlledRatio) {
      setUncontrolledRatio(clamped);
    }
    stateRef.current.currentRatio = clamped;

    onRatioChange?.(clamped);
  }

  function setPrimaryCollapsed(nextCollapsed: boolean): void {
    if (!hasControlledPrimaryCollapsed) {
      setUncontrolledPrimaryCollapsed(nextCollapsed);
    }
    stateRef.current.isPrimaryCollapsed = nextCollapsed;

    onPrimaryCollapsedChange?.(nextCollapsed);
  }

  function setSecondaryCollapsed(nextCollapsed: boolean): void {
    if (!hasControlledSecondaryCollapsed) {
      setUncontrolledSecondaryCollapsed(nextCollapsed);
    }
    stateRef.current.isSecondaryCollapsed = nextCollapsed;

    onSecondaryCollapsedChange?.(nextCollapsed);
  }

  function rawRatio(mousePos: number): number {
    const container = containerRef.current;
    if (!container) return stateRef.current.currentRatio;
    const rect = container.getBoundingClientRect();
    const start = orientation === "horizontal" ? rect.left : rect.top;
    const total = orientation === "horizontal" ? rect.width : rect.height;
    if (total <= 0) return stateRef.current.currentRatio;
    return (mousePos - start) / total;
  }

  function handleResizeStart(position: number): void {
    dragMousePos.current = position;

    // Legacy hidden-collapse panes re-open on drag start. Railed panes
    // (a collapsed size is configured) stay railed until the drag pulls
    // them past their collapse threshold in handleResizeMove.
    if (stateRef.current.isPrimaryCollapsed && primaryCollapsedSize == null) {
      setRatio(0.05);
      setPrimaryCollapsed(false);
    }
    if (stateRef.current.isSecondaryCollapsed && secondaryCollapsedSize == null) {
      setRatio(0.95);
      setSecondaryCollapsed(false);
    }
  }

  function handleResizeMove(delta: number): void {
    const container = containerRef.current;
    if (!container) return;
    dragMousePos.current += delta;
    const rect = container.getBoundingClientRect();
    const total = orientation === "horizontal" ? rect.width : rect.height;
    if (total <= 0) return;
    const raw = rawRatio(dragMousePos.current);

    // Rail-collapse lanes resolve collapse/expand from drag intent here,
    // preserve the last expanded ratio, and emit no ratio while railed.
    if (collapsePrimaryBelowSize != null) {
      const primaryPx = raw * total;
      if (stateRef.current.isPrimaryCollapsed) {
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
      if (stateRef.current.isSecondaryCollapsed) {
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
      if (!stateRef.current.isPrimaryCollapsed) {
        setPrimaryCollapsed(true);
        setRatio(0.5);
      }
      return;
    }

    if (raw >= 0.98 && collapseSecondaryBelowSize == null) {
      if (!stateRef.current.isSecondaryCollapsed) {
        setSecondaryCollapsed(true);
        setRatio(0.5);
      }
      return;
    }

    if (stateRef.current.isPrimaryCollapsed && primaryCollapsedSize == null) {
      setPrimaryCollapsed(false);
    }
    if (stateRef.current.isSecondaryCollapsed && secondaryCollapsedSize == null) {
      setSecondaryCollapsed(false);
    }
    setRatio(raw);
  }

  function handleResizeStep(delta: number): void {
    const container = containerRef.current;
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const total = orientation === "horizontal" ? rect.width : rect.height;
    if (total <= 0) return;
    setRatio(stateRef.current.currentRatio + delta / total);
  }

  return (
    <div
      className="poodle-split-view"
      data-orientation={orientation}
      data-primary-collapsed={isPrimaryCollapsed || undefined}
      data-secondary-collapsed={isSecondaryCollapsed || undefined}
      data-toggle-visibility={toggleVisibility}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      aria-label={ariaLabel ?? "Split view"}
      ref={containerRef}
    >
      <div className="poodle-split-view__pane poodle-split-view__pane--primary" style={primaryStyle}>
        {!isPrimaryGone || isPrimaryRailed ? primary : null}
      </div>

      <div
        className="poodle-split-view__divider"
        data-orientation={orientation}
        data-disabled={disabled || undefined}
        data-has-toggles={hasToggles || undefined}
      >
        <ResizeHandle
          orientation={orientation}
          disabled={disabled}
          ariaLabel="Resize"
          onResizeStart={handleResizeStart}
          onResizeMove={handleResizeMove}
          onResizeStep={handleResizeStep}
        />

        {hasToggles ? (
          <div className="poodle-split-view__toggles">
            {showCollapsePrimary && (!isSecondaryCollapsed || isPrimaryCollapsed) ? (
              <CollapseToggle
                direction={beforeDirection}
                collapsed={isPrimaryCollapsed}
                disabled={disabled}
                ariaLabel={isPrimaryCollapsed ? "Expand primary" : "Collapse primary"}
                onToggle={(next) => setPrimaryCollapsed(next)}
              />
            ) : null}
            {showCollapseSecondary && (!isPrimaryCollapsed || isSecondaryCollapsed) ? (
              <CollapseToggle
                direction={afterDirection}
                collapsed={isSecondaryCollapsed}
                disabled={disabled}
                ariaLabel={isSecondaryCollapsed ? "Expand secondary" : "Collapse secondary"}
                onToggle={(next) => setSecondaryCollapsed(next)}
              />
            ) : null}
          </div>
        ) : null}
      </div>

      <div className="poodle-split-view__pane poodle-split-view__pane--secondary" style={secondaryStyle}>
        {!isSecondaryGone || isSecondaryRailed ? secondary : null}
      </div>
    </div>
  );
}
