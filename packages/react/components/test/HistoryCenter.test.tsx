import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { HistoryCenter } from "../src";

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

describe("HistoryCenter (react)", () => {
  it("renders the undo/list/redo cluster with enablement from canUndo/canRedo and busy", () => {
    render(<HistoryCenter canUndo canRedo={false} />);

    expect(screen.getByRole("button", { name: "Undo" }).hasAttribute("disabled")).toBe(false);
    expect(screen.getByRole("button", { name: "Redo" }).hasAttribute("disabled")).toBe(true);
  });

  it("disables undo/redo while busy even when the host can undo", () => {
    render(<HistoryCenter canUndo busy />);

    expect(screen.getByRole("button", { name: "Undo" }).hasAttribute("disabled")).toBe(true);
  });

  it("opens the popover and selects an entry", () => {
    const onSelectEntry = vi.fn();
    render(<HistoryCenter entries={linearEntries} onSelectEntry={onSelectEntry} />);

    fireEvent.click(screen.getByRole("button", { name: "History" }));

    expect(screen.getByRole("dialog", { name: "History" })).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Committed mix 1" }));

    expect(onSelectEntry).toHaveBeenCalledWith("c1");
  });

  it("keeps the linear list plain when branches are not supplied", () => {
    render(<HistoryCenter entries={forkedEntries} defaultOpen />);

    expect(screen.queryByRole("button", { name: "Show branches at Fork point" })).toBeNull();
    expect(screen.queryByRole("button", { name: "Rename feature/lead" })).toBeNull();
    expect(document.querySelector(".poodle-history-center__pin")).toBeNull();
  });

  it("expands fork points into branch rows and emits checkout with the fork context", () => {
    const onCheckout = vi.fn();
    render(<HistoryCenter entries={forkedEntries} branches={branches} totalBranches={2} onCheckout={onCheckout} defaultOpen />);

    fireEvent.click(screen.getByRole("button", { name: "Show branches at Fork point" }));

    const branchButton = screen.getByRole("button", { name: /feature\/lead 3 entries/ });
    expect(screen.getByText("3 entries")).toBeTruthy();
    fireEvent.click(branchButton);

    expect(onCheckout).toHaveBeenCalledWith("b1", "fork");
  });

  it("commits inline rename through onRenameBranch", () => {
    const onRenameBranch = vi.fn();
    render(<HistoryCenter entries={forkedEntries} branches={branches} onRenameBranch={onRenameBranch} defaultOpen />);

    fireEvent.click(screen.getByRole("button", { name: "Show branches at Fork point" }));
    fireEvent.click(screen.getByRole("button", { name: "Rename feature/lead" }));

    const input = screen.getByRole("textbox", { name: "Rename branch feature/lead" }) as HTMLInputElement;
    fireEvent.input(input, { target: { value: "feature/lead-v2" } });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(onRenameBranch).toHaveBeenCalledWith("b1", "feature/lead-v2");
  });

  it("shows rejection as a visible dismissible notice", () => {
    render(<HistoryCenter entries={linearEntries} defaultOpen rejection="Branch name is taken" />);

    expect(screen.getByText("Branch name is taken")).toBeTruthy();

    fireEvent.click(screen.getByRole("button", { name: "Dismiss" }));

    expect(screen.queryByText("Branch name is taken")).toBeNull();
  });

  it("offers load-more with the supplied count as offset", () => {
    const onLoadMoreEntries = vi.fn();
    render(<HistoryCenter entries={linearEntries} hasMoreEntries onLoadMoreEntries={onLoadMoreEntries} defaultOpen />);

    fireEvent.click(screen.getByRole("button", { name: "Load more entries" }));

    expect(onLoadMoreEntries).toHaveBeenCalledWith(3);
  });

  it("renders the empty message and the loading row", () => {
    const { unmount } = render(<HistoryCenter entries={[]} defaultOpen />);

    expect(screen.getByText("No history entries yet.")).toBeTruthy();

    unmount();
    render(<HistoryCenter entries={[]} status="loading" defaultOpen />);

    expect(screen.getByText("Loading history…")).toBeTruthy();
  });

  it("navigates rows with arrow keys", () => {
    render(<HistoryCenter entries={linearEntries} defaultOpen />);

    const first = screen.getByRole("button", { name: "Committed mix 1" });
    first.focus();
    fireEvent.keyDown(first, { key: "ArrowDown" });

    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Arranged intro" }));
  });
});
