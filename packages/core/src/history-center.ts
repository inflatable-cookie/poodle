/**
 * HistoryCenter v2 behavior machine and history-tree stitcher.
 * Contract: docs/contracts/components/history-center.md, "Behavior Machine".
 *
 * v2 (card g13-023) replaces the v1 fork-expander model with a unified
 * history tree. `historyCenterRows` is a pure stitcher: it takes branch
 * records plus a per-branch entry path and returns the ordered rows — every
 * entry in the fork graph exactly once, at its true position, in topological
 * order. Each run attaches at the deepest entry its path shares with the
 * already-stitched tree; the authority's divergence id is never used for
 * attachment (v1 pinned rows to divergence ids computed relative to the
 * current branch, which collapsed genuinely different forks onto one entry).
 *
 * The machine owns popover open state, linear keyboard traversal over the
 * stitched rows, and transient rejection display. Branches and paths are part
 * of context and are supplied by the caller on every transition (the styled
 * layer owns rendering and paging). Commands go out as effects — the machine
 * never validates protocol rules and never invokes a callback speculatively.
 *
 * No clocks: `recordedAtMs` is modelled on the record types so the data can
 * arrive later, but nothing here invents timestamps (ruling D2). Branch UI is
 * presence-driven: when `branches` is null the machine has no rows and every
 * row event is inert.
 */

import type { TransitionResult } from "./machine";

export type HistoryEntryPosition = "past" | "current" | "future";

export interface HistoryEntry {
  id: string;
  label: string;
  position: HistoryEntryPosition;
  /** Renders as a named pin. */
  checkpoint?: boolean;
  groupId?: string | null;
  /** Optional recorded-at timestamp supplied by the authority; never invented here (D2). */
  recordedAtMs?: number;
}

export interface HistoryBranch {
  id: string;
  /** Auto-named by the authority; null means the id is displayed. */
  name: string | null;
  annotation?: string | null;
  /** Head entry of this branch; absent when the branch has no entries. */
  headEntryId?: string;
  /** Entry on the current branch after which this branch diverged; absent at root. */
  divergedAfterEntryId?: string;
  /** Optional recorded-at timestamp supplied by the authority; never invented here (D2). */
  recordedAtMs?: number;
  entryCount?: number;
  current?: boolean;
  pinned?: boolean;
}

export type HistoryCenterState = "closed" | "open";

