/**
 * HistoryCenter behavior machine.
 * Contract: docs/contracts/components/history-center.md, "Behavior Machine".
 *
 * Owns four areas: popover open state, list keyboard navigation, fork-branch
 * expansion, and transient rejection display. Entries and branches are part
 * of context and are supplied by the caller on every transition (the styled
 * layer owns rendering and paging). Commands go out as effects — the machine
 * never validates protocol rules and never invokes a callback speculatively:
 * undo/redo are plain button commands the adapter forwards, and the host owns
 * what they do.
 *
 * Branch UI is presence-driven: when `branches` is null the machine treats
 * every row as an entry and fork expansion is inert.
 */

import type { TransitionResult } from "./machine";

export type HistoryEntryPosition = "past" | "current" | "future";

export interface HistoryEntry {
  id: string;
  label: string;
  position: HistoryEntryPosition;
  /** Renders as a named pin. */
  checkpoint?: boolean;
  /** > 1 marks a fork point, expandable to the supplied branch rows. */
  branchCount?: number;
  groupId?: string | null;
}

export interface HistoryBranch {
  id: string;
  /** Auto-named by the authority; null means the id is displayed. */
  name: string | null;
  annotation?: string | null;
  entryCount?: number;
  current?: boolean;
  pinned?: boolean;
}

export type HistoryCenterState = "closed" | "open";

export interface HistoryCenterContext {
  entries: HistoryEntry[];
  /** null disables all branch and checkpoint presentation (presence-driven). */
  branches: HistoryBranch[] | null;
  expandedBranchIds: string[];
  focusIndex: number;
  /** Currently displayed rejection message; null when none. */
  rejection: string | null;
}

export type HistoryCenterEvent =
  | { type: "TOGGLE" }
  | { type: "OPEN" }
  | { type: "CLOSE" }
  | { type: "FOCUS_MOVE"; direction: "next" | "prev" | "first" | "last" }
  | { type: "ACTIVATE_ROW"; index?: number }
  | { type: "TOGGLE_BRANCHES"; entryId: string }
  | { type: "EXPAND_BRANCHES"; entryId: string }
  | { type: "COLLAPSE_BRANCHES"; entryId: string }
  | { type: "CHECKOUT"; branchId: string; entryId: string }
  | { type: "RENAME"; branchId: string; name: string }
  | { type: "SHOW_REJECTION"; message: string }
  | { type: "DISMISS_REJECTION" };

export type HistoryCenterEffect =
  | { type: "emitOpenChange"; open: boolean }
  | { type: "focusRow"; index: number }
  | { type: "emitSelectEntry"; id: string }
  | { type: "emitCheckout"; branchId: string; entryId: string }
  | { type: "emitRenameBranch"; branchId: string; name: string };

export type HistoryCenterResult = TransitionResult<HistoryCenterState, HistoryCenterContext, HistoryCenterEffect>;

/**
 * One navigable row: an entry, or a branch row under an expanded fork point.
 * Branch rows are inserted directly after their fork entry, so the row order
 * is stable under expansion and index math stays linear.
 */
export type HistoryCenterRow =
  | { kind: "entry"; index: number; entry: HistoryEntry }
  | { kind: "branch"; index: number; entry: HistoryEntry; branch: HistoryBranch };

export function historyCenterRows(
  entries: HistoryEntry[],
  branches: HistoryBranch[] | null,
  expandedBranchIds: string[],
): HistoryCenterRow[] {
  const rows: HistoryCenterRow[] = [];

  for (const entry of entries) {
    rows.push({ kind: "entry", index: rows.length, entry });

    if (branches !== null && isForkPoint(entry) && expandedBranchIds.includes(entry.id)) {
      for (const branch of branches) {
        rows.push({ kind: "branch", index: rows.length, entry, branch });
      }
    }
  }

  return rows;
}

