import "@inflatable-cookie/poodle-core/styles/tabs.css";

import {
  Fragment,
  useEffect,
  useId,
  useRef,
  useState,
  type DragEvent as ReactDragEvent,
  type KeyboardEvent as ReactKeyboardEvent,
  type PointerEvent as ReactPointerEvent,
  type ReactNode,
} from "react";

import {
  firstEnabledIndex,
  tabsKeydownEvent,
  tabsTransition,
  type TabsContext as HeadlessTabsContext,
  type TabsEvent as HeadlessTabsEvent,
} from "@inflatable-cookie/poodle-core";

import { AnchoredSurface } from "./AnchoredSurface";
import { Button } from "./Button";
import { Icon } from "./Icon";
import { Menu } from "./Menu";
import { Pill } from "./Pill";
import {
  handleDragStart as startDrag,
  handleDragOver as overDrag,
  handleDrop as dropDrag,
} from "./tabs-reorder";
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
  TabActivationMode,
  TabItem,
  TabVariant,
} from "./types";

/** @deprecated Use TabItem instead (pilot-era alias). */
export type TabsItem = TabItem;

export interface TabsProps {
  value?: string | null;
  defaultValue?: string | null;
  items?: TabItem[];
  variant?: TabVariant;
  /**
   * Opt-in outline on the active tab — the decoration the former `card`
   * variant had by default. Reuses the old card selected-border value so
   * the opted-in look matches what card gave.
   */
  activeOutline?: boolean;
  /**
   * Selection treatment on the active tab: `"tint"` is the accent-tinted
   * fill; `"solid"` fills the tab with `accent-base` and switches the
   * foreground to `text-inverse` for contrast.
   */
  activeFill?: "tint" | "solid";
  orientation?: Orientation;
  activationMode?: TabActivationMode;
  bordered?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
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
  fullWidth?: boolean;
  collapseLabel?: string | null;
  reorderable?: boolean;
  ariaLabel?: string | null;
  showTooltips?: boolean;
  historyKey?: string | null;
  onValueChange?: ((value: string) => void) | undefined;
  onReorder?: ((items: string[]) => void) | undefined;
  // Forwarded so a host (DockRegion) can run its own drag session on top of
  // the reorder plumbing; the tab still reorders locally either way.
  onDragPrepare?: ((value: string, event: ReactPointerEvent) => void) | undefined;
  onDragStart?: ((value: string, event: ReactDragEvent) => void) | undefined;
  onDragEnd?: ((value: string, event: ReactDragEvent) => void) | undefined;
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
  activeOutline = false,
  activeFill = "tint",
  orientation = "horizontal",
  activationMode = "automatic",
  bordered = true,
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
  onDragPrepare = undefined,
  onDragStart = undefined,
  onDragEnd = undefined,
  onClose = undefined,
  children,
  actions,
}: TabsProps) {
  const tabsId = useId();
  const uiPresentation = useUiPresentation();

  const rootRef = useRef<HTMLDivElement | null>(null);
  const measureListRef = useRef<HTMLDivElement | null>(null);
  const tabRefs = useRef<Array<HTMLButtonElement | null>>([]);
  const tooltipTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const pendingTabFocus = useRef<number | null>(null);
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
    setTooltipAnchor(tooltipIndex === null ? null : (tabRefs.current[tooltipIndex] ?? null));
  }, [tooltipIndex]);
  const [dragSourceIndex, setDragSourceIndex] = useState<number | null>(null);
  const [dropTargetIndex, setDropTargetIndex] = useState<number | null>(null);
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

  const effectiveFocusIndex = selectedIndex >= 0 ? selectedIndex : focusIndex;

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
          pendingTabFocus.current = effect.index;
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

  // ── Reorder (drag-and-drop DOM plumbing; final reorder routes through the machine) ──

  // The dragged tab's value, kept for `onDragEnd`: dragend fires after the
  // index state has been reset, so reading it back from `dragSourceIndex`
  // would report nothing.
  const dragSourceValue = useRef<string | null>(null);

  function handleDragPrepare(event: ReactPointerEvent, item: TabItem): void {
    if (!reorderable || event.button !== 0 || item.disabled === true) return;
    if (currentValue !== item.value) {
      send({ type: "SELECT", value: item.value });
    }
    onDragPrepare?.(item.value, event);
  }

  function handleDragStart(event: ReactDragEvent, index: number): void {
    const result = startDrag(event.nativeEvent, index, reorderable);
    if (result.dragSourceIndex !== null) {
      setDragSourceIndex(result.dragSourceIndex);
      dragSourceValue.current = renderedItems[index].value;
      onDragStart?.(dragSourceValue.current, event);
    }
  }

  function handleDragOver(event: ReactDragEvent, index: number): void {
    const result = overDrag(event.nativeEvent, index, dragSourceIndex);
    if (result.dropTargetIndex !== null) {
      setDropTargetIndex(result.dropTargetIndex);
    }
  }

  function handleDrop(event: ReactDragEvent, index: number): void {
    const result = dropDrag(event.nativeEvent, index, dragSourceIndex);
    if (result.fromIndex !== null && result.toIndex !== null) {
      send({ type: "REORDER", fromIndex: result.fromIndex, toIndex: result.toIndex });
    }
    setDragSourceIndex(null);
    setDropTargetIndex(null);
  }

  function handleDragEnd(event: ReactDragEvent): void {
    if (dragSourceValue.current) {
      onDragEnd?.(dragSourceValue.current, event);
      dragSourceValue.current = null;
    }
    setDragSourceIndex(null);
    setDropTargetIndex(null);
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

    if (machineEvent) {
      event.preventDefault();
      send(machineEvent);
    }
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

  return (
    <div
      ref={rootRef}
      className="poodle-tabs"
      data-variant={variant}
      data-bordered={bordered}
      data-active-outline={activeOutline || undefined}
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
              <div
                className="poodle-tabs__item"
                role="presentation"
                data-selected={currentValue === item.value}
                data-drag-source={dragSourceIndex === index || undefined}
                data-drop-target={(dropTargetIndex === index && dropTargetIndex !== dragSourceIndex) || undefined}
                draggable={reorderable && !item.disabled}
                onDragStart={(e) => handleDragStart(e, index)}
                onDragOver={(e) => handleDragOver(e, index)}
                onDragLeave={() => setDropTargetIndex(null)}
                onDrop={(e) => handleDrop(e, index)}
                onDragEnd={handleDragEnd}
                onMouseEnter={() => hasTooltips && scheduleTooltip(index)}
                onMouseLeave={() => hasTooltips && dismissTooltip()}
              >
                <button
                  ref={(el) => {
                    tabRefs.current[index] = el;
                  }}
                  type="button"
                  className="poodle-tabs__tab"
                  disabled={item.disabled === true}
                  draggable={reorderable && !item.disabled}
                  id={`poodle-tab-${tabsId}-${item.value}`}
                  role="tab"
                  tabIndex={effectiveFocusIndex === index ? 0 : -1}
                  aria-selected={currentValue === item.value ? "true" : "false"}
                  aria-controls={hasPanel ? `poodle-tabpanel-${tabsId}-${item.value}` : undefined}
                  onFocus={() => {
                    setFocusIndex(index);
                    if (isVertical) scheduleTooltip(index);
                  }}
                  onBlur={() => hasTooltips && dismissTooltip()}
                  onPointerDown={(event) => handleDragPrepare(event, item)}
                  onClick={() => send({ type: "SELECT", value: item.value })}
                  onKeyDown={(event) => {
                    if (event.key === "Escape" && hasTooltips) dismissTooltip();
                    handleKeydown(event, index);
                  }}
                >
                  {tabContent(item)}
                </button>

                {item.closable ? (
                  <button
                    type="button"
                    className="poodle-tabs__close"
                    aria-label={`Close ${item.label}`}
                    onClick={(event) => {
                      event.stopPropagation();
                      send({ type: "CLOSE", value: item.value });
                    }}
                  >
                    <Icon name="x" size={resolvedIconSize} />
                  </button>
                ) : null}

                {hasTooltips && tooltipIndex === index ? (
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
                ) : null}
              </div>
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
          role="tabpanel"
          tabIndex={0}
          aria-labelledby={`poodle-tab-${tabsId}-${currentValue}`}
        >
          {children?.(currentValue)}
        </div>
      ) : null}
    </div>
  );
}
