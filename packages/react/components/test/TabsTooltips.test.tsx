import { act, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { Tabs } from "../src/Tabs";

const items = [
  { value: "explorer", label: "Explorer", icon: "folder" },
  { value: "search", label: "Search", icon: "search" },
  { value: "git", label: "Git", icon: "git-branch" },
];

afterEach(() => {
  vi.useRealTimers();
});

function itemOf(container: HTMLElement, value: string): HTMLElement {
  const tab = container.querySelector<HTMLElement>(`.poodle-tabs__tab[data-value="${value}"]`);
  const item = tab?.closest<HTMLElement>(".poodle-tabs__item");
  if (!item) throw new Error(`item ${value}`);
  return item;
}

function tooltip(): HTMLElement | null {
  return document.querySelector<HTMLElement>(".poodle-tabs__tooltip");
}

function advance(ms: number): void {
  act(() => {
    vi.advanceTimersByTime(ms);
  });
}

describe("Tabs tooltips (react)", () => {
  it("stays inert when showTooltips is false", () => {
    vi.useFakeTimers();
    const { container } = render(<Tabs items={items} defaultValue="explorer" />);
    fireEvent.mouseEnter(itemOf(container, "search"));
    advance(300);
    expect(tooltip()).toBeNull();
  });

  it("shows the hovered label after 300ms and not before", () => {
    vi.useFakeTimers();
    const { container } = render(<Tabs items={items} defaultValue="explorer" showTooltips />);
    fireEvent.mouseEnter(itemOf(container, "search"));
    advance(299);
    expect(tooltip()).toBeNull();
    advance(1);
    expect(tooltip()?.getAttribute("role")).toBe("tooltip");
    expect(tooltip()?.textContent?.trim()).toBe("Search");
  });

  it("cancels a pending show on leave", () => {
    vi.useFakeTimers();
    const { container } = render(<Tabs items={items} defaultValue="explorer" showTooltips />);
    const search = itemOf(container, "search");
    fireEvent.mouseEnter(search);
    advance(100);
    fireEvent.mouseLeave(search);
    advance(300);
    expect(tooltip()).toBeNull();
  });

  it("hides on blur and Escape", () => {
    vi.useFakeTimers();
    const { container } = render(<Tabs items={items} defaultValue="explorer" showTooltips />);
    const searchTab = container.querySelector<HTMLElement>('.poodle-tabs__tab[data-value="search"]');
    if (!searchTab) throw new Error("search tab");
    act(() => {
      searchTab.focus();
    });
    fireEvent.mouseEnter(itemOf(container, "search"));
    advance(300);
    expect(tooltip()).not.toBeNull();
    act(() => {
      fireEvent.blur(searchTab);
    });
    expect(tooltip()).toBeNull();

    fireEvent.mouseEnter(itemOf(container, "search"));
    advance(300);
    expect(tooltip()).not.toBeNull();
    act(() => {
      fireEvent.keyDown(searchTab, { key: "Escape" });
    });
    expect(tooltip()).toBeNull();
  });

  it("clears a pending timer on unmount", () => {
    vi.useFakeTimers();
    const { container, unmount } = render(<Tabs items={items} defaultValue="explorer" showTooltips />);
    fireEvent.mouseEnter(itemOf(container, "search"));
    unmount();
    advance(300);
    expect(tooltip()).toBeNull();
  });

  it("shows after the delay on vertical strips without showTooltips", () => {
    vi.useFakeTimers();
    const { container } = render(
      <Tabs items={items} defaultValue="explorer" orientation="vertical" />,
    );
    fireEvent.mouseEnter(itemOf(container, "search"));
    advance(300);
    expect(tooltip()?.textContent?.trim()).toBe("Search");
  });
});
