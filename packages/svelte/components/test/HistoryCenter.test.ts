import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";
import type {
  HistoryContinuation,
  HistoryPathPage,
} from "@inflatable-cookie/poodle-core";

import HistoryCenter from "../src/HistoryCenter.svelte";
import HistoryCenterHostHarness from "./HistoryCenterHostHarness.svelte";

// v3 data: root path pages arrive newest-first (R3); display is oldest-first.
// `continuationCount` counts the run's own next row, so forkCount is one less
// (R4): cc 1 → forkCount 0 (inert), cc 2 → forkCount 1 (icon + chevron, no
// badge, no picker), cc 3 → forkCount 2 (icon + badge reading 2 + picker).

function page(entries: HistoryPathPage["entries"], precedingContinuationCount = 1): HistoryPathPage {
  return {
    entries,
    offset: 0,
    precedingContinuationCount,
    truncatedBefore: false,
    truncatedAfter: false,
  };
}

const continuation = (
  entryId: string,
  overrides: Partial<HistoryContinuation> = {},
): HistoryContinuation => ({
  entryId,
  label: entryId,
  preferred: false,
  entryCount: 2,
  branchId: `b-${entryId}`,
  branchName: null,
  ...overrides,
});

// c2 carries continuationCount 3 → forkCount 2: two forks at one entry.
const twoForkPages = [
  page([
    { id: "c3", label: "Current draft", position: "current", continuationCount: 0 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 3 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1 },
  ]),
];

const twoForkContinuations: Record<string, HistoryContinuation[]> = {
  c2: [
    continuation("l1", { label: "Lead intro", branchName: "feature/lead" }),
    continuation("x1", { label: "Alt intro", preferred: true, branchName: "feature/alt", entryCount: 1 }),
  ],
};

const twoForkRuns: Record<string, HistoryPathPage[]> = {
  x1: [
    page([
      { id: "x2", label: "Alt mix", position: "past", continuationCount: 0 },
      { id: "x1", label: "Alt intro", position: "past", continuationCount: 1 },
    ]),
  ],
  l1: [
    page([
      { id: "l2", label: "Lead mix", position: "past", continuationCount: 0 },
      { id: "l1", label: "Lead intro", position: "past", continuationCount: 1 },
    ]),
  ],
};

// c2 carries continuationCount 2 → forkCount 1: one fork, auto-chosen, never
// a picker. The run [l1, l2, l3] forks again off l2 → a fork off a fork.
const singleForkPages = [
  page([
    { id: "c3", label: "Current draft", position: "current", continuationCount: 0 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 2 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1 },
  ]),
];

const singleForkContinuations: Record<string, HistoryContinuation[]> = {
  c2: [continuation("l1", { label: "Lead intro", branchName: "feature/lead", preferred: true, entryCount: 3 })],
  l2: [continuation("i1", { label: "Inner intro", branchName: "feature/inner", preferred: true, entryCount: 2 })],
};

// A two-entry run variant used by the run-header time tests.
const twoEntryContinuations: Record<string, HistoryContinuation[]> = {
  c2: [continuation("l1", { label: "Lead intro", branchName: "feature/lead", preferred: true, entryCount: 2 })],
};

const twoEntryRuns: Record<string, HistoryPathPage[]> = {
  l1: [
    page([
      { id: "l2", label: "Lead mix", position: "past", continuationCount: 0 },
      { id: "l1", label: "Lead intro", position: "past", continuationCount: 1 },
    ]),
  ],
};

const nestedRuns: Record<string, HistoryPathPage[]> = {
  l1: [
    page([
      { id: "l3", label: "Lead outro", position: "past", continuationCount: 0 },
      { id: "l2", label: "Lead bridge", position: "past", continuationCount: 2 },
      { id: "l1", label: "Lead intro", position: "past", continuationCount: 1 },
    ]),
  ],
  i1: [
    page([
      { id: "i2", label: "Inner mix", position: "past", continuationCount: 0 },
      { id: "i1", label: "Inner intro", position: "past", continuationCount: 1 },
    ]),
  ],
};

