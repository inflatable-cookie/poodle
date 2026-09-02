/**
 * HistoryCentre v3 (card g13-028) — flat visible-row derivation and behavior
 * machine over path pages and continuations.
 *
 * Contract: docs/contracts/components/history-center.md, "Behavior Machine".
 *
 * v3 replaces the v2 stitcher (historyCenterRows, historyCenterRowCount,
 * HistoryCenterRow, HistoryRowLane, HISTORY_TREE_DEPTH_CAP, HistoryBranch and
 * the `paths` context field — all deleted with this card) with a flat list
 * that owns forks on the entry (ruling R1). The derivation walks the open
 * forks and emits ONE flat array of rows, each with a `depth` number, the
 * entry it hangs off (`parentEntryId`) and the fork it belongs to (`forkId`)
 * as data — never as indentation. No renderer recurses; core knows the
 * topology, which is core's job. `flattenVisibleTreeRows` in `tree.ts` is the
 * in-repo precedent; `packages/render/src/tree.rs` the native one.
 *
 * The record types below are structural mirrors of the authority's shapes
 * (ruling R2): Poodle never imports a Longhorn type and no manifest gains one.
 * The three continuation operations — load the continuations at an anchor,
 * load a continuation run, checkout a continuation — arrive as caller-supplied
 * callbacks and leave as effects. Checkout is Poodle's own word: the host
 * maps the callback onto Longhorn's `preferContinuation`, which is exactly
 * the decoupling ruling R2 established (b028).
 *
 * Display order is core-owned and reversed exactly once (ruling R3): pages
 * arrive newest-first and display oldest-first, so a page at a higher offset
 * (older) renders before the first page — joining in fetch order puts history
 * backwards. Every level — the root and each nested run — reverses by the
 * same join. `continuations` is not reversed: it is stable graph order, a
 * picker, not a timeline.
 *
 * `forkCount = max(0, continuationCount - 1)` (ruling R4): continuationCount counts
 * every continuation including the run's own next row, so a run's last entry
 * carries 0 and one fork reads 2. The continuations page also returns the
 * child already on the list; the derivation filters it out by id and never
 * assumes its position.
 *
 * Core holds only what is open (ruling R5): the loaded continuations and the
 * loaded run belong to the currently open entries and are dropped on close —
 * nothing is cached across a close/reopen; pages are not cached and not
 * refreshed.
 *
 * Rename survives (ruling R6): `emitRenameBranch` stays a pass-through
 * client-side affordance (it enforces no protocol rule) surfaced in the opened
 * region.
 *
 * No clocks: `recordedAtMs` is modelled on the record types so the data can
 * arrive later, but nothing here invents timestamps and no clock is ever read.
 */

import type { TransitionResult } from "./machine";

// ── Record types (structural mirrors of the authority's shapes; R2) ─────

export type HistoryEntryPosition = "past" | "current" | "future";

export interface HistoryEntry {
  id: string;
  label: string;
  position: HistoryEntryPosition;
  /** Renders as a named pin. */
  checkpoint?: boolean;
  groupId?: string | null;
  /** Optional recorded-at timestamp supplied by the authority; never invented here. */
  recordedAtMs?: number;
  /**
   * How many entries continue from this one, the run's own next row included
   * (the authority's `ForkEntryRecord.continuation_count`). A fork count is
   * one less (R4); a run's last entry is always zero.
   */
  continuationCount: number;
}

/**
 * One bounded page of a path, newest first (the authority's
 * `ForkPathPageSnapshot`). `offset` counts from the newest entry, so the page
 * at a higher offset holds older entries (R3).
 */
export interface HistoryPathPage {
  /** Bounded records, newest first. */
  entries: HistoryEntry[];
  /** Newest-first offset of this page. */
  offset: number;
  /**
   * How many entries continue from the root — the same fact as an entry's
   * `continuationCount`, one level above the first entry (R4). A root fork
   * count is one less. Carried on the record for the host and the renderer;
   * the derivation emits no root-level row for it.
   */
  precedingContinuationCount: number;
  /** Whether newer records precede this page. */
  truncatedBefore: boolean;
  /** Whether older records follow this page. */
  truncatedAfter: boolean;
}

/**
 * One continuation at an anchor (the authority's `ForkContinuationRecord`):
 * the operator's fork. The continuations page returns every child of the
 * anchor, including the child already rendered on the list — the derivation
 * filters that one out by id and never assumes its position (R4).
 */
export interface HistoryContinuation {
  /** Stable identity of the continuing entry — the run's first entry. */
  entryId: string;
  /** Consumer-owned label. */
  label: string;
  /** Optional host-supplied recorded-at stamp; never invented here. */
  recordedAtMs?: number;
  /** Whether a redo from the anchor takes this continuation. */
  preferred: boolean;
  /** Entries in the run starting here, this one included. */
  entryCount: number;
  /** Branch a consumer lands on by taking this continuation. */
  branchId: string;
  /** That branch's optional name. */
  branchName: string | null;
}

// ── Visible-row model (R1) ─────────────────────────────────────────────

/**
 * Stable identity of one visible row, used for roving focus. Keyed by kind
 * plus entry id, never by array position: a disclosure toggle changes the
 * list shape underneath an index, which is the bug v3 exists to avoid.
 */
export type HistoryCenterRowId =
  | { kind: "entry"; entryId: string }
  | { kind: "picker"; entryId: string }
  | { kind: "not-yet-loaded"; entryId: string };

