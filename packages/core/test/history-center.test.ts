import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, test } from "bun:test";

import {
  historyCenterDefaultContext,
  historyCenterForkCount,
  historyCenterJoinPages,
  historyCenterKeydownEvent,
  historyCenterRejectionMessage,
  historyCenterTransition,
  historyCenterVisibleRows,
  type HistoryCenterContext,
  type HistoryCenterOpenFork,
  type HistoryCenterRow,
  type HistoryContinuation,
  type HistoryEntry,
  type HistoryEntryPosition,
  type HistoryPathPage,
} from "../src/history-center.ts";

function entry(id: string, continuationCount = 0, position: HistoryEntryPosition = "past"): HistoryEntry {
  return { id, label: `Label ${id}`, position, continuationCount };
}

function page(entries: HistoryEntry[], overrides: Partial<HistoryPathPage> = {}): HistoryPathPage {
  return {
    entries,
    offset: 0,
    rootContinuationCount: 0,
    truncatedBefore: false,
    truncatedAfter: false,
    ...overrides,
  };
}

function continuation(entryId: string, overrides: Partial<HistoryContinuation> = {}): HistoryContinuation {
  return {
    entryId,
    label: `Label ${entryId}`,
    preferred: false,
    entryCount: 1,
    branchId: `b-${entryId}`,
    branchName: null,
    ...overrides,
  };
}

/** A level of the disclosure tree. */
function level(anchorEntryId: string, overrides: Partial<HistoryCenterOpenFork> = {}): HistoryCenterOpenFork {
  return { anchorEntryId, continuations: null, pick: null, chosen: null, runPages: [], inner: null, ...overrides };
}

function open(levels: HistoryCenterOpenFork[]): Map<string, HistoryCenterOpenFork> {
  return new Map(levels.map((forkLevel) => [forkLevel.anchorEntryId, forkLevel]));
}

/**
 * Renders rows as compact strings. Entry rows carry their depth, parent and
 * fork identity so the R1 condition is asserted in the strings themselves:
 * `entry:id@depth:parent:fork:forkCount`. Picker rows carry their forks.
 */
function render(rows: HistoryCenterRow[]): string[] {
  return rows.map((row) => {
    switch (row.kind) {
      case "entry":
        return `entry:${row.entry.id}@${row.depth}:${row.parentEntryId ?? "-"}:${row.forkId ?? "-"}:${row.forkCount}`;
      case "picker":
        return `picker:${row.anchorEntryId}@${row.depth}:${row.continuations.map((fork) => fork.entryId).join(",")}`;
      case "not-yet-loaded":
        return `not-yet-loaded:${row.anchorEntryId}@${row.depth}:${row.forkId ?? "-"}`;
    }
  });
}

function ctx(overrides: Partial<HistoryCenterContext> = {}): HistoryCenterContext {
  return historyCenterDefaultContext(overrides);
}

describe("fork count — R4", () => {
  test("forkCount is continuationCount minus one, floored at zero", () => {
    expect(historyCenterForkCount(0)).toBe(0);
    expect(historyCenterForkCount(1)).toBe(0);
    expect(historyCenterForkCount(2)).toBe(1);
    expect(historyCenterForkCount(3)).toBe(2);
  });

  test("continuationCount 1 yields no fork affordance and no picker row", () => {
    // e1 has only its own next row (e2): one continuation, zero forks.
    const rows = historyCenterVisibleRows([page([entry("e2", 0), entry("e1", 1)])], open([level("e1")]));

    const e1 = rows.find((row) => row.kind === "entry" && row.entry.id === "e1");
    expect(e1).toMatchObject({ kind: "entry", forkCount: 0 });
    expect(rows.some((row) => row.kind === "picker")).toBe(false);
  });

  test("continuationCount 0 on a run's last entry yields no fork affordance", () => {
    const rows = historyCenterVisibleRows([page([entry("e2", 0), entry("e1", 0)])], null);

    const e1 = rows.find((row) => row.kind === "entry" && row.entry.id === "e1");
    expect(e1).toMatchObject({ kind: "entry", forkCount: 0 });
  });

  test("forkCount > 1 yields a picker row; forkCount === 1 yields none", () => {
    const pageWithTwoForks = [page([entry("e2", 0), entry("e1", 3)])];
    const twoForks = historyCenterVisibleRows(pageWithTwoForks, open([level("e1")]));
    expect(twoForks.some((row) => row.kind === "picker")).toBe(true);
    expect(twoForks.some((row) => row.kind === "not-yet-loaded")).toBe(false);

    const pageWithOneFork = [page([entry("e2", 0), entry("e1", 2)])];
    const oneFork = historyCenterVisibleRows(pageWithOneFork, open([level("e1")]));
    expect(oneFork.some((row) => row.kind === "picker")).toBe(false);
  });
});

