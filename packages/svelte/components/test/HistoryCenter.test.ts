import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";
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

function page(entries: HistoryPathPage["entries"], rootContinuationCount = 1): HistoryPathPage {
  return {
    entries,
    offset: 0,
    rootContinuationCount,
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
  l1: [page([
    { id: "l2", label: "Lead mix", position: "past", continuationCount: 0 },
    { id: "l1", label: "Lead intro", position: "past", continuationCount: 1 },
  ])],
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
    // The single fork is auto-chosen — no picker row ever appears.
    expect(onLoadContinuationRun).toHaveBeenCalledWith("l1");
    expect(document.querySelector("[data-part='picker']")).toBeNull();

    // The run renders at depth 1 once the host feeds its pages back.
    expect(rowSummary()).toEqual([
      { kind: "entry", entry: "c1", depth: "0" },
      { kind: "entry", entry: "c2", depth: "0" },
      { kind: "entry", entry: "l1", depth: "1" },
      { kind: "entry", entry: "l2", depth: "1" },
      { kind: "entry", entry: "l3", depth: "1" },
      { kind: "entry", entry: "c3", depth: "0" },
    ]);
  });

  it("renders one badge reading 2 and one picker with two options at forkCount 2, distinct from a fork off a fork", async () => {
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

    // Exactly one picker row at depth 1, offering exactly the two forks —
    // asserted on the rendered rows, not the row data.
    expect(rowSummary()).toEqual([
      { kind: "entry", entry: "c1", depth: "0" },
      { kind: "entry", entry: "c2", depth: "0" },
      { kind: "picker", entry: "c2", depth: "1" },
      { kind: "entry", entry: "c3", depth: "0" },
    ]);
    const options = [...document.querySelectorAll("[data-part='picker-option']")];
    expect(options).toHaveLength(2);
    expect(screen.getByRole("button", { name: /Lead intro feature\/lead/ })).toBeTruthy();
    expect(screen.getByRole("button", { name: /Alt intro feature\/alt/ })).toBeTruthy();
  });

  it("picks the non-preferred fork and confirms: prefers it, reveals its run, never navigates", async () => {
    const onNavigateEntry = vi.fn();
    const onPreferContinuation = vi.fn();
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
        onNavigateEntry,
        onPreferContinuation,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);
    await fireEvent.click(screen.getByRole("button", { name: /Lead intro feature\/lead/ }));

    expect(screen.getByRole("button", { name: "Choose" }).hasAttribute("disabled")).toBe(false);

    await fireEvent.click(screen.getByRole("button", { name: "Choose" }));

    // Confirm = the picker's commit: prefer the picked future and reveal its
    // run — and never navigates anywhere.
    expect(onPreferContinuation).toHaveBeenCalledWith("l1");
    expect(onNavigateEntry).not.toHaveBeenCalled();

    expect(rowSummary()).toEqual([
      { kind: "entry", entry: "c1", depth: "0" },
      { kind: "entry", entry: "c2", depth: "0" },
      { kind: "entry", entry: "l1", depth: "1" },
      { kind: "entry", entry: "l2", depth: "1" },
      { kind: "entry", entry: "c3", depth: "0" },
    ]);
  });

  it("disables confirm while the picked continuation is already preferred", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
      },
    });

    await fireEvent.click(rowByEntry("c2").querySelector("[data-part=\"fork-disclosure\"]") as HTMLElement);

    const preferredOption = screen.getByRole("button", { name: /Alt intro feature\/alt/ });
    expect(preferredOption.querySelector("[data-part='preferred-badge']")?.textContent).toBe("Preferred");

    await fireEvent.click(preferredOption);
    expect(preferredOption.getAttribute("aria-pressed")).toBe("true");

    // Already at the requested target — a race, not a normal path (R4).
    expect(screen.getByRole("button", { name: "Choose" }).hasAttribute("disabled")).toBe(true);
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
      { kind: "entry", entry: "l1", depth: "1" },
      { kind: "entry", entry: "l2", depth: "1" },
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

    const runHeader = rowByEntry("l1").querySelector("[data-part='run-header']") as HTMLElement;
    expect(runHeader).toBeTruthy();
    expect(runHeader.querySelector(".poodle-history-center__run-header-name")?.textContent).toBe("feature/lead");
    expect(runHeader.querySelector(".poodle-history-center__run-header-meta")?.textContent).toBe("2 entries · 20m ago");
    // The run header belongs to the run's first entry only.
    expect(rowByEntry("l2").querySelector("[data-part='run-header']")).toBeNull();

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

    const runHeader = rowByEntry("l1").querySelector("[data-part='run-header']") as HTMLElement;
    expect(runHeader.querySelector(".poodle-history-center__run-header-meta")?.textContent).toBe("2 entries");
    expect(runHeader.textContent).not.toContain("ago");
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
    // from the first visible row.
    await fireEvent.keyDown(document.activeElement as HTMLElement, { key: "Home" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Committed mix 1" }));

    const c1 = screen.getByRole("button", { name: "Committed mix 1" });
    await fireEvent.keyDown(c1, { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Arranged intro" }));

    await fireEvent.keyDown(screen.getByRole("button", { name: "Arranged intro" }), { key: "ArrowDown" });
    // The picker row is a roving stop too — focus lands on the picker.
    expect(document.activeElement?.getAttribute("data-part")).toBe("picker");

    await fireEvent.keyDown(document.activeElement as HTMLElement, { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Current draft" }));

    // Wraps back to the first row.
    await fireEvent.keyDown(screen.getByRole("button", { name: "Current draft" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Committed mix 1" }));

    // Home / End land on the boundaries.
    await fireEvent.keyDown(screen.getByRole("button", { name: "Committed mix 1" }), { key: "End" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Current draft" }));
    await fireEvent.keyDown(screen.getByRole("button", { name: "Current draft" }), { key: "Home" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Committed mix 1" }));
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
      "3",
      "3",
      "2",
      "1",
    ]);
  });
});
