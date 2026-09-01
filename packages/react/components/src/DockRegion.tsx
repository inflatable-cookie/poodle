import "@inflatable-cookie/poodle-core/styles/dock-region.css";

import {
  useEffect,
  useRef,
  useState,
  type DragEvent as ReactDragEvent,
  type PointerEvent as ReactPointerEvent,
  type ReactNode,
} from "react";

import {
  createDragDropController,
  decodeDockPanelSubject,
  encodeDockPanelSubject,
  DOCK_PANEL_SUBJECT_KIND,
  type CrossWindowDragSourceBridge,
  type CrossWindowDragTargetBridge,
  type DragDropCommitResult,
} from "@inflatable-cookie/poodle-core";
import { DragDropProvider, useControllerDropTarget, useOptionalDragDrop } from "./drag-drop";
import { DockStackItem } from "./dock-region-parts/DockStackItem";

import { CollapseToggle } from "./CollapseToggle";
import { Tabs } from "./Tabs";
import type {
  ActiveEdge,
  ActiveFill,
  ControlDensity,
  ControlSize,
  DockCollapsedPosture,
  DockEdge,
  DockEmphasis,
  DockSizing,
  DockPanelDropPayload,
  PanelDragData,
  PanelTabItem,
  SemanticControlSizeRole,
  TabItem,
  TabVariant,
} from "./types";

export interface DockRegionProps {
  edge?: DockEdge;
  sizing?: DockSizing;
  collapsible?: boolean;
  collapsed?: boolean;
  collapsedPosture?: DockCollapsedPosture;
  emphasis?: DockEmphasis;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  tabVariant?: TabVariant;
  /** Selection edge on the active tab, forwarded to Tabs. Defaults to the
   * former strip indicator (`"underline"`). */
  tabActiveEdge?: ActiveEdge;
  /** Selection fill on the active tab, forwarded to Tabs. */
  tabActiveFill?: ActiveFill;
  /** Draw the strip border around the tab list, forwarded to Tabs. */
  tabBordered?: boolean;
  /** Stretch tabs to fill the strip, forwarded to Tabs. */
  tabFullWidth?: boolean;
  /** Allow dragging tabs to reorder; forwarded to Tabs. */
  tabReorderable?: boolean;
  items?: PanelTabItem[];
  value?: string | null;
  ariaLabel?: string | null;
  canAcceptPanel?: ((panelId: string, sourceEdge: DockEdge) => boolean) | null;
  /**
   * Host preparation for a panel that may leave this window.
   *
   * Forwarded to the tab strip as its `crossWindowSourceBridge`: the host owns
   * the transaction and only an opaque receipt leaves the window.
   */
  crossWindowDragSource?: CrossWindowDragSourceBridge;
  /** Distinguishes drop zones that share an edge; defaults to `edge`. */
  dragZoneId?: string | null;
  /**
   * Incoming host projection and commit for this window.
   *
   * Only meaningful when this region owns its controller. A region that joined
   * an ambient `DragDropProvider` is not the window: the provider is, and the
   * bridge belongs there.
   */
  crossWindowDropTarget?: CrossWindowDragTargetBridge;
  onValueChange?: ((value: string) => void) | undefined;
  onCollapsedChange?: ((isCollapsed: boolean) => void) | undefined;
  onClose?: ((value: string) => void) | undefined;
  onReorder?: ((items: string[]) => void) | undefined;
  onPanelDrop?: ((payload: DockPanelDropPayload) => void) | undefined;
  panel?: (item: PanelTabItem) => ReactNode;
  children?: (item: PanelTabItem | null) => ReactNode;
}