describe("display order — R3", () => {
  test("a newest-first root page renders oldest-first", () => {
    const rows = historyCenterVisibleRows(
      [page([entry("e4", 0, "current"), entry("e3"), entry("e2"), entry("e1")])],
      null,
    );

    expect(rows.map((row) => (row.kind === "entry" ? row.entry.id : "")).filter(Boolean)).toEqual([
      "e1",
      "e2",
      "e3",
      "e4",
    ]);
    // Every row is depth 0 and on the spine (no fork); only the first entry
    // hangs off the root — e2 hangs off e1, and so on (graph truth).
    expect(rows.every((row) => row.kind === "entry" && row.depth === 0 && row.forkId === null)).toBe(true);
    expect(rows.map((row) => (row.kind === "entry" ? row.parentEntryId : null))).toEqual([
      null,
      "e1",
      "e2",
      "e3",
    ]);
  });

  test("a nested run reverses by the same code — oldest run entry first", () => {
    const rows = historyCenterVisibleRows(
      [page([entry("e2", 0), entry("e1", 2)])],
      open([level("e1", { chosen: continuation("l1"), runPages: [page([entry("l1b"), entry("l1a", 0)])] })]),
    );

    expect(render(rows)).toEqual([
      "entry:e1@0:-:-:1",
      "entry:l1a@1:e1:l1:0",
      "entry:l1b@1:l1a:l1:0",
      "entry:e2@0:e1:-:0",
    ]);
  });

  test("two pages join with the older page first and the oldest entry renders first", () => {
    // p0 is the newest page (offset 0); p1 holds older entries (offset 2).
    const joined = historyCenterJoinPages([
      page([entry("e4", 0, "current"), entry("e3")], { offset: 0 }),
      page([entry("e2"), entry("e1")], { offset: 2 }),
    ]);

    expect(joined.map((item) => item.id)).toEqual(["e1", "e2", "e3", "e4"]);
    expect(joined[0].id).toBe("e1");

    const rows = historyCenterVisibleRows(
      [page([entry("e4", 0, "current"), entry("e3")], { offset: 0 }), page([entry("e2"), entry("e1")], { offset: 2 })],
      null,
    );
    expect(rows[0]).toMatchObject({ kind: "entry", entry: { id: "e1" } });
  });

  test("overlapping page seams dedupe by entry id", () => {
    const joined = historyCenterJoinPages([
      page([entry("e4", 0, "current"), entry("e3")]),
      page([entry("e3"), entry("e2"), entry("e1")]),
    ]);

    expect(joined.map((item) => item.id)).toEqual(["e1", "e2", "e3", "e4"]);
  });

  test("joining is deterministic", () => {
    const pages = [page([entry("e4", 0, "current"), entry("e3")]), page([entry("e2"), entry("e1")])];
    expect(historyCenterJoinPages(pages)).toEqual(historyCenterJoinPages(pages));
  });
});

