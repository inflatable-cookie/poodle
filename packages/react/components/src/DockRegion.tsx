import "@inflatable-cookie/poodle-styles/dock-region.css";

import {
  useEffect,
  useRef,
  useState,
  type DragEvent as ReactDragEvent,
  type PointerEvent as ReactPointerEvent,
  type ReactNode,
} from "react";

import { createDockExternalDragController } from "@inflatable-cookie/poodle-headless";

import { CollapseToggle } from "./CollapseToggle";
import { Tabs } from "./Tabs";
import type {
  ControlDensity,
  ControlSize,
  DockCollapsedPosture,
  DockEdge,
  DockEmphasis,
  DockExternalDragSource,
  DockExternalDropTarget,
  DockSizing,
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
  items?: PanelTabItem[];
  value?: string | null;
  ariaLabel?: string | null;
  canAcceptPanel?: ((panelId: string, sourceEdge: DockEdge) => boolean) | null;
  externalDragSource?: DockExternalDragSource | null;
  /** Distinguishes drop zones that share an edge; defaults to `edge`. */
  dragZoneId?: string | null;
  externalDropTarget?: DockExternalDropTarget | null;
  onValueChange?: ((value: string) => void) | undefined;
  onCollapsedChange?: ((isCollapsed: boolean) => void) | undefined;
  onClose?: ((value: string) => void) | undefined;
  onReorder?: ((items: string[]) => void) | undefined;
  onPanelDrop?: ((payload: { panel: PanelDragData; targetEdge: DockEdge }) => void) | undefined;
  panel?: (item: PanelTabItem) => ReactNode;
  children?: (item: PanelTabItem | null) => ReactNode;
}