export function DockRegion({
  edge = "left",
  sizing = "flexible",
  collapsible = false,
  collapsed = false,
  collapsedPosture = "icon-strip",
  emphasis = "standard",
  size = null,
  sizeRole = "chrome",
  density = null,
  tabVariant = "block",
  tabActiveEdge = "underline",
  tabActiveFill = "tint",
  tabBordered = false,
  tabFullWidth = false,
  tabReorderable = true,
  items = [],
  value = null,
  ariaLabel = null,
  canAcceptPanel = null,
  crossWindowDragSource = undefined,
  dragZoneId = null,
  crossWindowDropTarget = undefined,
  onValueChange = undefined,
  onCollapsedChange = undefined,
  onClose = undefined,
  onReorder = undefined,
  onPanelDrop = undefined,
  panel,
  children,
}: DockRegionProps) {
  const dropZoneId = dragZoneId ?? edge;
  const isVerticalEdge = edge === "left" || edge === "right";
  const activeItem = items.find((item) => item.value === value) ?? items[0] ?? null;
  const collapseDirection = ({ left: "left", right: "right", top: "up", bottom: "down" } as const)[edge];
  /**
   * The strip's tabs carry the encoded subject id as their value.
   *
   * That is the whole composition seam: the tab set shares the
   * `poodle.dock-panel` family, and each tab's semantic subject id says which
   * panel, from which edge, and from which zone — which is what a sibling
   * region needs during hover. It is substrate identity only, so every public
   * boundary decodes it back to the consumer's own panel value.
   */
  const tabItems: TabItem[] = items.map((item) => ({
    value: panelSubjectId(item.value),
    label: item.label,
    icon: item.icon ?? undefined,
    closable: item.closable,
  }));
  const stackDirection = isVerticalEdge ? "column" : "row";
  const showIconStrip = collapsed && collapsedPosture === "icon-strip";
  const showHidden = collapsed && collapsedPosture === "hidden";

  const stripTabsRef = useRef<HTMLDivElement | null>(null);
  const fullLabelScrollWidth = useRef(0);
  const [isCompact, setIsCompact] = useState(false);

  const isCompactRef = useRef(isCompact);
  isCompactRef.current = isCompact;
  const itemsLengthRef = useRef(items.length);
  itemsLengthRef.current = items.length;
  /* Compacting to icon-only is only a strategy when there is an icon to fall
     back to. Every panel needs one: hiding a label on a tab that has no icon
     leaves an empty 2.25rem square with nothing in it and no way to tell the
     panels apart. A strip of icon-less tabs overflows and scrolls instead —
     cramped beats unreadable. */
  const canCompactRef = useRef(false);
  canCompactRef.current = items.length > 0 && items.every((item) => item.icon);

  // Compact detection: observe the strip and its tablist; keep the natural
  // (uncompacted) label width so expansion re-checks against it.
  useEffect(() => {
    const el = stripTabsRef.current;
    if (!el) return;

    function checkCompact(): void {
      const target = stripTabsRef.current;
      if (!target || itemsLengthRef.current === 0 || !canCompactRef.current) {
        setIsCompact(false);
        return;
      }
      const list = target.querySelector("[role='tablist']") as HTMLElement | null;
      if (!list) return;

      const containerWidth = target.clientWidth;

      if (!isCompactRef.current) {
        fullLabelScrollWidth.current = list.scrollWidth;
        setIsCompact(list.scrollWidth > containerWidth + 2);
        return;
      }

      setIsCompact(fullLabelScrollWidth.current > containerWidth);
    }

    const resizeObserver = new ResizeObserver(checkCompact);
    resizeObserver.observe(el);
    const list = el.querySelector("[role='tablist']");
    if (list) resizeObserver.observe(list);
    checkCompact();

    return () => resizeObserver.disconnect();
  }, [showIconStrip, showHidden, sizing, items.length]);

  // Every one of these takes a strip value and hands out a panel value. The
  // encoding is substrate identity; it must never reach a consumer callback.
  function handleValueChange(nextValue: string): void {
    onValueChange?.(panelValueOf(nextValue));
    if (collapsed) {
      onCollapsedChange?.(false);
    }
  }

  function handleCollapseToggle(): void {
    onCollapsedChange?.(!collapsed);
  }

  // ── Panel movement (shared drag substrate) ─────────────────────────────

  /**
   * Join the nearest provider, or own a controller.
   *
   * Two sibling regions can only see each other's targets when one controller
   * holds both registrations, so a consumer that wants cross-region transfer
   * wraps its regions in one `DragDropProvider`. A region with no provider
   * still reorders its own stack — it simply does not discover anyone else.
   * There is no document-global session restoring that link implicitly.
   */
  const ambient = useOptionalDragDrop();
  if (ambient && crossWindowDropTarget) {
    throw new Error(
      "DockRegion: crossWindowDropTarget belongs on the DragDropProvider that owns this window, not on a region that joined one",
    );
  }
  const [ownController] = useState(() =>
    ambient ? null : createDragDropController({ crossWindowTargetBridge: crossWindowDropTarget }),
  );
  const controller = ambient?.controller ?? ownController!;

  function panelSubjectId(panelId: string): string {
    return encodeDockPanelSubject({ panelId, sourceEdge: edge, sourceZone: dropZoneId });
  }

  /** The public panel value behind a strip value. Never leaks the encoding. */
  function panelValueOf(encoded: string): string {
    return decodeDockPanelSubject(encoded)?.panelId ?? encoded;
  }

  function acceptsPanel(subjectId: string): boolean {
    const panel = decodeDockPanelSubject(subjectId);
    if (!panel) return false;
    return canAcceptPanel === null || canAcceptPanel(panel.panelId, panel.sourceEdge as DockEdge);
  }

  function panelFrom(subjectId: string): PanelDragData | null {
    const decoded = decodeDockPanelSubject(subjectId);
    if (!decoded) return null;
    return {
      panelId: decoded.panelId,
      sourceEdge: decoded.sourceEdge as DockEdge,
      sourceZone: decoded.sourceZone,
    };
  }

  function liveSubjectId(): string {
    return controller.getSnapshot().session?.subject.id ?? "";
  }

  const region = useControllerDropTarget(controller, {
    targetId: dropZoneId,
    acceptedKinds: [DOCK_PANEL_SUBJECT_KIND],
    label: ariaLabel ?? `${edge} dock`,
    // Deepest wins, so a stack item always beats the region it sits in.
    priority: -1,
    resolvePosition: () => "inside",
    canDrop: (intent, subject) => {
      const panel = decodeDockPanelSubject(subject.id);
      if (!panel) return { accepted: false, reason: "not a panel" };
      // A panel dropped back on its own flexible zone is a same-strip reorder,
      // which the strip already owns.
      if (panel.sourceZone === dropZoneId && sizing === "flexible") {
        return { accepted: false, reason: "same zone" };
      }
      return acceptsPanel(subject.id)
        ? { accepted: true, intent }
        : { accepted: false, reason: "refused by host" };
    },
    onDrop: (): DragDropCommitResult => {
      const panel = panelFrom(liveSubjectId());
      if (!panel) return { status: "rejected", reason: "not a panel" };
      onPanelDrop?.({ panel, targetEdge: edge, index: items.length });
      return { status: "committed" };
    },
  });
  const isDragOver = region.accepted;

  const stripTabs = (orientation: "horizontal" | "vertical", withTooltips: boolean) => (
    <Tabs
      variant={tabVariant}
      activeEdge={tabActiveEdge}
      activeFill={tabActiveFill}
      bordered={tabBordered}
      fullWidth={tabFullWidth}
      orientation={orientation}
      size={size}
      sizeRole={sizeRole}
      density={density}
      showTooltips={withTooltips}
      items={tabItems}
      value={activeItem ? panelSubjectId(activeItem.value) : ""}
      reorderable={tabReorderable}
      ariaLabel={ariaLabel ?? `${edge} dock panels`}
      onValueChange={handleValueChange}
      onReorder={(next) => onReorder?.(next.map(panelValueOf))}
      onClose={(next) => onClose?.(panelValueOf(next))}
      crossWindowSourceBridge={crossWindowDragSource}
      dragSubjectKind={DOCK_PANEL_SUBJECT_KIND}
      onForeignDrop={handleForeignDrop}
    />
  );

  function handleForeignDrop(subjectId: string, index: number): void {
    const panel = panelFrom(subjectId);
    if (panel) onPanelDrop?.({ panel, targetEdge: edge, index });
  }

  const region_ = (

    <section
      className="poodle-dock-region"
      data-edge={edge}
      data-sizing={sizing}
      data-emphasis={emphasis}
      data-collapsed={collapsed || undefined}
      data-collapsed-posture={collapsed ? collapsedPosture : undefined}
      aria-label={ariaLabel ?? `${edge} dock`}
      {...region.getTargetProps()}
    >
      {isDragOver ? <div className="poodle-dock-region__drop-zone" /> : null}

      {sizing === "static" ? (
        <div className="poodle-dock-region__stack" data-direction={stackDirection}>
          {items.map((item, index) => (
            <DockStackItem
              key={item.value}
              item={item}
              index={index}
              edge={edge}
              dropZoneId={dropZoneId}
              items={items}
              canAcceptPanel={canAcceptPanel}
              crossWindowDragSource={crossWindowDragSource}
              liveSubjectId={liveSubjectId}
              onReorder={onReorder}
              onPanelDrop={onPanelDrop}
            >
              {panel?.(item)}
            </DockStackItem>
          ))}
        </div>
      ) : showHidden ? (
        collapsible ? (
          <div className="poodle-dock-region__edge-toggle">
            <CollapseToggle
              collapsed={collapsed}
              direction={collapseDirection}
              ariaLabel={`Expand ${edge} dock`}
              onToggle={handleCollapseToggle}
            />
          </div>
        ) : null
      ) : showIconStrip && isVerticalEdge ? (
        <div className="poodle-dock-region__strip" data-orientation="vertical">
          {collapsible ? (
            <CollapseToggle
              collapsed={collapsed}
              direction={collapseDirection}
              ariaLabel={`Expand ${edge} dock`}
              onToggle={handleCollapseToggle}
            />
          ) : null}
          {stripTabs("vertical", false)}
        </div>
      ) : showIconStrip ? (
        <div
          className="poodle-dock-region__strip"
          data-orientation="horizontal"
          data-compact={isCompact || undefined}
        >
          <div className="poodle-dock-region__tabs" ref={stripTabsRef}>
            {stripTabs("horizontal", isCompact)}
          </div>
          {collapsible ? (
            <CollapseToggle
              collapsed={collapsed}
              direction={collapseDirection}
              ariaLabel={`Expand ${edge} dock`}
              onToggle={handleCollapseToggle}
            />
          ) : null}
        </div>
      ) : (
        <>
          <div
            className="poodle-dock-region__strip"
            data-orientation="horizontal"
            data-compact={isCompact || undefined}
          >
            <div className="poodle-dock-region__tabs" ref={stripTabsRef}>
              {stripTabs("horizontal", isCompact)}
            </div>
            {collapsible ? (
              <CollapseToggle
                collapsed={collapsed}
                direction={collapseDirection}
                ariaLabel={`Collapse ${edge} dock`}
                onToggle={handleCollapseToggle}
              />
            ) : null}
          </div>

          <div className="poodle-dock-region__body">{children?.(activeItem)}</div>
        </>
      )}
    </section>
  );

  // A region that joined a provider contributes registrations to it. A region
  // with none owns a controller so its own stack still reorders — and stays
  // invisible to any sibling that did the same.
  return ambient ? region_ : (
    <DragDropProvider controller={ownController!}>{region_}</DragDropProvider>
  );
}