describe("visible-row derivation — forks as data (R1)", () => {
  test("two forks at one entry are not confusable with a fork off a fork", () => {
    // Case A: two forks f1 and f2 both hang off e1.
    const twoAtOne = historyCenterVisibleRows(
      [page([entry("e2", 0), entry("e1", 3)])],
      open([
        level("e1", {
          continuations: [continuation("e2", { preferred: true }), continuation("f1"), continuation("f2")],
          chosen: continuation("f1"),
          runPages: [page([entry("f1b"), entry("f1a", 0)])],
        }),
      ]),
    );

    // Case B: l1 forks off e1; h1 forks off l1.
    const forkOffFork = historyCenterVisibleRows(
      [page([entry("e2", 0), entry("e1", 2)])],
      open([
        level("e1", {
          continuations: [continuation("e2", { preferred: true }), continuation("l1")],
          chosen: continuation("l1"),
          runPages: [page([entry("l1b"), entry("l1a", 2)])],
          inner: open([
            level("l1a", {
              continuations: [continuation("l1b", { preferred: true }), continuation("h1")],
              chosen: continuation("h1"),
              runPages: [page([entry("h1b"), entry("h1a", 0)])],
            }),
          ]),
        }),
      ]),
    );

    // Same shape would be the v2 defect; the two cases must differ.
    expect(render(twoAtOne)).not.toEqual(render(forkOffFork));

    // Case A rows carry parentEntryId e1 for both forks' runs.
    expect(render(twoAtOne)).toEqual([
      "entry:e1@0:-:-:2",
      "entry:f1a@1:e1:f1:0",
      "entry:f1b@1:f1a:f1:0",
      "entry:e2@0:e1:-:0",
    ]);

    // Case B: the depth-2 rows hang off l1 (not e1) and belong to fork h1.
    expect(render(forkOffFork)).toEqual([
      "entry:e1@0:-:-:1",
      "entry:l1a@1:e1:l1:1",
      "entry:h1a@2:l1a:h1:0",
      "entry:h1b@2:h1a:h1:0",
      "entry:l1b@1:l1a:l1:0",
      "entry:e2@0:e1:-:0",
    ]);
  });

  test("the continuations page includes the child already on the list; the derivation filters it by id", () => {
    // The own continuation (e2, the anchor's successor) is NOT first — the
    // filter must not assume its position.
    const rows = historyCenterVisibleRows(
      [page([entry("e2", 0), entry("e1", 3)])],
      open([level("e1", { continuations: [continuation("f1"), continuation("e2", { preferred: true }), continuation("f2")] })]),
    );

    const picker = rows.find((row) => row.kind === "picker");
    expect(picker).toBeDefined();
    expect(picker?.kind === "picker" && picker.continuations.map((fork) => fork.entryId)).toEqual(["f1", "f2"]);
  });

  test("when the successor is not on a loaded page, the preferred continuation is the child already on the list", () => {
    // e1 is the last loaded entry of its page (truncated after): its successor
    // is unknown, so the filter falls back to the preferred flag — which names
    // the same record, because the run follows preferred children.
    const rows = historyCenterVisibleRows(
      [page([entry("e1", 3)], { truncatedAfter: true })],
      open([level("e1", { continuations: [continuation("own", { preferred: true }), continuation("f1"), continuation("f2")] })]),
    );

    const picker = rows.find((row) => row.kind === "picker");
    expect(picker?.kind === "picker" && picker.continuations.map((fork) => fork.entryId)).toEqual(["f1", "f2"]);
  });

  test("an open entry with no loaded run yields a not-yet-loaded row — never a gap, never a dropped entry", () => {
    const rows = historyCenterVisibleRows(
      [page([entry("e2", 0), entry("e1", 1)])],
      open([level("e1")]),
    );

    // e1's own entry row is present, and the open disclosure renders a
    // placeholder instead of leaving an empty region.
    expect(render(rows)).toEqual([
      "entry:e1@0:-:-:0",
      "not-yet-loaded:e1@1:-",
      "entry:e2@0:e1:-:0",
    ]);
  });

  test("a chosen fork whose run has not arrived carries its identity on the not-yet-loaded row", () => {
    const rows = historyCenterVisibleRows(
      [page([entry("e2", 0), entry("e1", 3)])],
      open([level("e1", { chosen: continuation("f1", { branchId: "b-f1" }), runPages: [] })]),
    );

    expect(render(rows)).toEqual([
      "entry:e1@0:-:-:2",
      "not-yet-loaded:e1@1:f1",
      "entry:e2@0:e1:-:0",
    ]);
  });

  test("every row carries parent entry id and fork identity at every depth, past where a v2 depth cap would have saturated", () => {
    // A five-deep fork chain. v2 saturated depth at 3; v3 must not, and each
    // row must carry its real parent and fork ids.
    const rows = historyCenterVisibleRows(
      [page([entry("s2", 0), entry("s1", 2)])],
      open([
        level("s1", {
          chosen: continuation("a1"),
          runPages: [page([entry("a2", 0), entry("a1", 2)])],
          inner: open([
            level("a1", {
              chosen: continuation("b1"),
              runPages: [page([entry("b2", 0), entry("b1", 2)])],
              inner: open([
                level("b1", {
                  chosen: continuation("c1"),
                  runPages: [page([entry("c2", 0), entry("c1", 2)])],
                  inner: open([
                    level("c1", {
                      chosen: continuation("d1"),
                      runPages: [page([entry("d2", 0), entry("d1", 2)])],
                      inner: open([
                        level("d1", {
                          chosen: continuation("e1"),
                          runPages: [page([entry("e2", 0), entry("e1", 0)])],
                        }),
                      ]),
                    }),
                  ]),
                }),
              ]),
            }),
          ]),
        }),
      ]),
    );

    const rendered = render(rows);
    expect(rendered).toEqual([
      "entry:s1@0:-:-:1",
      "entry:a1@1:s1:a1:1",
      "entry:b1@2:a1:b1:1",
      "entry:c1@3:b1:c1:1",
      "entry:d1@4:c1:d1:1",
      "entry:e1@5:d1:e1:0",
      "entry:e2@5:e1:e1:0",
      "entry:d2@4:d1:d1:0",
      "entry:c2@3:c1:c1:0",
      "entry:b2@2:b1:b1:0",
      "entry:a2@1:a1:a1:0",
      "entry:s2@0:s1:-:0",
    ]);

    // Past the v2 cap (3): depth is true and the identity fields are real.
    const deep = rows.filter((row) => row.kind === "entry" && row.depth >= 3);
    for (const row of deep) {
      expect(row.kind === "entry" && row.parentEntryId).not.toBeNull();
      expect(row.kind === "entry" && row.forkId).not.toBeNull();
    }
  });

  test("determinism — same input, same rows, same order", () => {
    const pages = [page([entry("e2", 0), entry("e1", 3)])];
    const forkOpen = open([
      level("e1", {
        continuations: [continuation("e2", { preferred: true }), continuation("f1"), continuation("f2")],
        chosen: continuation("f1"),
        runPages: [page([entry("f1b"), entry("f1a", 0)])],
      }),
    ]);

    const first = historyCenterVisibleRows(pages, forkOpen);
    const second = historyCenterVisibleRows(pages, forkOpen);

    expect(second).toEqual(first);
    expect(render(second)).toEqual(render(first));
  });

  test("null pages yield no rows; nothing invents a clock", () => {
    expect(historyCenterVisibleRows(null, null)).toEqual([]);
    expect(historyCenterVisibleRows([], null)).toEqual([]);

    // recordedAtMs absent: rows pass the supplied entry through unchanged —
    // no time is derived or invented.
    const noTime = entry("e1", 1);
    const rows = historyCenterVisibleRows([page([noTime])], null);
    expect(rows[0]).toMatchObject({ kind: "entry", entry: noTime });
    expect(JSON.stringify(rows)).not.toContain("recordedAtMs");
    expect(rows[0].kind === "entry" && rows[0].entry.recordedAtMs).toBeUndefined();

    const source = readFileSync(join(import.meta.dir, "..", "src", "history-center.ts"), "utf8");
    expect(source).not.toMatch(/Date\.now|performance\.now|new Date|setTimeout/);
  });
});

