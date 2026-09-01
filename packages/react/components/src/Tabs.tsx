import "@inflatable-cookie/poodle-core/styles/tabs.css";

import {
  Fragment,
  useEffect,
  useId,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
  type ReactNode,
} from "react";

import {
  createDragDropController,
  firstEnabledIndex,
  tabsKeydownEvent,
  tabsTransition,
  type CrossWindowDragSourceBridge,
  type DragDropCommitResult,
  type DropIntent,
  type TabsContext as HeadlessTabsContext,
  type TabsEvent as HeadlessTabsEvent,
} from "@inflatable-cookie/poodle-core";
import { AnchoredSurface } from "./AnchoredSurface";
import { Button } from "./Button";
import { DragDropProvider, useOptionalDragDrop } from "./drag-drop";
import { Icon } from "./Icon";
import { Menu } from "./Menu";
import { Pill } from "./Pill";
import { TabsItem } from "./tabs-parts/TabsItem";
import { TabsKeyboardTargets } from "./tabs-parts/TabsKeyboardTargets";
import {
  resolveSemanticControlSize,
  resolveSupportingVisualSize,
  useUiPresentation,
} from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  Orientation,
  SemanticControlSizeRole,
  TabItem,
} from "./types";

/** @deprecated Use TabItem instead (pilot-era alias). */
export type TabsItem = TabItem;

/**
 * The contracted Tabs surface. `items` takes the framework's `TabItem` shape;
 * everything else is the contract's own vocabulary.
 *
 * Contract: `docs/contracts/components/tabs.md`. The Svelte pair is
 * `packages/svelte/components/src/Tabs.svelte`; the Rust counterpart is
 * `poodle_specs::TabsSpec`.
 */
export interface TabsProps {
  value?: string | null;
  defaultValue?: string | null;
  items?: TabItem[];
  variant?: "card" | "pill" | "block";
  activeEdge?: "none" | "outline" | "underline";
  activeFill?: "none" | "tint" | "solid";
  orientation?: Orientation;
  activationMode?: "automatic" | "manual";
  bordered?: boolean;
  fullWidth?: boolean;
  reorderable?: boolean;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  /**
   * Selection edge on the active tab: `"none"` draws no edge, `"outline"`
   * draws the accent border around the active item (the former `card`
   * variant's selected-border value), `"underline"` draws the accent edge
   * along the inline-end side (the former `strip` variant's indicator).
   * The edge axis is an enum, so outline and underline cannot both apply.
   */
  /**
   * Selection treatment on the active tab: `"none"` draws no fill (the
   * edge and the selected text colour carry selection alone), `"tint"` is
   * the accent-tinted fill; `"solid"` fills the tab with `accent-base` and
   * switches the foreground to `text-inverse` for contrast.
   */
  collapseWhenOverflow?: boolean;
  /**
   * What to do as the strip stops fitting. `"collapse"` is today's single
   * threshold into a menu; `"shed"` gives up decoration first — see `shed`.
   */
  overflowStrategy?: "collapse" | "shed";
  /**
   * Which parts to give up, in order, before collapsing. Icons first by
   * default: an icon usually repeats the label, where a count does not. Labels
   * are never shed.
   */
  shed?: ("icon" | "count")[];
  collapseLabel?: string | null;
  showTooltips?: boolean;
  historyKey?: string | null;
  onValueChange?: ((value: string) => void) | undefined;
  onReorder?: ((items: string[]) => void) | undefined;
  // Forwarded so a host (DockRegion) can run its own drag session on top of
  // the reorder plumbing; the tab still reorders locally either way.
  /**
   * Host preparation for a tab that may be moved to another window.
   *
   * The whole cross-window seam, in one semantic prop: preparation runs on
   * the pre-drag gesture, the host owns the transaction, and only an opaque
   * receipt leaves the window. Tabs keeps its local reorder either way.
   */
  crossWindowSourceBridge?: CrossWindowDragSourceBridge;
  /**
   * The semantic drag family this strip belongs to.
   *
   * `null` mints a family scoped to this Tabs instance, so two ordinary tab
   * sets sharing one provider are never eligible for each other. An owning
   * composite passes an explicit kind to put the strip in a shared family —
   * without taking over reorder, which stays Tabs' own.
   */
  dragSubjectKind?: string | null;
  /**
   * Owning composite hook: a subject of this strip's family that is not in
   * `items` lands at the hovered tab and reports here instead of `onReorder`.
   * Absent, the tab refuses so an ancestor target can take the drop.
   */
  onForeignDrop?: ((id: string, index: number) => void) | undefined;
  onClose?: ((value: string) => void) | undefined;
  children?: (value: string) => ReactNode;
  actions?: ReactNode;
}

