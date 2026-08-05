/**
 * Hover-intent behavior machine (Tooltip, HoverCard).
 * Contracts: docs/contracts/components/tooltip.md, hover-card.md,
 * "Behavior Machine" sections.
 *
 * States: closed | opening (open-delay pending) | open | closing
 * (close-delay pending). A zero close delay collapses LEAVE/DISMISS into an
 * immediate close. Immediate and timer-driven closes both emit
 * `emitOpenChange(false)` even when the surface never became visible,
 * matching the pre-machine components.
 */

import type { TransitionResult } from "./machine.ts";

export type HoverState = "closed" | "opening" | "open" | "closing";

export interface HoverContext {
  openDelayMs: number;
  closeDelayMs: number;
}

export type HoverEvent =
  | { type: "ENTER" }
  | { type: "LEAVE" }
  | { type: "TIMER_FIRE" }
  | { type: "DISMISS" }
  | { type: "SET_OPEN"; open: boolean };

export type HoverEffect =
  | { type: "startTimer"; ms: number }
  | { type: "clearTimer" }
  | { type: "emitOpenChange"; open: boolean };

export type HoverResult = TransitionResult<HoverState, HoverContext, HoverEffect>;

function closeNow(context: HoverContext): HoverResult {
  return {
    state: "closed",
    context,
    effects: [{ type: "clearTimer" }, { type: "emitOpenChange", open: false }],
  };
}

export function hoverTransition(
  state: HoverState,
  context: HoverContext,
  event: HoverEvent,
): HoverResult {
  const stay: HoverResult = { state, context, effects: [] };

  switch (event.type) {
    case "ENTER": {
      if (state === "open") {
        // Re-entering while open cancels a pending close.
        return { state: "open", context, effects: [{ type: "clearTimer" }] };
      }

      if (state === "closing") {
        return { state: "open", context, effects: [{ type: "clearTimer" }] };
      }

      return {
        state: "opening",
        context,
        effects: [{ type: "clearTimer" }, { type: "startTimer", ms: context.openDelayMs }],
      };
    }

    case "LEAVE": {
      if (state === "closed") {
        return stay;
      }

      if (context.closeDelayMs <= 0) {
        return closeNow(context);
      }

      return {
        state: "closing",
        context,
        effects: [{ type: "clearTimer" }, { type: "startTimer", ms: context.closeDelayMs }],
      };
    }

    case "TIMER_FIRE": {
      if (state === "opening") {
        return { state: "open", context, effects: [{ type: "emitOpenChange", open: true }] };
      }

      if (state === "closing") {
        return { state: "closed", context, effects: [{ type: "emitOpenChange", open: false }] };
      }

      return stay;
    }

    case "DISMISS": {
      return state === "closed" ? stay : closeNow(context);
    }

    case "SET_OPEN": {
      return { state: event.open ? "open" : "closed", context, effects: [{ type: "clearTimer" }] };
    }
  }
}