export interface HistoryCenterContext {
  /** Branch records in supplied order; null disables the tree entirely (presence-driven). */
  branches: HistoryBranch[] | null;
  /** Per-branch entry path (branchId -> root-to-head entries); null when branches is null. */
  paths: Record<string, HistoryEntry[]> | null;
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
  | { type: "RENAME"; branchId: string; name: string }
  | { type: "SHOW_REJECTION"; message: string }
  | { type: "DISMISS_REJECTION" };

export type HistoryCenterEffect =
  | { type: "emitOpenChange"; open: boolean }
  | { type: "focusRow"; index: number }
  | { type: "emitNavigateEntry"; branchId: string; entryId: string }
  | { type: "emitRenameBranch"; branchId: string; name: string };

export type HistoryCenterResult = TransitionResult<HistoryCenterState, HistoryCenterContext, HistoryCenterEffect>;

/**
 * Visual depth cap (ruling D3): runs deeper than this keep rendering flat at
 * the cap rather than indenting further. The row still carries its true
 * `branchId` and lane structure — only indentation saturates.
 */
export const HISTORY_TREE_DEPTH_CAP = 3;

/**
 * Lane metadata for one entry row — sufficient for a renderer to draw a
 * git-graph lane without re-deriving structure. The flags describe the run's
 * shape within the supplied data: `start` is the run's first entry row (the
 * elbow, except at depth 0 where the spine's run is the trunk), `end` its
 * last, `continue` any row in between. A single-entry run is both `start`
 * and `end`. Lane structure is always true; only the row `depth` saturates.
 */
export interface HistoryRowLane {
  /** Branch whose run this row belongs to (same as the row's branchId). */
  branchId: string;
  /** Branch whose run this run attaches to; null for the spine and root-attached runs. */
  parentBranchId: string | null;
  /** First entry row of the run. */
  start: boolean;
  /** The run's lane passes through this row (any row that is not the run's first). */
  continue: boolean;
  /** Last entry row of the run. */
  end: boolean;
}

/**
 * One navigable row of the stitched history tree.
 *
 * - `entry`: a row in the fork graph, owned by the branch whose run it sits
 *   in — the spine's rows carry the current branch id. Activation navigates
 *   to exactly this row's branch and entry.
 * - `caption`: a fork run's label (branch name / description), rendered at
 *   the run's depth before its first entry row. Captions are focusable for
 *   rename but never navigate.
 *
 * `index` always equals the row's position in the returned array; keyboard
 * traversal is linear over the array in visual order.
 */
export type HistoryCenterRow =
  | { kind: "entry"; index: number; branchId: string; entry: HistoryEntry; depth: number; lane: HistoryRowLane }
  | { kind: "caption"; index: number; branch: HistoryBranch; depth: number };

/**
 * The stitcher (pure, exported): takes branch records plus a per-branch
 * entry path and returns the ordered rows. The spine is the branch marked
 * `current` (fallback: the first supplied branch); its path renders at depth
 * 0. Every other branch attaches its run — caption plus unique entries —
 * immediately after the deepest entry its path shares with the already
 * stitched tree, in supplied order (ruling D4: order is supplied, not
 * invented). Runs never re-emit shared entries (dedupe by entryId) and never
 * fetch anything (ruling D5). A branch with no unique entries (empty path,
 * or a path fully shared with the already stitched tree) is omitted — there
 * is no path-derived position to attach it at.
 */
export function historyCenterRows(
  branches: HistoryBranch[] | null,
  paths: Record<string, HistoryEntry[]> | null,
): HistoryCenterRow[] {
  const rows: HistoryCenterRow[] = [];

  if (branches === null || paths === null || branches.length === 0) {
    return rows;
  }

  const byId = new Map(branches.map((branch) => [branch.id, branch] as const));
  const spine = branches.find((candidate) => candidate.current === true) ?? branches[0];
  const spinePath = paths[spine.id] ?? [];

  // Entry ids already placed, and the branch whose run emitted each.
  const stitched = new Set<string>();
  const ownerOf = new Map<string, string>();
  for (const entry of spinePath) {
    stitched.add(entry.id);
    ownerOf.set(entry.id, spine.id);
  }

  // Runs in supplied order: run depth, unique suffix, parent, attach entry.
  const runDepth = new Map<string, number>([[spine.id, 0]]);
  const runSuffix = new Map<string, HistoryEntry[]>();
  const runParent = new Map<string, string | null>();
  const childrenOf = new Map<string, string[]>();
  const topRuns: string[] = [];

  for (const candidate of branches) {
    if (candidate.id === spine.id) {
      continue;
    }

    const path = paths[candidate.id];
    if (path === undefined || path.length === 0) {
      // Empty branch head: no entries, so no shared prefix and no attach
      // point. The branch is omitted entirely (no caption, no rows).
      continue;
    }

    // Deepest entry of this path already placed. Computing attachment against
    // the already-stitched set (never against the divergence id or other
    // unstitched paths) is what keeps fork-off-fork runs nested: the outer
    // run is stitched first, so the inner run's attach entry is its entry.
    let attachIndex = -1;
    for (let i = path.length - 1; i >= 0; i -= 1) {
      if (stitched.has(path[i].id)) {
        attachIndex = i;
        break;
      }
    }

    // Unique suffix, deduped by entry id so a paged path whose pages overlap
    // at the seam never doubles an entry.
    const suffix: HistoryEntry[] = [];
    const seen = new Set<string>();
    for (let i = attachIndex + 1; i < path.length; i += 1) {
      const entry = path[i];
      if (seen.has(entry.id)) {
        continue;
      }
      seen.add(entry.id);
      suffix.push(entry);
    }

    // A branch with no unique entries has nothing to render.
    if (suffix.length === 0) {
      continue;
    }

    const attachEntry = attachIndex < 0 ? null : path[attachIndex];
    const parent = attachEntry === null ? null : ownerOf.get(attachEntry.id) ?? null;

    runSuffix.set(candidate.id, suffix);
    runParent.set(candidate.id, parent);
    // A run attached to the spine or an outer run nests one level deeper; a
    // root-attached run is a peer of the spine at depth 0.
    runDepth.set(candidate.id, parent === null ? 0 : (runDepth.get(parent) ?? 0) + 1);

    for (const entry of suffix) {
      stitched.add(entry.id);
      ownerOf.set(entry.id, candidate.id);
    }

    if (attachEntry === null) {
      // Shares nothing with the placed structure: attach at the root, before
      // the spine (the authority's "or root" case, e.g. a spine page that
      // starts below this branch's divergence).
      topRuns.push(candidate.id);
    } else {
      const siblings = childrenOf.get(attachEntry.id) ?? [];
      siblings.push(candidate.id);
      childrenOf.set(attachEntry.id, siblings);
    }
  }

  const emitRun = (branchId: string, isSpine: boolean): void => {
    const entries = isSpine ? spinePath : (runSuffix.get(branchId) ?? []);
    const depth = isSpine ? 0 : (runDepth.get(branchId) ?? 0);
    const renderedDepth = Math.min(depth, HISTORY_TREE_DEPTH_CAP);

    if (!isSpine) {
      const branch = byId.get(branchId);
      if (branch === undefined) {
        return;
      }
      rows.push({ kind: "caption", index: rows.length, branch, depth: renderedDepth });
    }

    for (let i = 0; i < entries.length; i += 1) {
      rows.push({
        kind: "entry",
        index: rows.length,
        branchId,
        entry: entries[i],
        depth: renderedDepth,
        lane: {
          branchId,
          parentBranchId: isSpine ? null : (runParent.get(branchId) ?? null),
          start: i === 0,
          continue: i > 0,
          end: i === entries.length - 1,
        },
      });

      const children = childrenOf.get(entries[i].id);
      if (children !== undefined) {
        for (const child of children) {
          emitRun(child, false);
        }
      }
    }
  };

  for (const branchId of topRuns) {
    emitRun(branchId, false);
  }
  emitRun(spine.id, true);

  return rows;
}

export function historyCenterRowCount(
  branches: HistoryBranch[] | null,
  paths: Record<string, HistoryEntry[]> | null,
): number {
  return historyCenterRows(branches, paths).length;
}

export function historyCenterDefaultContext(
  overrides: Partial<HistoryCenterContext> = {},
): HistoryCenterContext {
  return {
    branches: null,
    paths: null,
    focusIndex: 0,
    rejection: null,
    ...overrides,
  };
}

function stay(state: HistoryCenterState, context: HistoryCenterContext): HistoryCenterResult {
  return { state, context, effects: [] };
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
  const count = historyCenterRowCount(context.branches, context.paths);

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
  const row = historyCenterRows(context.branches, context.paths)[index];

  if (row === undefined) {
    return stay("open", context);
  }

  if (row.kind === "caption") {
    // Captions are focusable for rename but never navigate.
    return { state: "open", context: { ...context, focusIndex: index }, effects: [] };
  }

  // Always the entry actually clicked, on the branch that owns its run —
  // never an ancestor or a divergence entry belonging to another branch.
  return {
    state: "open",
    context: { ...context, focusIndex: index },
    effects: [{ type: "emitNavigateEntry", branchId: row.branchId, entryId: row.entry.id }],
  };
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