describe("machine — popover open state", () => {
  test("TOGGLE opens from closed and closes from open, emitting open-change", () => {
    const opened = historyCenterTransition("closed", ctx({ pages: [page([entry("e1", 0)])] }), { type: "TOGGLE" });
    expect(opened.state).toBe("open");
    expect(opened.effects).toEqual([{ type: "emitOpenChange", open: true }]);

    const closed = historyCenterTransition("open", ctx({ pages: [page([entry("e1", 0)])] }), { type: "TOGGLE" });
    expect(closed.state).toBe("closed");
    expect(closed.effects).toEqual([{ type: "emitOpenChange", open: false }]);
  });

  test("CLOSE drops the disclosure tree and focus — nothing is cached across a close/reopen (R5)", () => {
    const context = ctx({
      pages: [page([entry("e2", 0), entry("e1", 1)])],
      open: open([level("e1", { chosen: continuation("l1"), runPages: [page([entry("l1a", 0)])] })]),
      focusRow: { kind: "entry", entryId: "l1a" },
    });

    const closed = historyCenterTransition("open", context, { type: "CLOSE" });

    expect(closed.context.open).toBeNull();
    expect(closed.context.focusRow).toBeNull();
    expect(historyCenterVisibleRows(closed.context.pages, closed.context.open).map((row) => (row.kind === "entry" ? row.entry.id : ""))).toEqual(["e1", "e2"]);
  });

  test("OPEN and CLOSE are idempotent in their target state", () => {
    expect(historyCenterTransition("open", ctx(), { type: "OPEN" }).effects).toEqual([]);
    expect(historyCenterTransition("closed", ctx(), { type: "CLOSE" }).effects).toEqual([]);
  });

  test("row events are inert while closed", () => {
    const result = historyCenterTransition("closed", ctx({ pages: [page([entry("e1", 0)])] }), {
      type: "ACTIVATE_ROW",
      row: { kind: "entry", entryId: "e1" },
    });

    expect(result.state).toBe("closed");
    expect(result.effects).toEqual([]);
  });
});

