/**
 * Modal overlay behavior machine (Dialog, AlertDialog via Dialog, Drawer).
 * Contracts: docs/contracts/components/dialog.md, alert-dialog.md, drawer.md,
 * "Behavior Machine" sections.
 *
 * States: closed | open. User-initiated close paths (escape, backdrop, close
 * button) emit `emitRequestClose` before `emitOpenChange(false)`, preserving
 * the onRequestClose → onOpenChange ordering. Open/close side work (focus
 * save + trap entry, focus restore, body scroll lock) are effect intents the
 * adapter executes.
 */

import type { TransitionResult } from "./machine";

export type ModalState = "closed" | "open";

export interface ModalContext {
  dismissOnEscape: boolean;
  dismissOnBackdrop: boolean;
}

export type ModalEvent =
  | { type: "OPEN" }
  | { type: "CLOSE" }
  | { type: "REQUEST_CLOSE" }
  | { type: "ESCAPE" }
  | { type: "BACKDROP_CLICK" };

export type ModalEffect =
  | { type: "emitOpenChange"; open: boolean }
  | { type: "emitRequestClose" }
  | { type: "saveFocusAndEnter" }
  | { type: "restoreFocus" }
  | { type: "lockBodyScroll" }
  | { type: "unlockBodyScroll" };

export type ModalResult = TransitionResult<ModalState, ModalContext, ModalEffect>;

function open(context: ModalContext): ModalResult {
  return {
    state: "open",
    context,
    effects: [
      { type: "emitOpenChange", open: true },
      { type: "saveFocusAndEnter" },
      { type: "lockBodyScroll" },
    ],
  };
}

function close(context: ModalContext, requested: boolean): ModalResult {
  const effects: ModalEffect[] = requested ? [{ type: "emitRequestClose" }] : [];

  return {
    state: "closed",
    context,
    effects: [
      ...effects,
      { type: "emitOpenChange", open: false },
      { type: "unlockBodyScroll" },
      { type: "restoreFocus" },
    ],
  };
}

export function modalTransition(
  state: ModalState,
  context: ModalContext,
  event: ModalEvent,
): ModalResult {
  const stay: ModalResult = { state, context, effects: [] };

  switch (event.type) {
    case "OPEN":
      return state === "closed" ? open(context) : stay;
    case "CLOSE":
      return state === "open" ? close(context, false) : stay;
    case "REQUEST_CLOSE":
      return state === "open" ? close(context, true) : stay;
    case "ESCAPE":
      return state === "open" && context.dismissOnEscape ? close(context, true) : stay;
    case "BACKDROP_CLICK":
      return state === "open" && context.dismissOnBackdrop ? close(context, true) : stay;
  }
}