/**
 * One row of the flat visible list. Every row carries `depth` (indentation
 * only), `parentEntryId` (the entry it hangs off; null at the root) and
 * `forkId` (the run's first entry — the fork it belongs to; null on the spine
 * and before a run is chosen) as identifiers, not as indentation (R1
 * condition: two forks at one entry are never confusable with a fork off a
 * fork, at any depth, with no cap).
 */
export type HistoryCenterRow =
  | {
      kind: "entry";
      entry: HistoryEntry;
      depth: number;
      parentEntryId: string | null;
      forkId: string | null;
      /** Branch this row's run lands on; null on the spine (the host knows its own branch). */
      branchId: string | null;
      /** Forks at this entry (continuationCount - 1, floored at 0; R4). */
      forkCount: number;
    }
  | {
      kind: "picker";
      /** Entry whose forks are offered. */
      anchorEntryId: string;
      depth: number;
      parentEntryId: string | null;
      forkId: string | null;
      /** Forks at the anchor, the child already on the list filtered out (R4). Empty while loading. */
      continuations: HistoryContinuation[];
      /**
       * The select's value — the shown fork: the tentative pick (its first
       * entry id), else the auto-chosen single fork (b033 R3). Null only
       * while the level's continuations are still in flight. The picker
       * persists for as long as the level is open (R1), so the current
       * selection stays visible and reachable.
       */
      pickedEntryId: string | null;
      /**
       * The renderer disables the Select on this signal — it never infers it
       * from `continuations.length`. True when the level has exactly one
       * fork: there is nothing to choose between (b033 R3). The row still
       * carries the pencil, the opt-in delete and checkout.
       */
      disabled: boolean;
    }
  | {
      kind: "not-yet-loaded";
      /** Open entry whose run has not arrived — never an empty gap, never a dropped entry. */
      anchorEntryId: string;
      depth: number;
      parentEntryId: string | null;
      forkId: string | null;
      branchId: string | null;
    };

/**
 * One level of the disclosure tree: a fork open at an entry. Levels nest
 * because forks fork (R1) — a fork open inside an open run is that level's
 * `inner` — and multiple entries can be open at any level, keyed by anchor
 * entry id. Core holds only what is open (R5): closing a level drops its
 * continuations, pick, chosen fork, run pages and everything inner.
 */
export interface HistoryCenterOpenFork {
  /** Entry the fork hangs off. */
  anchorEntryId: string;
  /**
   * Forks at the anchor in supplied order; null until loaded. The child
   * already on the list is filtered by the derivation, never here.
   */
  continuations: HistoryContinuation[] | null;
  /** Tentatively picked fork (the select's value); null until the pick or
   *  R3's open-selects-current. One tentative pick at a time across levels. */
  pick: HistoryContinuation | null;
  /**
   * The chosen fork — the auto-chosen single fork (`forkCount === 1`), which
   * is also every run row's `forkId`. The multi-fork picker never commits a
   * `chosen`: checkout (CONFIRM) clears the level and the host supplies the
   * new root (R2).
   */
  chosen: HistoryContinuation | null;
  /** The chosen fork's run pages in fetch order (newest page first); empty until loaded. */
  runPages: HistoryPathPage[];
  /** Forks open inside this run, keyed by anchor entry id; null when none. */
  inner: ReadonlyMap<string, HistoryCenterOpenFork> | null;
}

// ── Derivation helpers (R3, R4) ────────────────────────────────────────

/** Forks at an entry: one less than its continuation count, floored at 0 (R4). */
export function historyCenterForkCount(continuationCount: number): number {
  return continuationCount <= 1 ? 0 : continuationCount - 1;
}

/**
 * Join path pages — the root or a run — into display order (oldest entry
 * first). Pages arrive newest-first in fetch order and display oldest-first,
 * so the later-fetched (older) page renders before the first page (R3). This
 * is the only reversal and every level reverses by this same code. Overlapping
 * page seams dedupe by entry id; entries are immutable graph nodes, so an id
 * names exactly one entry.
 */
export function historyCenterJoinPages(pages: readonly HistoryPathPage[]): HistoryEntry[] {
  const newestFirst: HistoryEntry[] = [];
  const seen = new Set<string>();
  for (const page of pages) {
    for (const entry of page.entries) {
      if (!seen.has(entry.id)) {
        seen.add(entry.id);
        newestFirst.push(entry);
      }
    }
  }
  newestFirst.reverse();
  return newestFirst;
}

/**
 * The forks at an anchor: the loaded continuations minus the child already on
 * the list (R4). That child is the anchor's successor in the run — filter by
 * id, never by array position. When the successor is not on a loaded page
 * (truncated paging), the preferred flag identifies the same record: the run
 * follows preferred children, so the child already on the list is the
 * preferred continuation.
 */
export function historyCenterForksAt(
  continuations: HistoryContinuation[] | null,
  runEntries: readonly HistoryEntry[],
  anchorIndex: number,
): HistoryContinuation[] {
  if (continuations === null) {
    return [];
  }
  const ownId = anchorIndex + 1 < runEntries.length ? runEntries[anchorIndex + 1].id : null;
  return continuations.filter((continuation) =>
    ownId === null ? !continuation.preferred : continuation.entryId !== ownId,
  );
}

/**
 * The visible-row derivation (pure, exported): one flat array of rows in
 * display order over the root path pages plus the open forks. The renderer
 * receives a depth number and knows nothing about topology — core knows it,
 * which is core's job (R1).
 */