describe("machine — disclosure flow", () => {
  const forkedPages = (continuationCount: number): HistoryPathPage[] => [page([entry("e2", 0), entry("e1", continuationCount)])];

  test("DISCLOSE opens a fork at an entry and emits loadContinuations", () => {
    const result = historyCenterTransition("open", ctx({ pages: forkedPages(2) }), { type: "DISCLOSE", entryId: "e1" });

    expect(result.effects).toEqual([{ type: "loadContinuations", entryId: "e1" }]);
    expect(result.context.open?.get("e1")).toMatchObject({
      anchorEntryId: "e1",
      continuations: null,
      chosen: null,
      runPages: [],
    });
  });

  test("DISCLOSE on an entry without forks is inert", () => {
    const result = historyCenterTransition("open", ctx({ pages: forkedPages(1) }), { type: "DISCLOSE", entryId: "e1" });

    expect(result.context.open).toBeNull();
    expect(result.effects).toEqual([]);
  });

  test("DISCLOSE on an entry that is not visible is inert", () => {
    const result = historyCenterTransition("open", ctx({ pages: forkedPages(1) }), { type: "DISCLOSE", entryId: "ghost" });

    expect(result.context.open).toBeNull();
    expect(result.effects).toEqual([]);
  });

  test("a single fork auto-requests its run once continuations arrive", () => {
    const afterDisclose = historyCenterTransition("open", ctx({ pages: forkedPages(2) }), {
      type: "DISCLOSE",
      entryId: "e1",
    });
    const result = historyCenterTransition("open", afterDisclose.context, {
      type: "CONTINUATIONS_LOADED",
      entryId: "e1",
      continuations: [continuation("e2", { preferred: true }), continuation("l1")],
    });

    expect(result.effects).toEqual([{ type: "loadContinuationRun", fromEntryId: "l1" }]);
    expect(result.context.open?.get("e1")?.chosen?.entryId).toBe("l1");
  });

  test("more than one fork waits for the picker", () => {
    const afterDisclose = historyCenterTransition("open", ctx({ pages: forkedPages(3) }), {
      type: "DISCLOSE",
      entryId: "e1",
    });
    const result = historyCenterTransition("open", afterDisclose.context, {
      type: "CONTINUATIONS_LOADED",
      entryId: "e1",
      continuations: [continuation("f1"), continuation("e2", { preferred: true }), continuation("f2")],
    });

    expect(result.effects).toEqual([]);
    expect(result.context.open?.get("e1")?.chosen).toBeNull();
    expect(result.context.open?.get("e1")?.continuations).toHaveLength(3);
  });

  test("PICK_CONTINUATION sets the tentative pick; CONFIRM commits it with prefer + load-run", () => {
    const afterDisclose = historyCenterTransition("open", ctx({ pages: forkedPages(3) }), {
      type: "DISCLOSE",
      entryId: "e1",
    });
    const loaded = historyCenterTransition("open", afterDisclose.context, {
      type: "CONTINUATIONS_LOADED",
      entryId: "e1",
      continuations: [continuation("f1"), continuation("e2", { preferred: true }), continuation("f2")],
    });

    const picked = historyCenterTransition("open", loaded.context, { type: "PICK_CONTINUATION", entryId: "f1" });
    expect(picked.effects).toEqual([]);
    expect(picked.context.open?.get("e1")?.pick?.entryId).toBe("f1");
    expect(picked.context.open?.get("e1")?.chosen).toBeNull();

    const confirmed = historyCenterTransition("open", picked.context, { type: "CONFIRM" });
    expect(confirmed.effects).toEqual([
      { type: "preferContinuation", entryId: "f1" },
      { type: "loadContinuationRun", fromEntryId: "f1" },
    ]);
    expect(confirmed.context.open?.get("e1")?.chosen?.entryId).toBe("f1");
  });

  test("PICK of a fork the picker does not offer is inert", () => {
    const afterDisclose = historyCenterTransition("open", ctx({ pages: forkedPages(3) }), {
      type: "DISCLOSE",
      entryId: "e1",
    });
    const loaded = historyCenterTransition("open", afterDisclose.context, {
      type: "CONTINUATIONS_LOADED",
      entryId: "e1",
      continuations: [continuation("e2", { preferred: true }), continuation("f1"), continuation("f2")],
    });

    // The own continuation (e2) was filtered from the picker.
    const result = historyCenterTransition("open", loaded.context, { type: "PICK_CONTINUATION", entryId: "e2" });
    expect(result.effects).toEqual([]);
    expect(result.context.open?.get("e1")?.pick).toBeNull();
  });

  test("CONFIRM without a pick is inert", () => {
    const result = historyCenterTransition("open", ctx({ pages: forkedPages(1) }), { type: "CONFIRM" });
    expect(result.effects).toEqual([]);
  });

  test("RUN_LOADED appends pages to the matching level and the run renders", () => {
    const afterDisclose = historyCenterTransition("open", ctx({ pages: forkedPages(2) }), {
      type: "DISCLOSE",
      entryId: "e1",
    });
    const loaded = historyCenterTransition("open", afterDisclose.context, {
      type: "CONTINUATIONS_LOADED",
      entryId: "e1",
      continuations: [continuation("e2", { preferred: true }), continuation("l1")],
    });
    const result = historyCenterTransition("open", loaded.context, {
      type: "RUN_LOADED",
      fromEntryId: "l1",
      pages: [page([entry("l1b"), entry("l1a", 0)])],
    });

    expect(result.effects).toEqual([]);
    expect(render(historyCenterVisibleRows(result.context.pages, result.context.open))).toEqual([
      "entry:e1@0:-:-:1",
      "entry:l1a@1:e1:l1:0",
      "entry:l1b@1:l1a:l1:0",
      "entry:e2@0:e1:-:0",
    ]);
  });

  test("a run page arriving in two pages joins oldest-first under the run", () => {
    const afterDisclose = historyCenterTransition("open", ctx({ pages: forkedPages(2) }), {
      type: "DISCLOSE",
      entryId: "e1",
    });
    const loaded = historyCenterTransition("open", afterDisclose.context, {
      type: "CONTINUATIONS_LOADED",
      entryId: "e1",
      continuations: [continuation("e2", { preferred: true }), continuation("l1")],
    });
    const first = historyCenterTransition("open", loaded.context, {
      type: "RUN_LOADED",
      fromEntryId: "l1",
      pages: [page([entry("l4", 0), entry("l3")])],
    });
    const second = historyCenterTransition("open", first.context, {
      type: "RUN_LOADED",
      fromEntryId: "l1",
      pages: [page([entry("l2"), entry("l1", 0)])],
    });

    expect(render(historyCenterVisibleRows(second.context.pages, second.context.open))).toEqual([
      "entry:e1@0:-:-:1",
      "entry:l1@1:e1:l1:0",
      "entry:l2@1:l1:l1:0",
      "entry:l3@1:l2:l1:0",
      "entry:l4@1:l3:l1:0",
      "entry:e2@0:e1:-:0",
    ]);
  });

  test("stale continuations and runs for entries that are not open are inert", () => {
    const staleContinuations = historyCenterTransition("open", ctx({ pages: forkedPages(1) }), {
      type: "CONTINUATIONS_LOADED",
      entryId: "ghost",
      continuations: [continuation("x1")],
    });
    expect(staleContinuations.effects).toEqual([]);
    expect(staleContinuations.context.open).toBeNull();

    const staleRun = historyCenterTransition("open", ctx({ pages: forkedPages(1) }), {
      type: "RUN_LOADED",
      fromEntryId: "ghost",
      pages: [page([entry("x1", 0)])],
    });
    expect(staleRun.effects).toEqual([]);
    expect(staleRun.context.open).toBeNull();
  });

  test("closing an entry drops its loaded run from state (R5)", () => {
    let context = ctx({ pages: forkedPages(2) });
    context = historyCenterTransition("open", context, { type: "DISCLOSE", entryId: "e1" }).context;
    context = historyCenterTransition("open", context, {
      type: "CONTINUATIONS_LOADED",
      entryId: "e1",
      continuations: [continuation("e2", { preferred: true }), continuation("l1")],
    }).context;
    context = historyCenterTransition("open", context, {
      type: "RUN_LOADED",
      fromEntryId: "l1",
      pages: [page([entry("l1a", 0)])],
    }).context;
    expect(historyCenterVisibleRows(context.pages, context.open).some((row) => row.kind === "entry" && row.entry.id === "l1a")).toBe(true);

    const closed = historyCenterTransition("open", context, { type: "DISCLOSE", entryId: "e1" });

    expect(closed.context.open).toBeNull();
    expect(historyCenterVisibleRows(closed.context.pages, closed.context.open).some((row) => row.kind === "entry" && row.entry.id === "l1a")).toBe(false);
  });

  test("a fork can open inside an open run", () => {
    let context = ctx({ pages: forkedPages(2) });
    context = historyCenterTransition("open", context, { type: "DISCLOSE", entryId: "e1" }).context;
    context = historyCenterTransition("open", context, {
      type: "CONTINUATIONS_LOADED",
      entryId: "e1",
      continuations: [continuation("e2", { preferred: true }), continuation("l1")],
    }).context;
    context = historyCenterTransition("open", context, {
      type: "RUN_LOADED",
      fromEntryId: "l1",
      pages: [page([entry("l1b"), entry("l1a", 2)])],
    }).context;

    // e1's run renders l1a/l1b; the fork inside the run is anchored at l1a.
    const nested = historyCenterTransition("open", context, { type: "DISCLOSE", entryId: "l1a" });
    expect(nested.effects).toEqual([{ type: "loadContinuations", entryId: "l1a" }]);
    expect(nested.context.open?.get("e1")?.inner?.get("l1a")).toBeDefined();
  });
});