// Timed variant of the single-fork fixture: the run's head (l2) sits 20
// minutes before the data's newest entry (c3), so the run header derives
// "20m ago" from supplied data only (D2) — there is no clock.
const timedSingleForkPages = [
  page([
    { id: "c3", label: "Current draft", position: "current", continuationCount: 0, recordedAtMs: 3_600_000 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 2, recordedAtMs: 600_000 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1, recordedAtMs: 0 },
  ]),
];

const timedRuns: Record<string, HistoryPathPage[]> = {
  l1: [
    page([
      { id: "l2", label: "Lead mix", position: "past", continuationCount: 0, recordedAtMs: 2_400_000 },
      { id: "l1", label: "Lead intro", position: "past", continuationCount: 1, recordedAtMs: 1_200_000 },
    ]),
  ],
};

// A chain of single forks: run k forks off its middle entry into run k+1.
// Rows reach depth 5 — v2's depth cap (3) is gone and must not return.
const deepPages = [
  page([
    { id: "r3", label: "Root tail", position: "past", continuationCount: 0 },
    { id: "r2", label: "Root bridge", position: "past", continuationCount: 2 },
    { id: "r1", label: "Root head", position: "past", continuationCount: 1 },
  ]),
];

const deepContinuations: Record<string, HistoryContinuation[]> = {
  r2: [continuation("a1", { preferred: true, entryCount: 3 })],
  a2: [continuation("b1", { preferred: true, entryCount: 3 })],
  b2: [continuation("c1", { preferred: true, entryCount: 3 })],
  c2: [continuation("d1", { preferred: true, entryCount: 3 })],
  d2: [continuation("e1", { preferred: true, entryCount: 3 })],
};

function deepRun(first: string, mid: string, tail: string): HistoryPathPage {
  return page([
    { id: tail, label: `${tail} tail`, position: "past", continuationCount: 0 },
    { id: mid, label: `${mid} bridge`, position: "past", continuationCount: 2 },
    { id: first, label: `${first} head`, position: "past", continuationCount: 1 },
  ]);
}

const deepRuns: Record<string, HistoryPathPage[]> = {
  a1: [deepRun("a1", "a2", "a3")],
  b1: [deepRun("b1", "b2", "b3")],
  c1: [deepRun("c1", "c2", "c3")],
  d1: [deepRun("d1", "d2", "d3")],
  e1: [deepRun("e1", "e2", "e3")],
};

function rowSummary(): Array<{ kind: string; entry: string; depth: string }> {
  return [...document.querySelectorAll("[data-row-kind]")].map((el) => ({
    kind: el.getAttribute("data-row-kind") ?? "",
    entry: el.getAttribute("data-row-entry") ?? "",
    depth: el.getAttribute("data-depth") ?? "",
  }));
}

function rowByEntry(entryId: string): HTMLElement {
  const row = document.querySelector(`[data-row-kind="entry"][data-row-entry="${entryId}"]`) as HTMLElement;
  if (row === null) {
    throw new Error(`no entry row for ${entryId}`);
  }
  return row;
}