export function historyCenterVisibleRows(
  pages: readonly HistoryPathPage[] | null,
  open: ReadonlyMap<string, HistoryCenterOpenFork> | null,
): HistoryCenterRow[] {
  const rows: HistoryCenterRow[] = [];
  if (pages === null || pages.length === 0) {
    return rows;
  }
  const rootEntries = historyCenterJoinPages(pages);
  pushRun(rows, rootEntries, 0, null, null, null, open, rootEntries);
  return rows;
}

function pushRun(
  rows: HistoryCenterRow[],
  entries: readonly HistoryEntry[],
  depth: number,
  parentOfFirst: string | null,
  forkId: string | null,
  branchId: string | null,
  open: ReadonlyMap<string, HistoryCenterOpenFork> | null,
  rootEntries: readonly HistoryEntry[],
): void {
  for (let i = 0; i < entries.length; i += 1) {
    const entry = entries[i];
    rows.push({
      kind: "entry",
      entry,
      depth,
      parentEntryId: i === 0 ? parentOfFirst : entries[i - 1].id,
      forkId,
      branchId,
      forkCount: historyCenterForkCount(entry.continuationCount),
    });

    const level = open?.get(entry.id);
    if (level !== undefined) {
      pushDisclosed(rows, level, entry, entries, i, depth, rootEntries);
    }
  }
}

function pushDisclosed(
  rows: HistoryCenterRow[],
  level: HistoryCenterOpenFork,
  entry: HistoryEntry,
  runEntries: readonly HistoryEntry[],
  anchorIndex: number,
  depth: number,
  rootEntries: readonly HistoryEntry[],
): void {
  const childDepth = depth + 1;
  const forkCount = historyCenterForkCount(entry.continuationCount);
  // The run below the select follows the pick (R2): the displayed fork is the
  // tentative pick, falling back to the auto-chosen single fork.
  const shown = level.pick ?? level.chosen;
  // R2 (g13-034): a level is stale when its shown fork's first entry id sits
  // on the joined root pages — the host navigated into the fork and supplied
  // root pages that already contain the run. Staleness is a data fact, never
  // an array-identity fact: a host that rebuilds its pages array each render
  // cannot loop it. A stale level never splices its cached run — that would
  // duplicate spine entries — and renders the existing not-yet-loaded row
  // until the machine drops the level's data and re-requests its
  // continuations. Its picker renders empty, exactly as it will after the
  // drop, so the row set does not change shape at the reconcile boundary.
  const stale =
    shown !== null && rootEntries.some((rootEntry) => rootEntry.id === shown.entryId);

  if (forkCount >= 1) {
    // b033 (R3): the picker row serves every open level — the single fork
    // included. The renderer disables the Select on the row's `disabled`
    // signal (nothing to choose between); it never infers it from
    // `continuations.length`. The picker persists for as long as the level
    // is open, whatever `chosen` holds (R1) — a second fork is one
    // interaction away, never a close-and-reopen.
    rows.push({
      kind: "picker",
      anchorEntryId: entry.id,
      depth: childDepth,
      parentEntryId: entry.id,
      forkId: null,
      continuations: stale ? [] : historyCenterForksAt(level.continuations, runEntries, anchorIndex),
      pickedEntryId: stale ? null : level.pick?.entryId ?? level.chosen?.entryId ?? null,
      disabled: forkCount <= 1,
    });
  }

  if (stale) {
    rows.push({
      kind: "not-yet-loaded",
      anchorEntryId: entry.id,
      depth: childDepth,
      parentEntryId: entry.id,
      forkId: shown.entryId,
      branchId: shown.branchId,
    });
    return;
  }

  if (shown === null) {
    if (forkCount <= 1) {
      // Single fork (or its continuations still in flight): the run is what
      // will show, so mark it not-yet-loaded rather than leaving a gap.
      rows.push({
        kind: "not-yet-loaded",
        anchorEntryId: entry.id,
        depth: childDepth,
        parentEntryId: entry.id,
        forkId: null,
        branchId: null,
      });
    }
    return;
  }

  if (level.runPages.length === 0) {
    rows.push({
      kind: "not-yet-loaded",
      anchorEntryId: entry.id,
      depth: childDepth,
      parentEntryId: entry.id,
      forkId: shown.entryId,
      branchId: shown.branchId,
    });
    return;
  }

  pushRun(
    rows,
    historyCenterJoinPages(level.runPages),
    childDepth,
    entry.id,
    shown.entryId,
    shown.branchId,
    level.inner,
    rootEntries,
  );
}

// ── Machine ────────────────────────────────────────────────────────────

export type HistoryCenterState = "closed" | "open";

/**
 * Rejections the machine can display, structurally declared (R2 — no Longhorn
 * type reaches Poodle). The host's bridge maps protocol rejections onto these
 * five renderer-neutral meanings; the machine owns the display copy.
 *
 * The deletion refusals are deliberately three, not one: a stale revision, an
 * entry the authority protects, and a deletion the authority cannot offer at
 * all are different facts, and collapsing them onto `UnknownEntry` told an
 * operator their entry had vanished when it had not. Current-line and
 * pinned/checkpoint policy share `ProtectedEntry` — both are entry-level
 * protection, and Poodle has no business splitting the host's reason.
 */
export type HistoryCenterRejectionCode =
  | "AlreadyAtTarget"
  | "UnknownEntry"
  | "StaleHistory"
  | "ProtectedEntry"
  | "DeletionUnavailable";

