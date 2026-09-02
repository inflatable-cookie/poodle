import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import Tabs from "../src/Tabs.svelte";

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

describe("Tabs tooltips (svelte)", () => {
  it("stays inert when showTooltips is false", async () => {
    vi.useFakeTimers();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "explorer" },
    });
    await fireEvent.mouseEnter(itemOf(container, "search"));
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()).toBeNull();
  });

  it("shows the hovered label after 300ms and not before", async () => {
    vi.useFakeTimers();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "explorer", showTooltips: true },
    });
    await fireEvent.mouseEnter(itemOf(container, "search"));
    await vi.advanceTimersByTimeAsync(299);
    expect(tooltip()).toBeNull();
    await vi.advanceTimersByTimeAsync(1);
    expect(tooltip()?.getAttribute("role")).toBe("tooltip");
    expect(tooltip()?.textContent?.trim()).toBe("Search");
  });

  it("cancels a pending show on leave", async () => {
    vi.useFakeTimers();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "explorer", showTooltips: true },
    });
    const search = itemOf(container, "search");
    await fireEvent.mouseEnter(search);
    await vi.advanceTimersByTimeAsync(100);
    await fireEvent.mouseLeave(search);
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()).toBeNull();
  });

  it("hides on blur and Escape", async () => {
    vi.useFakeTimers();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "explorer", showTooltips: true },
    });
    const searchTab = container.querySelector<HTMLElement>('.poodle-tabs__tab[data-value="search"]');
    if (!searchTab) throw new Error("search tab");
    searchTab.focus();
    await fireEvent.mouseEnter(itemOf(container, "search"));
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()).not.toBeNull();
    await fireEvent.blur(searchTab);
    expect(tooltip()).toBeNull();

    await fireEvent.mouseEnter(itemOf(container, "search"));
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()).not.toBeNull();
    await fireEvent.keyDown(searchTab, { key: "Escape" });
    expect(tooltip()).toBeNull();
  });

  it("clears a pending timer on unmount", async () => {
    vi.useFakeTimers();
    const { container, unmount } = render(Tabs, {
      props: { items, defaultValue: "explorer", showTooltips: true },
    });
    await fireEvent.mouseEnter(itemOf(container, "search"));
    unmount();
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()).toBeNull();
  });

  it("shows after the delay on vertical strips without showTooltips", async () => {
    vi.useFakeTimers();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "explorer", orientation: "vertical" },
    });
    await fireEvent.mouseEnter(itemOf(container, "search"));
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()?.textContent?.trim()).toBe("Search");
  });
});
