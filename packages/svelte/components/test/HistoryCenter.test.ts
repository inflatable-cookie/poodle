import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import HistoryCenter from "../src/HistoryCenter.svelte";

const linearBranches = [{ id: "b-main", name: "main", current: true }];

const linearPaths = {
  "b-main": [
    { id: "c1", label: "Committed mix 1", position: "past" as const },
    { id: "c2", label: "Arranged intro", position: "past" as const },
    { id: "c3", label: "Current draft", position: "current" as const },
  ],
};

const forkBranches = [
  { id: "b-main", name: "main", current: true },
  { id: "b-lead", name: "feature/lead" },
];

const forkPaths = {
  "b-main": [
    { id: "c1", label: "Committed mix 1", position: "past" as const },
    { id: "c2", label: "Arranged intro", position: "past" as const },
    { id: "c3", label: "Current draft", position: "current" as const },
  ],
  "b-lead": [
    { id: "c1", label: "Committed mix 1", position: "past" as const },
    { id: "c2", label: "Arranged intro", position: "past" as const },
    { id: "l1", label: "Lead intro", position: "past" as const },
    { id: "l2", label: "Lead mix", position: "past" as const },
  ],
};

// The fork run's head (l2) sits 20 minutes before the data's newest entry
// (c3), so its caption derives "20m ago" from supplied data only (D2).
const timedPaths = {
  "b-main": [
    { id: "c1", label: "Committed mix 1", position: "past" as const, recordedAtMs: 0 },
    { id: "c2", label: "Arranged intro", position: "past" as const, recordedAtMs: 600_000 },
    { id: "c3", label: "Current draft", position: "current" as const, recordedAtMs: 3_600_000 },
  ],
  "b-lead": [
    { id: "c1", label: "Committed mix 1", position: "past" as const, recordedAtMs: 0 },
    { id: "c2", label: "Arranged intro", position: "past" as const, recordedAtMs: 600_000 },
    { id: "l1", label: "Lead intro", position: "past" as const, recordedAtMs: 1_200_000 },
    { id: "l2", label: "Lead mix", position: "past" as const, recordedAtMs: 2_400_000 },
  ],
};

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

  it("opens the popover and navigates a spine entry with its own branch and entry", async () => {
    const onNavigateEntry = vi.fn();
    render(HistoryCenter, {
      props: { branches: linearBranches, paths: linearPaths, onNavigateEntry },
    });

    await fireEvent.click(screen.getByRole("button", { name: "History" }));

    expect(screen.getByRole("dialog", { name: "History" })).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Committed mix 1" }));

    expect(onNavigateEntry).toHaveBeenCalledWith("b-main", "c1");
  });

  it("reports the run's own branch for an entry inside a fork run", async () => {
    const onNavigateEntry = vi.fn();
    render(HistoryCenter, {
      props: { branches: forkBranches, paths: forkPaths, onNavigateEntry, defaultOpen: true },
    });

    await fireEvent.click(screen.getByRole("button", { name: "Lead mix" }));

    expect(onNavigateEntry).toHaveBeenCalledWith("b-lead", "l2");
    expect(onNavigateEntry).not.toHaveBeenCalledWith("b-main", "l2");
  });

  it("renders a run caption at the run's depth with its own branch label, and never navigates", async () => {
    const onNavigateEntry = vi.fn();
    render(HistoryCenter, {
      props: { branches: forkBranches, paths: forkPaths, onNavigateEntry, defaultOpen: true },
    });

    const caption = document.querySelector('[data-part="caption"]') as HTMLElement;
    expect(caption).toBeTruthy();
    expect(caption.getAttribute("data-depth")).toBe("1");
    expect(caption.getAttribute("aria-level")).toBe("2");
    expect(caption.querySelector(".poodle-history-center__caption-name")?.textContent).toBe("feature/lead");
    expect(caption.querySelector('[data-lane="caption"]')).toBeTruthy();

    const renameButton = screen.getByRole("button", { name: "Rename feature/lead" });
    renameButton.focus();
    // Native Enter/Space activation of a focused caption resolves to this
    // click: it opens rename, it never navigates.
    await fireEvent.click(renameButton);

    expect(onNavigateEntry).not.toHaveBeenCalled();
    expect(screen.getByRole("textbox", { name: "Rename branch feature/lead" })).toBeTruthy();
  });

  it("keeps a linear-only tree plain: trunk lanes, no captions, no elbows", async () => {
    render(HistoryCenter, { props: { branches: linearBranches, paths: linearPaths, defaultOpen: true } });

    expect(document.querySelector('[data-part="caption"]')).toBeNull();
    expect(document.querySelector('[data-lane="trunk"]')).toBeTruthy();
    expect(document.querySelector('[data-lane="elbow"]')).toBeNull();
    expect(screen.getByRole("button", { name: "Committed mix 1" })).toBeTruthy();
  });

  it("draws lane metadata for a fork: elbow, continue, end, and a single-entry run", async () => {
    const singleBranches = [
      { id: "b-main", name: "main", current: true },
      { id: "b-one", name: "feature/one" },
    ];
    const singlePaths = {
      "b-main": [
        { id: "c1", label: "Committed mix 1", position: "past" as const },
        { id: "c2", label: "Arranged intro", position: "past" as const },
      ],
      "b-one": [{ id: "c1", label: "Committed mix 1", position: "past" as const }, { id: "x1", label: "One shot", position: "past" as const }],
    };

    render(HistoryCenter, { props: { branches: forkBranches, paths: forkPaths, defaultOpen: true } });

    const laneOf = (label: string): string[] => {
      const li = screen.getByRole("button", { name: label }).closest("li") as HTMLElement;
      return [...li.querySelectorAll("[data-lane]")].map((el) => el.getAttribute("data-lane") ?? "");
    };

    expect(laneOf("Committed mix 1")).toEqual(["trunk"]);
    expect(laneOf("Lead intro")).toEqual(["ancestor", "elbow"]);
    expect(laneOf("Lead mix")).toEqual(["ancestor", "end"]);

    const { unmount } = render(HistoryCenter, { props: { branches: singleBranches, paths: singlePaths, defaultOpen: true } });
    expect(laneOf("One shot")).toEqual(["ancestor", "single"]);
    unmount();
  });

  it("saturates indentation at depth 3 while the run's branch stays true", async () => {
    const deepBranches = [
      { id: "b-main", name: "main", current: true },
      { id: "b-1", name: "f1" },
      { id: "b-2", name: "f2" },
      { id: "b-3", name: "f3" },
      { id: "b-4", name: "f4" },
    ];
    const deepPaths = {
      "b-main": [{ id: "e1", label: "Root", position: "past" as const }, { id: "e2", label: "Shared", position: "past" as const }],
      "b-1": [{ id: "e1", label: "Root", position: "past" as const }, { id: "e2", label: "Shared", position: "past" as const }, { id: "a1", label: "Level 1", position: "past" as const }],
      "b-2": [{ id: "e1", label: "Root", position: "past" as const }, { id: "e2", label: "Shared", position: "past" as const }, { id: "a1", label: "Level 1", position: "past" as const }, { id: "b1", label: "Level 2", position: "past" as const }],
      "b-3": [{ id: "e1", label: "Root", position: "past" as const }, { id: "e2", label: "Shared", position: "past" as const }, { id: "a1", label: "Level 1", position: "past" as const }, { id: "b1", label: "Level 2", position: "past" as const }, { id: "c1", label: "Level 3", position: "past" as const }],
      "b-4": [{ id: "e1", label: "Root", position: "past" as const }, { id: "e2", label: "Shared", position: "past" as const }, { id: "a1", label: "Level 1", position: "past" as const }, { id: "b1", label: "Level 2", position: "past" as const }, { id: "c1", label: "Level 3", position: "past" as const }, { id: "d1", label: "Level 4", position: "past" as const }],
    };
    const onNavigateEntry = vi.fn();

    render(HistoryCenter, {
      props: { branches: deepBranches, paths: deepPaths, onNavigateEntry, defaultOpen: true },
    });

    const level4 = screen.getByRole("button", { name: "Level 4" }).closest("li") as HTMLElement;
    expect(level4.getAttribute("data-depth")).toBe("3");
    // The run's branch stays true — only indentation saturates.
    await fireEvent.click(screen.getByRole("button", { name: "Level 4" }));
    expect(onNavigateEntry).toHaveBeenCalledWith("b-4", "d1");
  });

  it("traverses spine and runs linearly in visual order, wrapping at the ends", async () => {
    render(HistoryCenter, { props: { branches: forkBranches, paths: forkPaths, defaultOpen: true } });

    const first = screen.getByRole("button", { name: "Committed mix 1" });
    first.focus();
    await fireEvent.keyDown(first, { key: "ArrowDown" });

    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Arranged intro" }));

    await fireEvent.keyDown(screen.getByRole("button", { name: "Arranged intro" }), { key: "ArrowDown" });

    // The run caption sits between the shared spine entry and the run's own
    // entries — focus lands on its rename button.
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Rename feature/lead" }));

    await fireEvent.keyDown(screen.getByRole("button", { name: "Rename feature/lead" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Lead intro" }));

    await fireEvent.keyDown(screen.getByRole("button", { name: "Lead intro" }), { key: "ArrowDown" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Lead mix" }));

    await fireEvent.keyDown(screen.getByRole("button", { name: "Lead mix" }), { key: "ArrowDown" });
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
    render(HistoryCenter, {
      props: { branches: forkBranches, paths: forkPaths, onRenameBranch, defaultOpen: true },
    });

    await fireEvent.click(screen.getByRole("button", { name: "Rename feature/lead" }));

    const input = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "feature/lead-v2" } });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(onRenameBranch).toHaveBeenCalledWith("b-lead", "feature/lead-v2");

    // Escape cancels without emitting.
    await fireEvent.click(screen.getByRole("button", { name: "Rename feature/lead" }));
    const second = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    await fireEvent.keyDown(second, { key: "Escape" });

    expect(onRenameBranch).toHaveBeenCalledTimes(1);
    expect(screen.queryByRole("textbox", { name: "Rename branch feature/lead" })).toBeNull();
  });

  it("derives a caption's relative time from its run's most recent entry only when supplied", async () => {
    const { unmount } = render(HistoryCenter, {
      props: { branches: forkBranches, paths: timedPaths, defaultOpen: true },
    });

    expect(screen.getByText(/20m ago/)).toBeTruthy();
    expect(screen.getByText("2 entries · 20m ago")).toBeTruthy();

    unmount();
    render(HistoryCenter, { props: { branches: forkBranches, paths: forkPaths, defaultOpen: true } });

    expect(screen.queryByText(/ago/)).toBeNull();
    expect(screen.getByText("2 entries")).toBeTruthy();
  });

  it("shows rejection as a visible dismissible notice", async () => {
    render(HistoryCenter, {
      props: { branches: forkBranches, paths: forkPaths, defaultOpen: true, rejection: "Branch name is taken" },
    });

    expect(screen.getByText("Branch name is taken")).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Dismiss" }));

    expect(screen.queryByText("Branch name is taken")).toBeNull();
  });

  it("offers load-more with the stitched entry-row count as the entries offset", async () => {
    const onLoadMoreEntries = vi.fn();
    const onLoadMoreBranches = vi.fn();
    render(HistoryCenter, {
      props: {
        branches: forkBranches,
        paths: forkPaths,
        hasMoreEntries: true,
        hasMoreBranches: true,
        onLoadMoreEntries,
        onLoadMoreBranches,
        defaultOpen: true,
      },
    });

    await fireEvent.click(screen.getByRole("button", { name: "Load more entries" }));
    // 5 stitched entry rows: 3 spine + 2 run.
    expect(onLoadMoreEntries).toHaveBeenCalledWith(5);

    await fireEvent.click(screen.getByRole("button", { name: "Load more branches" }));
    expect(onLoadMoreBranches).toHaveBeenCalledWith(2);
  });

  it("renders the empty message and the loading row when the tree has no rows", async () => {
    const { unmount } = render(HistoryCenter, { props: { defaultOpen: true } });

    expect(screen.getByText("No history entries yet.")).toBeTruthy();
    expect(document.querySelector('[data-part="entry"]')).toBeNull();

    unmount();
    render(HistoryCenter, { props: { status: "loading", defaultOpen: true } });

    expect(screen.getByText("Loading history…")).toBeTruthy();
  });

  it("exposes depth to assistive tech through aria-level on every row", async () => {
    render(HistoryCenter, { props: { branches: forkBranches, paths: forkPaths, defaultOpen: true } });

    const caption = document.querySelector('[data-part="caption"]') as HTMLElement;
    const runEntry = screen.getByRole("button", { name: "Lead intro" }).closest("li") as HTMLElement;
    const spineEntry = screen.getByRole("button", { name: "Committed mix 1" }).closest("li") as HTMLElement;

    expect(spineEntry.getAttribute("aria-level")).toBe("1");
    expect(caption.getAttribute("aria-level")).toBe("2");
    expect(runEntry.getAttribute("aria-level")).toBe("2");
  });
});