export function historyCenterRejectionMessage(code: HistoryCenterRejectionCode): string {
  switch (code) {
    case "AlreadyAtTarget":
      return "Already at the requested target";
    case "UnknownEntry":
      return "Entry does not exist";
    case "StaleHistory":
      return "History changed; this entry was not deleted";
    case "ProtectedEntry":
      return "This history entry is protected";
    case "DeletionUnavailable":
      return "History deletion is unavailable";
  }
}

export interface HistoryCenterContext {
  /** Root path pages in fetch order (newest page first); null disables the list. */
  pages: HistoryPathPage[] | null;
  /** Open forks at root entries, keyed by anchor entry id; null when none is open. */
  open: ReadonlyMap<string, HistoryCenterOpenFork> | null;
  /** Currently focused visible row identity; null when nothing is focused. */
  focusRow: HistoryCenterRowId | null;
  /** Currently displayed rejection message; null when none. */
  rejection: string | null;
}

export type HistoryCenterEvent =
  | { type: "TOGGLE" }
  | { type: "OPEN" }
  | { type: "CLOSE" }
  | { type: "FOCUS_MOVE"; direction: "next" | "prev" | "first" | "last" }
  | { type: "ACTIVATE_ROW"; row?: HistoryCenterRowId }
  | { type: "DISCLOSE"; entryId: string }
  | { type: "CONTINUATIONS_LOADED"; entryId: string; continuations: HistoryContinuation[] }
  | { type: "PICK_CONTINUATION"; entryId: string }
  | { type: "CONFIRM" }
  | { type: "DELETE_CONTINUATION"; entryId: string }
  | { type: "RUN_LOADED"; fromEntryId: string; pages: HistoryPathPage[] }
  | { type: "RENAME"; branchId: string; name: string }
  | { type: "SHOW_REJECTION"; code: HistoryCenterRejectionCode }
  | { type: "DISMISS_REJECTION" }
  /**
   * The host supplied new root pages. Carries nothing and changes nothing on
   * its own — its only job is to let the standing stale-level reconcile run
   * (g13-034 R2), which needs a transition to ride and a pages prop change
   * dispatches none. Without it the reconcile was correct code that never
   * fired: a level went stale, rendered `not-yet-loaded`, and sat there until
   * the operator closed and reopened the popover.
   *
   * Idempotent by construction. The reconcile drops a stale level's data once
   * and then has no shown fork to find, so repeat sends emit nothing. A host
   * that rebuilds its pages array every render costs one no-op transition, not
   * a loop — the adapter reference-diffs before sending anyway.
   */
  | { type: "PAGES_CHANGED" };

export type HistoryCenterEffect =
  | { type: "emitOpenChange"; open: boolean }
  | { type: "focusRow"; row: HistoryCenterRowId }
  | { type: "emitNavigateEntry"; branchId: string | null; entryId: string }
  | { type: "emitRenameBranch"; branchId: string; name: string }
  | { type: "loadContinuations"; entryId: string }
  | { type: "loadContinuationRun"; fromEntryId: string }
  | { type: "checkoutContinuation"; entryId: string }
  | { type: "deleteContinuation"; entryId: string };

export type HistoryCenterResult = TransitionResult<HistoryCenterState, HistoryCenterContext, HistoryCenterEffect>;

export function historyCenterDefaultContext(
  overrides: Partial<HistoryCenterContext> = {},
): HistoryCenterContext {
  return {
    pages: null,
    open: null,
    focusRow: null,
    rejection: null,
    ...overrides,
  };
}

function stay(state: HistoryCenterState, context: HistoryCenterContext): HistoryCenterResult {
  return { state, context, effects: [] };
}

function openResult(context: HistoryCenterContext): HistoryCenterResult {
  return {
    state: "open",
    context,
    effects: [{ type: "emitOpenChange", open: true }],
  };
}

function closeResult(context: HistoryCenterContext): HistoryCenterResult {
  // R5: nothing is cached across a close/reopen — the disclosure tree and the
  // loaded pages it holds are dropped with the popover.
  return {
    state: "closed",
    context: { ...context, open: null, focusRow: null },
    effects: [{ type: "emitOpenChange", open: false }],
  };
}

// ── Row identity and focus ─────────────────────────────────────────────

function rowIdOf(row: HistoryCenterRow): HistoryCenterRowId {
  return row.kind === "entry"
    ? { kind: "entry", entryId: row.entry.id }
    : { kind: row.kind, entryId: row.anchorEntryId };
}

function indexOfRow(rows: readonly HistoryCenterRow[], id: HistoryCenterRowId | null): number {
  if (id === null) {
    return -1;
  }
  return rows.findIndex((row) => {
    const candidate = rowIdOf(row);
    return candidate.kind === id.kind && candidate.entryId === id.entryId;
  });
}

/**
 * Keep focus on the same row identity after a disclosure toggle. If the
 * focused row vanished (its level closed), fall back to the toggled anchor's
 * entry row, then the first row — never a stale identity into a list that
 * changed shape (R1: identity, not index).
 */
function clampFocus(context: HistoryCenterContext, anchorEntryId: string | null): HistoryCenterContext {
  const rows = historyCenterVisibleRows(context.pages, context.open);
  if (rows.length === 0) {
    return { ...context, focusRow: null };
  }
  if (context.focusRow !== null && indexOfRow(rows, context.focusRow) !== -1) {
    return context;
  }
  if (anchorEntryId !== null) {
    const anchor = rows.findIndex((row) => row.kind === "entry" && row.entry.id === anchorEntryId);
    if (anchor !== -1) {
      return { ...context, focusRow: rowIdOf(rows[anchor]) };
    }
  }
  return { ...context, focusRow: rowIdOf(rows[0]) };
}

