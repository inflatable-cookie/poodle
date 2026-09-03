/**
 * Tabs behavior machine.
 * Contract: docs/contracts/components/tabs.md, "Behavior Machine".
 *
 * Selection + roving tabindex. Items are part of context and are supplied by
 * the caller on every transition (the styled layer owns item rendering and
 * removal — CLOSE only emits a request). Drag-and-drop DOM plumbing stays in
 * the adapter; the final reorder routes through REORDER. URL-history sync and
 * overflow measurement are adapter effects that re-enter via SELECT.
 */

import type { PartAttrs, TransitionResult } from "./machine";
import { findNextEnabledIndex, firstEnabledIndex } from "./nav";

export interface TabsItem {
  value: string;
  disabled?: boolean;
  closable?: boolean;
}

export type TabsState = "idle";

export interface TabsContext<T extends TabsItem = TabsItem> {
  items: T[];
  value: string | null;
  focusIndex: number;
  activationMode: "automatic" | "manual";
  reorderable: boolean;
}

export type TabsEvent =
  | { type: "SELECT"; value: string }
  | { type: "FOCUS_MOVE"; direction: "next" | "prev" | "first" | "last"; fromIndex?: number }
  | { type: "ACTIVATE"; index?: number }
  | { type: "CLOSE"; value: string }
  | { type: "REORDER_STEP"; direction: -1 | 1; fromIndex?: number }
  | { type: "REORDER"; fromIndex: number; toIndex: number };

export type TabsEffect =
  | { type: "emitValueChange"; value: string }
  | { type: "emitReorder"; order: string[] }
  | { type: "emitClose"; value: string }
  | { type: "focusTab"; index: number };

export type TabsResult<T extends TabsItem> = TransitionResult<TabsState, TabsContext<T>, TabsEffect>;

/** Selected value with the contract fallback: first enabled item. */
export function resolveTabsValue<T extends TabsItem>(items: T[], value: string | null): string | null {
  if (value !== null && items.some((item) => item.value === value)) {
    return value;
  }

  return items[firstEnabledIndex(items)]?.value ?? null;
}

export function applyReorder<T>(
  items: T[],
  fromIndex: number,
  toIndex: number,
): { items: T[]; focusIndex: number } {
  if (fromIndex === toIndex) {
    return { items: [...items], focusIndex: fromIndex };
  }

  const nextItems = [...items];
  const [moved] = nextItems.splice(fromIndex, 1);

  if (moved === undefined) {
    return { items: [...items], focusIndex: fromIndex };
  }

  nextItems.splice(toIndex, 0, moved);

  return { items: nextItems, focusIndex: toIndex };
}

function select<T extends TabsItem>(context: TabsContext<T>, value: string): TabsResult<T> {
  const index = context.items.findIndex((item) => item.value === value);

  if (index < 0 || context.items[index]?.disabled) {
    return { state: "idle", context, effects: [] };
  }

  return {
    state: "idle",
    context: { ...context, value, focusIndex: index },
    effects: [{ type: "emitValueChange", value }],
  };
}

function reorder<T extends TabsItem>(
  context: TabsContext<T>,
  fromIndex: number,
  toIndex: number,
): TabsResult<T> {
  if (
    !context.reorderable ||
    fromIndex < 0 ||
    toIndex < 0 ||
    fromIndex >= context.items.length ||
    toIndex >= context.items.length
  ) {
    return { state: "idle", context, effects: [] };
  }

  const result = applyReorder(context.items, fromIndex, toIndex);

  return {
    state: "idle",
    context: { ...context, items: result.items, focusIndex: result.focusIndex },
    effects: [
      { type: "focusTab", index: result.focusIndex },
      { type: "emitReorder", order: result.items.map((item) => item.value) },
    ],
  };
}

