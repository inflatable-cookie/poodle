import { describe, expect, test } from "bun:test";

import {
  HISTORY_TREE_DEPTH_CAP,
  historyCenterDefaultContext,
  historyCenterKeydownEvent,
  historyCenterRowCount,
  historyCenterRows,
  historyCenterTransition,
  type HistoryBranch,
  type HistoryCenterContext,
  type HistoryCenterRow,
  type HistoryEntry,
  type HistoryEntryPosition,
} from "../src/history-center.ts";

function entry(id: string, position: HistoryEntryPosition = "past"): HistoryEntry {
  return { id, label: `Label ${id}`, position };
}

function branch(id: string, overrides: Partial<HistoryBranch> = {}): HistoryBranch {
  return { id, name: `Branch ${id}`, ...overrides };
}

/** Renders rows as compact strings: "entry:id@depth" and "caption:id@depth". */
function render(rows: HistoryCenterRow[]): string[] {
  return rows.map((row) =>
    row.kind === "entry" ? `entry:${row.entry.id}@${row.depth}` : `caption:${row.branch.id}@${row.depth}`,
  );
}

function ctx(overrides: Partial<HistoryCenterContext> = {}): HistoryCenterContext {
  return historyCenterDefaultContext(overrides);
}

const spineAndRun = (): { branches: HistoryBranch[]; paths: Record<string, HistoryEntry[]> } => ({
  branches: [
    branch("main", { current: true }),
    branch("topic", { divergedAfterEntryId: "e2" }),
  ],
  paths: {
    main: [entry("e1"), entry("e2"), entry("e3", "current")],
    topic: [entry("e1"), entry("e2"), entry("t1"), entry("t2")],
  },
});

