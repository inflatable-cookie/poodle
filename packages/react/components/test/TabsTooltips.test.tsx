import { act, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { Tabs } from "../src/Tabs";

const items = [
  { value: "explorer", label: "Explorer", icon: "folder" },
  { value: "search", label: "Search", icon: "search" },
  { value: "git", label: "Git", icon: "git-branch" },
];

const itemsWithDisabledGit = [
  ...items.slice(0, 2),
  { value: "git", label: "Git", icon: "git-branch", disabled: true },
];

afterEach(() => {
  vi.useRealTimers();
});

function tabOf(container: HTMLElement, value: string): HTMLElement {
  const tab = container.querySelector<HTMLElement>(`.poodle-tabs__tab[data-value="${value}"]`);
  if (!tab) throw new Error(`tab ${value}`);
  return tab;
}

function itemOf(container: HTMLElement, value: string): HTMLElement {
  const item = tabOf(container, value).closest<HTMLElement>(".poodle-tabs__item");
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

    act(() => {
      fireEvent.focus(tabOf(container, "search"));
    });
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
    const searchTab = tabOf(container, "search");
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

  it("never schedules or paints a tooltip for a disabled tab", () => {
    vi.useFakeTimers();
    const { container } = render(
      <Tabs items={itemsWithDisabledGit} defaultValue="explorer" showTooltips />,
    );
    fireEvent.mouseEnter(itemOf(container, "git"));
    advance(300);
    expect(tooltip()).toBeNull();

    fireEvent.mouseEnter(itemOf(container, "search"));
    advance(300);
    expect(tooltip()?.textContent?.trim()).toBe("Search");
    fireEvent.mouseEnter(itemOf(container, "git"));
    expect(tooltip()).toBeNull();
    advance(300);
    expect(tooltip()).toBeNull();
  });

  it("schedules on horizontal keyboard focus and paints at 300ms", () => {
    vi.useFakeTimers();
    const { container } = render(<Tabs items={items} defaultValue="explorer" showTooltips />);
    act(() => {
      fireEvent.focus(tabOf(container, "search"));
    });
    advance(299);
    expect(tooltip()).toBeNull();
    advance(1);
    expect(tooltip()?.getAttribute("role")).toBe("tooltip");
    expect(tooltip()?.textContent?.trim()).toBe("Search");
  });

  it("schedules vertical keyboard focus without showTooltips", () => {
    vi.useFakeTimers();
    const { container } = render(
      <Tabs items={items} defaultValue="explorer" orientation="vertical" />,
    );
    act(() => {
      fireEvent.focus(tabOf(container, "search"));
    });
    advance(300);
    expect(tooltip()?.textContent?.trim()).toBe("Search");
  });
});
