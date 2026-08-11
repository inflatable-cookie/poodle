import { describe, expect, test } from "bun:test";

import {
  historyCenterDefaultContext,
  historyCenterKeydownEvent,
  historyCenterRowCount,
  historyCenterRows,
  historyCenterTransition,
  isForkPoint,
  type HistoryCenterContext,
  type HistoryEntry,
} from "../src/history-center.ts";

const entries: HistoryEntry[] = [
  { id: "e1", label: "Commit A", position: "past" },
  { id: "fork", label: "Fork point", position: "past", branchCount: 2 },
  { id: "e2", label: "Commit B", position: "current" },
];

const branches = [
  { id: "b1", name: "feature/audio", entryCount: 3, current: true },
  { id: "b2", name: null, entryCount: 1, pinned: true },
];

function ctx(overrides: Partial<HistoryCenterContext> = {}): HistoryCenterContext {
  return historyCenterDefaultContext({ entries, branches, ...overrides });
}

describe("popover open state", () => {
  test("TOGGLE opens from closed and closes from open, emitting open-change", () => {
    const opened = historyCenterTransition("closed", ctx(), { type: "TOGGLE" });

    expect(opened.state).toBe("open");
    expect(opened.effects).toEqual([{ type: "emitOpenChange", open: true }]);

    const closed = historyCenterTransition("open", ctx(), { type: "TOGGLE" });

    expect(closed.state).toBe("closed");
    expect(closed.effects).toEqual([{ type: "emitOpenChange", open: false }]);
  });

  test("OPEN and CLOSE are idempotent in their target state", () => {
    expect(historyCenterTransition("open", ctx(), { type: "OPEN" }).effects).toEqual([]);
    expect(historyCenterTransition("closed", ctx(), { type: "CLOSE" }).effects).toEqual([]);
  });

  test("navigation and activation are inert while closed", () => {
    const result = historyCenterTransition("closed", ctx(), { type: "ACTIVATE_ROW", index: 0 });

    expect(result.state).toBe("closed");
    expect(result.effects).toEqual([]);
  });
});

describe("row resolution", () => {
  test("entries become rows; expanded forks interleave branch rows after the fork", () => {
    const rows = historyCenterRows(entries, branches, ["fork"]);

    expect(rows.map((row) => (row.kind === "entry" ? `entry:${row.entry.id}` : `branch:${row.branch.id}`))).toEqual([
      "entry:e1",
      "entry:fork",
      "branch:b1",
      "branch:b2",
      "entry:e2",
    ]);
    expect(rows[2]).toMatchObject({ kind: "branch", entry: { id: "fork" }, branch: { id: "b1" } });
  });

  test("null branches disables branch rows entirely", () => {
    const rows = historyCenterRows(entries, null, ["fork"]);

    expect(rows.every((row) => row.kind === "entry")).toBe(true);
    expect(historyCenterRowCount(entries, null, ["fork"])).toBe(3);
  });

  test("non-fork entries never expand, even with branches present", () => {
    expect(historyCenterRowCount(entries, branches, ["e1"])).toBe(3);
  });

  test("isForkPoint reads the card's rule: branchCount > 1", () => {
    expect(isForkPoint({ id: "a", label: "A", position: "past", branchCount: 2 })).toBe(true);
    expect(isForkPoint({ id: "b", label: "B", position: "past", branchCount: 1 })).toBe(false);
    expect(isForkPoint({ id: "c", label: "C", position: "past" })).toBe(false);
  });
});

describe("list keyboard navigation", () => {
  test("next and prev wrap around the full row set", () => {
    expect(historyCenterTransition("open", ctx(), { type: "FOCUS_MOVE", direction: "next" }).context.focusIndex).toBe(1);
    expect(
      historyCenterTransition("open", ctx({ focusIndex: 2 }), { type: "FOCUS_MOVE", direction: "prev" }).context.focusIndex,
    ).toBe(1);
    expect(
      historyCenterTransition("open", ctx({ focusIndex: 2 }), { type: "FOCUS_MOVE", direction: "next" }).context.focusIndex,
    ).toBe(0);
    expect(
      historyCenterTransition("open", ctx({ focusIndex: 0 }), { type: "FOCUS_MOVE", direction: "prev" }).context.focusIndex,
    ).toBe(2);
  });

  test("first and last land on boundaries", () => {
    expect(
      historyCenterTransition("open", ctx({ focusIndex: 1 }), { type: "FOCUS_MOVE", direction: "first" }).context.focusIndex,
    ).toBe(0);
    expect(
      historyCenterTransition("open", ctx({ focusIndex: 1 }), { type: "FOCUS_MOVE", direction: "last" }).context.focusIndex,
    ).toBe(2);
  });

  test("focus moves emit a focusRow effect", () => {
    expect(historyCenterTransition("open", ctx(), { type: "FOCUS_MOVE", direction: "last" }).effects).toEqual([
      { type: "focusRow", index: 2 },
    ]);
  });

  test("navigation over an empty list is inert", () => {
    const result = historyCenterTransition("open", ctx({ entries: [] }), { type: "FOCUS_MOVE", direction: "next" });

    expect(result.context.focusIndex).toBe(0);
    expect(result.effects).toEqual([]);
  });
});

