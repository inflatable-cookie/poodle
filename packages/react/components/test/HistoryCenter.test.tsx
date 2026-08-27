import { act, fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type {
  HistoryContinuation,
  HistoryPathPage,
} from "@inflatable-cookie/poodle-core";

import { HistoryCenter } from "../src";
import type { HistoryEntry } from "../src/types";

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

// Stable result feeds — the host passes the same reference until a NEW result
// arrives; the component's reference-diff dispatches each exactly once.
const c2Result = { entryId: "c2", continuations: twoForkContinuations.c2 };
const l1Result = { entryId: "c2", continuations: singleForkContinuations.c2 };
const l2Result = { entryId: "l2", continuations: singleForkContinuations.l2 };
const l1NestedRun = { fromEntryId: "l1", pages: nestedRuns.l1 };
const i1Run = { fromEntryId: "i1", pages: nestedRuns.i1 };
const l1TwoForkRun = { fromEntryId: "l1", pages: twoForkRuns.l1 };
const x1TwoForkRun = { fromEntryId: "x1", pages: twoForkRuns.x1 };
const l1TwoEntryRun = { fromEntryId: "l1", pages: twoEntryRuns.l1 };
const l1TimedRun = { fromEntryId: "l1", pages: timedRuns.l1 };
const c2TwoEntryResult = { entryId: "c2", continuations: twoEntryContinuations.c2 };

// A single NON-preferred fork: forkCount 1 auto-chooses it, and it is not
// the current line, so checkout is legal (R1, g13-034).
const singleForkNonPreferredResult = {
  entryId: "c2",
  continuations: [continuation("l1", { label: "Lead intro", branchName: "feature/lead", entryCount: 2 })],
};

// The host navigated into the x1 fork: its entries are the primary line now
// and arrive on the new root spine (R2, g13-034).
const navigatedIntoForkPages = [
  page([
    { id: "x2", label: "Alt mix", position: "past", continuationCount: 0 },
    { id: "x1", label: "Alt intro", position: "past", continuationCount: 1 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 2 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1 },
  ]),
];

function rowSummary(): Array<{ kind: string; entry: string; depth: string }> {  return [...document.querySelectorAll("[data-row-kind]")].map((el) => ({
    kind: el.getAttribute("data-row-kind") ?? "",
    entry: el.getAttribute("data-row-entry") ?? "",
    depth: el.getAttribute("data-depth") ?? "",
  }));
}

/**
 * The fork row's three actions live behind one … menu now, so a test cannot
 * click them directly. These open the menu if it is closed — calling twice in
 * a row must not toggle it shut — and resolve an item by label.
 */
async function openForkMenu(): Promise<void> {
  if (document.querySelector('[role="menu"]') !== null) {
    return;
  }
  const trigger = screen.getAllByRole("button", { name: /^(Fork actions|Actions for )/ })[0];
  await fireEvent.click(trigger);
}

/** The menu item, without activating it — for enabled/disabled assertions. */
async function forkActionItem(name: string | RegExp): Promise<HTMLElement> {
  await openForkMenu();
  return screen.findByRole("menuitem", { name });
}

/** Opens the menu and activates the named action. */
async function runForkAction(name: string | RegExp): Promise<void> {
  await fireEvent.click(await forkActionItem(name));
}

function rowByEntry(entryId: string): HTMLElement {
  const row = document.querySelector(`[data-row-kind="entry"][data-row-entry="${entryId}"]`) as HTMLElement;
  if (row === null) {
    throw new Error(`no entry row for ${entryId}`);
  }
  return row;
}

describe("HistoryCenter (react)", () => {
  it("renders the undo/list/redo cluster with enablement from canUndo/canRedo and busy", () => {
    render(<HistoryCenter canUndo canRedo={false} busy={false} />);

    expect(screen.getByRole("button", { name: "Undo" }).hasAttribute("disabled")).toBe(false);
    expect(screen.getByRole("button", { name: "Redo" }).hasAttribute("disabled")).toBe(true);
  });

  it("disables undo/redo while busy even when the host can undo", () => {
    render(<HistoryCenter canUndo busy />);

    expect(screen.getByRole("button", { name: "Undo" }).hasAttribute("disabled")).toBe(true);
  });

  it("opens the popover and navigates a spine entry with the host's own branch (null)", () => {
    const onNavigateEntry = vi.fn();
    render(<HistoryCenter pages={twoForkPages} onNavigateEntry={onNavigateEntry} />);

    fireEvent.click(screen.getByRole("button", { name: "History" }));
    expect(screen.getByRole("dialog", { name: "History" })).toBeTruthy();

    fireEvent.click(screen.getByRole("button", { name: "Committed mix 1" }));

    // Spine rows carry no branch — the host's own branch is null (never an
    // ancestor or another branch's divergence entry).
    expect(onNavigateEntry).toHaveBeenCalledWith(null, "c1");
  });

  it("renders no fork affordance when forkCount is 0 — the entry is inert", () => {
    render(<HistoryCenter pages={twoForkPages} defaultOpen />);

    // c1 (continuationCount 1) and c3 (continuationCount 0) have forkCount 0:
    // no fork icon, no badge, no chevron, no picker row.
    expect(rowByEntry("c1").querySelector('[data-part="fork-disclosure"]')).toBeNull();
    expect(rowByEntry("c3").querySelector('[data-part="fork-disclosure"]')).toBeNull();
    expect(rowByEntry("c1").querySelector('[data-part="fork-badge"]')).toBeNull();
    expect(rowByEntry("c3").querySelector('[data-part="fork-badge"]')).toBeNull();
    expect(document.querySelector('[data-part="picker"]')).toBeNull();
    expect(rowByEntry("c1").getAttribute("data-fork-count")).toBe("0");
    expect(rowByEntry("c3").getAttribute("data-fork-count")).toBe("0");
  });

  it("shows icon + chevron without badge or picker at forkCount 1, auto-choosing the single fork", () => {
    const onLoadContinuations = vi.fn();
    const onLoadContinuationRun = vi.fn();
    const { rerender } = render(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        onLoadContinuations={onLoadContinuations}
        onLoadContinuationRun={onLoadContinuationRun}
      />,
    );

    const c2 = rowByEntry("c2");
    expect(c2.getAttribute("data-fork-count")).toBe("1");
    const disclosure = c2.querySelector('[data-part="fork-disclosure"]') as HTMLElement;
    expect(disclosure).toBeTruthy();
    // forkCount 1: a fork icon and a chevron, no counter badge.
    expect(c2.querySelector('[data-part="fork-badge"]')).toBeNull();
    expect(disclosure.getAttribute("aria-label")).toBe("Show 1 continuation");

    fireEvent.click(disclosure);

    expect(onLoadContinuations).toHaveBeenCalledWith("c2");

    // The host feeds the continuations back; the single fork is auto-chosen —
    // no picker row ever appears, and the run load is requested.
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        onLoadContinuations={onLoadContinuations}
        onLoadContinuationRun={onLoadContinuationRun}
        continuationsResult={l1Result}
      />,
    );
    expect(onLoadContinuationRun).toHaveBeenCalledWith("l1");

    // b033 R3: one row shape serves both fork counts. A single fork still
    // gets the picker row, with the select disabled.
    const singlePicker = document.querySelector('[data-part="picker"]');
    expect(singlePicker).toBeTruthy();
    expect((singlePicker?.querySelector(".poodle-select__trigger") as HTMLButtonElement | null)?.disabled).toBe(true);

    // The run renders at depth 1 once the host feeds its pages back.
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        onLoadContinuations={onLoadContinuations}
        onLoadContinuationRun={onLoadContinuationRun}
        continuationsResult={l1Result}
        runResult={l1NestedRun}
      />,
    );
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

  it("single fork: the Select is disabled but Checkout and Rename stay live on their own gates (R1)", async () => {
    const onCheckoutContinuation = vi.fn();
    const { rerender } = render(
      <HistoryCenter pages={singleForkPages} defaultOpen onCheckoutContinuation={onCheckoutContinuation} />,
    );

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        onCheckoutContinuation={onCheckoutContinuation}
        continuationsResult={singleForkNonPreferredResult}
      />,
    );
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        onCheckoutContinuation={onCheckoutContinuation}
        continuationsResult={singleForkNonPreferredResult}
        runResult={l1TwoEntryRun}
      />,
    );

    // Nothing to choose between: the Select is disabled.
    const picker = document.querySelector('[data-part="picker"]') as HTMLElement;
    expect((picker?.querySelector(".poodle-select__trigger") as HTMLButtonElement | null)?.disabled).toBe(
      true,
    );

    // The actions menu lives on its own gates (R1): the auto-chosen single
    // fork counts as picked, so a non-preferred one can be checked out and
    // renamed — the row's disabled signal never reaches the menu.
    expect((await forkActionItem("Checkout")).hasAttribute("disabled")).toBe(false);
    expect((await forkActionItem("Rename")).hasAttribute("disabled")).toBe(false);

    // And checkout is real, not decorative: the command names the
    // auto-chosen fork and clears the disclosure.
    await runForkAction("Checkout");
    expect(onCheckoutContinuation).toHaveBeenCalledWith("l1");
    expect(document.querySelector('[data-part="picker"]')).toBeNull();
  });

  it("single fork: checkout stays disabled when the auto-chosen fork is the preferred one (R1)", async () => {
    const { rerender } = render(<HistoryCenter pages={singleForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(<HistoryCenter pages={singleForkPages} defaultOpen continuationsResult={l1Result} />);
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l1Result}
        runResult={l1NestedRun}
      />,
    );

    // The picked.preferred gate, not the row gate: the single fork is the
    // current line, so checkout stays disabled — rename is still live.
    expect((await forkActionItem("Checkout")).hasAttribute("disabled")).toBe(true);
    expect((await forkActionItem("Rename")).hasAttribute("disabled")).toBe(false);
  });

  it("supplies root pages containing an open level's run without duplicating any row (R2)", () => {
    const { rerender } = render(<HistoryCenter pages={twoForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(<HistoryCenter pages={twoForkPages} defaultOpen continuationsResult={c2Result} />);
    rerender(
      <HistoryCenter pages={twoForkPages} defaultOpen continuationsResult={c2Result} runResult={x1TwoForkRun} />,
    );

    // R3 selected the current fork (x1); its run renders below the picker.
    expect(rowByEntry("x1")).toBeTruthy();

    // The host navigated into the fork: x1/x2 arrive on the new root spine,
    // the open level untouched. The stale level's run must not splice again.
    rerender(
      <HistoryCenter
        pages={navigatedIntoForkPages}
        defaultOpen
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    const ids = [...document.querySelectorAll('[data-row-kind="entry"]')].map(
      (el) => el.getAttribute("data-row-entry"),
    );
    expect(ids.filter((id, i) => ids.indexOf(id) !== i)).toEqual([]);
  });

  it("renders one badge reading 2 and a persistent Select picker at forkCount 2, distinct from a fork off a fork", async () => {
    const onNavigateEntry = vi.fn();
    const { rerender } = render(
      <HistoryCenter pages={twoForkPages} defaultOpen onNavigateEntry={onNavigateEntry} />,
    );

    const c2 = rowByEntry("c2");
    expect(c2.getAttribute("data-fork-count")).toBe("2");
    const badge = c2.querySelector('[data-part="fork-badge"]') as HTMLElement;
    expect(badge?.textContent).toBe("2");

    fireEvent.click(c2.querySelector('[data-part="fork-disclosure"]') as HTMLElement);

    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onNavigateEntry={onNavigateEntry}
        continuationsResult={c2Result}
      />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onNavigateEntry={onNavigateEntry}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

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
    expect(document.querySelector('[data-part="picker-select"]')).toBeTruthy();
    expect(await forkActionItem("Checkout")).toBeTruthy();

    // R1: the pencil sits between the Select and checkout, renames the
    // selection, and no "Current" badge marks the trigger (R4a).
    const trigger = screen.getByRole("button", { name: "Continuations" });
    expect(trigger.textContent).toContain("Alt intro");
    expect(trigger.textContent).toContain("feature/alt");
    expect(trigger.textContent).not.toContain("Current");

    const select = document.querySelector('[data-part="picker-select"] .poodle-select') as HTMLElement;
    // The three buttons are one … menu now, so the row order is Select then
    // menu; the action order lives inside the menu instead.
    const actions = document.querySelector("[data-part='picker-actions']") as HTMLElement;
    expect(select.compareDocumentPosition(actions) & Node.DOCUMENT_POSITION_FOLLOWING).not.toBe(0);
    await openForkMenu();
    const labels = screen.getAllByRole("menuitem").map((el) => el.textContent?.trim());
    expect(labels).toEqual(["Rename", "Checkout"]);
    expect(document.querySelector('[data-part="current-badge"]')).toBeNull();

    // Opening the select offers exactly the two forks with the same anatomy.
    fireEvent.click(trigger);
    expect(screen.getByRole("option", { name: /Lead intro feature\/lead/ })).toBeTruthy();
    expect(screen.getByRole("option", { name: /Alt intro feature\/alt/ })).toBeTruthy();
    expect(document.querySelector('[data-part="current-badge"]')).toBeNull();
  });

  it("re-requests continuations when the host supplies pages containing the open run", async () => {
    // g13-034 gave the machine a stale-level reconcile but nothing fired it:
    // the trigger is a pages prop change, which dispatches no event. The
    // level derived as stale, rendered "not-yet-loaded", and stayed there.
    const onLoadContinuations = vi.fn();
    const props = { onLoadContinuations };
    const { rerender } = render(<HistoryCenter pages={twoForkPages} defaultOpen {...props} />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter pages={twoForkPages} defaultOpen {...props} continuationsResult={c2Result} />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        {...props}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );
    expect(onLoadContinuations).toHaveBeenCalledTimes(1);

    // The host navigates into the fork: x1/x2 are the primary line now.
    const movedPages = [
      page([
        { id: "x2", label: "Alt mix", position: "current", continuationCount: 0 },
        { id: "x1", label: "Alt intro", position: "past", continuationCount: 1 },
        { id: "c2", label: "Arranged intro", position: "past", continuationCount: 3 },
        { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1 },
      ]),
    ];
    rerender(
      <HistoryCenter
        pages={movedPages}
        defaultOpen
        {...props}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    // The level is stale. It must ask the host for continuations again rather
    // than sit on "not-yet-loaded" until the operator closes and reopens.
    expect(onLoadContinuations).toHaveBeenCalledTimes(2);
    expect(onLoadContinuations).toHaveBeenLastCalledWith("c2");
  });

  it("omits Delete when the host supplies no callback (b033 R4 opt-in)", async () => {
    const { rerender } = render(<HistoryCenter pages={twoForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(<HistoryCenter pages={twoForkPages} defaultOpen continuationsResult={c2Result} />);
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    await openForkMenu();

    // Absent callback, absent item — never a disabled stand-in.
    expect(screen.getAllByRole("menuitem").map((el) => el.textContent?.trim())).toEqual([
      "Rename",
      "Checkout",
    ]);
  });

  it("Delete asks first: the command waits for the confirmation", async () => {
    const onDeleteContinuation = vi.fn();
    const { rerender } = render(
      <HistoryCenter pages={twoForkPages} defaultOpen onDeleteContinuation={onDeleteContinuation} />,
    );

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onDeleteContinuation={onDeleteContinuation}
        continuationsResult={c2Result}
      />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onDeleteContinuation={onDeleteContinuation}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    await runForkAction("Delete");

    // Picking the menu item opens the dialog and emits nothing on its own:
    // one click must not destroy history.
    expect(onDeleteContinuation).not.toHaveBeenCalled();
    const dialog = await screen.findByRole("alertdialog");
    expect(dialog.textContent).toContain("Delete this fork?");
    // It names the fork the Select currently shows, so the operator can see
    // which one is about to go.
    expect(dialog.textContent).toContain("Alt intro");
  });

  it("confirming the delete emits once, for the selected fork", async () => {
    const onDeleteContinuation = vi.fn();
    const onCheckoutContinuation = vi.fn();
    const props = { onDeleteContinuation, onCheckoutContinuation };
    const { rerender } = render(<HistoryCenter pages={twoForkPages} defaultOpen {...props} />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter pages={twoForkPages} defaultOpen {...props} continuationsResult={c2Result} />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        {...props}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    // Move the Select off the current fork, so the assertion proves the
    // command follows the selection rather than the preferred fork.
    fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        {...props}
        continuationsResult={c2Result}
        runResult={l1TwoForkRun}
      />,
    );

    await runForkAction("Delete");
    // AlertDialog awaits onConfirm before it closes, so the click leaves state
    // updates in flight past fireEvent's own act() window.
    const confirm = await screen.findByRole("button", { name: "Delete" });
    await act(async () => {
      fireEvent.click(confirm);
    });

    expect(onDeleteContinuation).toHaveBeenCalledTimes(1);
    expect(onDeleteContinuation).toHaveBeenCalledWith("l1");
    expect(onCheckoutContinuation).not.toHaveBeenCalled();
  });

  it("re-requests continuations after a delete so the deleted run stops rendering", async () => {
    // Field report: after a delete the fork's entries stayed in the list,
    // indented, and clicking one errored until the popover was reopened. The
    // stale rule cannot catch it — a deleted fork never reaches the spine.
    const onDeleteContinuation = vi.fn();
    const onLoadContinuations = vi.fn();
    const props = { onDeleteContinuation, onLoadContinuations };
    const { rerender } = render(<HistoryCenter pages={twoForkPages} defaultOpen {...props} />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter pages={twoForkPages} defaultOpen {...props} continuationsResult={c2Result} />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        {...props}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );
    expect(onLoadContinuations).toHaveBeenCalledTimes(1);

    await runForkAction("Delete");
    const confirm = await screen.findByRole("button", { name: "Delete" });
    await act(async () => {
      fireEvent.click(confirm);
    });

    expect(onDeleteContinuation).toHaveBeenCalledTimes(1);
    expect(onLoadContinuations).toHaveBeenCalledTimes(2);
    expect(onLoadContinuations).toHaveBeenLastCalledWith("c2");
  });

  it("cancelling the delete emits nothing and leaves the history list open", async () => {
    const onDeleteContinuation = vi.fn();
    const { rerender } = render(
      <HistoryCenter pages={twoForkPages} defaultOpen onDeleteContinuation={onDeleteContinuation} />,
    );

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onDeleteContinuation={onDeleteContinuation}
        continuationsResult={c2Result}
      />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onDeleteContinuation={onDeleteContinuation}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    await runForkAction("Delete");
    fireEvent.click(await screen.findByRole("button", { name: "Cancel" }));

    expect(onDeleteContinuation).not.toHaveBeenCalled();
    expect(screen.queryByRole("alertdialog")).toBeNull();
    // The dialog is a nested dismiss layer, so closing it must not take the
    // popover with it (b031 ancestry).
    expect(document.querySelector('[data-part="list"]')).not.toBeNull();
  });

  it("selects the non-preferred fork: preview swaps below the select, checkout emits, never navigates", async () => {
    const onNavigateEntry = vi.fn();
    const onCheckoutContinuation = vi.fn();
    const { rerender } = render(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onNavigateEntry={onNavigateEntry}
        onCheckoutContinuation={onCheckoutContinuation}
      />,
    );

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onNavigateEntry={onNavigateEntry}
        onCheckoutContinuation={onCheckoutContinuation}
        continuationsResult={c2Result}
      />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onNavigateEntry={onNavigateEntry}
        onCheckoutContinuation={onCheckoutContinuation}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    // R3 selected the current fork (x1): checkout is disabled on it.
    expect((await forkActionItem("Checkout")).hasAttribute("disabled")).toBe(true);

    // Select the non-preferred fork: the entries below swap to its run and
    // the select stays — the pick commits nothing.
    fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onNavigateEntry={onNavigateEntry}
        onCheckoutContinuation={onCheckoutContinuation}
        continuationsResult={c2Result}
        runResult={l1TwoForkRun}
      />,
    );

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
    expect((await forkActionItem("Checkout")).hasAttribute("disabled")).toBe(false);

    await runForkAction("Checkout");

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
    expect(document.querySelector('[data-part="picker"]')).toBeNull();
  });

  it("selecting a fork emits no host operation — only the preview run loads", () => {
    const onCheckoutContinuation = vi.fn();
    const onLoadContinuationRun = vi.fn();
    const onNavigateEntry = vi.fn();
    const { rerender } = render(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onCheckoutContinuation={onCheckoutContinuation}
        onLoadContinuationRun={onLoadContinuationRun}
        onNavigateEntry={onNavigateEntry}
      />,
    );

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onCheckoutContinuation={onCheckoutContinuation}
        onLoadContinuationRun={onLoadContinuationRun}
        onNavigateEntry={onNavigateEntry}
        continuationsResult={c2Result}
      />,
    );
    onLoadContinuationRun.mockClear();

    fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));

    // The pick loaded l1's run (R2) and committed nothing: no checkout, no
    // navigation.
    expect(onLoadContinuationRun).toHaveBeenCalledWith("l1");
    expect(onCheckoutContinuation).not.toHaveBeenCalled();
    expect(onNavigateEntry).not.toHaveBeenCalled();
  });

  it("disables checkout while the selection is already the current fork", async () => {
    const { rerender } = render(<HistoryCenter pages={twoForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        continuationsResult={c2Result}
      />,
    );

    // R3 opened on the current fork (x1, preferred): checkout is disabled on
    // it. The badge is gone (R4a) — the disabled button carries the fact.
    expect((await forkActionItem("Checkout")).hasAttribute("disabled")).toBe(true);
    expect(document.querySelector('[data-part="current-badge"]')).toBeNull();

    // Picking the other fork enables checkout.
    fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));
    expect((await forkActionItem("Checkout")).hasAttribute("disabled")).toBe(false);
  });

  it("renames the selected fork from the picker through onRenameBranch", async () => {
    const onRenameBranch = vi.fn();
    const { rerender } = render(
      <HistoryCenter pages={twoForkPages} defaultOpen onRenameBranch={onRenameBranch} />,
    );

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter pages={twoForkPages} defaultOpen onRenameBranch={onRenameBranch} continuationsResult={c2Result} />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onRenameBranch={onRenameBranch}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    // R3 opened on the current fork (x1): the pencil targets whatever the
    // Select shows — x1's branch, not the anchor's and not some other fork.
    await runForkAction("Rename");

    // The input replaces the Select while renaming (R3).
    expect(screen.queryByRole("button", { name: "Continuations" })).toBeNull();
    const input = screen.getByRole("textbox", { name: "Rename branch feature/alt" }) as HTMLInputElement;
    fireEvent.change(input, { target: { value: "feature/alt-v2" } });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(onRenameBranch).toHaveBeenCalledWith("b-x1", "feature/alt-v2");
  });

  it("changing the Select then renaming targets the newly selected fork", async () => {
    const onRenameBranch = vi.fn();
    const { rerender } = render(
      <HistoryCenter pages={twoForkPages} defaultOpen onRenameBranch={onRenameBranch} />,
    );

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter pages={twoForkPages} defaultOpen onRenameBranch={onRenameBranch} continuationsResult={c2Result} />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        onRenameBranch={onRenameBranch}
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    // Preview the non-preferred fork, then rename it: the pencil follows the
    // selection — b-l1, not the preferred fork (b-x1) and not the anchor's.
    fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));

    await runForkAction("Rename");
    const input = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    fireEvent.change(input, { target: { value: "feature/lead-v2" } });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(onRenameBranch).toHaveBeenCalledWith("b-l1", "feature/lead-v2");
  });

  it("disables checkout while a rename is open", async () => {
    const { rerender } = render(<HistoryCenter pages={twoForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(<HistoryCenter pages={twoForkPages} defaultOpen continuationsResult={c2Result} />);
    rerender(
      <HistoryCenter pages={twoForkPages} defaultOpen continuationsResult={c2Result} runResult={x1TwoForkRun} />,
    );

    // Enable checkout by picking the non-preferred fork…
    fireEvent.click(screen.getByRole("button", { name: "Continuations" }));
    fireEvent.click(screen.getByRole("option", { name: /Lead intro feature\/lead/ }));
    expect((await forkActionItem("Checkout")).hasAttribute("disabled")).toBe(false);

    // …then a rename opens and checkout goes inert (R3).
    await runForkAction("Rename");
    expect((await forkActionItem("Checkout")).hasAttribute("disabled")).toBe(true);
  });

  it("cancelling a picker rename restores the Select and returns focus to the pencil", async () => {
    const { rerender } = render(<HistoryCenter pages={twoForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(<HistoryCenter pages={twoForkPages} defaultOpen continuationsResult={c2Result} />);
    rerender(
      <HistoryCenter pages={twoForkPages} defaultOpen continuationsResult={c2Result} runResult={x1TwoForkRun} />,
    );

    await runForkAction("Rename");

    const input = screen.getByRole("textbox", { name: "Rename branch feature/alt" }) as HTMLInputElement;
    fireEvent.keyDown(input, { key: "Escape" });

    // The Select is back and focus sits on the pencil (R3).
    expect(screen.getByRole("button", { name: "Continuations" })).toBeTruthy();
    expect(screen.queryByRole("textbox", { name: "Rename branch feature/alt" })).toBeNull();
    // Focus returns to the actions menu trigger — the control the rename was
    // opened from, now that the pencil is a menu item.
    const trigger = document.querySelector(
      "[data-part='picker-actions'] .poodle-menu__trigger",
    );
    expect(trigger).toBeTruthy();
    expect(document.activeElement).toBe(trigger);
  });

  it("renders a fork off a fork at the inner depth, never confusable with a picker", () => {
    const { rerender } = render(<HistoryCenter pages={singleForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l1Result}
      />,
    );
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l1Result}
        runResult={l1NestedRun}
      />,
    );
    fireEvent.click(rowByEntry("l2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l2Result}
        runResult={l1NestedRun}
      />,
    );
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l2Result}
        runResult={i1Run}
      />,
    );

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

  it("renders no fork affordance on a run's last entry", () => {
    const { rerender } = render(<HistoryCenter pages={singleForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l1Result}
      />,
    );
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l1Result}
        runResult={l1NestedRun}
      />,
    );

    // l3 is the run's last entry (continuationCount 0 → forkCount 0).
    const l3 = rowByEntry("l3");
    expect(l3.getAttribute("data-fork-count")).toBe("0");
    expect(l3.querySelector('[data-part="fork-disclosure"]')).toBeNull();
  });

  it("shows the opened region on the run's first entry: branch name, count, derived time", () => {
    const { rerender } = render(<HistoryCenter pages={timedSingleForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={timedSingleForkPages}
        defaultOpen
        continuationsResult={c2TwoEntryResult}
      />,
    );
    rerender(
      <HistoryCenter
        pages={timedSingleForkPages}
        defaultOpen
        continuationsResult={c2TwoEntryResult}
        runResult={l1TimedRun}
      />,
    );

    const picker = document.querySelector('[data-part="picker"]') as HTMLElement;
    expect(picker).toBeTruthy();
    expect(picker.querySelector(".poodle-history-center__picker-option-branch")?.textContent?.trim()).toBe("feature/lead");
    expect(picker.querySelector(".poodle-history-center__picker-option-meta")?.textContent).toBe("2 entries · 20m ago");
    // The run header belongs to the run's first entry only.
    expect(rowByEntry("l2").querySelector('[data-part="run-header"]')).toBeNull();
  });

  it("renders a caption with no time when recordedAtMs is absent — never Invalid Date", () => {
    const { rerender } = render(<HistoryCenter pages={singleForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={c2TwoEntryResult}
      />,
    );
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={c2TwoEntryResult}
        runResult={l1TwoEntryRun}
      />,
    );

    const picker = document.querySelector('[data-part="picker"]') as HTMLElement;
    expect(picker.querySelector(".poodle-history-center__picker-option-meta")?.textContent).toBe("2 entries");
    expect(picker.textContent).not.toContain("ago");
    expect(document.body.textContent).not.toContain("Invalid Date");
  });

  it("renders rows at depth 3 and depth 5, neither clamped by a depth cap", () => {
    const { rerender } = render(<HistoryCenter pages={deepPages} defaultOpen />);

    const open = (entryId: string, continuations: HistoryContinuation[]) => {
      fireEvent.click(rowByEntry(entryId).querySelector('[data-part="fork-disclosure"]') as HTMLElement);
      // Two sequential rerenders sharing ONE result reference: the
      // CONTINUATIONS_LOADED dispatch must commit (auto-choose) before the
      // RUN_LOADED dispatch reads the machine context.
      const result = { entryId, continuations };
      rerender(<HistoryCenter pages={deepPages} defaultOpen continuationsResult={result} />);
      rerender(
        <HistoryCenter
          pages={deepPages}
          defaultOpen
          continuationsResult={result}
          runResult={{ fromEntryId: continuations[0].entryId, pages: deepRuns[continuations[0].entryId] }}
        />,
      );
    };

    open("r2", deepContinuations.r2);
    open("a2", deepContinuations.a2);
    open("b2", deepContinuations.b2);
    open("c2", deepContinuations.c2);
    open("d2", deepContinuations.d2);

    const depth3 = rowByEntry("c2");
    const depth5 = rowByEntry("e2");
    expect(depth3.getAttribute("data-depth")).toBe("3");
    expect(depth3.getAttribute("aria-level")).toBe("4");
    expect(depth5.getAttribute("data-depth")).toBe("5");
    expect(depth5.getAttribute("aria-level")).toBe("6");
    // Neither clamped: depth 5 indents five steps, depth 3 indents three.
    expect((depth5 as HTMLElement).style.getPropertyValue("--poodle-history-center-depth")).toBe("5");
    expect((depth3 as HTMLElement).style.getPropertyValue("--poodle-history-center-depth")).toBe("3");
  });

  it("traverses visible rows linearly in visual order, wrapping at the ends", () => {
    const { rerender } = render(<HistoryCenter pages={twoForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        continuationsResult={c2Result}
      />,
    );
    rerender(
      <HistoryCenter
        pages={twoForkPages}
        defaultOpen
        continuationsResult={c2Result}
        runResult={x1TwoForkRun}
      />,
    );

    // DISCLOSE clamps machine focus to the anchor; Home starts the traversal
    // from the first visible row. R3 loaded the current fork's run, so the
    // rows are c1, c2, picker, x1, x2, c3. (The popover's initial focus does
    // not run in jsdom, so the first row is focused explicitly.)
    const first = screen.getByRole("button", { name: "Committed mix 1" });
    first.focus();
    fireEvent.keyDown(first, { key: "Home" });
    expect(document.activeElement).toBe(first);

    fireEvent.keyDown(screen.getByRole("button", { name: "Committed mix 1" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Arranged intro" }));

    fireEvent.keyDown(screen.getByRole("button", { name: "Arranged intro" }), { key: "ArrowDown" });
    // The picker row is a roving stop too — focus lands on the picker.
    expect(document.activeElement?.getAttribute("data-part")).toBe("picker");

    fireEvent.keyDown(document.activeElement as HTMLElement, { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Alt intro" }));

    fireEvent.keyDown(screen.getByRole("button", { name: "Alt intro" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Alt mix" }));

    fireEvent.keyDown(screen.getByRole("button", { name: "Alt mix" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Current draft" }));

    // Wraps back to the first row.
    fireEvent.keyDown(screen.getByRole("button", { name: "Current draft" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Committed mix 1" }));

    // Home / End land on the boundaries.
    fireEvent.keyDown(screen.getByRole("button", { name: "Committed mix 1" }), { key: "End" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Current draft" }));
    fireEvent.keyDown(screen.getByRole("button", { name: "Current draft" }), { key: "Home" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Committed mix 1" }));

    // The Select owns its keys: ArrowDown on the picker's trigger opens its
    // listbox and never moves roving focus.
    const selectTrigger = screen.getByRole("button", { name: "Continuations" });
    selectTrigger.focus();
    fireEvent.keyDown(selectTrigger, { key: "ArrowDown" });
    expect(document.activeElement).toBe(selectTrigger);
    expect(screen.getByRole("option", { name: /Lead intro feature\/lead/ })).toBeTruthy();
  });

  it("commits inline rename through onRenameBranch and cancels on Escape", async () => {
    const onRenameBranch = vi.fn();
    const { rerender } = render(
      <HistoryCenter pages={singleForkPages} defaultOpen onRenameBranch={onRenameBranch} />,
    );

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        onRenameBranch={onRenameBranch}
        continuationsResult={l1Result}
      />,
    );
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        onRenameBranch={onRenameBranch}
        continuationsResult={l1Result}
        runResult={l1NestedRun}
      />,
    );

    await runForkAction("Rename");

    const input = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    fireEvent.change(input, { target: { value: "feature/lead-v2" } });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(onRenameBranch).toHaveBeenCalledWith("b-l1", "feature/lead-v2");

    // Escape cancels without emitting.
    await runForkAction("Rename");
    const second = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    fireEvent.keyDown(second, { key: "Escape" });

    expect(onRenameBranch).toHaveBeenCalledTimes(1);
    expect(screen.queryByRole("textbox", { name: "Rename branch feature/lead" })).toBeNull();
  });

  it("shows a rejection code as the machine's display copy, dismissible", () => {
    const { unmount } = render(<HistoryCenter pages={twoForkPages} defaultOpen rejection="AlreadyAtTarget" />);

    expect(screen.getByText("Already at the requested target")).toBeTruthy();

    fireEvent.click(screen.getByRole("button", { name: "Dismiss" }));
    expect(screen.queryByText("Already at the requested target")).toBeNull();

    unmount();
    render(<HistoryCenter pages={twoForkPages} defaultOpen rejection="UnknownEntry" />);

    expect(screen.getByText("Entry does not exist")).toBeTruthy();
  });

  it("renders the empty message, the loading row and the failed row when there are no rows", () => {
    const { unmount } = render(<HistoryCenter defaultOpen />);

    expect(screen.getByText("No history entries yet.")).toBeTruthy();
    expect(document.querySelector("[data-row-kind]")).toBeNull();

    unmount();
    render(<HistoryCenter status="loading" defaultOpen />);

    expect(screen.getByText("Loading history…")).toBeTruthy();

    unmount();
    render(<HistoryCenter status="failed" statusMessage="Authority unreachable" defaultOpen />);

    expect(screen.getByText("Authority unreachable")).toBeTruthy();
  });

  // g14.007 retained regression: the status row only rendered on the empty
  // branch, so a history that loaded entries and then failed to load more
  // showed nothing — no message, no spinner, a list that quietly stopped
  // growing. Both web shells carried it.
  it("still reports loading and failed status when rows are already listed", () => {
    const { unmount } = render(<HistoryCenter pages={twoForkPages} status="loading" defaultOpen />);

    expect(document.querySelectorAll("[data-row-kind]").length).toBeGreaterThan(0);
    expect(screen.getByText("Loading history…")).toBeTruthy();

    unmount();
    render(
      <HistoryCenter
        pages={twoForkPages}
        status="failed"
        statusMessage="Authority unreachable"
        defaultOpen
      />,
    );

    expect(document.querySelectorAll("[data-row-kind]").length).toBeGreaterThan(0);
    expect(screen.getByText("Authority unreachable")).toBeTruthy();
  });

  it("exposes depth to assistive tech through aria-level on every row", () => {
    const { rerender } = render(<HistoryCenter pages={singleForkPages} defaultOpen />);

    fireEvent.click(rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l1Result}
      />,
    );
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l1Result}
        runResult={l1NestedRun}
      />,
    );
    fireEvent.click(rowByEntry("l2").querySelector('[data-part="fork-disclosure"]') as HTMLElement);
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l2Result}
        runResult={l1NestedRun}
      />,
    );
    rerender(
      <HistoryCenter
        pages={singleForkPages}
        defaultOpen
        continuationsResult={l2Result}
        runResult={i1Run}
      />,
    );

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

describe("HistoryCenter (react) package types", () => {
  it("exports the live HistoryEntry shape, not v2 branchCount", () => {
    const entry: HistoryEntry = {
      id: "e1",
      label: "Edit",
      position: "current",
      continuationCount: 0,
    };
    expect(entry.continuationCount).toBe(0);
  });
});