export function tabsTransition<T extends TabsItem>(
  context: TabsContext<T>,
  event: TabsEvent,
): TabsResult<T> {
  const stay: TabsResult<T> = { state: "idle", context, effects: [] };

  switch (event.type) {
    case "SELECT":
      return select(context, event.value);

    case "FOCUS_MOVE": {
      // Keyboard events originate on a specific tab; that tab wins over the
      // tracked focusIndex when the two diverge.
      const fromIndex = event.fromIndex ?? context.focusIndex;
      let nextIndex: number;

      switch (event.direction) {
        case "next":
          nextIndex = findNextEnabledIndex(context.items, fromIndex, 1);
          break;
        case "prev":
          nextIndex = findNextEnabledIndex(context.items, fromIndex, -1);
          break;
        case "first":
          nextIndex = firstEnabledIndex(context.items);
          break;
        case "last":
          nextIndex = findNextEnabledIndex(context.items, 0, -1);
          break;
      }

      if (nextIndex < 0) {
        return stay;
      }

      const moved: TabsResult<T> = {
        state: "idle",
        context: { ...context, focusIndex: nextIndex },
        effects: [{ type: "focusTab", index: nextIndex }],
      };

      if (context.activationMode === "automatic") {
        const nextValue = context.items[nextIndex]?.value;

        if (nextValue !== undefined && nextValue !== context.value) {
          return {
            state: "idle",
            context: { ...moved.context, value: nextValue },
            effects: [...moved.effects, { type: "emitValueChange", value: nextValue }],
          };
        }
      }

      return moved;
    }

    case "ACTIVATE": {
      if (context.activationMode !== "manual") {
        return stay;
      }

      const value = context.items[event.index ?? context.focusIndex]?.value;

      return value === undefined ? stay : select(context, value);
    }

    case "CLOSE": {
      const item = context.items.find((candidate) => candidate.value === event.value);

      if (!item?.closable) {
        return stay;
      }

      return { state: "idle", context, effects: [{ type: "emitClose", value: event.value }] };
    }

    case "REORDER_STEP": {
      const fromIndex = event.fromIndex ?? context.focusIndex;

      return reorder(context, fromIndex, fromIndex + event.direction);
    }

    case "REORDER":
      return reorder(context, event.fromIndex, event.toIndex);
  }
}

/**
 * Map a tablist keydown to a machine event. Returns null for keys the
 * machine does not own (adapter lets them propagate).
 */
export function tabsKeydownEvent(
  key: string,
  altKey: boolean,
  orientation: "horizontal" | "vertical",
  context: { reorderable: boolean; activationMode: "automatic" | "manual" },
  fromIndex?: number,
): TabsEvent | null {
  const horizontal = orientation === "horizontal";
  const nextKey = horizontal ? "ArrowRight" : "ArrowDown";
  const prevKey = horizontal ? "ArrowLeft" : "ArrowUp";

  if (key === nextKey) {
    return context.reorderable && altKey
      ? { type: "REORDER_STEP", direction: 1, fromIndex }
      : { type: "FOCUS_MOVE", direction: "next", fromIndex };
  }

  if (key === prevKey) {
    return context.reorderable && altKey
      ? { type: "REORDER_STEP", direction: -1, fromIndex }
      : { type: "FOCUS_MOVE", direction: "prev", fromIndex };
  }

  if (key === "Home") {
    return { type: "FOCUS_MOVE", direction: "first" };
  }

  if (key === "End") {
    return { type: "FOCUS_MOVE", direction: "last" };
  }

  if (context.activationMode === "manual" && (key === "Enter" || key === " ")) {
    return { type: "ACTIVATE", index: fromIndex };
  }

  return null;
}

// ── Tooltip sub-machine (vertical / showTooltips mode) ──
// Adapters must not send ENTER for a disabled item; that target stays hidden.

export type TabsTooltipState =
  | { name: "hidden" }
  | { name: "pending"; index: number }
  | { name: "visible"; index: number };

export type TabsTooltipEvent =
  | { type: "POINTER_ENTER"; index: number }
  | { type: "FOCUS_ENTER"; index: number }
  | { type: "TIMER_FIRE" }
  | { type: "POINTER_LEAVE" };

export type TabsTooltipEffect = { type: "startTimer" } | { type: "clearTimer" };

