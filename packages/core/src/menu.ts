/**
 * Menu open/close behavior machine (Menu, ContextMenu).
 * Contracts: docs/contracts/components/menu.md, context-menu.md,
 * "Behavior Machine" sections.
 *
 * Owns overlay state and the action-then-close sequence. Item navigation
 * (roving focus, typeahead) lives in the menu surface and joins the machine
 * in a later batch. Unlike Popover, closing does not restore trigger focus
 * (matches pre-machine behavior).
 */

import type { TransitionResult } from "./machine.ts";

export type MenuState = "closed" | "open";

export interface MenuContext {
  disabled?: boolean;
}

export type MenuEvent =
  | { type: "TOGGLE" }
  | { type: "OPEN" }
  | { type: "CLOSE" }
  | { type: "ESCAPE" }
  | { type: "OUTSIDE_INTERACT" }
  | { type: "ACTION"; value: string };

export type MenuEffect =
  | { type: "emitOpenChange"; open: boolean }
  | { type: "emitAction"; value: string }
  | { type: "focusFirstItem" };

export type MenuResult = TransitionResult<MenuState, MenuContext, MenuEffect>;

function open(context: MenuContext): MenuResult {
  return {
    state: "open",
    context,
    effects: [
      { type: "emitOpenChange", open: true },
      { type: "focusFirstItem" },
    ],
  };
}

function close(context: MenuContext, extra: MenuEffect[] = []): MenuResult {
  return {
    state: "closed",
    context,
    effects: [...extra, { type: "emitOpenChange", open: false }],
  };
}

// ── Menu list navigation machinery (MenuSurface) ──

export interface MenuListItem {
  value: string;
  disabled?: boolean;
  kind?: string;
}

/** Items that participate in keyboard navigation (separators excluded). */
export function menuNavigableItems<T extends { kind?: string }>(items: T[]): T[] {
  return items.filter((item) => item.kind !== "separator");
}

export type MenuListMove = "next" | "prev" | "first" | "last";

/**
 * Next highlight index for a keyboard move: wraps and skips disabled items;
 * `first`/`last` land on the enabled boundaries. Returns the current index
 * when no enabled item exists in the requested direction.
 */
export function menuListNavigate<T extends MenuListItem>(
  items: T[],
  highlightIndex: number,
  move: MenuListMove,
): number {
  if (items.length === 0) {
    return 0;
  }

  switch (move) {
    case "next":
      return wrapNavigate(items, highlightIndex, 1);
    case "prev":
      return wrapNavigate(items, highlightIndex, -1);
    case "first": {
      const first = items.findIndex((item) => !item.disabled);
      return first < 0 ? 0 : first;
    }
    case "last": {
      for (let index = items.length - 1; index >= 0; index -= 1) {
        if (!items[index]?.disabled) {
          return index;
        }
      }
      return 0;
    }
  }
}

function wrapNavigate<T extends MenuListItem>(items: T[], startIndex: number, direction: 1 | -1): number {
  const count = items.length;
  let index = startIndex;

  for (let step = 0; step < count; step += 1) {
    index = (index + direction + count) % count;

    if (!items[index]?.disabled) {
      return index;
    }
  }

  return startIndex;
}

/** Activation guard: disabled items and separators do not activate. */
export function menuListCanActivate(item: { disabled?: boolean; kind?: string }): boolean {
  return !item.disabled && item.kind !== "separator";
}

/**
 * Submenu parent guard: an enabled non-separator item with at least one
 * child renders as a flyout parent. Parents open their submenu instead of
 * emitting an action; only leaf items activate.
 */
export function menuItemHasSubmenu(item: {
  disabled?: boolean;
  kind?: string;
  children?: unknown[];
}): boolean {
  return (
    item.kind !== "separator" &&
    Array.isArray(item.children) &&
    item.children.length > 0
  );
}

export function menuTransition(state: MenuState, context: MenuContext, event: MenuEvent): MenuResult {
  const stay: MenuResult = { state, context, effects: [] };

  if (context.disabled) {
    return stay;
  }

  switch (event.type) {
    case "TOGGLE":
      return state === "closed" ? open(context) : close(context);
    case "OPEN":
      return state === "closed" ? open(context) : stay;
    case "CLOSE":
    case "ESCAPE":
    case "OUTSIDE_INTERACT":
      return state === "open" ? close(context) : stay;
    case "ACTION":
      return state === "open"
        ? close(context, [{ type: "emitAction", value: event.value }])
        : stay;
  }
}
