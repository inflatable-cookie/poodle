import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import HistoryCenter from "../src/HistoryCenter.svelte";

const linearEntries = [
  { id: "c1", label: "Committed mix 1", position: "past" as const },
  { id: "c2", label: "Arranged intro", position: "past" as const },
  { id: "c3", label: "Current draft", position: "current" as const },
];

const forkedEntries = [
  { id: "c1", label: "Committed mix 1", position: "past" as const },
  { id: "fork", label: "Fork point", position: "past" as const, branchCount: 2, checkpoint: true },
  { id: "c3", label: "Current draft", position: "current" as const },
];

const branches = [
  { id: "b1", name: "feature/lead", entryCount: 3, current: true },
  { id: "b2", name: null, entryCount: 1 },
];

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

  it("opens the popover and selects an entry", async () => {
    const onSelectEntry = vi.fn();
    render(HistoryCenter, { props: { entries: linearEntries, onSelectEntry } });

    await fireEvent.click(screen.getByRole("button", { name: "History" }));

    expect(screen.getByRole("dialog", { name: "History" })).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Committed mix 1" }));

    expect(onSelectEntry).toHaveBeenCalledWith("c1");
  });

  it("keeps the linear list plain when branches are not supplied", async () => {
    render(HistoryCenter, { props: { entries: forkedEntries, defaultOpen: true } });

    expect(screen.queryByRole("button", { name: "Show branches at Fork point" })).toBeNull();
    expect(screen.queryByRole("button", { name: "Rename feature/lead" })).toBeNull();
    expect(document.querySelector(".poodle-history-center__pin")).toBeNull();
  });

  it("expands fork points into branch rows and emits checkout with the fork context", async () => {
    const onCheckout = vi.fn();
    render(HistoryCenter, {
      props: { entries: forkedEntries, branches, totalBranches: 2, onCheckout, defaultOpen: true },
    });

    await fireEvent.click(screen.getByRole("button", { name: "Show branches at Fork point" }));

    const branchButton = screen.getByRole("button", { name: /feature\/lead 3 entries/ });
    expect(screen.getByText("3 entries")).toBeTruthy();
    await fireEvent.click(branchButton);

    expect(onCheckout).toHaveBeenCalledWith("b1", "fork");
  });

  it("commits inline rename through onRenameBranch", async () => {
    const onRenameBranch = vi.fn();
    render(HistoryCenter, {
      props: { entries: forkedEntries, branches, onRenameBranch, defaultOpen: true },
    });

    await fireEvent.click(screen.getByRole("button", { name: "Show branches at Fork point" }));
    await fireEvent.click(screen.getByRole("button", { name: "Rename feature/lead" }));

    const input = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "feature/lead-v2" } });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(onRenameBranch).toHaveBeenCalledWith("b1", "feature/lead-v2");
  });

  it("shows rejection as a visible dismissible notice", async () => {
    render(HistoryCenter, {
      props: { entries: linearEntries, defaultOpen: true, rejection: "Branch name is taken" },
    });

    expect(screen.getByText("Branch name is taken")).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Dismiss" }));

    expect(screen.queryByText("Branch name is taken")).toBeNull();
  });

  it("offers load-more with the supplied count as offset", async () => {
    const onLoadMoreEntries = vi.fn();
    render(HistoryCenter, {
      props: { entries: linearEntries, hasMoreEntries: true, onLoadMoreEntries, defaultOpen: true },
    });

    await fireEvent.click(screen.getByRole("button", { name: "Load more entries" }));

    expect(onLoadMoreEntries).toHaveBeenCalledWith(3);
  });

  it("renders the empty message and the loading row", async () => {
    const { unmount } = render(HistoryCenter, { props: { entries: [], defaultOpen: true } });

    expect(screen.getByText("No history entries yet.")).toBeTruthy();

    unmount();
    render(HistoryCenter, { props: { entries: [], status: "loading", defaultOpen: true } });

    expect(screen.getByText("Loading history…")).toBeTruthy();
  });

  it("navigates rows with arrow keys", async () => {
    render(HistoryCenter, { props: { entries: linearEntries, defaultOpen: true } });

    const first = screen.getByRole("button", { name: "Committed mix 1" });
    first.focus();
    await fireEvent.keyDown(first, { key: "ArrowDown" });

    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Arranged intro" }));
  });
});