export function historyCenterRowCount(
  entries: HistoryEntry[],
  branches: HistoryBranch[] | null,
  expandedBranchIds: string[],
): number {
  return historyCenterRows(entries, branches, expandedBranchIds).length;
}

/** A fork point is an entry the host marks as forked; branch UI needs presence too. */
export function isForkPoint(entry: HistoryEntry): boolean {
  return (entry.branchCount ?? 0) > 1;
}

export function historyCenterDefaultContext(
  overrides: Partial<HistoryCenterContext> = {},
): HistoryCenterContext {
  return {
    entries: [],
    branches: null,
    expandedBranchIds: [],
    focusIndex: 0,
    rejection: null,
    ...overrides,
  };
}

function stay(state: HistoryCenterState, context: HistoryCenterContext): HistoryCenterResult {
  return { state, context, effects: [] };
}

/** Rebuilds the row list and clamps focus to a live row; branches of a collapsed fork never keep focus. */
function clampFocus(context: HistoryCenterContext, preferredIndex: number): HistoryCenterContext {
  const rows = historyCenterRows(context.entries, context.branches, context.expandedBranchIds);
  const maxIndex = rows.length - 1;
  const clamped = Math.min(Math.max(preferredIndex, 0), Math.max(maxIndex, 0));
  const focused = rows[clamped];

  // When the focused row is a branch whose fork just collapsed, fall back to the fork entry.
  if (focused?.kind === "branch" && !context.expandedBranchIds.includes(focused.entry.id)) {
    const entryIndex = rows.findIndex(
      (row) => row.kind === "entry" && row.entry.id === focused.entry.id,
    );
    return { ...context, focusIndex: entryIndex >= 0 ? entryIndex : 0 };
  }

  return { ...context, focusIndex: clamped };
}

function open(context: HistoryCenterContext): HistoryCenterResult {
  return {
    state: "open",
    context,
    effects: [{ type: "emitOpenChange", open: true }],
  };
}

function close(context: HistoryCenterContext): HistoryCenterResult {
  return {
    state: "closed",
    context,
    effects: [{ type: "emitOpenChange", open: false }],
  };
}

function moveFocus(context: HistoryCenterContext, direction: "next" | "prev" | "first" | "last"): HistoryCenterResult {
  const count = historyCenterRowCount(context.entries, context.branches, context.expandedBranchIds);

  if (count === 0) {
    return stay("open", context);
  }

  let next = context.focusIndex;

  if (direction === "first") {
    next = 0;
  } else if (direction === "last") {
    next = count - 1;
  } else if (direction === "next") {
    next = (context.focusIndex + 1) % count;
  } else {
    next = (context.focusIndex - 1 + count) % count;
  }

  return {
    state: "open",
    context: { ...context, focusIndex: next },
    effects: [{ type: "focusRow", index: next }],
  };
}

function activateRow(context: HistoryCenterContext, index: number): HistoryCenterResult {
  const row = historyCenterRows(context.entries, context.branches, context.expandedBranchIds)[index];

  if (row === undefined) {
    return stay("open", context);
  }

  if (row.kind === "branch") {
    return {
      state: "open",
      context: { ...context, focusIndex: index },
      effects: [{ type: "emitCheckout", branchId: row.branch.id, entryId: row.entry.id }],
    };
  }

  return {
    state: "open",
    context: { ...context, focusIndex: index },
    effects: [{ type: "emitSelectEntry", id: row.entry.id }],
  };
}

function expandBranches(context: HistoryCenterContext, entryId: string): HistoryCenterResult {
  const entry = context.entries.find((candidate) => candidate.id === entryId);

  // Presence-driven: no branches supplied, no expansion; non-fork entries are inert.
  if (context.branches === null || entry === undefined || !isForkPoint(entry)) {
    return stay("open", context);
  }

  if (context.expandedBranchIds.includes(entryId)) {
    return stay("open", context);
  }

  return {
    state: "open",
    context: clampFocus(
      { ...context, expandedBranchIds: [...context.expandedBranchIds, entryId] },
      context.focusIndex,
    ),
    effects: [],
  };
}

