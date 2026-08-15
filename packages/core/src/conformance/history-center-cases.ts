/**
 * HistoryCenter executable cases — composite profile (g14.007).
 *
 * The corpus covers the profiles the card names: empty, linear, multiple-fork,
 * nested-fork, deep depth, current entry, loading and disabled, and the narrow
 * bounded surface. Every case drives real interactions — a pointer on a
 * trigger, a real Escape through the dismissal route, arrow keys through the
 * roving list, a real edit in the rename input — and asserts what the runtime
 * actually did: which rows exist, at which hierarchy level, which command left
 * with which payload, and in what order.
 *
 * Fixture conventions (fixture hosts interpret these; generic runners do not):
 *
 * - `props.pages` is the spine, in fetch order, newest page first. Display is
 *   oldest-first, and the join reverses exactly once, so the page at the higher
 *   offset renders before the first page. `linear-page-seam` is the case that
 *   would fail if a host joined in fetch order.
 * - `host.continuations` is the fork catalogue. A fixture host answers
 *   `loadContinuations(entryId)` with every record whose `anchorEntryId`
 *   matches — the child already on the list included, because filtering that
 *   one out by id is the component's job and not the host's.
 * - `host.runEntries` is every fork run's entries, newest first, tagged with
 *   the fork they belong to. A host answers `loadContinuationRun(fromEntryId)`
 *   with the matching records as one page.
 *
 * Nothing here invents a timestamp, and no case supplies one it does not then
 * assert: `recordedAtMs` is a host fact or it is absent.
 */

import {
  actionDismiss,
  actionFocus,
  actionInsert,
  actionKey,
  actionPress,
  componentCase,
  expectEvents,
  expectPart,
  serializeCases,
  type ComponentCase,
} from "./define";
import { historyCenterInterface, type HistoryCenterInterface } from "./history-center";

type Case = ComponentCase<HistoryCenterInterface>;

/** One entry record, spelled once so a case reads as history rather than as
 * field noise. `continuations` counts every child, the run's own next row
 * included — a fork count is one less. */
function entry(
  id: string,
  label: string,
  position: "past" | "current" | "future",
  continuationCount: number,
  extra: { checkpoint?: boolean; groupId?: string } = {},
) {
  return { id, label, position, continuationCount, ...extra };
}

/** One page of a path: bounded records, newest first. */
function page(
  offset: number,
  entries: ReturnType<typeof entry>[],
  extra: {
    precedingContinuationCount?: number;
    truncatedBefore?: boolean;
    truncatedAfter?: boolean;
  } = {},
) {
  return {
    offset,
    precedingContinuationCount: extra.precedingContinuationCount ?? 0,
    truncatedBefore: extra.truncatedBefore ?? false,
    truncatedAfter: extra.truncatedAfter ?? false,
    entries,
  };
}

/** One fork at an anchor. `preferred` marks the continuation a redo would
 * take; it is also how the derivation recognises the child already on the
 * list when paging truncated the successor away. */
function fork(
  anchorEntryId: string,
  entryId: string,
  label: string,
  branchId: string,
  options: { preferred?: boolean; entryCount?: number; branchName?: string } = {},
) {
  return {
    anchorEntryId,
    entryId,
    label,
    preferred: options.preferred ?? false,
    entryCount: options.entryCount ?? 1,
    branchId,
    ...(options.branchName === undefined ? {} : { branchName: options.branchName }),
  };
}

/** One entry of a fork's run, tagged with the fork it belongs to. */
function runEntry(
  fromEntryId: string,
  id: string,
  label: string,
  position: "past" | "current" | "future",
  continuationCount: number,
  extra: { checkpoint?: boolean } = {},
) {
  return { fromEntryId, ...entry(id, label, position, continuationCount, extra) };
}

// ── Shared fixtures ────────────────────────────────────────────────────────

/** A three-entry spine with the current position in the middle and no forks.
 * Newest first, so display order is e1, e2, e3. */
const LINEAR_PAGES = [page(0, [entry("e3", "Raise gain", "future", 0), entry("e2", "Trim tail", "current", 1), entry("e1", "Import stems", "past", 1)])];

/** The same spine with two forks at `e2`: its continuation count is 3, so the
 * fork count is 2 and the picker appears. */
