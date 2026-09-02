/**
 * Edit-mode machinery (EditableLabel, EditableList).
 * Contracts: docs/contracts/components/editable-label.md, editable-list.md,
 * "Behavior Machine" sections.
 *
 * Edit-label machine: view/editing with portable set-T commit trim,
 * Unicode-scalar maxLength, and cancel-restores semantics. List grab-and-move:
 * pure keyboard-intent resolution for accessible reordering (grab, arrow-move,
 * boundary, escape-cancel). Focus calls, announcements, and drag DOM plumbing
 * stay adapter-side.
 */

import type { TransitionResult } from "./machine";

// ── Portable set-T trim (Unicode White_Space + U+FEFF) ──

const EDITABLE_LABEL_TRIM_SCALARS: ReadonlySet<number> = new Set([
  0x0009, 0x000a, 0x000b, 0x000c, 0x000d, 0x0020, 0x0085, 0x00a0, 0x1680, 0x2000, 0x2001, 0x2002,
  0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x2008, 0x2009, 0x200a, 0x2028, 0x2029, 0x202f, 0x205f,
  0x3000, 0xfeff,
]);

function isTrimScalar(char: string): boolean {
  const codePoint = char.codePointAt(0);
  return codePoint !== undefined && EDITABLE_LABEL_TRIM_SCALARS.has(codePoint);
}

/** Drop the longest prefix and suffix in portable set T. Interior scalars stay. */
export function trimEditableLabel(value: string): string {
  const chars = [...value];
  let start = 0;
  let end = chars.length;

  while (start < end && isTrimScalar(chars[start] as string)) {
    start += 1;
  }

  while (end > start && isTrimScalar(chars[end - 1] as string)) {
    end -= 1;
  }

  return chars.slice(start, end).join("");
}

/** Keep at most `maxLength` Unicode scalar values. */
export function clampEditableLabelDraft(draft: string, maxLength: number | null): string {
  if (maxLength === null || !Number.isFinite(maxLength) || maxLength < 0) {
    return draft;
  }

  const limit = Math.floor(maxLength);
  const chars = [...draft];
  return chars.length <= limit ? draft : chars.slice(0, limit).join("");
}

// ── Edit-label machine ──

export type EditLabelState = "view" | "editing";

export interface EditLabelContext {
  value: string;
  draft: string;
  disabled: boolean;
  maxLength: number | null;
}

export type EditLabelEvent =
  | { type: "START_EDIT" }
  | { type: "SET_DRAFT"; draft: string }
  | { type: "COMMIT" }
  | { type: "COMMIT_BLUR" }
  | { type: "CANCEL" }
  | { type: "REPLACE_VALUE"; value: string }
  | { type: "SET_DISABLED"; disabled: boolean }
  | { type: "TEARDOWN" };

export type EditLabelEffect =
  | { type: "emitEditStart" }
  | { type: "focusInput" }
  | { type: "emitCommit"; value: string; previousValue: string; restoreFocus: boolean }
  | { type: "emitCancel"; restoreFocus: boolean };

export type EditLabelResult = TransitionResult<EditLabelState, EditLabelContext, EditLabelEffect>;

export function editLabelContext(init: Partial<EditLabelContext> = {}): EditLabelContext {
  return {
    value: "",
    draft: "",
    disabled: false,
    maxLength: null,
    ...init,
  };
}

function commitFromEditing(
  context: EditLabelContext,
  restoreFocus: boolean,
): EditLabelResult {
  const value = trimEditableLabel(context.draft);

  return {
    state: "view",
    context: { ...context, draft: context.value },
    effects: [{ type: "emitCommit", value, previousValue: context.value, restoreFocus }],
  };
}

function cancelFromEditing(context: EditLabelContext, restoreFocus: boolean): EditLabelResult {
  return {
    state: "view",
    context: { ...context, draft: context.value },
    effects: [{ type: "emitCancel", restoreFocus }],
  };
}

export function editLabelTransition(
  state: EditLabelState,
  context: EditLabelContext,
  event: EditLabelEvent,
): EditLabelResult {
  const stay: EditLabelResult = { state, context, effects: [] };

  switch (event.type) {
    case "START_EDIT": {
      if (state !== "view" || context.disabled) {
        return stay;
      }

      return {
        state: "editing",
        context: { ...context, draft: context.value },
        effects: [{ type: "emitEditStart" }, { type: "focusInput" }],
      };
    }
    case "SET_DRAFT": {
      if (state !== "editing") {
        return stay;
      }

      const draft = clampEditableLabelDraft(event.draft, context.maxLength);
      return { state, context: { ...context, draft }, effects: [] };
    }
    case "COMMIT": {
      return state === "editing" ? commitFromEditing(context, true) : stay;
    }
    case "COMMIT_BLUR": {
      return state === "editing" ? commitFromEditing(context, false) : stay;
    }
    case "CANCEL": {
      return state === "editing" ? cancelFromEditing(context, true) : stay;
    }
    case "REPLACE_VALUE": {
      if (state !== "editing") {
        return {
          state,
          context: { ...context, value: event.value, draft: event.value },
          effects: [],
        };
      }

      if (event.value === context.value) {
        return stay;
      }

      return {
        state: "view",
        context: { ...context, value: event.value, draft: event.value },
        effects: [],
      };
    }
    case "SET_DISABLED": {
      const next = { ...context, disabled: event.disabled };

      if (state === "editing" && event.disabled) {
        return {
          state: "view",
          context: { ...next, draft: context.value },
          effects: [{ type: "emitCancel", restoreFocus: false }],
        };
      }

      return { state, context: next, effects: [] };
    }
    case "TEARDOWN": {
      if (state !== "editing") {
        return stay;
      }

      return {
        state: "view",
        context: { ...context, draft: context.value },
        effects: [],
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
