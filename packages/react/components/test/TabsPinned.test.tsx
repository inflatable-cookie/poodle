import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Tabs } from "../src/Tabs";

const items = [
  { value: "home", label: "Home", pinned: "start" as const },
  { value: "mix", label: "Mix" },
  { value: "master", label: "Master" },
  { value: "logs", label: "Logs", pinned: "end" as const },
];

function tabs() {
  return screen.getAllByRole("tab");
}

describe("Tabs pinned (react)", () => {
  it("pins stay first and last with partition markers", () => {
    const { container } = render(<Tabs items={items} defaultValue="mix" reorderable />);
    const rendered = [...container.querySelectorAll<HTMLElement>(".poodle-tabs__item")];
    expect(rendered.map((item) => item.dataset.pinned ?? null)).toEqual([
      "start",
      null,
      null,
      "end",
    ]);
    expect(tabs().map((tab) => tab.textContent?.trim())).toEqual([
      "Home",
      "Mix",
      "Master",
      "Logs",
    ]);
  });

  it("a pinned tab cannot be picked up with Alt+Arrow", async () => {
    const onReorder = vi.fn();
    render(<Tabs items={items} defaultValue="home" reorderable onReorder={onReorder} />);
    await fireEvent.keyDown(tabs()[0], { key: "ArrowRight", altKey: true });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("an unpinned tab cannot cross into or over a pinned partition", async () => {
    const onReorder = vi.fn();
    render(<Tabs items={items} defaultValue="mix" reorderable onReorder={onReorder} />);
    await fireEvent.keyDown(tabs()[1], { key: "ArrowLeft", altKey: true });
    expect(onReorder).not.toHaveBeenCalled();
    await fireEvent.keyDown(tabs()[2], { key: "ArrowRight", altKey: true });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("unpinned tabs still reorder among themselves", async () => {
    const onReorder = vi.fn();
    render(<Tabs items={items} defaultValue="mix" reorderable onReorder={onReorder} />);
    await fireEvent.keyDown(tabs()[1], { key: "ArrowRight", altKey: true });
    expect(onReorder).toHaveBeenCalledWith(["home", "master", "mix", "logs"]);
  });

  it("an invalid partition warns instead of silently accepting", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    try {
      render(
        <Tabs
          items={[
            { value: "mix", label: "Mix" },
            { value: "home", label: "Home", pinned: "start" as const },
          ]}
        />,
      );
      expect(warn).toHaveBeenCalledWith(expect.stringContaining("pinned"));
    } finally {
      warn.mockRestore();
    }
  });
});
