/**
 * Popover behavior machine.
 * Contract: docs/contracts/components/popover.md, "Behavior Machine".
 *
 * States: closed | open. Focus restoration runs on every close path
 * (explicit, escape, outside). Dismissal wiring is the adapter's job via
 * registerDismissLayer while open; positioning stays CSS-anchored (documented
 * delta until the shared anchor-positioning service lands).
 */

import type { PartAttrs, TransitionResult } from "./machine.ts";

export type PopoverState = "closed" | "open";

export type PopoverInitialFocus = "first-focusable" | "content" | "none";

export interface PopoverContext {
  disabled: boolean;
  dismissOnOutsideInteract: boolean;
  initialFocus: PopoverInitialFocus;
}

export type PopoverEvent =
  | { type: "TOGGLE" }
  | { type: "OPEN" }
  | { type: "CLOSE" }
  | { type: "ESCAPE" }
  | { type: "OUTSIDE_INTERACT" };

export type PopoverEffect =
  | { type: "emitOpenChange"; open: boolean }
  | { type: "focusOnOpen"; strategy: PopoverInitialFocus }
  | { type: "restoreTriggerFocus" };

export type PopoverResult = TransitionResult<PopoverState, PopoverContext, PopoverEffect>;

function open(context: PopoverContext): PopoverResult {
  return {
    state: "open",
    context,
    effects: [
      { type: "emitOpenChange", open: true },
      { type: "focusOnOpen", strategy: context.initialFocus },
    ],
  };
}

function close(context: PopoverContext): PopoverResult {
  return {
    state: "closed",
    context,
    effects: [
      { type: "emitOpenChange", open: false },
      { type: "restoreTriggerFocus" },
    ],
  };
}

export function popoverTransition(
  state: PopoverState,
  context: PopoverContext,
  event: PopoverEvent,
): PopoverResult {
  const stay: PopoverResult = { state, context, effects: [] };

  if (context.disabled) {
    return stay;
  }

  switch (event.type) {
    case "TOGGLE":
      return state === "closed" ? open(context) : close(context);
    case "OPEN":
      return state === "closed" ? open(context) : stay;
    case "CLOSE":
      return state === "open" ? close(context) : stay;
    case "ESCAPE":
      return state === "open" ? close(context) : stay;
    case "OUTSIDE_INTERACT":
      return state === "open" && context.dismissOnOutsideInteract ? close(context) : stay;
  }
}

export interface PopoverPartProps {
  surfaceId: string;
  ariaLabel?: string | null;
  block: boolean;
  placement: string;
  surfaceWidth: "content" | "trigger";
}

export interface PopoverParts {
  root: PartAttrs;
  trigger: PartAttrs;
  surface: PartAttrs;
}

export function popoverParts(
  state: PopoverState,
  context: PopoverContext,
  props: PopoverPartProps,
): PopoverParts {
  const isOpen = state === "open";

  return {
    root: {
      "data-scope": "popover",
      "data-part": "root",
      "data-state": state,
      "data-block": props.block,
    },
    trigger: {
      "data-part": "trigger",
      "data-state": state,
      "data-block": props.block,
      "data-disabled": context.disabled,
      role: "button",
      tabindex: context.disabled ? -1 : 0,
      "aria-disabled": context.disabled ? "true" : undefined,
      "aria-expanded": isOpen ? "true" : "false",
      "aria-controls": isOpen ? props.surfaceId : undefined,
    },
    surface: {
      "data-part": "surface",
      "data-state": state,
      "data-placement": props.placement,
      "data-surface-width": props.surfaceWidth,
      id: props.surfaceId,
      role: "dialog",
      "aria-label": props.ariaLabel ?? undefined,
      tabindex: context.initialFocus === "content" ? 0 : -1,
    },
  };
}