export function tabsTooltipTransition(
  state: TabsTooltipState,
  event: TabsTooltipEvent,
): { state: TabsTooltipState; effects: TabsTooltipEffect[] } {
  switch (event.type) {
    case "POINTER_ENTER":
    case "FOCUS_ENTER":
      return {
        state: { name: "pending", index: event.index },
        effects: [{ type: "clearTimer" }, { type: "startTimer" }],
      };
    case "TIMER_FIRE":
      return state.name === "pending"
        ? { state: { name: "visible", index: state.index }, effects: [] }
        : { state, effects: [] };
    case "POINTER_LEAVE":
      return { state: { name: "hidden" }, effects: [{ type: "clearTimer" }] };
  }
}

// ── Part attribute output ──

export interface TabsPartProps {
  instanceId: string;
  ariaLabel?: string | null;
  orientation: "horizontal" | "vertical";
  hasPanel: boolean;
}

export function tabsListParts(props: TabsPartProps): PartAttrs {
  return {
    "data-scope": "tabs",
    "data-part": "list",
    role: "tablist",
    "aria-label": props.ariaLabel ?? undefined,
    "aria-orientation": props.orientation,
  };
}

export function tabsTabId(instanceId: string, value: string): string {
  return `${instanceId}-tab-${value}`;
}

export function tabsPanelId(instanceId: string, value: string): string {
  return `${instanceId}-panel-${value}`;
}

export function tabsTabParts<T extends TabsItem>(
  context: TabsContext<T>,
  props: TabsPartProps,
  index: number,
): PartAttrs {
  const item = context.items[index];
  const value = item?.value ?? "";
  const selected = resolveTabsValue(context.items, context.value) === value;

  return {
    "data-part": "trigger",
    "data-state": selected ? "active" : "inactive",
    id: tabsTabId(props.instanceId, value),
    role: "tab",
    tabindex: context.focusIndex === index ? 0 : -1,
    "aria-selected": selected ? "true" : "false",
    "aria-controls": props.hasPanel ? tabsPanelId(props.instanceId, value) : undefined,
    disabled: item?.disabled === true,
  };
}

export function tabsPanelParts<T extends TabsItem>(
  context: TabsContext<T>,
  props: TabsPartProps,
): PartAttrs {
  const value = resolveTabsValue(context.items, context.value) ?? "";

  return {
    "data-part": "panel",
    "data-state": "active",
    id: tabsPanelId(props.instanceId, value),
    role: "tabpanel",
    tabindex: 0,
    "aria-labelledby": tabsTabId(props.instanceId, value),
  };
}

/**
 * Web-only controlled-panel focus policy. `"preserve"` keeps today's behaviour.
 * `"selected-tab"` focuses the newly selected enabled tab after render when
 * the outgoing selected panel owned focus.
 */
export type TabsFocusOnValueChange = "preserve" | "selected-tab";

/**
 * Decide the pending destination for a controlled value change.
 *
 * Capture requires focus inside the outgoing panel. Once a request exists,
 * later controlled changes retarget it so only the latest destination applies.
 * Preserve, uncontrolled, and unchanged values never start a transfer.
 */
export function nextTabsControlledFocusDestination(input: {
  policy: TabsFocusOnValueChange;
  controlled: boolean;
  previousValue: string | null;
  nextValue: string | null;
  focusWasInOutgoingPanel: boolean;
  pendingValue: string | null;
}): string | null {
  if (!input.controlled || input.policy !== "selected-tab") {
    return null;
  }

  if (input.previousValue === input.nextValue) {
    return input.pendingValue;
  }

  if (input.nextValue === null) {
    return null;
  }

  if (input.focusWasInOutgoingPanel || input.pendingValue !== null) {
    return input.nextValue;
  }

  return null;
}

/** Apply a pending request only while Tabs is alive and the destination exists and is enabled. */
export function resolveTabsControlledFocusDestination(input: {
  pendingValue: string | null;
  items: ReadonlyArray<{ value: string; disabled?: boolean }>;
  alive: boolean;
}): string | null {
  if (!input.alive || input.pendingValue === null) {
    return null;
  }

  const item = input.items.find((candidate) => candidate.value === input.pendingValue);

  if (item === undefined || item.disabled === true) {
    return null;
  }

  return item.value;
}