function moveFocus(context: HistoryCenterContext, direction: "next" | "prev" | "first" | "last"): HistoryCenterResult {
  const rows = historyCenterVisibleRows(context.pages, context.open);
  if (rows.length === 0) {
    return stay("open", context);
  }

  const current = indexOfRow(rows, context.focusRow);
  let next: number;
  if (current === -1) {
    // No focus yet (or the focused row is gone): land on a boundary.
    next = direction === "prev" || direction === "last" ? rows.length - 1 : 0;
  } else if (direction === "first") {
    next = 0;
  } else if (direction === "last") {
    next = rows.length - 1;
  } else if (direction === "next") {
    next = (current + 1) % rows.length;
  } else {
    next = (current - 1 + rows.length) % rows.length;
  }

  const row = rowIdOf(rows[next]);
  return {
    state: "open",
    context: { ...context, focusRow: row },
    effects: [{ type: "focusRow", row }],
  };
}

function activateRow(context: HistoryCenterContext, rowId: HistoryCenterRowId | null): HistoryCenterResult {
  if (rowId === null) {
    return stay("open", context);
  }
  const rows = historyCenterVisibleRows(context.pages, context.open);
  const index = indexOfRow(rows, rowId);
  const row = index === -1 ? undefined : rows[index];
  if (row === undefined) {
    return stay("open", context);
  }

  // Sync focus to the activated row; only entry rows navigate.
  if (row.kind === "entry") {
    return {
      state: "open",
      context: { ...context, focusRow: rowId },
      effects: [{ type: "emitNavigateEntry", branchId: row.branchId, entryId: row.entry.id }],
    };
  }
  return { state: "open", context: { ...context, focusRow: rowId }, effects: [] };
}

// ── Disclosure tree helpers ────────────────────────────────────────────

function anchorEntryRow(
  rows: readonly HistoryCenterRow[],
  entryId: string,
): Extract<HistoryCenterRow, { kind: "entry" }> | null {
  const row = rows.find((candidate) => candidate.kind === "entry" && candidate.entry.id === entryId);
  return row !== undefined && row.kind === "entry" ? row : null;
}

function runContains(pages: readonly HistoryPathPage[], entryId: string): boolean {
  return pages.some((page) => page.entries.some((entry) => entry.id === entryId));
}

/** The level whose run contains the entry, deepest first; null when none does. */
function containingLevel(
  open: ReadonlyMap<string, HistoryCenterOpenFork> | null,
  entryId: string,
): HistoryCenterOpenFork | null {
  if (open === null) {
    return null;
  }
  for (const level of open.values()) {
    const inner = containingLevel(level.inner, entryId);
    if (inner !== null) {
      return inner;
    }
    if (runContains(level.runPages, entryId)) {
      return level;
    }
  }
  return null;
}

/** The level anchored at the entry, at any depth; null when none. */
function findLevel(
  open: ReadonlyMap<string, HistoryCenterOpenFork> | null,
  entryId: string,
): HistoryCenterOpenFork | null {
  if (open === null) {
    return null;
  }
  const own = open.get(entryId);
  if (own !== undefined) {
    return own;
  }
  for (const level of open.values()) {
    const found = findLevel(level.inner, entryId);
    if (found !== null) {
      return found;
    }
  }
  return null;
}

/** Remove the level anchored at the entry, dropping its whole subtree (R5). */
function withoutLevel(
  open: ReadonlyMap<string, HistoryCenterOpenFork> | null,
  entryId: string,
): ReadonlyMap<string, HistoryCenterOpenFork> | null {
  if (open === null) {
    return null;
  }
  if (open.has(entryId)) {
    const next = new Map(open);
    next.delete(entryId);
    return next.size === 0 ? null : next;
  }
  let changed = false;
  const next = new Map<string, HistoryCenterOpenFork>();
  for (const [id, level] of open) {
    const inner = withoutLevel(level.inner, entryId);
    next.set(id, inner === level.inner ? level : { ...level, inner });
    changed ||= inner !== level.inner;
  }
  return changed ? next : open;
}

/**
 * Add a level to the tree: under the level whose run contains the entry
 * (inner), or at the root when `container` is null. The container is always
 * found — DISCLOSE only fires when the anchor's entry row is visible, which
 * means its containing run is loaded.
 */
function withAddedLevel(
  open: ReadonlyMap<string, HistoryCenterOpenFork> | null,
  container: HistoryCenterOpenFork | null,
  level: HistoryCenterOpenFork,
): ReadonlyMap<string, HistoryCenterOpenFork> {
  if (container === null) {
    const next = new Map(open ?? []);
    next.set(level.anchorEntryId, level);
    return next;
  }
  // The container level must already be in the tree; rebuild it with the new
  // inner level. Unreachable when DISCLOSE's visibility guard held.
  const base = new Map(open ?? []);
  for (const [id, candidate] of base) {
    if (candidate === container) {
      base.set(id, { ...candidate, inner: withAddedLevel(candidate.inner, null, level) });
      return base;
    }
    base.set(id, replaceContainer(candidate, container, level));
  }
  return base;
}