describe("stitcher — required tests", () => {
  test("linear only — no fork branches; output equals the spine, all depth 0", () => {
    const rows = historyCenterRows([branch("main", { current: true })], {
      main: [entry("e1"), entry("e2"), entry("e3", "current")],
    });

    expect(render(rows)).toEqual(["entry:e1@0", "entry:e2@0", "entry:e3@0"]);
    expect(rows.every((row) => row.kind === "entry" && row.branchId === "main" && row.depth === 0)).toBe(true);
    expect(rows.map((row) => row.index)).toEqual([0, 1, 2]);
  });

  test("one fork mid-spine — the run attaches at the last shared entry, not at the divergence id", () => {
    // The authority reports divergence after e1 (a coarse, current-branch-relative
    // divergence id), but the path shares [e1, e2] with the spine: the run must
    // attach after e2, its last shared entry.
    const rows = historyCenterRows(
      [branch("main", { current: true }), branch("topic", { divergedAfterEntryId: "e1" })],
      {
        main: [entry("e1"), entry("e2"), entry("e3"), entry("e4")],
        topic: [entry("e1"), entry("e2"), entry("t1"), entry("t2")],
      },
    );

    expect(render(rows)).toEqual([
      "entry:e1@0",
      "entry:e2@0",
      "caption:topic@1",
      "entry:t1@1",
      "entry:t2@1",
      "entry:e3@0",
      "entry:e4@0",
    ]);
  });

  test("fork off a fork — depth 2, and the inner run attaches to the outer run", () => {
    const rows = historyCenterRows(
      [branch("main", { current: true }), branch("outer"), branch("inner")],
      {
        main: [entry("e1"), entry("e2"), entry("e3")],
        outer: [entry("e1"), entry("e2"), entry("o1"), entry("o2")],
        inner: [entry("e1"), entry("e2"), entry("o1"), entry("i1")],
      },
    );

    expect(render(rows)).toEqual([
      "entry:e1@0",
      "entry:e2@0",
      "caption:outer@1",
      "entry:o1@1",
      "caption:inner@2",
      "entry:i1@2",
      "entry:o2@1",
      "entry:e3@0",
    ]);

    const o1 = rows.find((row) => row.kind === "entry" && row.entry.id === "o1");
    const i1 = rows.find((row) => row.kind === "entry" && row.entry.id === "i1");
    const o2 = rows.find((row) => row.kind === "entry" && row.entry.id === "o2");

    expect(o1).toMatchObject({ kind: "entry", branchId: "outer", depth: 1 });
    // The inner run attaches to the outer run, not to the spine.
    expect(i1).toMatchObject({
      kind: "entry",
      branchId: "inner",
      depth: 2,
      lane: { parentBranchId: "outer", start: true, continue: false, end: true },
    });
    // The outer run continues past the inner run.
    expect(o2).toMatchObject({ kind: "entry", branchId: "outer", depth: 1, lane: { continue: true, end: true } });
  });

  test("depth cap — a chain deeper than the cap saturates at depth 3 and keeps true branchIds", () => {
    expect(HISTORY_TREE_DEPTH_CAP).toBe(3);

    const rows = historyCenterRows(
      [branch("main", { current: true }), branch("a"), branch("b"), branch("c"), branch("d")],
      {
        main: [entry("e1")],
        a: [entry("e1"), entry("a1")],
        b: [entry("e1"), entry("a1"), entry("b1")],
        c: [entry("e1"), entry("a1"), entry("b1"), entry("c1")],
        d: [entry("e1"), entry("a1"), entry("b1"), entry("c1"), entry("d1")],
      },
    );

    expect(render(rows)).toEqual([
      "entry:e1@0",
      "caption:a@1",
      "entry:a1@1",
      "caption:b@2",
      "entry:b1@2",
      "caption:c@3",
      "entry:c1@3",
      "caption:d@3",
      "entry:d1@3",
    ]);

    const d1 = rows.find((row) => row.kind === "entry" && row.entry.id === "d1");
    // Depth saturates at 3, but the true branch id and lane structure survive.
    expect(d1).toMatchObject({
      kind: "entry",
      branchId: "d",
      depth: 3,
      lane: { branchId: "d", parentBranchId: "c", start: true, end: true },
    });
    const c1 = rows.find((row) => row.kind === "entry" && row.entry.id === "c1");
    expect(c1).toMatchObject({ depth: 3 });
  });

  test("many shallow forks — each fork is a distinct run; none collapse onto one entry", () => {
    const forks = ["f1", "f2", "f3", "f4", "f5", "f6"];
    const branches = [branch("main", { current: true }), ...forks.map((id) => branch(id))];
    const paths: Record<string, HistoryEntry[]> = {
      main: [entry("e1"), entry("e2"), entry("e3")],
    };
    for (const id of forks) {
      paths[id] = [entry("e1"), entry("e2"), entry(`${id}-a`), entry(`${id}-b`)];
    }

    const rows = historyCenterRows(branches, paths);

    // e1, e2, six runs of caption + two entries each, e3.
    expect(render(rows)).toEqual([
      "entry:e1@0",
      "entry:e2@0",
      "caption:f1@1",
      "entry:f1-a@1",
      "entry:f1-b@1",
      "caption:f2@1",
      "entry:f2-a@1",
      "entry:f2-b@1",
      "caption:f3@1",
      "entry:f3-a@1",
      "entry:f3-b@1",
      "caption:f4@1",
      "entry:f4-a@1",
      "entry:f4-b@1",
      "caption:f5@1",
      "entry:f5-a@1",
      "entry:f5-b@1",
      "caption:f6@1",
      "entry:f6-a@1",
      "entry:f6-b@1",
      "entry:e3@0",
    ]);

    // Six distinct runs in supplied order — the v1 "6 branches off one entry" collapse is gone.
    const captions = rows.filter((row) => row.kind === "caption");
    expect(captions.map((row) => (row.kind === "caption" ? row.branch.id : ""))).toEqual(forks);
    for (const id of forks) {
      const run = rows.filter((row) => row.kind === "entry" && row.branchId === id);
      expect(run.map((row) => (row.kind === "entry" ? row.entry.id : ""))).toEqual([`${id}-a`, `${id}-b`]);
      expect(run.every((row) => row.depth === 1)).toBe(true);
    }
  });

  test("empty branch head — a branch with no entries is omitted entirely", () => {
    const rows = historyCenterRows(
      [branch("main", { current: true }), branch("empty", { headEntryId: undefined })],
      {
        main: [entry("e1"), entry("e2")],
        empty: [],
      },
    );

    expect(render(rows)).toEqual(["entry:e1@0", "entry:e2@0"]);
    expect(rows.some((row) => (row.kind === "caption" ? row.branch.id : row.branchId) === "empty")).toBe(false);

    // A branch whose path page has not arrived yet is omitted the same way.
    const missingPath = historyCenterRows([branch("main", { current: true }), branch("lazy")], {
      main: [entry("e1"), entry("e2")],
    });
    expect(render(missingPath)).toEqual(["entry:e1@0", "entry:e2@0"]);
  });

  test("page-boundary split — a path supplied in two pages stitches into one run without duplicates or drops", () => {
    const page1 = [entry("e1"), entry("e2")];
    const page2 = [entry("t1"), entry("t2")];
    const rows = historyCenterRows(
      [branch("main", { current: true }), branch("topic")],
      {
        main: [entry("e1"), entry("e2"), entry("e3")],
        topic: [...page1, ...page2],
      },
    );

    expect(render(rows)).toEqual([
      "entry:e1@0",
      "entry:e2@0",
      "caption:topic@1",
      "entry:t1@1",
      "entry:t2@1",
      "entry:e3@0",
    ]);

    // An overlapping seam (the page boundary repeats e2) still yields one
    // contiguous run with every unique entry exactly once.
    const overlapped = historyCenterRows(
      [branch("main", { current: true }), branch("topic")],
      {
        main: [entry("e1"), entry("e2"), entry("e3")],
        topic: [entry("e1"), entry("e2"), entry("e2"), entry("t1"), entry("t2")],
      },
    );
    expect(render(overlapped)).toEqual(render(rows));
  });

  test("shared prefix dedupe — an entry present in three paths appears exactly once", () => {
    const rows = historyCenterRows(
      [branch("main", { current: true }), branch("a"), branch("b")],
      {
        main: [entry("e1"), entry("e2"), entry("e3")],
        a: [entry("e1"), entry("e2"), entry("a1")],
        b: [entry("e1"), entry("e2"), entry("b1")],
      },
    );

    expect(render(rows)).toEqual([
      "entry:e1@0",
      "entry:e2@0",
      "caption:a@1",
      "entry:a1@1",
      "caption:b@1",
      "entry:b1@1",
      "entry:e3@0",
    ]);

    for (const id of ["e1", "e2"]) {
      const occurrences = rows.filter((row) => row.kind === "entry" && row.entry.id === id);
      expect(occurrences).toHaveLength(1);
    }
  });

  test("determinism — same input, same output, including row order and indices", () => {
    const { branches, paths } = spineAndRun();
    const first = historyCenterRows(branches, paths);
    const second = historyCenterRows(branches, paths);

    expect(second).toEqual(first);
    expect(second.map((row) => row.index)).toEqual(first.map((row) => row.index));
  });
});

