import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ActionDiscoveryPanel from "../src/ActionDiscoveryPanel.svelte";
import type { CommandActionItem } from "../src/types";

const items: CommandActionItem[] = [
  { id: "save", title: "Save", group: "File", shortcut: "Cmd+S" },
  { id: "open", title: "Open File", group: "File", description: "Open a file" },
  { id: "terminal", title: "Toggle Terminal", group: "View", badge: "New" },
  { id: "locked", title: "Locked action", group: "View", disabled: true },
];

describe("ActionDiscoveryPanel (svelte)", () => {
  it("renders a labelled listbox with grouped actions", () => {
    const { container } = render(ActionDiscoveryPanel, {
      props: { items, ariaLabel: "Command results" },
    });
    const root = container.querySelector(".poodle-action-discovery-panel") as HTMLElement;
    expect(root.getAttribute("role")).toBe("listbox");
    expect(root.getAttribute("aria-label")).toBe("Command results");

    const options = [...container.querySelectorAll('[role="option"]')];
    expect(options.length).toBe(4);
    expect(container.querySelector(".poodle-action-discovery-panel__group")?.textContent).toContain(
      "File",
    );
    expect(container.querySelector(".poodle-action-discovery-panel__group")?.textContent).toContain(
      "Save",
    );
  });

  it("marks the active item with aria-selected", () => {
    const { container } = render(ActionDiscoveryPanel, { props: { items, activeId: "save" } });
    const options = [...container.querySelectorAll('[role="option"]')];
    expect(options[0].getAttribute("aria-selected")).toBe("true");
    expect(options[1].getAttribute("aria-selected")).toBe("false");
  });

  it("renders shortcuts and badges on their rows", () => {
    const { container } = render(ActionDiscoveryPanel, { props: { items } });
    expect(container.querySelector(".poodle-action-discovery-panel__kbd")?.textContent).toContain(
      "Cmd+S",
    );
    expect(container.querySelector(".poodle-action-discovery-panel__badge")?.textContent).toContain(
      "New",
    );
  });

  it("reports item selection on click", async () => {
    const onItemSelect = vi.fn();
    const { container } = render(ActionDiscoveryPanel, { props: { items, onItemSelect } });
    const saveRow = [...container.querySelectorAll('[role="option"]')].find((option) =>
      option.textContent?.includes("Save"),
    ) as HTMLElement;
    await fireEvent.click(saveRow.querySelector(".poodle-list-card") as HTMLElement);
    expect(onItemSelect).toHaveBeenCalledWith("save");
  });

  it("renders five skeleton rows while loading", () => {
    const { container } = render(ActionDiscoveryPanel, { props: { items, state: "loading" } });
    expect(container.querySelectorAll(".poodle-action-discovery-panel__skeleton-row").length).toBe(
      5,
    );
  });

  it("renders contextual empty states for error, empty, and no-results", () => {
    const error = render(ActionDiscoveryPanel, { props: { items: [], state: "error" } });
    expect(error.container.textContent).toContain("Could not load actions");

    const empty = render(ActionDiscoveryPanel, { props: { items: [], state: "empty" } });
    expect(empty.container.textContent).toContain("No actions available");

    const noResults = render(ActionDiscoveryPanel, { props: { items: [], state: "no-results" } });
    expect(noResults.container.textContent).toContain("No matching actions");
  });
});