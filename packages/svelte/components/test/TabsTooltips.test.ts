import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import Tabs from "../src/Tabs.svelte";

const items = [
  { value: "explorer", label: "Explorer", icon: "folder" },
  { value: "search", label: "Search", icon: "search" },
  { value: "git", label: "Git", icon: "git-branch" },
];

const itemsWithDisabledGit = [
  ...items.slice(0, 2),
  { value: "git", label: "Git", icon: "git-branch", disabled: true },
];

function withDisabled(value: string, disabled: boolean) {
  return items.map((item) => (item.value === value ? { ...item, disabled } : item));
}

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

describe("Tabs tooltips (svelte)", () => {
  it("stays inert when showTooltips is false", async () => {
    vi.useFakeTimers();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "explorer" },
    });
    await fireEvent.mouseEnter(itemOf(container, "search"));
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()).toBeNull();

    await fireEvent.focus(tabOf(container, "search"));
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
    const searchTab = tabOf(container, "search");
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

  it("never schedules or paints a tooltip for a disabled tab", async () => {
    vi.useFakeTimers();
    const { container } = render(Tabs, {
      props: { items: itemsWithDisabledGit, defaultValue: "explorer", showTooltips: true },
    });
    await fireEvent.mouseEnter(itemOf(container, "git"));
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()).toBeNull();

    await fireEvent.mouseEnter(itemOf(container, "search"));
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()?.textContent?.trim()).toBe("Search");
    await fireEvent.mouseEnter(itemOf(container, "git"));
    expect(tooltip()).toBeNull();
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()).toBeNull();
  });

  it("schedules on horizontal keyboard focus and paints at 300ms", async () => {
    vi.useFakeTimers();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "explorer", showTooltips: true },
    });
    await fireEvent.focus(tabOf(container, "search"));
    await vi.advanceTimersByTimeAsync(299);
    expect(tooltip()).toBeNull();
    await vi.advanceTimersByTimeAsync(1);
    expect(tooltip()?.getAttribute("role")).toBe("tooltip");
    expect(tooltip()?.textContent?.trim()).toBe("Search");
  });

  it("schedules vertical keyboard focus without showTooltips", async () => {
    vi.useFakeTimers();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "explorer", orientation: "vertical" },
    });
    await fireEvent.focus(tabOf(container, "search"));
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()?.textContent?.trim()).toBe("Search");
  });

  it("cancels a pending tooltip when that tab becomes disabled", async () => {
    vi.useFakeTimers();
    const { container, rerender } = render(Tabs, {
      props: { items, defaultValue: "explorer", showTooltips: true },
    });
    await fireEvent.mouseEnter(itemOf(container, "search"));
    await vi.advanceTimersByTimeAsync(100);
    await rerender({ items: withDisabled("search", true), defaultValue: "explorer", showTooltips: true });
    expect(tooltip()).toBeNull();
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()).toBeNull();
    await rerender({ items, defaultValue: "explorer", showTooltips: true });
    expect(tooltip()).toBeNull();
  });

  it("hides a visible tooltip immediately when that tab becomes disabled", async () => {
    vi.useFakeTimers();
    const { container, rerender } = render(Tabs, {
      props: { items, defaultValue: "explorer", showTooltips: true },
    });
    await fireEvent.mouseEnter(itemOf(container, "search"));
    await vi.advanceTimersByTimeAsync(300);
    expect(tooltip()?.textContent?.trim()).toBe("Search");
    await rerender({ items: withDisabled("search", true), defaultValue: "explorer", showTooltips: true });
    expect(tooltip()).toBeNull();
    await rerender({ items, defaultValue: "explorer", showTooltips: true });
    expect(tooltip()).toBeNull();
  });
});
