import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import CommandPalette from "../src/CommandPalette.svelte";
import type { CommandActionItem } from "../src/types";

const items: CommandActionItem[] = [
  { id: "save", title: "Save", group: "File" },
  { id: "open", title: "Open File", group: "File" },
  { id: "toggle-terminal", title: "Toggle Terminal", group: "View" },
];

describe("CommandPalette (svelte)", () => {
  it("renders nothing while closed", () => {
    const { container } = render(CommandPalette, { props: { open: false, items } });
    expect(container.querySelector(".poodle-command-palette")).toBeNull();
  });

  it("renders a modal dialog with title, query, and status when open", () => {
    const { container } = render(CommandPalette, {
      props: { open: true, items, title: "Commands", description: "Run an action" },
    });
    const dialog = container.querySelector(".poodle-command-palette") as HTMLElement;
    expect(dialog.getAttribute("role")).toBe("dialog");
    expect(dialog.getAttribute("aria-modal")).toBe("true");
    expect(dialog.getAttribute("aria-label")).toBe("Commands");
    expect(container.querySelector("h3")?.textContent).toBe("Commands");

    const query = container.querySelector("#command-palette-query") as HTMLInputElement;
    expect(query).not.toBeNull();

    const status = container.querySelector("#command-palette-status") as HTMLElement;
    expect(status.getAttribute("role")).toBe("status");
    expect(status.getAttribute("aria-live")).toBe("polite");
    expect(status.textContent).toContain("3 commands available");
  });

  it("reports the query changes and command selection", async () => {
    const onQueryChange = vi.fn();
    const onCommandSelect = vi.fn();
    const { container } = render(CommandPalette, {
      props: { open: true, items, onQueryChange, onCommandSelect },
    });

    const query = container.querySelector("#command-palette-query") as HTMLInputElement;
    await fireEvent.input(query, { target: { value: "save" } });
    expect(onQueryChange).toHaveBeenCalledWith("save");

    await waitFor(() => {
      expect(container.querySelector('[role="option"]')).not.toBeNull();
    });
    const saveOption = [...container.querySelectorAll('[role="option"]')].find((option) =>
      option.textContent?.includes("Save"),
    ) as HTMLElement;
    await fireEvent.click(saveOption.querySelector(".poodle-list-card") as HTMLElement);
    expect(onCommandSelect).toHaveBeenCalledWith("save");
  });

  it("reports the no-results status copy when the host sets the no-results state", () => {
    const { container } = render(CommandPalette, {
      props: { open: true, items, query: "zzz", state: "no-results" },
    });
    const status = container.querySelector("#command-palette-status") as HTMLElement;
    expect(status.textContent).toContain('No commands match "zzz".');
  });

  it("closes via Escape and the close button", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(CommandPalette, {
      props: { open: true, items, onOpenChange },
    });
    await fireEvent.keyDown(window, { key: "Escape" });
    expect(onOpenChange).toHaveBeenCalledWith(false);

    onOpenChange.mockClear();
    const close = container.querySelector(
      'button[aria-label="Close command palette"]',
    ) as HTMLButtonElement;
    await fireEvent.click(close);
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("selects the active command on Enter", async () => {
    const onCommandSelect = vi.fn();
    const { container } = render(CommandPalette, {
      props: { open: true, items, onCommandSelect },
    });
    await waitFor(() => {
      expect(container.querySelector('[role="option"]')).not.toBeNull();
    });
    await fireEvent.keyDown(window, { key: "Enter" });
    expect(onCommandSelect).toHaveBeenCalledWith("save");
  });

  it("renders the invocation hint when provided", () => {
    const { container } = render(CommandPalette, {
      props: { open: true, items, invocationHint: "Cmd+K" },
    });
    expect(container.querySelector(".poodle-command-palette__hint")?.textContent).toContain(
      "Cmd+K",
    );
  });
});