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

import type { TransitionResult } from "./machine";

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