function getItemsSignature(nextItems: TabItem[]): string {
  return JSON.stringify(
    nextItems.map((item) => ({
      value: item.value,
      label: item.label,
      icon: item.icon ?? null,
      disabled: item.disabled ?? false,
      closable: item.closable ?? false,
      count: item.count ?? null,
      separator: item.separator ?? false,
    })),
  );
}

export function Tabs({
  value: controlledValue = null,
  defaultValue = null,
  items = [],
  variant = "card",
  activeEdge = "none",
  activeFill = "tint",
  orientation = "horizontal",
  activationMode = "automatic",
  bordered = false,
  size = null,
  sizeRole = "chrome",
  density = null,
  collapseWhenOverflow = false,
  overflowStrategy = "collapse",
  shed = ["icon", "count"],
  fullWidth = false,
  collapseLabel = null,
  reorderable = false,
  ariaLabel = null,
  showTooltips = false,
  historyKey = null,
  onValueChange = undefined,
  onReorder = undefined,
  crossWindowSourceBridge = undefined,
  dragSubjectKind = null,
  onForeignDrop = undefined,
  onClose = undefined,
  children,
  actions,
}: TabsProps) {
  const tabsId = useId();
  const uiPresentation = useUiPresentation();

  const rootRef = useRef<HTMLDivElement | null>(null);
  const measureListRef = useRef<HTMLDivElement | null>(null);
  const tabRefs = useRef<Record<string, HTMLButtonElement | null>>({});
  const tooltipTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const pendingTabFocus = useRef<string | null>(null);
  const lastItemsSignature = useRef("");
  const lastSyncedValue = useRef<string | null>(null);
  const historyReady = useRef(false);

  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);
  const [renderedItems, setRenderedItems] = useState<TabItem[]>(items);
  const [focusIndex, setFocusIndex] = useState(0);
  const [tooltipIndex, setTooltipIndex] = useState<number | null>(null);
  // The hovered tab is promoted to state so the portalled tooltip can be
  // positioned against it.
  const [tooltipAnchor, setTooltipAnchor] = useState<HTMLElement | null>(null);

  useEffect(() => {
    const value = tooltipIndex === null ? undefined : renderedItems[tooltipIndex]?.value;
    setTooltipAnchor(value ? (tabRefs.current[value] ?? null) : null);
  }, [tooltipIndex, renderedItems]);
  const [collapsedByOverflow, setCollapsedByOverflow] = useState(false);
  /** How many entries of `shed` are currently given up. */
  const [shedCount, setShedCount] = useState(0);
  /**
   * Re-entrancy guard for the measurement pass.
   *
   * The ResizeObserver watches the measure list, and deciding a level changes
   * that list's width twice — once to shed, once to restore. Without this the
   * observer re-enters mid-measurement and the transient states reach the
   * screen: narrowing past the icon threshold flashed the collapsed menu
   * before settling.
   */
  const measuringRef = useRef(false);

  // Prop items replace rendered (possibly machine-reordered) items only when
  // their content actually changes — mirrors the Svelte signature sync. The
  // render-phase setState re-runs this render with fresh state.
  const itemsSignature = getItemsSignature(items);
  if (itemsSignature !== lastItemsSignature.current) {
    lastItemsSignature.current = itemsSignature;
    setRenderedItems(items);
  }

  const isControlled = controlledValue !== null && controlledValue !== undefined;
  const currentValue =
    (isControlled ? controlledValue : uncontrolledValue) ??
    renderedItems[firstEnabledIndex(renderedItems)]?.value ??
    null;
  const selectedIndex = renderedItems.findIndex((item) => item.value === currentValue);
  const hasPanel = children !== undefined;
  const isVertical = orientation === "vertical";
  const hasTooltips = isVertical || showTooltips;
  const canCollapse = collapseWhenOverflow && !isVertical;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedIconSize = resolveSupportingVisualSize(resolvedSize);
  const selectedItem = renderedItems.find((item) => item.value === currentValue) ?? null;
  const collapseTriggerLabel = collapseLabel ?? selectedItem?.label ?? "Sections";
  const collapsedMenuItems = renderedItems.map((item) => ({
    value: item.value,
    label: item.count === undefined ? item.label : `${item.label} (${item.count})`,
    disabled: item.disabled,
    kind: "radio" as const,
    checked: item.value === currentValue,
  }));

  // Selection seeds the roving tab stop, but manual activation may then move
  // focus without moving selection. Deriving the tab stop from selection on
  // every render made React's focused manual tab remain `tabIndex=-1`.
  useEffect(() => {
    if (selectedIndex >= 0) setFocusIndex(selectedIndex);
  }, [selectedIndex]);
  const effectiveFocusIndex = focusIndex;

  const machineContextRef = useRef<HeadlessTabsContext<TabItem> | null>(null);
  machineContextRef.current = {
    items: renderedItems,
    value: currentValue,
    focusIndex: effectiveFocusIndex,
    activationMode,
    reorderable,
  };

  function setValue(nextValue: string): void {
    if (!isControlled) {
      setUncontrolledValue(nextValue);
    }

    onValueChange?.(nextValue);
  }

  function send(event: HeadlessTabsEvent): void {
    const context = machineContextRef.current!;
    const result = tabsTransition(context, event);

    if (result.context.items !== context.items) {
      setRenderedItems(result.context.items);
    }

    if (result.context.focusIndex !== context.focusIndex) {
      setFocusIndex(result.context.focusIndex);
    }

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitValueChange": {
          setValue(effect.value);
          break;
        }
        case "focusTab": {
          // The button follows the tab's value across keyed reorder; index slots do not.
          pendingTabFocus.current = result.context.items[effect.index]?.value ?? null;
          break;
        }
        case "emitReorder": {
          onReorder?.(effect.order);
          break;
        }
        case "emitClose": {
          onClose?.(effect.value);
          break;
        }
      }
    }
  }

  // focusTab effects run after the render they were emitted in.
  useEffect(() => {
    if (pendingTabFocus.current === null) return;
    tabRefs.current[pendingTabFocus.current]?.focus();
    pendingTabFocus.current = null;
  });

  // ── Tooltip (vertical icon-only mode) ──

  function clearTooltip(): void {
    if (tooltipTimer.current) {
      clearTimeout(tooltipTimer.current);
      tooltipTimer.current = null;
    }
  }

  function scheduleTooltip(index: number): void {
    clearTooltip();
    tooltipTimer.current = setTimeout(() => setTooltipIndex(index), 300);
  }

  function dismissTooltip(): void {
    clearTooltip();
    setTooltipIndex(null);
  }

  useEffect(() => clearTooltip, []);

  // ── Overflow collapse ──

  const isShedding = overflowStrategy === "shed" && !isVertical;

  useEffect(() => {
    if (!canCollapse && !isShedding) {
      setCollapsedByOverflow(false);
      setShedCount(0);
      return;
    }

    /**
     * Walk the ladder: full → shed the first part → shed the second → collapse.
     *
     * Measurements come from the hidden measure list, which stays at full
     * fidelity at rest; the shed attribute is set and removed inside this
     * function so no paint sees it. Measuring the real strip instead would
     * oscillate — shedding icons narrows it, which says icons fit, which puts
     * them back.
     */
    function evaluate(): void {
      const root = rootRef.current;
      const measureList = measureListRef.current;
      if (!root || !measureList) return;

      if (measuringRef.current) return;
      measuringRef.current = true;

      const availableWidth = root.getBoundingClientRect().width;
      const fits = (level: number): boolean => {
        measureList.dataset.shed = shed.slice(0, level).join(" ");
        return measureList.getBoundingClientRect().width <= availableWidth + 1;
      };

      try {
        if (!isShedding) {
          setCollapsedByOverflow(!fits(0));
          setShedCount(0);
          return;
        }

        for (let level = 0; level <= shed.length; level += 1) {
          if (fits(level)) {
            setShedCount(level);
            setCollapsedByOverflow(false);
            return;
          }
        }

        setCollapsedByOverflow(canCollapse);
        // Once collapsed there is no strip to shed, and leaving the state set
        // hides the icon on the menu's own trigger. Parts return with the menu.
        setShedCount(canCollapse ? 0 : shed.length);
      } finally {
        delete measureList.dataset.shed;
        // Released after a frame: the observer fires asynchronously, so
        // clearing synchronously would let the restore notification re-enter.
        requestAnimationFrame(() => {
          measuringRef.current = false;
        });
      }
    }

    evaluate();

    const resizeObserver = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(evaluate);
    if (resizeObserver) {
      if (rootRef.current) resizeObserver.observe(rootRef.current);
      if (measureListRef.current) resizeObserver.observe(measureListRef.current);
    }
    window.addEventListener("resize", evaluate);

    return () => {
      resizeObserver?.disconnect();
      window.removeEventListener("resize", evaluate);
    };
    // `isShedding` and `shed` belong here: a strategy or order change has to
    // re-run the ladder, not wait for the next resize.
  }, [canCollapse, isShedding, shed, itemsSignature, resolvedSize, resolvedDensity, variant, actions]);

  // ── URL history sync ──

  const currentValueRef = useRef(currentValue);
  currentValueRef.current = currentValue;
  const renderedItemsRef = useRef(renderedItems);
  renderedItemsRef.current = renderedItems;

  function replaceUrlTabParam(nextValue: string): void {
    if (!historyKey) return;
    const url = new URL(window.location.href);
    const fallback = renderedItemsRef.current[firstEnabledIndex(renderedItemsRef.current)]?.value ?? null;
    if (fallback && nextValue === fallback) {
      url.searchParams.delete(historyKey);
    } else {
      url.searchParams.set(historyKey, nextValue);
    }
    window.history.replaceState(window.history.state, "", url);
  }

  useEffect(() => {
    if (!historyKey) {
      historyReady.current = true;
      return;
    }

    const urlValue = new URL(window.location.href).searchParams.get(historyKey);
    if (urlValue) {
      setValue(urlValue);
      lastSyncedValue.current = urlValue;
    } else if (currentValueRef.current) {
      replaceUrlTabParam(currentValueRef.current);
      lastSyncedValue.current = currentValueRef.current;
    }

    function handlePopState(): void {
      const nextValue = new URL(window.location.href).searchParams.get(historyKey!);
      if (nextValue && nextValue !== currentValueRef.current) {
        setValue(nextValue);
        lastSyncedValue.current = nextValue;
        return;
      }

      if (!nextValue) {
        const fallbackValue =
          renderedItemsRef.current[firstEnabledIndex(renderedItemsRef.current)]?.value ?? null;
        if (fallbackValue && fallbackValue !== currentValueRef.current) {
          setValue(fallbackValue);
          lastSyncedValue.current = fallbackValue;
        }
      }
    }

    window.addEventListener("popstate", handlePopState);
    historyReady.current = true;

    return () => window.removeEventListener("popstate", handlePopState);
  }, [historyKey]);

  useEffect(() => {
    if (!historyReady.current) return;
    if (historyKey && currentValue && currentValue !== lastSyncedValue.current) {
      replaceUrlTabParam(currentValue);
      lastSyncedValue.current = currentValue;
    }
  }, [historyKey, currentValue]);

  // ── Reorder (shared drag substrate; local and cross-window are one session) ──

  /**
   * Join the nearest provider, or own a controller.
   *
   * Joining is what lets an owning composite arbitrate a tab drop against its
   * own targets. Isolation does not depend on the controller: it comes from
   * the subject family and the registration ids, so a Tabs that joined a
   * shared provider is still unreachable from an ordinary sibling strip.
   */
  const ambient = useOptionalDragDrop();
  const [ownDragController] = useState(() => (ambient ? null : createDragDropController()));
  const dragController = ambient?.controller ?? ownDragController!;

  /**
   * The semantic family, and the registration namespace, are different things.
   *
   * `subjectKind` is what other surfaces can match on — shared when a
   * composite says so. `sourceId` / `targetId` are always scoped to this
   * instance, because two strips in one ambient controller may legitimately
   * hold the same tab values and duplicate live ids are an error, not
   * last-writer-wins.
   */
  const subjectKind = dragSubjectKind ?? `poodle.reorder-item:tabs:${tabsId}`;
  const registrationScope = `tabs:${tabsId}`;

  function sourceIdOf(value: string): string {
    return `${registrationScope}:source:${value}`;
  }

  function targetIdOf(value: string): string {
    return `${registrationScope}:target:${value}`;
  }

  function valueOfTargetId(targetId: string): string {
    const prefix = `${registrationScope}:target:`;
    return targetId.startsWith(prefix) ? targetId.slice(prefix.length) : "";
  }

  function indexOfValue(value: string): number {
    return renderedItemsRef.current.findIndex((item) => item.value === value);
  }

  /** Whether a subject belongs to this strip at all. */
  function ownsValue(value: string): boolean {
    return indexOfValue(value) >= 0;
  }

  /**
   * Turn one revalidated intent into the machine's reorder.
   *
   * `before`/`after` are relative to the target's own position, and
   * `applyReorder` splices into the *shortened* array, so the index shifts by
   * one when the tab is moving forward.
   */
  function handleDrop(intent: DropIntent): DragDropCommitResult {
    const subjectId = dragController.getSnapshot().session?.subject.id ?? "";
    const from = indexOfValue(subjectId);
    const target = indexOfValue(valueOfTargetId(intent.targetId));
    if (from < 0) {
      if (!onForeignDrop || target < 0) {
        return { status: "rejected", reason: "same tab" };
      }
      onForeignDrop(subjectId, intent.position === "after" ? target + 1 : target);
      return { status: "committed" };
    }
    if (target < 0 || from === target) {
      return { status: "rejected", reason: "same tab" };
    }

    const to =
      intent.position === "before"
        ? from < target
          ? target - 1
          : target
        : from < target
          ? target
          : target + 1;

    if (from === to) {
      return { status: "rejected", reason: "same tab" };
    }

    send({ type: "REORDER", fromIndex: from, toIndex: to });
    return { status: "committed" };
  }

  /** Alt+Arrow: the established one-keystroke move, run as a real session. */
  function moveTab(fromIndex: number, direction: -1 | 1): void {
    const from = renderedItems[fromIndex];
    const target = renderedItems[fromIndex + direction];
    if (!from || !target) return;
    dragController.requestKeyboardDrop({
      sourceId: sourceIdOf(from.value),
      targetId: targetIdOf(target.value),
      position: direction === 1 ? "after" : "before",
    });
  }

  function handleKeydown(event: ReactKeyboardEvent, index: number): void {
    const item = renderedItems[index];

    if (event.key === "Delete" && item?.closable) {
      event.preventDefault();
      send({ type: "CLOSE", value: item.value });
      return;
    }

    const machineEvent = tabsKeydownEvent(
      event.key,
      event.altKey,
      orientation,
      { reorderable, activationMode },
      index,
    );

    if (!machineEvent) return;
    event.preventDefault();

    // The reorder step is the one machine event the substrate owns: it must
    // reach the same session, announcement, and revalidation a pointer drop
    // does rather than mutating the order behind the controller's back.
    if (machineEvent.type === "REORDER_STEP") {
      moveTab(machineEvent.fromIndex ?? index, machineEvent.direction);
      return;
    }

    send(machineEvent);
  }

  function tabContent(item: TabItem): ReactNode {
    return (
      <>
        {item.icon ? <Icon icon={item.icon} size={resolvedIconSize} /> : null}
        <span className="poodle-tabs__label">{item.label}</span>
        {item.count !== undefined ? (
          <Pill tone="neutral" appearance="badge" size={resolvedIconSize} muted adaptiveWidth ariaLabel={`${item.count}`}>
            {item.count}
          </Pill>
        ) : null}
      </>
    );
  }

  const strip = (
    <>
      <TabsKeyboardTargets
        items={renderedItems}
        reorderable={reorderable}
        subjectKind={subjectKind}
        targetIdOf={targetIdOf}
        ownsValue={ownsValue}
        onDrop={handleDrop}
      />
      <div
        ref={rootRef}
        className="poodle-tabs"
      data-variant={variant}
      data-bordered={bordered}
      data-active-edge={activeEdge}
      data-active-fill={activeFill}
      data-orientation={orientation}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-collapsed={collapsedByOverflow || undefined}
      data-shed={shedCount > 0 ? shed.slice(0, shedCount).join(" ") : undefined}
      data-full-width={fullWidth || undefined}
    >
      {canCollapse ? (
        <div className="poodle-tabs__measure-shell" aria-hidden="true">
          <div ref={measureListRef} className="poodle-tabs__list poodle-tabs__list--measure">
            {renderedItems.map((item, index) => (
              <Fragment key={item.value}>
                <div className="poodle-tabs__item" role="presentation" data-selected={currentValue === item.value}>
                  <span className="poodle-tabs__tab">{tabContent(item)}</span>

                  {item.closable ? (
                    <span className="poodle-tabs__close" aria-hidden="true">
                      <Icon name="x" size={resolvedIconSize} />
                    </span>
                  ) : null}
                </div>
                {item.separator && index < renderedItems.length - 1 ? (
                  <span className="poodle-tabs__separator" aria-hidden="true" />
                ) : null}
              </Fragment>
            ))}
          </div>
        </div>
      ) : null}

      {collapsedByOverflow ? (
        <div className="poodle-tabs__collapsed">
          <Menu
            items={collapsedMenuItems}
            ariaLabel={ariaLabel ?? "Sections"}
            triggerAriaLabel={ariaLabel ?? "Sections"}
            size={resolvedSize}
            density={resolvedDensity}
            onAction={(nextValue) => send({ type: "SELECT", value: nextValue })}
            trigger={
              <Button
                type="button"
                variant="secondary"
                size={resolvedSize}
                density={resolvedDensity}
                leadingIcon="menu"
                chevron
                ariaLabel={ariaLabel ?? collapseTriggerLabel}
              >
                {collapseTriggerLabel}
              </Button>
            }
          />

          {actions ? <div className="poodle-tabs__actions">{actions}</div> : null}
        </div>
      ) : (
        <div className="poodle-tabs__list" role="tablist" aria-label={ariaLabel ?? undefined} aria-orientation={orientation}>
          {renderedItems.map((item, index) => (
            <Fragment key={item.value}>
              <TabsItem
                item={item}
                index={index}
                tabsId={tabsId}
                subjectKind={subjectKind}
                reorderable={reorderable}
                hasPanel={hasPanel}
                crossWindowSourceBridge={crossWindowSourceBridge}
                indexOfValue={indexOfValue}
                ownsValue={ownsValue}
                acceptsForeign={onForeignDrop !== undefined}
                isVertical={isVertical}
                sourceId={sourceIdOf(item.value)}
                targetId={targetIdOf(item.value)}
                selected={currentValue === item.value}
                focused={effectiveFocusIndex === index}
                iconSize={resolvedIconSize}
                onDrop={handleDrop}
                onElement={(element) => {
                  tabRefs.current[item.value] = element;
                }}
                onSelect={() => send({ type: "SELECT", value: item.value })}
                onClose={() => send({ type: "CLOSE", value: item.value })}
                onFocus={() => {
                  setFocusIndex(index);
                  if (isVertical) scheduleTooltip(index);
                }}
                onBlur={() => hasTooltips && dismissTooltip()}
                onEnter={() => hasTooltips && scheduleTooltip(index)}
                onLeave={() => hasTooltips && dismissTooltip()}
                onKeyDown={(event) => {
                  if (event.key === "Escape" && hasTooltips) dismissTooltip();
                  handleKeydown(event, index);
                }}
                content={tabContent(item)}
                tooltip={
                  hasTooltips && tooltipIndex === index ? (
                    <AnchoredSurface
                      tag="span"
                      anchor={tooltipAnchor}
                      placement={isVertical ? "right" : "bottom"}
                      offset={6}
                      className="poodle-tabs__tooltip"
                      data-placement={isVertical ? "right" : "bottom"}
                      role="tooltip"
                    >
                      {item.label}
                    </AnchoredSurface>
                  ) : null
                }
              />
              {item.separator && index < renderedItems.length - 1 ? (
                <span className="poodle-tabs__separator" aria-hidden="true" />
              ) : null}
            </Fragment>
          ))}

          {actions ? <div className="poodle-tabs__actions">{actions}</div> : null}
        </div>
      )}

      {hasPanel && currentValue ? (
        <div
          className="poodle-tabs__panel"
          id={`poodle-tabpanel-${tabsId}-${currentValue}`}
          data-value={currentValue}
          role="tabpanel"
          tabIndex={0}
          aria-labelledby={`poodle-tab-${tabsId}-${currentValue}`}
        >
          {children?.(currentValue)}
        </div>
      ) : null}
      </div>
    </>
  );

  // A Tabs that joined a provider contributes registrations to it. One with no
  // provider owns a controller so it still reorders on its own.
  return ambient ? strip : (
    <DragDropProvider controller={ownDragController!}>{strip}</DragDropProvider>
  );
}