/** Open the fork at an entry with a harness that feeds results synchronously. */
describe("HistoryCenter (svelte)", () => {
  // The helpers query `document` globally, so a test that fails before its
  // own unmount() leaves rows behind and poisons the next one. Isolate.
  afterEach(cleanup);

  it("renders the undo/list/redo cluster with enablement from canUndo/canRedo and busy", async () => {
    render(HistoryCenter, { props: { canUndo: true, canRedo: false, busy: false } });

    expect(screen.getByRole("button", { name: "Undo" }).hasAttribute("disabled")).toBe(false);
    expect(screen.getByRole("button", { name: "Redo" }).hasAttribute("disabled")).toBe(true);
  });

  it("disables undo/redo while busy even when the host can undo", async () => {
    render(HistoryCenter, { props: { canUndo: true, busy: true } });

    expect(screen.getByRole("button", { name: "Undo" }).hasAttribute("disabled")).toBe(true);
  });

  it("opens the popover and navigates a spine entry with the host's own branch (null)", async () => {
    const onNavigateEntry = vi.fn();
    render(HistoryCenter, {
      props: { pages: twoForkPages, onNavigateEntry },
    });

    await fireEvent.click(screen.getByRole("button", { name: "History" }));

    expect(screen.getByRole("dialog", { name: "History" })).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Committed mix 1" }));

    // Spine rows carry no branch — the host's own branch is null (never an
    // ancestor or another branch's divergence entry).
    expect(onNavigateEntry).toHaveBeenCalledWith(null, "c1");
  });

  it("renders no fork affordance when forkCount is 0 — the entry is inert", async () => {
    render(HistoryCenter, { props: { pages: twoForkPages, defaultOpen: true } });

    // c1 (continuationCount 1) and c3 (continuationCount 0) have forkCount 0:
    // no fork icon, no badge, no chevron, no picker row.
    expect(rowByEntry("c1").querySelector("[data-part=\"fork-disclosure\"]")).toBeNull();
    expect(rowByEntry("c3").querySelector("[data-part=\"fork-disclosure\"]")).toBeNull();
    expect(rowByEntry("c1").querySelector("[data-part=\"fork-badge\"]")).toBeNull();
    expect(rowByEntry("c3").querySelector("[data-part=\"fork-badge\"]")).toBeNull();
    expect(document.querySelector("[data-part=\"picker\"]")).toBeNull();
    expect(rowByEntry("c1").getAttribute("data-fork-count")).toBe("0");
    expect(rowByEntry("c3").getAttribute("data-fork-count")).toBe("0");
  });

  it("shows icon + chevron without badge or picker at forkCount 1, auto-choosing the single fork", async () => {
    const onLoadContinuations = vi.fn();
    const onLoadContinuationRun = vi.fn();
    render(HistoryCenterHostHarness, {
      props: {
        pages: singleForkPages,
        continuationsByEntry: singleForkContinuations,
        runsByFork: nestedRuns,
        defaultOpen: true,
        onLoadContinuations,
        onLoadContinuationRun,
      },
    });

    const c2 = rowByEntry("c2");
    expect(c2.getAttribute("data-fork-count")).toBe("1");
    const disclosure = c2.querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement;
    expect(disclosure).toBeTruthy();
    // forkCount 1: a fork icon and a chevron, no counter badge.
    expect(c2.querySelector("[data-part=\"fork-badge\"]")).toBeNull();
    expect(disclosure.getAttribute("aria-label")).toBe("Show 1 continuation");

    await fireEvent.click(disclosure);

    expect(onLoadContinuations).toHaveBeenCalledWith("c2");
    expect(onLoadContinuationRun).toHaveBeenCalledWith("l1");

    // b033 R3: one row shape serves both fork counts. A single fork still gets
    // the picker row, with the select disabled because there is nothing to
    // choose between.
    const picker = document.querySelector("[data-part='picker']");
    expect(picker).toBeTruthy();
    const trigger = picker?.querySelector(".poodle-select__trigger") as HTMLButtonElement | null;
    expect(trigger).toBeTruthy();
    expect(trigger?.disabled).toBe(true);

    // The run renders at depth 1 once the host feeds its pages back.
    expect(rowSummary()).toEqual([
      { kind: "entry", entry: "c1", depth: "0" },
      { kind: "entry", entry: "c2", depth: "0" },
      { kind: "picker", entry: "c2", depth: "1" },
      { kind: "entry", entry: "l1", depth: "1" },
      { kind: "entry", entry: "l2", depth: "1" },
      { kind: "entry", entry: "l3", depth: "1" },
      { kind: "entry", entry: "c3", depth: "0" },
    ]);
  });

  it("renders one badge reading 2 and a persistent Select picker at forkCount 2, distinct from a fork off a fork", async () => {
    const onNavigateEntry = vi.fn();
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
        onNavigateEntry,
      },
    });

    const c2 = rowByEntry("c2");
    expect(c2.getAttribute("data-fork-count")).toBe("2");
    const badge = c2.querySelector("[data-part=\"fork-badge\"]") as HTMLElement;
    expect(badge?.textContent).toBe("2");

    await fireEvent.click(c2.querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // R3 selects the current fork (x1, preferred) and shows its run; the
    // picker row persists above it (R1) — a Select plus a checkout button.
    expect(rowSummary()).toEqual([
      { kind: "entry", entry: "c1", depth: "0" },
      { kind: "entry", entry: "c2", depth: "0" },
      { kind: "picker", entry: "c2", depth: "1" },
      { kind: "entry", entry: "x1", depth: "1" },
      { kind: "entry", entry: "x2", depth: "1" },
      { kind: "entry", entry: "c3", depth: "0" },
    ]);
    expect(document.querySelector("[data-part='picker-select']")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Checkout" })).toBeTruthy();

    // R1: the pencil sits between the Select and checkout, renames the
    // selection, and no "Current" badge marks the trigger (R4a).
    const trigger = screen.getByRole("button", { name: "Continuations" });
    expect(trigger.textContent).toContain("Alt intro");
    expect(trigger.textContent).toContain("feature/alt");
    expect(trigger.textContent).not.toContain("Current");

    const select = document.querySelector("[data-part='picker-select'] .poodle-select") as HTMLElement;
    const pencil = screen.getByRole("button", { name: "Rename feature/alt" });
    const checkout = screen.getByRole("button", { name: "Checkout" });
    expect(select.compareDocumentPosition(pencil) & Node.DOCUMENT_POSITION_FOLLOWING).not.toBe(0);
    expect(pencil.compareDocumentPosition(checkout) & Node.DOCUMENT_POSITION_FOLLOWING).not.toBe(0);
    expect(document.querySelector("[data-part='current-badge']")).toBeNull();

    // Opening the select offers exactly the two forks with the same anatomy.
    await fireEvent.click(trigger);
    expect(screen.getByRole("option", { name: /Lead intro feature\/lead/ })).toBeTruthy();
    expect(screen.getByRole("option", { name: /Alt intro feature\/alt/ })).toBeTruthy();
    expect(document.querySelector("[data-part='current-badge']")).toBeNull();
  });

  it("selects the non-preferred fork: preview swaps below the select, checkout emits, never navigates", async () => {
    const onNavigateEntry = vi.fn();
    const onCheckoutContinuation = vi.fn();
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
        onNavigateEntry,
        onCheckoutContinuation,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // R3 selected the current fork (x1): checkout is disabled on it.
    expect(screen.getByRole("button", { name: "Checkout" }).hasAttribute("disabled")).toBe(true);

    // Select the non-preferred fork: the entries below swap to its run and
    // the select stays — the pick commits nothing.
    await fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    await fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));

    expect(rowSummary()).toEqual([
      { kind: "entry", entry: "c1", depth: "0" },
      { kind: "entry", entry: "c2", depth: "0" },
      { kind: "picker", entry: "c2", depth: "1" },
      { kind: "entry", entry: "l1", depth: "1" },
      { kind: "entry", entry: "l2", depth: "1" },
      { kind: "entry", entry: "c3", depth: "0" },
    ]);
    expect(onCheckoutContinuation).not.toHaveBeenCalled();
    expect(onNavigateEntry).not.toHaveBeenCalled();
    expect(screen.getByRole("button", { name: "Checkout" }).hasAttribute("disabled")).toBe(false);

    await fireEvent.click(screen.getByRole("button", { name: "Checkout" }));

    // Checkout makes the fork primary: the command leaves, the disclosure
    // state for the anchor is cleared, and the root list renders again —
    // no fabrication, no navigation.
    expect(onCheckoutContinuation).toHaveBeenCalledWith("l1");
    expect(onNavigateEntry).not.toHaveBeenCalled();
    expect(rowSummary()).toEqual([
      { kind: "entry", entry: "c1", depth: "0" },
      { kind: "entry", entry: "c2", depth: "0" },
      { kind: "entry", entry: "c3", depth: "0" },
    ]);
    expect(document.querySelector("[data-part='picker']")).toBeNull();
  });

  it("selecting a fork emits no host operation — only the preview run loads", async () => {
    const onCheckoutContinuation = vi.fn();
    const onLoadContinuationRun = vi.fn();
    const onNavigateEntry = vi.fn();
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
        onCheckoutContinuation,
        onLoadContinuationRun,
        onNavigateEntry,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);
    onLoadContinuationRun.mockClear();

    await fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    await fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));

    // The pick loaded l1's run (R2) and committed nothing: no checkout, no
    // navigation, no open-change.
    expect(onLoadContinuationRun).toHaveBeenCalledWith("l1");
    expect(onCheckoutContinuation).not.toHaveBeenCalled();
    expect(onNavigateEntry).not.toHaveBeenCalled();
  });

  it("disables checkout while the selection is already the current fork", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // R3 opened on the current fork (x1, preferred): checkout is disabled on
    // it. The badge is gone (R4a) — the disabled button carries the fact.
    expect(screen.getByRole("button", { name: "Checkout" }).hasAttribute("disabled")).toBe(true);
    expect(document.querySelector("[data-part='current-badge']")).toBeNull();

    // Picking the other fork enables checkout.
    await fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    await fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));
    expect(screen.getByRole("button", { name: "Checkout" }).hasAttribute("disabled")).toBe(false);
  });

  it("renames the selected fork from the picker through onRenameBranch", async () => {
    const onRenameBranch = vi.fn();
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
        onRenameBranch,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // R3 opened on the current fork (x1): the pencil targets whatever the
    // Select shows — x1's branch, not the anchor's and not some other fork.
    await fireEvent.click(screen.getByRole("button", { name: "Rename feature/alt" }));

    // The input replaces the Select while renaming (R3).
    expect(screen.queryByRole("button", { name: "Continuations" })).toBeNull();
    const input = screen.getByRole("textbox", { name: "Rename branch feature/alt" }) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "feature/alt-v2" } });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(onRenameBranch).toHaveBeenCalledWith("b-x1", "feature/alt-v2");
  });

  it("changing the Select then renaming targets the newly selected fork", async () => {
    const onRenameBranch = vi.fn();
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
        onRenameBranch,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // Preview the non-preferred fork, then rename it: the pencil follows the
    // selection — b-l1, not the preferred fork (b-x1) and not the anchor's.
    await fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    await fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));

    await fireEvent.click(screen.getByRole("button", { name: "Rename feature/lead" }));
    const input = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "feature/lead-v2" } });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(onRenameBranch).toHaveBeenCalledWith("b-l1", "feature/lead-v2");
  });

  it("disables checkout while a rename is open", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // Enable checkout by picking the non-preferred fork…
    await fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    await fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));
    expect(screen.getByRole("button", { name: "Checkout" }).hasAttribute("disabled")).toBe(false);

    // …then a rename opens and checkout goes inert (R3).
    await fireEvent.click(screen.getByRole("button", { name: "Rename feature/lead" }));
    expect(screen.getByRole("button", { name: "Checkout" }).hasAttribute("disabled")).toBe(true);
  });

  it("cancelling a picker rename restores the Select and returns focus to the pencil", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);
    await fireEvent.click(screen.getByRole("button", { name: "Rename feature/alt" }));

    const input = screen.getByRole("textbox", { name: "Rename branch feature/alt" }) as HTMLInputElement;
    await fireEvent.keyDown(input, { key: "Escape" });

    // The Select is back and focus sits on the pencil (R3).
    expect(screen.getByRole("button", { name: "Continuations" })).toBeTruthy();
    expect(screen.queryByRole("textbox", { name: "Rename branch feature/alt" })).toBeNull();
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Rename feature/alt" }));
  });

  it("renders a fork off a fork at the inner depth, never confusable with a picker", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: singleForkPages,
        continuationsByEntry: singleForkContinuations,
        runsByFork: nestedRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);
    await fireEvent.click(rowByEntry("l2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // Outer run at depth 1, inner run at depth 2 — the fork identity is data
    // (data-fork-id), never indentation alone.
    expect(rowSummary()).toEqual([
      { kind: "entry", entry: "c1", depth: "0" },
      { kind: "entry", entry: "c2", depth: "0" },
      { kind: "picker", entry: "c2", depth: "1" },
      { kind: "entry", entry: "l1", depth: "1" },
      { kind: "entry", entry: "l2", depth: "1" },
      { kind: "picker", entry: "l2", depth: "2" },
      { kind: "entry", entry: "i1", depth: "2" },
      { kind: "entry", entry: "i2", depth: "2" },
      { kind: "entry", entry: "l3", depth: "1" },
      { kind: "entry", entry: "c3", depth: "0" },
    ]);
    const inner = rowByEntry("i1");
    expect(inner.getAttribute("data-parent-entry")).toBe("l2");
    expect(inner.getAttribute("data-fork-id")).toBe("i1");
    const outer = rowByEntry("l1");
    expect(outer.getAttribute("data-parent-entry")).toBe("c2");
    expect(outer.getAttribute("data-fork-id")).toBe("l1");
  });

  it("renders no fork affordance on a run's last entry", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: singleForkPages,
        continuationsByEntry: singleForkContinuations,
        runsByFork: nestedRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // l3 is the run's last entry (continuationCount 0 → forkCount 0).
    const l3 = rowByEntry("l3");
    expect(l3.getAttribute("data-fork-count")).toBe("0");
    expect(l3.querySelector("[data-part=\"fork-disclosure\"]")).toBeNull();
  });

  it("shows the opened region on the run's first entry: branch name, count, derived time", async () => {
    const { unmount } = render(HistoryCenterHostHarness, {
      props: {
        pages: timedSingleForkPages,
        continuationsByEntry: twoEntryContinuations,
        runsByFork: timedRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // b033 R3: the run header is gone. Its branch name and its
    // "N entries · time" meta now live in the picker trigger.
    const picker = document.querySelector("[data-part='picker']") as HTMLElement;
    expect(picker).toBeTruthy();
    expect(picker.querySelector(".poodle-history-center__picker-option-branch")?.textContent?.trim()).toBe("feature/lead");
    expect(picker.querySelector(".poodle-history-center__picker-option-meta")?.textContent).toBe("2 entries · 20m ago");
    // The picker is one row anchored at the forked entry, not a header
    // repeated per run entry.
    // `data-part="picker"` is on the row <li> as well as the inner control,
    // so assert the row's own entry rather than the inner data-anchor.
    expect(picker.getAttribute("data-row-entry")).toBe("c2");

    unmount();
  });

  it("renders a caption with no time when recordedAtMs is absent — never Invalid Date", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: singleForkPages,
        continuationsByEntry: twoEntryContinuations,
        runsByFork: twoEntryRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // b033 R3: the run header is gone. Its branch name and its
    // "N entries · time" meta now live in the picker trigger.
    const picker = document.querySelector("[data-part='picker']") as HTMLElement;
    expect(picker.querySelector(".poodle-history-center__picker-option-meta")?.textContent).toBe("2 entries");
    expect(picker.textContent).not.toContain("ago");
    expect(document.body.textContent).not.toContain("Invalid Date");
  });

  it("renders rows at depth 3 and depth 5, neither clamped by a depth cap", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: deepPages,
        continuationsByEntry: deepContinuations,
        runsByFork: deepRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("r2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);
    await fireEvent.click(rowByEntry("a2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);
    await fireEvent.click(rowByEntry("b2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);
    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);
    await fireEvent.click(rowByEntry("d2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    const depth3 = rowByEntry("c2");
    const depth5 = rowByEntry("e2");
    expect(depth3.getAttribute("data-depth")).toBe("3");
    expect(depth3.getAttribute("aria-level")).toBe("4");
    expect(depth5.getAttribute("data-depth")).toBe("5");
    expect(depth5.getAttribute("aria-level")).toBe("6");
    // Neither clamped: depth 5 indents five steps, depth 3 indents three.
    expect(depth5.style.getPropertyValue("--poodle-history-center-depth")).toBe("5");
    expect(depth3.style.getPropertyValue("--poodle-history-center-depth")).toBe("3");
  });

  it("traverses visible rows linearly in visual order, wrapping at the ends", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    // DISCLOSE clamps machine focus to the anchor; Home starts the traversal
    // from the first visible row. R3 loaded the current fork's run, so the
    // rows are c1, c2, picker, x1, x2, c3.
    await fireEvent.keyDown(document.activeElement as HTMLElement, { key: "Home" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Committed mix 1" }));

    const c1 = screen.getByRole("button", { name: "Committed mix 1" });
    await fireEvent.keyDown(c1, { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Arranged intro" }));

    await fireEvent.keyDown(screen.getByRole("button", { name: "Arranged intro" }), { key: "ArrowDown" });
    // The picker row is a roving stop too — focus lands on the picker.
    expect(document.activeElement?.getAttribute("data-part")).toBe("picker");

    await fireEvent.keyDown(document.activeElement as HTMLElement, { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Alt intro" }));

    await fireEvent.keyDown(screen.getByRole("button", { name: "Alt intro" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Alt mix" }));

    await fireEvent.keyDown(screen.getByRole("button", { name: "Alt mix" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Current draft" }));

    // Wraps back to the first row.
    await fireEvent.keyDown(screen.getByRole("button", { name: "Current draft" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Committed mix 1" }));

    // Home / End land on the boundaries.
    await fireEvent.keyDown(screen.getByRole("button", { name: "Committed mix 1" }), { key: "End" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Current draft" }));
    await fireEvent.keyDown(screen.getByRole("button", { name: "Current draft" }), { key: "Home" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Committed mix 1" }));

    // The Select owns its keys: ArrowDown on the picker's trigger opens its
    // listbox and never moves roving focus.
    const selectTrigger = screen.getByRole("button", { name: "Continuations" });
    selectTrigger.focus();
    await fireEvent.keyDown(selectTrigger, { key: "ArrowDown" });
    expect(document.activeElement).toBe(selectTrigger);
    expect(screen.getByRole("option", { name: /Lead intro feature\/lead/ })).toBeTruthy();
  });

  it("commits inline rename through onRenameBranch and cancels on Escape", async () => {
    const onRenameBranch = vi.fn();
    render(HistoryCenterHostHarness, {
      props: {
        pages: singleForkPages,
        continuationsByEntry: singleForkContinuations,
        runsByFork: nestedRuns,
        defaultOpen: true,
        onRenameBranch,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    await fireEvent.click(screen.getByRole("button", { name: "Rename feature/lead" }));

    const input = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "feature/lead-v2" } });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(onRenameBranch).toHaveBeenCalledWith("b-l1", "feature/lead-v2");

    // Escape cancels without emitting.
    await fireEvent.click(screen.getByRole("button", { name: "Rename feature/lead" }));
    const second = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    await fireEvent.keyDown(second, { key: "Escape" });

    expect(onRenameBranch).toHaveBeenCalledTimes(1);
    expect(screen.queryByRole("textbox", { name: "Rename branch feature/lead" })).toBeNull();
  });

  it("shows a rejection code as the machine's display copy, dismissible", async () => {
    const { unmount } = render(HistoryCenterHostHarness, {
      props: { pages: twoForkPages, defaultOpen: true, rejection: "AlreadyAtTarget" },
    });

    expect(screen.getByText("Already at the requested target")).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Dismiss" }));
    expect(screen.queryByText("Already at the requested target")).toBeNull();

    unmount();
    render(HistoryCenterHostHarness, {
      props: { pages: twoForkPages, defaultOpen: true, rejection: "UnknownEntry" },
    });

    expect(screen.getByText("Entry does not exist")).toBeTruthy();
  });

  it("renders the empty message, the loading row and the failed row when there are no rows", async () => {
    const { unmount } = render(HistoryCenter, { props: { defaultOpen: true } });

    expect(screen.getByText("No history entries yet.")).toBeTruthy();
    expect(document.querySelector("[data-row-kind]")).toBeNull();

    unmount();
    render(HistoryCenter, { props: { status: "loading", defaultOpen: true } });

    expect(screen.getByText("Loading history…")).toBeTruthy();

    unmount();
    render(HistoryCenter, { props: { status: "failed", statusMessage: "Authority unreachable", defaultOpen: true } });

    expect(screen.getByText("Authority unreachable")).toBeTruthy();
  });

  it("exposes depth to assistive tech through aria-level on every row", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: singleForkPages,
        continuationsByEntry: singleForkContinuations,
        runsByFork: nestedRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);
    await fireEvent.click(rowByEntry("l2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    const rows = [...document.querySelectorAll("[data-row-kind]")];
    expect(rows.map((el) => el.getAttribute("aria-level"))).toEqual([
      "1",
      "1",
      "2",
      "2",
      "2",
      "3",
      "3",
      "3",
      "2",
      "1",
    ]);
  });
});