function replaceContainer(
  level: HistoryCenterOpenFork,
  container: HistoryCenterOpenFork,
  added: HistoryCenterOpenFork,
): HistoryCenterOpenFork {
  if (level === container) {
    return { ...level, inner: withAddedLevel(level.inner, null, added) };
  }
  if (level.inner === null) {
    return level;
  }
  const inner = new Map(level.inner);
  let changed = false;
  for (const [id, candidate] of inner) {
    const replaced = replaceContainer(candidate, container, added);
    if (replaced !== candidate) {
      inner.set(id, replaced);
      changed = true;
    }
  }
  return changed ? { ...level, inner } : level;
}

/** The anchor's run context (display-ordered entries and its index there). */
function anchorRunContext(
  context: HistoryCenterContext,
  entryId: string,
): { entries: HistoryEntry[]; index: number } | null {
  const root = historyCenterJoinPages(context.pages ?? []);
  const rootIndex = root.findIndex((entry) => entry.id === entryId);
  if (rootIndex !== -1) {
    return { entries: root, index: rootIndex };
  }
  for (const level of walkLevels(context.open)) {
    const run = historyCenterJoinPages(level.runPages);
    const index = run.findIndex((entry) => entry.id === entryId);
    if (index !== -1) {
      return { entries: run, index };
    }
  }
  return null;
}

function* walkLevels(open: ReadonlyMap<string, HistoryCenterOpenFork> | null): Generator<HistoryCenterOpenFork> {
  if (open === null) {
    return;
  }
  for (const level of open.values()) {
    yield level;
    yield* walkLevels(level.inner);
  }
}

function disclose(context: HistoryCenterContext, entryId: string): HistoryCenterResult {
  const rows = historyCenterVisibleRows(context.pages, context.open);
  const anchor = anchorEntryRow(rows, entryId);
  if (anchor === null) {
    return stay("open", context);
  }

  // Toggle: a fork already open at this entry closes, dropping its subtree (R5).
  if (findLevel(context.open, entryId) !== null) {
    const open = withoutLevel(context.open, entryId);
    return {
      state: "open",
      context: clampFocus({ ...context, open }, entryId),
      effects: [],
    };
  }

  if (historyCenterForkCount(anchor.entry.continuationCount) < 1) {
    return stay("open", context);
  }

  const level: HistoryCenterOpenFork = {
    anchorEntryId: entryId,
    continuations: null,
    pick: null,
    chosen: null,
    runPages: [],
    inner: null,
  };
  const container = containingLevel(context.open, entryId);
  const open = withAddedLevel(context.open, container, level);
  return {
    state: "open",
    context: clampFocus({ ...context, open }, entryId),
    effects: [{ type: "loadContinuations", entryId }],
  };
}

function continuationsLoaded(
  context: HistoryCenterContext,
  entryId: string,
  continuations: HistoryContinuation[],
): HistoryCenterResult {
  const level = findLevel(context.open, entryId);
  if (level === null) {
    // A response for an entry that is not open: stale, drop it.
    return stay("open", context);
  }

  let updated: HistoryCenterOpenFork = { ...level, continuations };
  const effects: HistoryCenterEffect[] = [];

  const runContext = anchorRunContext(context, entryId);
  if (runContext !== null) {
    const anchor = runContext.entries[runContext.index];
    const forkCount = historyCenterForkCount(anchor.continuationCount);
    if (forkCount === 1) {
      // Exactly one fork: no picker needed — choose it and request its run.
      const forks = historyCenterForksAt(continuations, runContext.entries, runContext.index);
      if (forks.length >= 1) {
        updated = { ...updated, chosen: forks[0] };
        effects.push({ type: "loadContinuationRun", fromEntryId: forks[0].entryId });
      }
    } else if (forkCount > 1) {
      // R3: more than one fork — select the current one (preferred; first in
      // supplied order when none is preferred) and show its run. The select
      // previews; checkout is CONFIRM's job (R2).
      const forks = historyCenterForksAt(continuations, runContext.entries, runContext.index);
      const initial = forks.find((fork) => fork.preferred) ?? forks[0] ?? null;
      if (initial !== null) {
        updated = { ...updated, pick: initial };
        effects.push({ type: "loadContinuationRun", fromEntryId: initial.entryId });
      }
    }
  }

  const open = replaceLevel(context.open, updated);
  return {
    state: "open",
    context: clampFocus({ ...context, open }, entryId),
    effects,
  };
}

function replaceLevel(
  open: ReadonlyMap<string, HistoryCenterOpenFork> | null,
  updated: HistoryCenterOpenFork,
): ReadonlyMap<string, HistoryCenterOpenFork> | null {
  if (open === null) {
    return null;
  }
  const own = open.get(updated.anchorEntryId);
  if (own !== undefined) {
    const next = new Map(open);
    next.set(updated.anchorEntryId, updated);
    return next;
  }
  let changed = false;
  const next = new Map<string, HistoryCenterOpenFork>();
  for (const [id, level] of open) {
    const inner = replaceLevel(level.inner, updated);
    next.set(id, inner === level.inner ? level : { ...level, inner });
    changed ||= inner !== level.inner;
  }
  return changed ? next : open;
}

