/**
 * Edit-mode machinery (EditableLabel, EditableList).
 * Contracts: docs/contracts/components/editable-label.md, editable-list.md,
 * "Behavior Machine" sections.
 *
 * Edit-label machine: view/editing with commit-trims-draft and
 * cancel-restores semantics. List grab-and-move: pure keyboard-intent
 * resolution for accessible reordering (grab, arrow-move, boundary,
 * escape-cancel). Focus calls, announcements, and drag DOM plumbing stay
 * adapter-side.
 */

import type { TransitionResult } from "./machine";

// ── Edit-label machine ──

export type EditLabelState = "view" | "editing";

export interface EditLabelContext {
  value: string;
  draft: string;
  disabled: boolean;
  canStartEdit: boolean;
}

export type EditLabelEvent =
  | { type: "START_EDIT" }
  | { type: "SET_DRAFT"; draft: string }
  | { type: "COMMIT" }
  | { type: "CANCEL" };

export type EditLabelEffect =
  | { type: "emitEditStart" }
  | { type: "focusInput" }
  | { type: "emitCommit"; value: string; previousValue: string }
  | { type: "emitCancel" };

export type EditLabelResult = TransitionResult<EditLabelState, EditLabelContext, EditLabelEffect>;

export function editLabelTransition(
  state: EditLabelState,
  context: EditLabelContext,
  event: EditLabelEvent,
): EditLabelResult {
  const stay: EditLabelResult = { state, context, effects: [] };

  switch (event.type) {
    case "START_EDIT": {
      if (state !== "view" || context.disabled || !context.canStartEdit) {
        return stay;
      }

      return {
        state: "editing",
        context: { ...context, draft: context.value },
        effects: [{ type: "emitEditStart" }, { type: "focusInput" }],
      };
    }
    case "SET_DRAFT": {
      return state === "editing" ? { state, context: { ...context, draft: event.draft }, effects: [] } : stay;
    }
    case "COMMIT": {
      if (state !== "editing") {
        return stay;
      }

      const value = context.draft.trim();

      return {
        state: "view",
        context: { ...context, draft: context.value },
        effects: [{ type: "emitCommit", value, previousValue: context.value }],
      };
    }
    case "CANCEL": {
      if (state !== "editing") {
        return stay;
      }

      return {
        state: "view",
        context: { ...context, draft: context.value },
        effects: [{ type: "emitCancel" }],
      };
    }
  }
}

// ── List grab-and-move keyboard intents ──

export type ListReorderIntent =
  | { type: "grab" }
  | { type: "drop" }
  | { type: "cancelGrab" }
  | { type: "move"; from: number; to: number }
  | { type: "boundary" };

/**
 * Accessible-reorder keydown resolution: Space/Enter toggles grab, Escape
 * cancels a grab, arrows move the active item (the grabbed one when a grab
 * is active, else the focused one) and report boundaries.
 */
export function listReorderKeyIntent(
  key: string,
  index: number,
  grabbedIndex: number | null,
  itemCount: number,
): ListReorderIntent | null {
  if (key === " " || key === "Enter") {
    return grabbedIndex === index ? { type: "drop" } : { type: "grab" };
  }

  if (key === "Escape" && grabbedIndex !== null) {
    return { type: "cancelGrab" };
  }

  if (key !== "ArrowUp" && key !== "ArrowDown") {
    return null;
  }

  const activeIndex = grabbedIndex ?? index;
  const targetIndex = key === "ArrowUp" ? activeIndex - 1 : activeIndex + 1;

  if (targetIndex < 0 || targetIndex >= itemCount) {
    return { type: "boundary" };
  }

  return { type: "move", from: activeIndex, to: targetIndex };
}