const FORKED_PAGES = [
  page(0, [
    entry("e3", "Raise gain", "future", 0),
    entry("e2", "Trim tail", "current", 3),
    entry("e1", "Import stems", "past", 1, { checkpoint: true }),
  ]),
];

/** The forks at `e2`. `e3` is the child already on the list; the derivation
 * filters it out by id, so the picker offers `f1` and `f2` alone. */
const FORKED_CONTINUATIONS = [
  fork("e2", "e3", "Raise gain", "main", { preferred: true, entryCount: 1 }),
  fork("e2", "f1", "Widen stereo", "branch-wide", { entryCount: 2, branchName: "Wide mix" }),
  fork("e2", "f2", "Duck bass", "branch-duck", { entryCount: 1, branchName: "Duck bass" }),
];

const FORKED_RUN_ENTRIES = [
  // The `f1` run, newest first: display order is f1, f1b. `f1b` forks again,
  // which is what the nested case discloses.
  runEntry("f1", "f1b", "Add shimmer", "future", 2),
  runEntry("f1", "f1", "Widen stereo", "future", 1),
  runEntry("f2", "f2", "Duck bass", "future", 0),
  // The fork inside the `f1` run, which forks again — the depth case walks
  // all the way down it.
  runEntry("f1b1", "f1b1", "Tame sibilance", "future", 2),
  runEntry("f1b1a", "f1b1a", "Soften air", "future", 0),
];

const NESTED_CONTINUATIONS = [
  ...FORKED_CONTINUATIONS,
  // `f1b` has a continuation count of 2, so one fork after its own next row is
  // filtered — except its run has no next row, so the preferred child is the
  // one already on the list and `f1b1` remains offered.
  fork("f1b", "f1b0", "Keep sibilance", "branch-wide", { preferred: true, entryCount: 1 }),
  fork("f1b", "f1b1", "Tame sibilance", "branch-tame", { entryCount: 1, branchName: "Tame" }),
  // And once more, so the depth case has a fourth level to reach.
  fork("f1b1", "f1b1p", "Keep air", "branch-tame", { preferred: true, entryCount: 1 }),
  fork("f1b1", "f1b1a", "Soften air", "branch-air", { entryCount: 1, branchName: "Air" }),
];