function pickContinuation(context: HistoryCenterContext, entryId: string): HistoryCenterResult {
  // The tentative pick lands on the level whose picker offers this fork.
  let picked: HistoryCenterOpenFork | null = null;
  let match: HistoryContinuation | null = null;
  for (const level of walkLevels(context.open)) {
    if (level.continuations === null) {
      continue;
    }
    const runContext = anchorRunContext(context, level.anchorEntryId);
    if (runContext === null) {
      continue;
    }
    const anchor = runContext.entries[runContext.index];
    if (historyCenterForkCount(anchor.continuationCount) <= 1) {
      continue;
    }
    const forks = historyCenterForksAt(level.continuations, runContext.entries, runContext.index);
    const candidate = forks.find((fork) => fork.entryId === entryId);
    if (candidate !== undefined) {
      picked = level;
      match = candidate;
      break;
    }
  }
  if (picked === null || match === null) {
    return stay("open", context);
  }

  // R2: the entries below the select follow the pick. When the loaded run
  // belongs to another fork, drop it and load the picked fork's run; the
  // pick itself commits nothing and emits no host operation — checkout is
  // CONFIRM's job.
  const previous = picked.pick ?? picked.chosen;
  const switching = previous === null || previous.entryId !== match.entryId;
  const updated = { ...picked, pick: match, ...(switching ? { runPages: [] } : {}) };

  // One tentative pick at a time: clear any other level's pick.
  let open = replaceLevel(context.open, updated);
  for (const level of walkLevels(open)) {
    if (level !== updated && level.pick !== null) {
      open = replaceLevel(open, { ...level, pick: null });
    }
  }
  return {
    state: "open",
    context: clampFocus({ ...context, open }, null),
    effects: switching ? [{ type: "loadContinuationRun", fromEntryId: match.entryId }] : [],
  };
}

function confirm(context: HistoryCenterContext): HistoryCenterResult {
  let pickedLevel: HistoryCenterOpenFork | null = null;
  // Captured in the loop rather than re-read from `pick`: the guard below
  // proves it non-null, but narrowing does not survive the assignment.
  let pick: HistoryContinuation | null = null;
  for (const level of walkLevels(context.open)) {
    // The auto-chosen single fork counts as picked (R1, g13-034): CONFIRM
    // commits the displayed fork — the tentative pick, else the chosen one.
    const shown = level.pick ?? level.chosen;
    if (shown !== null) {
      pickedLevel = level;
      pick = shown;
      break;
    }
  }
  if (pickedLevel === null || pick === null) {
    return stay("open", context);
  }

  // R2: checkout makes the selected fork primary. Poodle does not build the
  // new root — it emits the command and renders whatever root pages the host
  // supplies afterwards. The fork is becoming the root, so the open level no
  // longer describes anything: clear the disclosure state for the anchor.
  const open = withoutLevel(context.open, pickedLevel.anchorEntryId);
  return {
    state: "open",
    context: clampFocus({ ...context, open }, pickedLevel.anchorEntryId),
    effects: [{ type: "checkoutContinuation", entryId: pick.entryId }],
  };
}

/**
 * b033 (R4): delete is a host command for the selected fork — the fork the
 * level's picker shows, exactly as checkout would commit. Poodle deletes
 * nothing itself and does not guess at the resulting history: the effect
 * leaves, the host runs the operation and supplies new pages. No
 * confirmation inside Poodle — that is the host's call. The fork must be
 * offered by an open level's picker (the auto-chosen single fork included);
 * unknown forks are inert.
 */
function deleteContinuation(context: HistoryCenterContext, entryId: string): HistoryCenterResult {
  for (const level of walkLevels(context.open)) {
    if (level.continuations === null) {
      continue;
    }
    const runContext = anchorRunContext(context, level.anchorEntryId);
    if (runContext === null) {
      continue;
    }
    const forks = historyCenterForksAt(level.continuations, runContext.entries, runContext.index);
    if (forks.some((fork) => fork.entryId === entryId)) {
      // The level's cached data describes a fork that is about to stop
      // existing, so it is invalidated here rather than by the stale rule.
      // Staleness is "the shown fork's run is now on the spine", and a deleted
      // fork never reaches the spine — its entries simply vanish. Left alone,
      // the cached run keeps rendering indented under the anchor and clicking
      // it navigates to an entry the host has deleted ("Entry does not exist").
      //
      // Same shape as the stale reconcile: the anchor stays open (disclosure is
      // UI state, b028 R1), the loaded data goes, and the continuations are
      // re-requested so the picker re-reads without the deleted fork. Nested
      // anchors must be replaced where they already live — a root Map.set
      // leaves the stale inner level in place and adds a ghost root key.
      const invalidated: HistoryCenterOpenFork = {
        anchorEntryId: level.anchorEntryId,
        continuations: null,
        pick: null,
        chosen: null,
        runPages: [],
        inner: null,
      };
      const open = replaceLevel(context.open, invalidated);
      return {
        state: "open",
        context: { ...context, open },
        effects: [
          { type: "deleteContinuation", entryId },
          { type: "loadContinuations", entryId: level.anchorEntryId },
        ],
      };
    }
  }
  return stay("open", context);
}

function runLoaded(context: HistoryCenterContext, fromEntryId: string, pages: HistoryPathPage[]): HistoryCenterResult {
  let updated: HistoryCenterOpenFork | null = null;
  for (const level of walkLevels(context.open)) {
    // The run below the select follows the displayed fork — the pick, or the
    // auto-chosen single fork (R2, R3).
    const shown = level.pick ?? level.chosen;
    if (shown !== null && shown.entryId === fromEntryId) {
      // Pages arrive in fetch order; append so the join can reverse once.
      updated = { ...level, runPages: [...level.runPages, ...pages] };
      break;
    }
  }
  if (updated === null) {
    return stay("open", context);
  }
  const open = replaceLevel(context.open, updated);
  return {
    state: "open",
    context: clampFocus({ ...context, open }, null),
    effects: [],
  };
}

// ── Transitions ────────────────────────────────────────────────────────