describe("stitcher — edges and presence", () => {
  test("null branches or null paths produce no rows", () => {
    expect(historyCenterRows(null, null)).toEqual([]);
    expect(historyCenterRows([branch("main", { current: true })], null)).toEqual([]);
    expect(historyCenterRows([], {})).toEqual([]);
    expect(historyCenterRowCount(null, null)).toBe(0);
  });

  test("no current branch falls back to the first supplied branch as the spine", () => {
    const rows = historyCenterRows([branch("first"), branch("second")], {
      first: [entry("a1"), entry("a2")],
      second: [entry("a1"), entry("b1")],
    });

    // The run attaches immediately after its last shared entry (a1), before
    // the spine continues with a2.
    expect(render(rows)).toEqual([
      "entry:a1@0",
      "caption:second@1",
      "entry:b1@1",
      "entry:a2@0",
    ]);
  });

  test("a branch sharing nothing with the placed structure attaches at the root, before the spine", () => {
    const rows = historyCenterRows(
      [branch("main", { current: true }), branch("orphan")],
      {
        main: [entry("e40"), entry("e41")],
        orphan: [entry("x1"), entry("x2")],
      },
    );

    expect(render(rows)).toEqual([
      "caption:orphan@0",
      "entry:x1@0",
      "entry:x2@0",
      "entry:e40@0",
      "entry:e41@0",
    ]);
  });
});