describe("row activation", () => {
  test("entry row activation emits select-entry", () => {
    const result = historyCenterTransition("open", ctx(), { type: "ACTIVATE_ROW", index: 0 });

    expect(result.effects).toEqual([{ type: "emitSelectEntry", id: "e1" }]);
  });

  test("branch row activation emits checkout with the fork context", () => {
    const result = historyCenterTransition("open", ctx({ expandedBranchIds: ["fork"] }), {
      type: "ACTIVATE_ROW",
      index: 3,
    });

    expect(result.effects).toEqual([{ type: "emitCheckout", branchId: "b2", entryId: "fork" }]);
  });

  test("out-of-bounds activation is inert", () => {
    expect(historyCenterTransition("open", ctx(), { type: "ACTIVATE_ROW", index: 9 }).effects).toEqual([]);
  });

  test("activation defaults to the focused row", () => {
    expect(historyCenterTransition("open", ctx({ focusIndex: 2 }), { type: "ACTIVATE_ROW" }).effects).toEqual([
      { type: "emitSelectEntry", id: "e2" },
    ]);
  });
});

describe("branch expansion", () => {
  test("EXPAND adds the fork's branches and is idempotent", () => {
    const expanded = historyCenterTransition("open", ctx(), { type: "EXPAND_BRANCHES", entryId: "fork" });

    expect(expanded.context.expandedBranchIds).toEqual(["fork"]);
    expect(historyCenterRowCount(expanded.context.entries, expanded.context.branches, expanded.context.expandedBranchIds)).toBe(5);

    expect(
      historyCenterTransition("open", ctx({ expandedBranchIds: ["fork"] }), {
        type: "EXPAND_BRANCHES",
        entryId: "fork",
      }).effects,
    ).toEqual([]);
  });

  test("TOGGLE_BRANCHES flips expansion", () => {
    expect(
      historyCenterTransition("open", ctx(), { type: "TOGGLE_BRANCHES", entryId: "fork" }).context.expandedBranchIds,
    ).toEqual(["fork"]);
    expect(
      historyCenterTransition("open", ctx({ expandedBranchIds: ["fork"] }), {
        type: "TOGGLE_BRANCHES",
        entryId: "fork",
      }).context.expandedBranchIds,
    ).toEqual([]);
  });

  test("expansion is inert without branches or on non-fork entries", () => {
    expect(historyCenterTransition("open", ctx({ branches: null }), { type: "TOGGLE_BRANCHES", entryId: "fork" }).effects).toEqual([]);
    expect(historyCenterTransition("open", ctx(), { type: "TOGGLE_BRANCHES", entryId: "e1" }).effects).toEqual([]);
  });

  test("collapsing a fork moves focus off its branch rows onto the fork entry", () => {
    const collapsed = historyCenterTransition("open", ctx({ expandedBranchIds: ["fork"], focusIndex: 3 }), {
      type: "COLLAPSE_BRANCHES",
      entryId: "fork",
    });

    expect(collapsed.context.expandedBranchIds).toEqual([]);
    expect(collapsed.context.focusIndex).toBe(1); // the "fork" entry row
  });

  test("collapsing an unexpanded fork is inert", () => {
    expect(historyCenterTransition("open", ctx(), { type: "COLLAPSE_BRANCHES", entryId: "fork" }).effects).toEqual([]);
  });

  test("expansion clamps an out-of-range focus index", () => {
    const result = historyCenterTransition("open", ctx({ focusIndex: 99 }), { type: "EXPAND_BRANCHES", entryId: "fork" });

    expect(result.context.focusIndex).toBe(4);
  });
});

describe("transient rejection display", () => {
  test("SHOW_REJECTION displays a message and is idempotent for the same one", () => {
    const shown = historyCenterTransition("open", ctx(), { type: "SHOW_REJECTION", message: "Fork branch name is invalid" });

    expect(shown.context.rejection).toBe("Fork branch name is invalid");

    expect(
      historyCenterTransition("open", shown.context, { type: "SHOW_REJECTION", message: "Fork branch name is invalid" }).effects,
    ).toEqual([]);
  });

  test("DISMISS_REJECTION clears the notice and is inert when none is shown", () => {
    const cleared = historyCenterTransition("open", ctx({ rejection: "boom" }), { type: "DISMISS_REJECTION" });

    expect(cleared.context.rejection).toBeNull();
    expect(historyCenterTransition("open", ctx(), { type: "DISMISS_REJECTION" }).effects).toEqual([]);
  });
});

describe("command pass-through", () => {
  test("CHECKOUT and RENAME emit their command effects without state change", () => {
    const checkout = historyCenterTransition("open", ctx(), { type: "CHECKOUT", branchId: "b1", entryId: "fork" });

    expect(checkout.effects).toEqual([{ type: "emitCheckout", branchId: "b1", entryId: "fork" }]);
    expect(checkout.context).toEqual(ctx());

    const rename = historyCenterTransition("open", ctx(), { type: "RENAME", branchId: "b1", name: "mix/lead" });

    expect(rename.effects).toEqual([{ type: "emitRenameBranch", branchId: "b1", name: "mix/lead" }]);
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