describe("machine — traversal by row identity", () => {
  const pages = [page([entry("e2", 0), entry("e1", 3)])];

  test("FOCUS_MOVE wraps over the visible rows by identity", () => {
    const context = ctx({ pages, open: open([level("e1")]) });
    // rows: e1, picker:e1, e2 (e1 carries two forks)
    const first = historyCenterTransition("open", context, { type: "FOCUS_MOVE", direction: "next" });
    expect(first.context.focusRow).toEqual({ kind: "entry", entryId: "e1" });
    expect(first.effects).toEqual([{ type: "focusRow", row: { kind: "entry", entryId: "e1" } }]);

    const last = historyCenterTransition("open", context, { type: "FOCUS_MOVE", direction: "last" });
    expect(last.context.focusRow).toEqual({ kind: "entry", entryId: "e2" });

    const wrapped = historyCenterTransition("open", ctx({ pages, open: open([level("e1")]), focusRow: { kind: "entry", entryId: "e2" } }), {
      type: "FOCUS_MOVE",
      direction: "next",
    });
    expect(wrapped.context.focusRow).toEqual({ kind: "entry", entryId: "e1" });
  });

  test("traversal survives a disclosure toggle: focus stays on the same row identity, not the same index", () => {
    // e3 is the third row; opening e1's picker inserts a row before it, which
    // would shift an index-based focus.
    const plain = [page([entry("e3", 0, "current"), entry("e2", 0), entry("e1", 3)])];
    const context = ctx({ pages: plain, focusRow: { kind: "entry", entryId: "e3" } });
    const before = historyCenterVisibleRows(context.pages, context.open);
    expect(before.findIndex((row) => row.kind === "entry" && row.entry.id === "e3")).toBe(2);

    const disclosed = historyCenterTransition("open", context, { type: "DISCLOSE", entryId: "e1" });
    const after = historyCenterVisibleRows(disclosed.context.pages, disclosed.context.open);
    expect(after.findIndex((row) => row.kind === "entry" && row.entry.id === "e3")).toBe(3);

    // Identity held even though the index moved.
    expect(disclosed.context.focusRow).toEqual({ kind: "entry", entryId: "e3" });
  });

  test("closing a disclosure clamps focus to the anchor entry", () => {
    const context = ctx({
      pages,
      open: open([
        level("e1", {
          chosen: continuation("l1"),
          runPages: [page([entry("l1b"), entry("l1a", 0)])],
        }),
      ]),
      focusRow: { kind: "entry", entryId: "l1a" },
    });

    const closed = historyCenterTransition("open", context, { type: "DISCLOSE", entryId: "e1" });

    expect(closed.context.focusRow).toEqual({ kind: "entry", entryId: "e1" });
  });

  test("ACTIVATE_ROW emits navigate with the row's own branch and entry", () => {
    const spine = historyCenterTransition("open", ctx({ pages }), {
      type: "ACTIVATE_ROW",
      row: { kind: "entry", entryId: "e1" },
    });
    // The spine's branch id is null: the host knows its own branch.
    expect(spine.effects).toEqual([{ type: "emitNavigateEntry", branchId: null, entryId: "e1" }]);

    const runContext = ctx({
      pages,
      open: open([level("e1", { chosen: continuation("l1", { branchId: "b-lead" }), runPages: [page([entry("l1a", 0)])] })]),
    });
    const run = historyCenterTransition("open", runContext, {
      type: "ACTIVATE_ROW",
      row: { kind: "entry", entryId: "l1a" },
    });
    expect(run.effects).toEqual([{ type: "emitNavigateEntry", branchId: "b-lead", entryId: "l1a" }]);
  });

  test("ACTIVATE_ROW defaults to the focused row and is inert out of the list", () => {
    const activated = historyCenterTransition(
      "open",
      ctx({ pages, focusRow: { kind: "entry", entryId: "e2" } }),
      { type: "ACTIVATE_ROW" },
    );
    expect(activated.effects).toEqual([{ type: "emitNavigateEntry", branchId: null, entryId: "e2" }]);

    expect(historyCenterTransition("open", ctx({ pages }), { type: "ACTIVATE_ROW", row: { kind: "entry", entryId: "ghost" } }).effects).toEqual([]);
    expect(historyCenterTransition("open", ctx({ pages }), { type: "ACTIVATE_ROW" }).effects).toEqual([]);
  });

  test("activating a picker or not-yet-loaded row syncs focus but never navigates", () => {
    const pickerContext = ctx({ pages, open: open([level("e1")]) });
    const picker = historyCenterTransition("open", pickerContext, {
      type: "ACTIVATE_ROW",
      row: { kind: "picker", entryId: "e1" },
    });
    expect(picker.effects).toEqual([]);
    expect(picker.context.focusRow).toEqual({ kind: "picker", entryId: "e1" });

    const pending = historyCenterTransition("open", pickerContext, {
      type: "ACTIVATE_ROW",
      row: { kind: "not-yet-loaded", entryId: "e1" },
    });
    expect(pending.effects).toEqual([]);
  });
});