const machineFixture = (): { branches: HistoryBranch[]; paths: Record<string, HistoryEntry[]> } => spineAndRun();

describe("popover open state", () => {
  test("TOGGLE opens from closed and closes from open, emitting open-change", () => {
    const { branches, paths } = machineFixture();
    const context = ctx({ branches, paths });
    const opened = historyCenterTransition("closed", context, { type: "TOGGLE" });

    expect(opened.state).toBe("open");
    expect(opened.effects).toEqual([{ type: "emitOpenChange", open: true }]);

    const closed = historyCenterTransition("open", context, { type: "TOGGLE" });

    expect(closed.state).toBe("closed");
    expect(closed.effects).toEqual([{ type: "emitOpenChange", open: false }]);
  });

  test("OPEN and CLOSE are idempotent in their target state", () => {
    const { branches, paths } = machineFixture();
    expect(historyCenterTransition("open", ctx({ branches, paths }), { type: "OPEN" }).effects).toEqual([]);
    expect(historyCenterTransition("closed", ctx({ branches, paths }), { type: "CLOSE" }).effects).toEqual([]);
  });

  test("navigation and activation are inert while closed", () => {
    const { branches, paths } = machineFixture();
    const result = historyCenterTransition("closed", ctx({ branches, paths }), { type: "ACTIVATE_ROW", index: 0 });

    expect(result.state).toBe("closed");
    expect(result.effects).toEqual([]);
  });
});

describe("keyboard traversal across a spine-and-run list", () => {
  const { branches, paths } = machineFixture();
  // rows: e1, e2, caption:topic, t1, t2, e3  → 6 rows
  const context = ctx({ branches, paths });

  test("rows are stitched and the count covers spine and run", () => {
    expect(historyCenterRowCount(branches, paths)).toBe(6);
  });

  test("next and prev wrap around the full row set, captions included", () => {
    expect(historyCenterTransition("open", context, { type: "FOCUS_MOVE", direction: "next" }).context.focusIndex).toBe(1);
    expect(
      historyCenterTransition("open", ctx({ branches, paths, focusIndex: 2 }), { type: "FOCUS_MOVE", direction: "next" })
        .context.focusIndex,
    ).toBe(3);
    expect(
      historyCenterTransition("open", ctx({ branches, paths, focusIndex: 0 }), { type: "FOCUS_MOVE", direction: "prev" })
        .context.focusIndex,
    ).toBe(5);
    expect(
      historyCenterTransition("open", ctx({ branches, paths, focusIndex: 5 }), { type: "FOCUS_MOVE", direction: "next" })
        .context.focusIndex,
    ).toBe(0);
  });

  test("first and last land on boundaries and emit focusRow", () => {
    expect(
      historyCenterTransition("open", ctx({ branches, paths, focusIndex: 3 }), { type: "FOCUS_MOVE", direction: "first" }),
    ).toEqual({
      state: "open",
      context: { branches, paths, focusIndex: 0, rejection: null },
      effects: [{ type: "focusRow", index: 0 }],
    });
    expect(
      historyCenterTransition("open", ctx({ branches, paths, focusIndex: 0 }), { type: "FOCUS_MOVE", direction: "last" })
        .context.focusIndex,
    ).toBe(5);
  });

  test("navigation over an empty list is inert", () => {
    const result = historyCenterTransition("open", ctx(), { type: "FOCUS_MOVE", direction: "next" });

    expect(result.context.focusIndex).toBe(0);
    expect(result.effects).toEqual([]);
  });
});