const cases: Case[] = [
  // ── Empty ──────────────────────────────────────────────────────────────
  componentCase(historyCenterInterface, {
    id: "history-center/empty-no-pages",
    fixture: { props: { pages: null, defaultOpen: true }, regions: {} },
    specimen: {
      group: "Empty",
      caption: "No history supplied",
      captureId: "history-center/empty-no-pages",
      axes: [],
    },
    steps: [
      expectPart("surface", { present: true, role: "dialog" }),
      // Absence is the signal: no pages, no rows, no list — the empty state,
      // not an empty list region pretending to be one.
      expectPart("empty", { present: true }),
      expectPart("root", { states: { open: true, empty: true } }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/empty-zero-rows",
    fixture: { props: { pages: [], defaultOpen: true }, regions: {} },
    specimen: {
      group: "Empty",
      caption: "Pages supplied but no entries",
      captureId: "history-center/empty-zero-rows",
      axes: [],
    },
    steps: [
      expectPart("empty", { present: true }),
      expectPart("root", { states: { empty: true } }),
    ],
  }),

  // ── Linear spine ───────────────────────────────────────────────────────
  componentCase(historyCenterInterface, {
    id: "history-center/linear-rows",
    fixture: { props: { pages: LINEAR_PAGES, defaultOpen: true }, regions: {} },
    specimen: {
      group: "Linear",
      caption: "A flat spine, oldest first",
      captureId: "history-center/linear-rows",
      axes: ["size"],
    },
    steps: [
      // Display order is oldest first, and every row sits at level 1: the
      // spine is not a hierarchy.
      expectPart("row:e1", { present: true, level: 1 }),
      expectPart("row:e2", { present: true, level: 1 }),
      expectPart("row:e3", { present: true, level: 1 }),
      expectPart("entry:e1", { present: true, role: "button", name: "Import stems" }),
      expectPart("entry:e3", { present: true, name: "Raise gain" }),
      // No forks anywhere on the spine, so no disclosure is rendered at all —
      // never a disabled stand-in for "nothing to disclose".
      expectPart("disclosure:e3", { present: false }),
      expectPart("picker:e2", { present: false }),
      expectPart("empty", { present: false }),
      // The list is a bounded scroll region, not a column that grows with the
      // history.
      expectPart("list", { present: true, role: "list", scrollable: true }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/linear-page-seam",
    fixture: {
      props: {
        pages: [
          page(0, [entry("e4", "Bounce", "current", 0), entry("e3", "Raise gain", "past", 1)], {
            truncatedAfter: true,
          }),
          // Fetched second, older: it must render *before* the first page.
          page(2, [entry("e2", "Trim tail", "past", 1), entry("e1", "Import stems", "past", 1)], {
            truncatedBefore: true,
          }),
        ],
        defaultOpen: true,
      },
      regions: {},
    },
    specimen: {
      group: "Linear",
      caption: "Two pages joined into one timeline",
      captureId: "history-center/linear-page-seam",
      axes: [],
    },
    steps: [
      // Each entry appears exactly once and at the same level; joining in
      // fetch order would put the older page last and read history backwards.
      expectPart("row:e1", { present: true, level: 1 }),
      expectPart("row:e2", { present: true, level: 1 }),
      expectPart("row:e3", { present: true, level: 1 }),
      expectPart("row:e4", { present: true, level: 1 }),
      expectPart("entry:e1", { name: "Import stems" }),
      expectPart("entry:e4", { name: "Bounce" }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/navigate-spine-entry",
    fixture: { props: { pages: LINEAR_PAGES, defaultOpen: true }, regions: {} },
    specimen: {
      group: "Navigation",
      caption: "Activating a spine row",
      captureId: "history-center/navigate-spine-entry",
      axes: [],
    },
    steps: [
      actionPress("entry:e1"),
      // The entry actually activated, on the branch that owns its run. The
      // spine's branch is empty: the host knows its own branch.
      expectEvents([{ name: "navigateEntry", payload: { branchId: "", entryId: "e1" } }]),
      expectPart("root", { states: { open: true } }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/current-entry",
    fixture: { props: { pages: LINEAR_PAGES, defaultOpen: true }, regions: {} },
    specimen: {
      group: "Position",
      caption: "The current position is marked",
      captureId: "history-center/current-entry",
      axes: [],
    },
    steps: [
      // The marker is decorative; position is what the row projects, and the
      // label is what carries the meaning.
      expectPart("row:e2", { present: true, level: 1 }),
      expectPart("entry:e2", { name: "Trim tail" }),
      expectPart("entry:e1", { name: "Import stems" }),
    ],
  }),

  // ── Trigger cluster and open state ─────────────────────────────────────
  componentCase(historyCenterInterface, {
    id: "history-center/open-and-dismiss",
    fixture: { props: { pages: LINEAR_PAGES, canUndo: true, canRedo: true }, regions: {} },
    specimen: {
      group: "Disclosure",
      caption: "Open from the trigger, close with Escape",
      captureId: "history-center/open-and-dismiss",
      axes: ["placement"],
    },
    steps: [
      expectPart("root", { states: { open: false } }),
      expectPart("list-trigger", { present: true, role: "button", expanded: false }),
      actionPress("list-trigger"),
      expectEvents([{ name: "openChange", payload: { open: true } }]),
      expectPart("root", { states: { open: true } }),
      expectPart("list-trigger", { expanded: true }),
      // The real dismissal route, not a callback poke.
      actionDismiss("surface"),
      expectEvents([
        { name: "openChange", payload: { open: true } },
        { name: "openChange", payload: { open: false } },
      ]),
      expectPart("root", { states: { open: false } }),
      // Focus comes back to the trigger that opened the surface.
      expectPart("list-trigger", { focused: true }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/undo-redo-commands",
    fixture: { props: { pages: LINEAR_PAGES, canUndo: true, canRedo: true }, regions: {} },
    specimen: {
      group: "Commands",
      caption: "Undo and redo are plain commands",
      captureId: "history-center/undo-redo-commands",
      axes: [],
    },
    steps: [
      expectPart("undo", { present: true, role: "button", name: "Undo" }),
      expectPart("redo", { present: true, role: "button", name: "Redo" }),
      expectPart("root", { states: { undoDisabled: false, redoDisabled: false } }),
      actionPress("undo"),
      actionPress("redo"),
      // Order is the claim as much as the names are: the host decides what
      // undo means, and it must be told in the order the operator asked.
      expectEvents(["undo", "redo"]),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/busy-disables-commands",
    fixture: {
      props: { pages: LINEAR_PAGES, canUndo: true, canRedo: true, busy: true },
      regions: {},
    },
    specimen: {
      group: "Commands",
      caption: "An authority operation disables both triggers",
      captureId: "history-center/busy-disables-commands",
      axes: [],
    },
    steps: [
      expectPart("root", { states: { undoDisabled: true, redoDisabled: true } }),
      expectPart("undo", { focusable: false }),
      actionPress("undo"),
      actionPress("redo"),
      // Inert, not merely dimmed: a disabled trigger emits nothing.
      expectEvents([]),
    ],
  }),

  // ── Loading, failed, rejected ──────────────────────────────────────────
  componentCase(historyCenterInterface, {
    id: "history-center/loading-status",
    fixture: {
      props: { pages: LINEAR_PAGES, defaultOpen: true, status: "loading" },
      regions: {},
    },
    specimen: {
      group: "Status",
      caption: "Loading announces politely",
      captureId: "history-center/loading-status",
      axes: [],
    },
    steps: [
      expectPart("status", { present: true, role: "status" }),
      expectPart("root", { tokenRoles: { status: "loading" } }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/failed-status",
    fixture: {
      props: {
        pages: LINEAR_PAGES,
        defaultOpen: true,
        status: "failed",
        statusMessage: "History source unavailable",
      },
      regions: {},
    },
    specimen: {
      group: "Status",
      caption: "A failed source shows its message",
      captureId: "history-center/failed-status",
      axes: [],
    },
    steps: [
      expectPart("status", { present: true, role: "status", text: "History source unavailable" }),
      expectPart("root", { tokenRoles: { status: "failed" } }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/rejection-notice",
    fixture: {
      props: { pages: LINEAR_PAGES, defaultOpen: true, rejection: "AlreadyAtTarget" },
      regions: {},
    },
    specimen: {
      group: "Status",
      caption: "A mapped rejection is displayed",
      captureId: "history-center/rejection-notice",
      axes: [],
    },
    steps: [
      // The component owns the copy; the host's protocol never reaches it.
      expectPart("rejection", {
        present: true,
        role: "status",
        text: "Already at the requested target",
      }),
      expectPart("root", { states: { rejected: true } }),
    ],
  }),

  // ── Roving focus ───────────────────────────────────────────────────────
  componentCase(historyCenterInterface, {
    id: "history-center/roving-focus",
    fixture: { props: { pages: LINEAR_PAGES, defaultOpen: true }, regions: {} },
    specimen: {
      group: "Navigation",
      caption: "Arrow keys rove the visible rows",
      captureId: "history-center/roving-focus",
      axes: [],
    },
    steps: [
      // Roving focus is established by the keyboard, not by a programmatic
      // focus call: Home from nothing lands on the first row, which is what an
      // operator tabbing into the list and pressing Home actually does.
      actionKey("entry:e1", "Home"),
      expectPart("entry:e1", { focused: true, tabbable: true }),
      actionKey("entry:e1", "ArrowDown"),
      // Focus is the row identity, not an index — a disclosure toggle would
      // change the list shape under an index.
      expectPart("entry:e2", { focused: true, tabbable: true }),
      expectPart("entry:e1", { tabbable: false }),
      actionKey("entry:e2", "End"),
      expectPart("entry:e3", { focused: true }),
      actionKey("entry:e3", "Home"),
      expectPart("entry:e1", { focused: true }),
      actionKey("entry:e1", "Enter"),
      expectEvents([{ name: "navigateEntry", payload: { branchId: "", entryId: "e1" } }]),
    ],
  }),

  // ── Forks: disclosure, picker, selection, checkout, rename ─────────────
  componentCase(historyCenterInterface, {
    id: "history-center/multiple-fork-disclosure",
    fixture: {
      props: { pages: FORKED_PAGES, defaultOpen: true },
      regions: {},
      host: { continuations: FORKED_CONTINUATIONS, runEntries: FORKED_RUN_ENTRIES },
    },
    specimen: {
      group: "Forks",
      caption: "Two forks disclose a picker",
      captureId: "history-center/multiple-fork-disclosure",
      axes: [],
    },
    steps: [
      // Three children at e2, so two forks: the count on the record includes
      // the run's own next row.
      expectPart("disclosure:e2", { present: true, role: "button", expanded: false }),
      expectPart("picker:e2", { present: false }),
      actionPress("disclosure:e2"),
      // Disclosure asks the host, and the host answers in the same flush —
      // there is no moment where the request has left and the answer has not
      // arrived. It never loads speculatively, which is what the closed-state
      // assertion above proves.
      expectPart("disclosure:e2", { expanded: true }),
      // The picker sits one level below its anchor.
      expectPart("picker:e2", { present: true, level: 2 }),
      // The child already on the list is filtered out by id: e3 is not
      // offered. The options live in the select's listbox, so the operator
      // opens it to see them.
      actionPress("picker-select:e2"),
      expectPart("picker-option:f1", { present: true, role: "option" }),
      expectPart("picker-option:f2", { present: true, role: "option" }),
      expectPart("picker-option:e3", { present: false }),
      // The current fork is selected and its run is requested — the pick
      // previews, it does not commit.
      expectEvents([
        { name: "loadContinuations", payload: { entryId: "e2" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1" } },
      ]),
      // The run renders below the select, at the fork's own level.
      expectPart("row:f1", { present: true, level: 2 }),
      expectPart("row:f1b", { present: true, level: 2 }),
      // Every entry still appears exactly once: the spine is untouched.
      expectPart("row:e1", { level: 1 }),
      expectPart("row:e3", { level: 1 }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/fork-run-navigation",
    fixture: {
      props: { pages: FORKED_PAGES, defaultOpen: true },
      regions: {},
      host: { continuations: FORKED_CONTINUATIONS, runEntries: FORKED_RUN_ENTRIES },
    },
    specimen: {
      group: "Forks",
      caption: "A fork run row navigates on its own branch",
      captureId: "history-center/fork-run-navigation",
      axes: [],
    },
    steps: [
      actionPress("disclosure:e2"),
      actionPress("entry:f1b"),
      // The clicked row's own branch and entry — never the anchor, never the
      // branch's divergence entry.
      expectEvents([
        { name: "loadContinuations", payload: { entryId: "e2" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1" } },
        { name: "navigateEntry", payload: { branchId: "branch-wide", entryId: "f1b" } },
      ]),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/select-other-fork",
    fixture: {
      props: { pages: FORKED_PAGES, defaultOpen: true },
      regions: {},
      host: { continuations: FORKED_CONTINUATIONS, runEntries: FORKED_RUN_ENTRIES },
    },
    specimen: {
      group: "Forks",
      caption: "Picking the other fork previews its run",
      captureId: "history-center/select-other-fork",
      axes: [],
    },
    steps: [
      actionPress("disclosure:e2"),
      actionPress("picker-select:e2"),
      expectPart("picker-option:f1", { selected: true }),
      expectPart("picker-option:f2", { selected: false }),
      actionPress("picker-option:f2"),
      // The pick previews and emits no host operation of its own: the run
      // load is the preview, checkout is a separate, deliberate commit.
      expectEvents([
        { name: "loadContinuations", payload: { entryId: "e2" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f2" } },
      ]),
      actionPress("picker-select:e2"),
      expectPart("picker-option:f2", { selected: true }),
      expectPart("picker-option:f1", { selected: false }),
      // The previously shown run is gone; only the picked fork's run renders.
      expectPart("row:f2", { present: true, level: 2 }),
      expectPart("row:f1", { present: false }),
      expectPart("row:f1b", { present: false }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/checkout-selected-fork",
    fixture: {
      props: { pages: FORKED_PAGES, defaultOpen: true },
      regions: {},
      host: { continuations: FORKED_CONTINUATIONS, runEntries: FORKED_RUN_ENTRIES },
    },
    specimen: {
      group: "Forks",
      caption: "Checkout commits the shown fork",
      captureId: "history-center/checkout-selected-fork",
      axes: [],
    },
    steps: [
      actionPress("disclosure:e2"),
      actionPress("picker-actions:e2"),
      expectPart("action-checkout", { present: true, role: "menuitem" }),
      actionPress("action-checkout"),
      // Checkout leaves alone: the run was already previewed by the pick, and
      // the document does not move forward. Poodle clears the anchor's
      // disclosure and renders whatever root pages the host supplies next.
      expectEvents([
        { name: "loadContinuations", payload: { entryId: "e2" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1" } },
        { name: "checkoutContinuation", payload: { entryId: "f1" } },
      ]),
      expectPart("picker:e2", { present: false }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/rename-selected-branch",
    fixture: {
      props: { pages: FORKED_PAGES, defaultOpen: true, maxBranchNameBytes: 32 },
      regions: {},
      host: { continuations: FORKED_CONTINUATIONS, runEntries: FORKED_RUN_ENTRIES },
    },
    specimen: {
      group: "Forks",
      caption: "Rename the branch the picker shows",
      captureId: "history-center/rename-selected-branch",
      axes: [],
    },
    steps: [
      actionPress("disclosure:e2"),
      actionPress("picker-actions:e2"),
      actionPress("action-rename"),
      // The inline input takes the Select's place, seeded with the shown
      // fork's current name.
      expectPart("rename-input:e2", { present: true, role: "textbox", value: "Wide mix" }),
      expectPart("picker-select:e2", { present: false }),
      actionInsert("rename-input:e2", "Wide mix v2"),
      actionKey("rename-input:e2", "Enter"),
      // The branch the Select shows — never the anchor's own branch and never
      // the preferred fork.
      expectEvents([
        { name: "loadContinuations", payload: { entryId: "e2" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1" } },
        { name: "renameBranch", payload: { branchId: "branch-wide", name: "Wide mix v2" } },
      ]),
      expectPart("rename-input:e2", { present: false }),
      // Focus returns to what opened the rename.
      expectPart("picker-actions:e2", { focused: true }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/rename-escape-cancels",
    fixture: {
      props: { pages: FORKED_PAGES, defaultOpen: true },
      regions: {},
      host: { continuations: FORKED_CONTINUATIONS, runEntries: FORKED_RUN_ENTRIES },
    },
    specimen: {
      group: "Forks",
      caption: "Escape cancels a rename without emitting",
      captureId: "history-center/rename-escape-cancels",
      axes: [],
    },
    steps: [
      actionPress("disclosure:e2"),
      actionPress("picker-actions:e2"),
      actionPress("action-rename"),
      actionInsert("rename-input:e2", "Discarded"),
      actionKey("rename-input:e2", "Escape"),
      // Cancel emits nothing at all, and the surface stays open: Escape
      // inside the rename belongs to the rename, not to the popover.
      expectEvents([
        { name: "loadContinuations", payload: { entryId: "e2" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1" } },
      ]),
      expectPart("root", { states: { open: true } }),
      expectPart("rename-input:e2", { present: false }),
    ],
  }),

  // ── Single fork, nested fork, depth ────────────────────────────────────
  componentCase(historyCenterInterface, {
    id: "history-center/single-fork-auto-chosen",
    fixture: {
      props: {
        pages: [
          page(0, [entry("s2", "Trim tail", "current", 2), entry("s1", "Import stems", "past", 1)]),
        ],
        defaultOpen: true,
      },
      regions: {},
      host: {
        continuations: [
          fork("s2", "s3", "Raise gain", "main", { preferred: true }),
          fork("s2", "g1", "Only fork", "branch-only", { entryCount: 1, branchName: "Only" }),
        ],
        runEntries: [runEntry("g1", "g1", "Only fork", "future", 0)],
      },
    },
    specimen: {
      group: "Forks",
      caption: "One fork needs no choice",
      captureId: "history-center/single-fork-auto-chosen",
      axes: [],
    },
    steps: [
      actionPress("disclosure:s2"),
      // The picker still renders — it persists for as long as the level is
      // open — but with one fork there is nothing to choose between, so the
      // select alone is disabled. The actions menu is not.
      expectPart("picker:s2", { present: true, level: 2 }),
      expectPart("picker-select:s2", { present: true, focusable: false }),
      expectPart("picker-actions:s2", { present: true, focusable: true }),
      expectEvents([
        { name: "loadContinuations", payload: { entryId: "s2" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "g1" } },
      ]),
      expectPart("row:g1", { present: true, level: 2 }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/nested-fork-depth",
    fixture: {
      props: { pages: FORKED_PAGES, defaultOpen: true },
      regions: {},
      host: { continuations: NESTED_CONTINUATIONS, runEntries: FORKED_RUN_ENTRIES },
    },
    specimen: {
      group: "Forks",
      caption: "A fork inside a fork run",
      captureId: "history-center/nested-fork-depth",
      axes: [],
    },
    steps: [
      actionPress("disclosure:e2"),
      expectPart("row:f1b", { present: true, level: 2 }),
      // The run's own entry discloses its own fork: forks fork, and the level
      // nests rather than saturating.
      actionPress("disclosure:f1b"),
      expectEvents([
        { name: "loadContinuations", payload: { entryId: "e2" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1" } },
        { name: "loadContinuations", payload: { entryId: "f1b" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1b1" } },
      ]),
      expectPart("picker:f1b", { present: true, level: 3 }),
      expectPart("row:f1b1", { present: true, level: 3 }),
      // Two forks at one entry are never confusable with a fork off a fork:
      // the outer picker is still at its own level and its rows did not move.
      expectPart("picker:e2", { present: true, level: 2 }),
      expectPart("row:f1", { level: 2 }),
      expectPart("row:e1", { level: 1 }),
    ],
  }),
  componentCase(historyCenterInterface, {
    id: "history-center/deep-depth-uncapped",
    fixture: {
      props: { pages: FORKED_PAGES, defaultOpen: true },
      regions: {},
      host: { continuations: NESTED_CONTINUATIONS, runEntries: FORKED_RUN_ENTRIES },
    },
    specimen: {
      group: "Forks",
      caption: "Depth is never clamped",
      captureId: "history-center/deep-depth-uncapped",
      axes: [],
    },
    steps: [
      actionPress("disclosure:e2"),
      actionPress("disclosure:f1b"),
      actionPress("disclosure:f1b1"),
      // Each disclosure adds exactly one level and nothing saturates. The v2
      // renderer capped depth and hid nesting; the cap is gone and must not
      // come back in any runtime.
      expectPart("row:e2", { level: 1 }),
      expectPart("row:f1b", { level: 2 }),
      expectPart("row:f1b1", { present: true, level: 3 }),
      expectPart("row:f1b1a", { present: true, level: 4 }),
      expectEvents([
        { name: "loadContinuations", payload: { entryId: "e2" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1" } },
        { name: "loadContinuations", payload: { entryId: "f1b" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1b1" } },
        { name: "loadContinuations", payload: { entryId: "f1b1" } },
        { name: "loadContinuationRun", payload: { fromEntryId: "f1b1a" } },
      ]),
      // Closing the outer level drops the whole subtree, every inner level
      // included.
      actionPress("disclosure:e2"),
      expectPart("row:f1b1a", { present: false }),
      expectPart("row:f1b1", { present: false }),
      expectPart("row:f1", { present: false }),
      expectPart("picker:e2", { present: false }),
      expectPart("row:e2", { present: true, level: 1 }),
    ],
  }),

  // ── Narrow layout ──────────────────────────────────────────────────────
  componentCase(historyCenterInterface, {
    id: "history-center/narrow-surface-bounds",
    fixture: { props: { pages: FORKED_PAGES, defaultOpen: true }, regions: {} },
    specimen: {
      group: "Layout",
      caption: "The surface is bounded and the list scrolls",
      captureId: "history-center/narrow-surface-bounds",
      axes: [],
    },
    steps: [
      // The panel root sets no width of its own: the surface owns it, so the
      // surface is what the anchor's width is compared against.
      expectPart("surface", { present: true, overlay: true, parent: "root" }),
      // The list scrolls inside its own bounds rather than growing the
      // surface to the height of the history.
      //
      // The height *cap* is deliberately not asserted. The web expresses it as
      // `min(28rem, 60vh)`, which is not a resolvable computed length outside
      // a real viewport, so a runtime that reports a number and one that
      // cannot are not disagreeing about the component — they are disagreeing
      // about what a stylesheet means without layout. Asserting it would have
      // made the corpus fail for a reason that is not about HistoryCenter.
      expectPart("list", { present: true, scrollable: true }),
    ],
  }),
];

export const historyCenterCases = serializeCases("history-center", cases);