describe("machine — rename and rejection", () => {
  test("RENAME emits rename-branch unchanged (R6)", () => {
    const result = historyCenterTransition("open", ctx(), {
      type: "RENAME",
      branchId: "b-lead",
      name: "mix/lead",
    });

    expect(result.effects).toEqual([{ type: "emitRenameBranch", branchId: "b-lead", name: "mix/lead" }]);
    expect(result.context).toEqual(ctx());
  });

  test("SHOW_REJECTION maps the two codes to messages and is idempotent for the same one", () => {
    expect(historyCenterRejectionMessage("AlreadyAtTarget")).toBe("Already at the requested target");
    expect(historyCenterRejectionMessage("UnknownEntry")).toBe("Entry does not exist");

    const shown = historyCenterTransition("open", ctx(), { type: "SHOW_REJECTION", code: "AlreadyAtTarget" });
    expect(shown.context.rejection).toBe("Already at the requested target");

    expect(
      historyCenterTransition("open", shown.context, { type: "SHOW_REJECTION", code: "AlreadyAtTarget" }).effects,
    ).toEqual([]);

    const other = historyCenterTransition("open", shown.context, { type: "SHOW_REJECTION", code: "UnknownEntry" });
    expect(other.context.rejection).toBe("Entry does not exist");
  });

  test("DISMISS_REJECTION clears the notice and is inert when none is shown", () => {
    const cleared = historyCenterTransition("open", ctx({ rejection: "Entry does not exist" }), {
      type: "DISMISS_REJECTION",
    });
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