describe("row activation — navigate", () => {
  const { branches, paths } = machineFixture();

  test("activating a spine entry emits navigate-entry with the spine branch and that entry", () => {
    const result = historyCenterTransition("open", ctx({ branches, paths }), { type: "ACTIVATE_ROW", index: 0 });

    expect(result.effects).toEqual([{ type: "emitNavigateEntry", branchId: "main", entryId: "e1" }]);
  });

  test("activating a run entry emits the clicked entry on the branch owning its run — never an ancestor or another branch's divergence entry", () => {
    const t1 = historyCenterTransition("open", ctx({ branches, paths }), { type: "ACTIVATE_ROW", index: 3 });
    expect(t1.effects).toEqual([{ type: "emitNavigateEntry", branchId: "topic", entryId: "t1" }]);

    const t2 = historyCenterTransition("open", ctx({ branches, paths }), { type: "ACTIVATE_ROW", index: 4 });
    expect(t2.effects).toEqual([{ type: "emitNavigateEntry", branchId: "topic", entryId: "t2" }]);

    // The divergence entry e2 belongs to the spine — clicking the run's own
    // entries never reports it.
    expect(t1.effects[0].entryId).not.toBe("e2");
  });

  test("activating a caption syncs focus but does not navigate", () => {
    const result = historyCenterTransition("open", ctx({ branches, paths }), { type: "ACTIVATE_ROW", index: 2 });

    expect(result.context.focusIndex).toBe(2);
    expect(result.effects).toEqual([]);
  });

  test("activation defaults to the focused row", () => {
    expect(
      historyCenterTransition("open", ctx({ branches, paths, focusIndex: 3 }), { type: "ACTIVATE_ROW" }).effects,
    ).toEqual([{ type: "emitNavigateEntry", branchId: "topic", entryId: "t1" }]);
  });

  test("out-of-bounds activation is inert", () => {
    expect(historyCenterTransition("open", ctx({ branches, paths }), { type: "ACTIVATE_ROW", index: 99 }).effects).toEqual([]);
  });
});

describe("rename and rejection", () => {
  test("RENAME emits rename-branch without state change", () => {
    const { branches, paths } = machineFixture();
    const result = historyCenterTransition("open", ctx({ branches, paths }), {
      type: "RENAME",
      branchId: "topic",
      name: "mix/lead",
    });

    expect(result.effects).toEqual([{ type: "emitRenameBranch", branchId: "topic", name: "mix/lead" }]);
    expect(result.context).toEqual(ctx({ branches, paths }));
  });

  test("SHOW_REJECTION displays a message and is idempotent for the same one", () => {
    const { branches, paths } = machineFixture();
    const shown = historyCenterTransition("open", ctx({ branches, paths }), {
      type: "SHOW_REJECTION",
      message: "Fork branch name is invalid",
    });

    expect(shown.context.rejection).toBe("Fork branch name is invalid");

    expect(
      historyCenterTransition("open", shown.context, { type: "SHOW_REJECTION", message: "Fork branch name is invalid" })
        .effects,
    ).toEqual([]);
  });

  test("DISMISS_REJECTION clears the notice and is inert when none is shown", () => {
    const cleared = historyCenterTransition("open", ctx({ rejection: "boom" }), { type: "DISMISS_REJECTION" });

    expect(cleared.context.rejection).toBeNull();
    expect(historyCenterTransition("open", ctx(), { type: "DISMISS_REJECTION" }).effects).toEqual([]);
  });
});

describe("historyCenterKeydownEvent", () => {
  test("maps list keys to machine events and leaves others to the adapter", () => {
    expect(historyCenterKeydownEvent("ArrowDown")).toEqual({ type: "FOCUS_MOVE", direction: "next" });
    expect(historyCenterKeydownEvent("ArrowUp")).toEqual({ type: "FOCUS_MOVE", direction: "prev" });
    expect(historyCenterKeydownEvent("Home")).toEqual({ type: "FOCUS_MOVE", direction: "first" });
    expect(historyCenterKeydownEvent("End")).toEqual({ type: "FOCUS_MOVE", direction: "last" });
    expect(historyCenterKeydownEvent("Enter")).toEqual({ type: "ACTIVATE_ROW" });
    expect(historyCenterKeydownEvent(" ")).toEqual({ type: "ACTIVATE_ROW" });
    expect(historyCenterKeydownEvent("Tab")).toBeNull();
    expect(historyCenterKeydownEvent("x")).toBeNull();
  });
});