const PANEL_DRAG_TYPE = "application/x-poodle-panel-drag";

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
  tabVariant = "strip",
  items = [],
  value = null,
  ariaLabel = null,
  canAcceptPanel = null,
  externalDragSource = null,
  dragZoneId = null,
  externalDropTarget = null,
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
  const tabItems: TabItem[] = items.map((item) => ({
    value: item.value,
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

  // Compact detection: observe the strip and its tablist; keep the natural
  // (uncompacted) label width so expansion re-checks against it.
  useEffect(() => {
    const el = stripTabsRef.current;
    if (!el) return;

    function checkCompact(): void {
      const target = stripTabsRef.current;
      if (!target || itemsLengthRef.current === 0) {
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

  const [isDragOver, setIsDragOver] = useState(false);
  const [dropInsertIndex, setDropInsertIndex] = useState(-1);
  const dragSourceIndex = useRef(-1);
  const [dragSourceState, setDragSourceState] = useState(-1);

  // The controller holds a live drag session, so it must survive re-renders —
  // rebuilding it would drop a pending preparation on the floor and leak
  // whatever the host allocated in `prepare`. Latest props reach it through
  // accessor refs rather than by reconstruction.
  const externalDragSourceRef = useRef(externalDragSource);
  externalDragSourceRef.current = externalDragSource;
  const itemsRef = useRef(items);
  itemsRef.current = items;
  const edgeRef = useRef(edge);
  edgeRef.current = edge;

  const externalDragRef = useRef<ReturnType<typeof createDockExternalDragController> | null>(null);
  if (externalDragRef.current === null) {
    externalDragRef.current = createDockExternalDragController<PanelTabItem, DockEdge>({
      source: () => externalDragSourceRef.current,
      panel: (panelId) => itemsRef.current.find((item) => item.value === panelId),
      edge: () => edgeRef.current,
    });
  }
  const externalDrag = externalDragRef.current;

  // Unmounting mid-preparation is a cancel, not a silent drop.
  useEffect(() => () => externalDrag.cancel("unmounted"), [externalDrag]);

  function handleValueChange(nextValue: string): void {
    onValueChange?.(nextValue);
    if (collapsed) {
      onCollapsedChange?.(false);
    }
  }

  function handleCollapseToggle(): void {
    onCollapsedChange?.(!collapsed);
  }

  // Driven by Tabs' own per-tab drag events rather than a wrapper listener
  // sniffing `[role='tab']` out of the DOM: the panel is handed over directly,
  // and the vertical strip gets the same treatment as the horizontal one.
  function handleTabDragStart(panelId: string, event: ReactDragEvent): void {
    if (!event.dataTransfer) return;
    if (externalDragSource) {
      externalDrag.start(panelId, event.nativeEvent);
      return;
    }

    const data: PanelDragData = {
      panelId,
      sourceEdge: edge,
      sourceZone: dropZoneId,
    };
    event.dataTransfer.setData(PANEL_DRAG_TYPE, JSON.stringify(data));
    event.dataTransfer.effectAllowed = "move";
  }

  function handleTabDragEnd(panelId: string, event: ReactDragEvent): void {
    externalDrag.end(panelId, event.nativeEvent);
  }

  function canAcceptExternalDrop(phase: "over" | "drop", event: ReactDragEvent): boolean {
    if (!externalDropTarget || !event.dataTransfer) return false;
    return externalDropTarget.canDrop({
      phase,
      targetEdge: edge,
      event: event.nativeEvent,
      dataTransfer: event.dataTransfer,
    });
  }

  function handleRegionDragOver(event: ReactDragEvent): void {
    const hasPoodlePanel = event.dataTransfer?.types.includes(PANEL_DRAG_TYPE) === true;
    const acceptsExternal = !hasPoodlePanel && canAcceptExternalDrop("over", event);
    if (!hasPoodlePanel && !acceptsExternal) return;

    event.preventDefault();
    setIsDragOver(true);
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = "move";
    }
  }

  function handleRegionDragLeave(event: ReactDragEvent): void {
    const current = event.currentTarget as HTMLElement;
    const related = event.relatedTarget as Node | null;
    if (related && current.contains(related)) return;
    setIsDragOver(false);
  }

  function handleRegionDrop(event: ReactDragEvent): void {
    setIsDragOver(false);

    const raw = event.dataTransfer?.getData(PANEL_DRAG_TYPE);
    if (!raw) {
      // Not a Poodle panel — offer it to the host before letting it fall
      // through to the page.
      if (!canAcceptExternalDrop("drop", event) || !event.dataTransfer) return;

      event.preventDefault();
      void externalDropTarget?.drop({
        targetEdge: edge,
        event: event.nativeEvent,
        dataTransfer: event.dataTransfer,
      });
      return;
    }

    event.preventDefault();

    let data: PanelDragData;
    try {
      data = JSON.parse(raw);
    } catch {
      return;
    }

    // Zone, not edge: a host can map several regions onto one edge, and
    // comparing edges reads those cross-region drops as same-zone. A legacy
    // payload with no zone falls back to its edge.
    if ((data.sourceZone ?? data.sourceEdge) === dropZoneId && sizing === "flexible") return;
    if (canAcceptPanel && !canAcceptPanel(data.panelId, data.sourceEdge)) return;

    onPanelDrop?.({ panel: data, targetEdge: edge });
  }

  function handleStackItemDragStart(event: ReactDragEvent, index: number): void {
    if (!event.dataTransfer) return;
    dragSourceIndex.current = index;
    setDragSourceState(index);
    if (externalDragSource) {
      externalDrag.start(items[index].value, event.nativeEvent);
      return;
    }
    const data: PanelDragData = {
      panelId: items[index].value,
      sourceEdge: edge,
      sourceZone: dropZoneId,
    };
    event.dataTransfer.setData(PANEL_DRAG_TYPE, JSON.stringify(data));
    event.dataTransfer.effectAllowed = "move";
  }

  function handleStackItemDragOver(event: ReactDragEvent, index: number): void {
    if (!event.dataTransfer?.types.includes(PANEL_DRAG_TYPE)) return;
    event.preventDefault();
    setDropInsertIndex(index);
    event.dataTransfer.dropEffect = "move";
  }

  function handleStackItemDrop(event: ReactDragEvent, index: number): void {
    event.preventDefault();
    event.stopPropagation();
    setIsDragOver(false);
    setDropInsertIndex(-1);

    const raw = event.dataTransfer?.getData(PANEL_DRAG_TYPE);
    if (!raw) return;

    let data: PanelDragData;
    try {
      data = JSON.parse(raw);
    } catch {
      return;
    }

    if (data.sourceEdge === edge && dragSourceIndex.current >= 0) {
      const order = items.map((item) => item.value);
      const [moved] = order.splice(dragSourceIndex.current, 1);
      order.splice(index, 0, moved);
      dragSourceIndex.current = -1;
      setDragSourceState(-1);
      onReorder?.(order);
      return;
    }

    if (canAcceptPanel && !canAcceptPanel(data.panelId, data.sourceEdge)) return;
    onPanelDrop?.({ panel: data, targetEdge: edge });
  }

  function handleStackDragEnd(event: ReactDragEvent): void {
    const panelId = externalDrag.activePanelId();
    if (panelId) {
      externalDrag.end(panelId, event.nativeEvent);
    }
    setIsDragOver(false);
    dragSourceIndex.current = -1;
    setDragSourceState(-1);
    setDropInsertIndex(-1);
  }

  const stripTabs = (orientation: "horizontal" | "vertical", withTooltips: boolean) => (
    <Tabs
      variant={tabVariant}
      orientation={orientation}
      size={size}
      sizeRole={sizeRole}
      density={density}
      showTooltips={withTooltips}
      items={tabItems}
      value={activeItem?.value ?? ""}
      reorderable={true}
      ariaLabel={ariaLabel ?? `${edge} dock panels`}
      onValueChange={handleValueChange}
      onReorder={(next) => onReorder?.(next)}
      onClose={(next) => onClose?.(next)}
      onDragPrepare={(panelId, event) => externalDrag.prepare(panelId, event.nativeEvent)}
      onDragStart={handleTabDragStart}
      onDragEnd={handleTabDragEnd}
    />
  );

  return (
    <section
      className="poodle-dock-region"
      data-edge={edge}
      data-sizing={sizing}
      data-emphasis={emphasis}
      data-collapsed={collapsed || undefined}
      data-collapsed-posture={collapsed ? collapsedPosture : undefined}
      aria-label={ariaLabel ?? `${edge} dock`}
      onDragOver={handleRegionDragOver}
      onDragLeave={handleRegionDragLeave}
      onDrop={handleRegionDrop}
    >
      {isDragOver ? <div className="poodle-dock-region__drop-zone" /> : null}

      {sizing === "static" ? (
        <div className="poodle-dock-region__stack" data-direction={stackDirection}>
          {items.map((item, index) => (
            <div
              key={item.value}
              className="poodle-dock-region__stack-item"
              data-drop-target={dropInsertIndex === index || undefined}
              data-drag-source={dragSourceState === index || undefined}
              draggable="true"
              role="group"
              aria-label={item.label ?? `Panel ${index + 1}`}
              onPointerDown={(event: ReactPointerEvent) =>
                externalDrag.prepare(items[index].value, event.nativeEvent)}
              onDragStart={(event) => handleStackItemDragStart(event, index)}
              onDragOver={(event) => handleStackItemDragOver(event, index)}
              onDragLeave={() => setDropInsertIndex(-1)}
              onDrop={(event) => handleStackItemDrop(event, index)}
              onDragEnd={handleStackDragEnd}
            >
              {panel?.(item)}
            </div>
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
}
