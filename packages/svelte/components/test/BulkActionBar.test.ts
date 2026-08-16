import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import BulkActionBar from "../src/BulkActionBar.svelte";
import type { BulkAction } from "../src/types";

const actions = [
  { id: "approve", label: "Approve" },
  { id: "delete", label: "Delete", tone: "danger" },
] satisfies BulkAction[];

describe("BulkActionBar (svelte)", () => {
  it("summarises the selection and the total", () => {
    const { container } = render(BulkActionBar, {
      props: { selectionCount: 3, totalCount: 12 },
    });
    expect(container.querySelector(".poodle-bulk-action-bar__summary")?.textContent).toContain(
      "3 selected",
    );
    expect(container.querySelector(".poodle-bulk-action-bar__summary")?.textContent).toContain(
      "of 12",
    );
  });

  it("disables every action while nothing is selected (clear stays available)", () => {
    const { container } = render(BulkActionBar, { props: { selectionCount: 0, actions } });
    const actionButtons = [...container.querySelectorAll("button")].filter((el) =>
      el.getAttribute("aria-label")?.match(/^(Approve|Delete)$/),
    );
    expect(actionButtons.length).toBe(2);
    expect(actionButtons.every((el) => el.disabled)).toBe(true);
    const clear = [...container.querySelectorAll("button")].find(
      (el) => el.getAttribute("aria-label") === "Clear selection",
    ) as HTMLButtonElement;
    expect(clear.disabled).toBe(false);
  });

  it("emits the action id and the clear event", async () => {
    const onAction = vi.fn();
    const onClear = vi.fn();
    const { container } = render(BulkActionBar, {
      props: { selectionCount: 2, actions, onAction, onClear },
    });
    const buttons = [...container.querySelectorAll("button")];

    await fireEvent.click(buttons.find((el) => el.getAttribute("aria-label") === "Approve") as HTMLButtonElement);
    expect(onAction).toHaveBeenCalledWith("approve");

    await fireEvent.click(
      buttons.find((el) => el.getAttribute("aria-label") === "Clear selection") as HTMLButtonElement,
    );
    expect(onClear).toHaveBeenCalledTimes(1);
  });

  it("shows select-all only when offered and not already selected", async () => {
    const onSelectAll = vi.fn();
    const shown = render(BulkActionBar, {
      props: { selectionCount: 2, totalCount: 5, showSelectAll: true, onSelectAll },
    });
    const selectAll = [...shown.container.querySelectorAll("button")].find((el) =>
      el.getAttribute("aria-label")?.startsWith("Select all"),
    ) as HTMLButtonElement;
    expect(selectAll.getAttribute("aria-label")).toBe("Select all (5)");

    await fireEvent.click(selectAll);
    expect(onSelectAll).toHaveBeenCalledTimes(1);

    const hidden = render(BulkActionBar, {
      props: { selectionCount: 5, totalCount: 5, showSelectAll: true, allSelected: true },
    });
    expect(
      [...hidden.container.querySelectorAll("button")].some((el) =>
        el.getAttribute("aria-label")?.startsWith("Select all"),
      ),
    ).toBe(false);
  });
});