/**
 * R2 (g13-034): reconcile open levels against the current root pages before
 * an open-state event runs. A level whose shown fork's first entry now sits
 * on the joined root pages is stale — the host navigated into the fork and
 * supplied root pages that already contain its run. The level drops its
 * loaded data (continuations, pick, chosen, run pages and the inner subtree:
 * they describe a run the host already made primary) but stays open at its
 * anchor — disclosure is UI state and persists (b028 R1); this is not a
 * close. The re-request rides the existing `loadContinuations` effect so the
 * picker re-reads its continuations and offers the line just left; until the
 * result lands the level renders the existing not-yet-loaded row. The check
 * is data-based and idempotent: once dropped, the level has no shown fork,
 * so a later transition emits nothing — exactly one re-request, never per
 * derivation.
 */
function reconcileStaleLevels(
  context: HistoryCenterContext,
): { context: HistoryCenterContext; effects: HistoryCenterEffect[] } {
  if (context.pages === null || context.pages.length === 0 || context.open === null) {
    return { context, effects: [] };
  }
  const rootIds = new Set(historyCenterJoinPages(context.pages).map((entry) => entry.id));
  const effects: HistoryCenterEffect[] = [];
  const open = dropStaleLevels(context.open, rootIds, effects);
  if (open === context.open) {
    return { context, effects: [] };
  }
  return { context: { ...context, open }, effects };
}

function dropStaleLevels(
  open: ReadonlyMap<string, HistoryCenterOpenFork> | null,
  rootIds: ReadonlySet<string>,
  effects: HistoryCenterEffect[],
): ReadonlyMap<string, HistoryCenterOpenFork> | null {
  if (open === null) {
    return null;
  }
  let changed = false;
  const next = new Map<string, HistoryCenterOpenFork>();
  for (const [id, level] of open) {
    const shown = level.pick ?? level.chosen;
    if (shown !== null && rootIds.has(shown.entryId)) {
      // Stale: drop the level's loaded data and its subtree, keep the anchor
      // open, re-request the continuations. Same shape as a fresh disclosure
      // (R5) minus the anchor, so the level renders the not-yet-loaded row.
      next.set(id, {
        anchorEntryId: id,
        continuations: null,
        pick: null,
        chosen: null,
        runPages: [],
        inner: null,
      });
      effects.push({ type: "loadContinuations", entryId: id });
      changed = true;
    } else {
      const inner = dropStaleLevels(level.inner, rootIds, effects);
      next.set(id, inner === level.inner ? level : { ...level, inner });
      changed ||= inner !== level.inner;
    }
  }
  return changed ? next : open;
}

export function historyCenterTransition(
  state: HistoryCenterState,
  context: HistoryCenterContext,
  event: HistoryCenterEvent,
): HistoryCenterResult {
  // R2 (g13-034): reconcile before every open-state event except the two that
  // close the popover — CLOSE and a closing TOGGLE drop the whole tree anyway
  // and must not emit a re-request on the way out. The caller supplies the
  // current pages on every transition, so the pass sees the new root as soon
  // as the host has navigated; the event then runs against the reconciled
  // state.
  if (state === "open" && event.type !== "CLOSE" && event.type !== "TOGGLE") {
    const reconciled = reconcileStaleLevels(context);
    if (reconciled.context !== context || reconciled.effects.length > 0) {
      const next = dispatch(state, reconciled.context, event);
      return { state: next.state, context: next.context, effects: [...reconciled.effects, ...next.effects] };
    }
  }
  return dispatch(state, context, event);
}

function dispatch(
  state: HistoryCenterState,
  context: HistoryCenterContext,
  event: HistoryCenterEvent,
): HistoryCenterResult {
  switch (event.type) {
    case "PAGES_CHANGED":
      // Deliberately inert. The reconcile above this call is the whole point;
      // returning `context` unchanged keeps the adapter's state assignment a
      // no-op when nothing was stale.
      return { state, context, effects: [] };
    case "TOGGLE":
      return state === "closed" ? openResult(context) : closeResult(context);
    case "OPEN":
      return state === "closed" ? openResult(context) : stay(state, context);
    case "CLOSE":
      return state === "open" ? closeResult(context) : stay(state, context);
    case "FOCUS_MOVE":
      return state === "open" ? moveFocus(context, event.direction) : stay(state, context);
    case "ACTIVATE_ROW":
      return state === "open" ? activateRow(context, event.row ?? context.focusRow) : stay(state, context);
    case "DISCLOSE":
      return state === "open" ? disclose(context, event.entryId) : stay(state, context);
    case "CONTINUATIONS_LOADED":
      return state === "open"
        ? continuationsLoaded(context, event.entryId, event.continuations)
        : stay(state, context);
    case "PICK_CONTINUATION":
      return state === "open" ? pickContinuation(context, event.entryId) : stay(state, context);
    case "CONFIRM":
      return state === "open" ? confirm(context) : stay(state, context);
    case "DELETE_CONTINUATION":
      return state === "open" ? deleteContinuation(context, event.entryId) : stay(state, context);
    case "RUN_LOADED":
      return state === "open" ? runLoaded(context, event.fromEntryId, event.pages) : stay(state, context);
    case "RENAME":
      return {
        state,
        context,
        effects: [{ type: "emitRenameBranch", branchId: event.branchId, name: event.name }],
      };
    case "SHOW_REJECTION": {
      const message = historyCenterRejectionMessage(event.code);
      return message === context.rejection
        ? stay(state, context)
        : { state, context: { ...context, rejection: message }, effects: [] };
    }
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