function collapseBranches(context: HistoryCenterContext, entryId: string): HistoryCenterResult {
  if (!context.expandedBranchIds.includes(entryId)) {
    return stay("open", context);
  }

  const rowsBefore = historyCenterRows(context.entries, context.branches, context.expandedBranchIds);
  const focusedBefore = rowsBefore[context.focusIndex];
  const nextContext: HistoryCenterContext = {
    ...context,
    expandedBranchIds: context.expandedBranchIds.filter((id) => id !== entryId),
  };

  // Focus that sat on a branch of this fork moves up to the fork entry.
  if (focusedBefore?.kind === "branch" && focusedBefore.entry.id === entryId) {
    const forkIndex = historyCenterRows(nextContext.entries, nextContext.branches, nextContext.expandedBranchIds).findIndex(
      (row) => row.kind === "entry" && row.entry.id === entryId,
    );

    return {
      state: "open",
      context: { ...nextContext, focusIndex: forkIndex >= 0 ? forkIndex : context.focusIndex },
      effects: [],
    };
  }

  return { state: "open", context: clampFocus(nextContext, context.focusIndex), effects: [] };
}

export function historyCenterTransition(
  state: HistoryCenterState,
  context: HistoryCenterContext,
  event: HistoryCenterEvent,
): HistoryCenterResult {
  switch (event.type) {
    case "TOGGLE":
      return state === "closed" ? open(context) : close(context);
    case "OPEN":
      return state === "closed" ? open(context) : stay(state, context);
    case "CLOSE":
      return state === "open" ? close(context) : stay(state, context);
    case "FOCUS_MOVE":
      return state === "open" ? moveFocus(context, event.direction) : stay(state, context);
    case "ACTIVATE_ROW":
      return state === "open"
        ? activateRow(context, event.index ?? context.focusIndex)
        : stay(state, context);
    case "TOGGLE_BRANCHES": {
      if (state !== "open") {
        return stay(state, context);
      }

      return context.expandedBranchIds.includes(event.entryId)
        ? collapseBranches(context, event.entryId)
        : expandBranches(context, event.entryId);
    }
    case "EXPAND_BRANCHES":
      return state === "open" ? expandBranches(context, event.entryId) : stay(state, context);
    case "COLLAPSE_BRANCHES":
      return state === "open" ? collapseBranches(context, event.entryId) : stay(state, context);
    case "CHECKOUT":
      return {
        state,
        context,
        effects: [{ type: "emitCheckout", branchId: event.branchId, entryId: event.entryId }],
      };
    case "RENAME":
      return {
        state,
        context,
        effects: [{ type: "emitRenameBranch", branchId: event.branchId, name: event.name }],
      };
    case "SHOW_REJECTION":
      return event.message === context.rejection
        ? stay(state, context)
        : { state, context: { ...context, rejection: event.message }, effects: [] };
    case "DISMISS_REJECTION":
      return context.rejection === null
        ? stay(state, context)
        : { state, context: { ...context, rejection: null }, effects: [] };
  }
}

/**
 * Map a row-list keydown to a machine event. Returns null for keys the
 * machine does not own (adapter lets them propagate).
 */
export function historyCenterKeydownEvent(key: string): HistoryCenterEvent | null {
  if (key === "ArrowDown") {
    return { type: "FOCUS_MOVE", direction: "next" };
  }

  if (key === "ArrowUp") {
    return { type: "FOCUS_MOVE", direction: "prev" };
  }

  if (key === "Home") {
    return { type: "FOCUS_MOVE", direction: "first" };
  }

  if (key === "End") {
    return { type: "FOCUS_MOVE", direction: "last" };
  }

  if (key === "Enter" || key === " ") {
    return { type: "ACTIVATE_ROW" };
  }

  return null;
}
