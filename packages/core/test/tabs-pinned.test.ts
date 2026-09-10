import { describe, expect, test } from "bun:test";

import {
  applyReorder,
  isTabsReorderAllowed,
  isValidTabsPinnedOrder,
  tabsTransition,
  type TabsContext,
  type TabsItem,
} from "../src/tabs.ts";

interface PinnedItem extends TabsItem {
  pinned?: "start" | "end" | null;
}

function context(items: PinnedItem[]): TabsContext<PinnedItem> {
  return { items, value: null, focusIndex: 0, activationMode: "automatic", reorderable: true };
}

describe("tabs pinned partitions", () => {
  test("unpinned, leading start, and trailing end partitions validate", () => {
    expect(isValidTabsPinnedOrder([{ value: "a" }, { value: "b" }])).toBe(true);
    expect(
      isValidTabsPinnedOrder([
        { value: "a", pinned: "start" },
        { value: "b" },
        { value: "c", pinned: "end" },
      ]),
    ).toBe(true);
  });

  test("a start pin after unpinned and an unpinned item after an end pin are invalid", () => {
    expect(
      isValidTabsPinnedOrder([{ value: "a" }, { value: "b", pinned: "start" }]),
    ).toBe(false);
    expect(
      isValidTabsPinnedOrder([{ value: "a", pinned: "end" }, { value: "b" }]),
    ).toBe(false);
    expect(
      isValidTabsPinnedOrder([
        { value: "a", pinned: "start" },
        { value: "b", pinned: "end" },
        { value: "c", pinned: "start" },
      ]),
    ).toBe(false);
  });

  test("a pinned item is never a reorder source", () => {
    const items: PinnedItem[] = [
      { value: "a", pinned: "start" },
      { value: "b" },
      { value: "c" },
    ];
    expect(isTabsReorderAllowed(items, 0, 1)).toBe(false);
    expect(isTabsReorderAllowed(items, 1, 2)).toBe(true);
  });

  test("no move may land inside or cross a pinned partition", () => {
    const items: PinnedItem[] = [
      { value: "a", pinned: "start" },
      { value: "b" },
      { value: "c" },
      { value: "d", pinned: "end" },
    ];
    expect(isTabsReorderAllowed(items, 1, 2)).toBe(true);
    expect(isTabsReorderAllowed(items, 1, 0)).toBe(false);
    expect(isTabsReorderAllowed(items, 2, 0)).toBe(false);
    expect(isTabsReorderAllowed(items, 1, 3)).toBe(false);
    expect(isTabsReorderAllowed(items, 2, 3)).toBe(false);
    expect(isTabsReorderAllowed(items, 3, 1)).toBe(false);
  });

  test("the machine refuses pinned reorders with no effects", () => {
    const items: PinnedItem[] = [
      { value: "a", pinned: "start" },
      { value: "b" },
      { value: "c" },
    ];
    const refused = tabsTransition(context(items), { type: "REORDER", fromIndex: 0, toIndex: 1 });
    expect(refused.effects).toEqual([]);
    expect(refused.context.items.map((item) => item.value)).toEqual(["a", "b", "c"]);

    const allowed = tabsTransition(context(items), { type: "REORDER", fromIndex: 1, toIndex: 2 });
    expect(allowed.effects.some((effect) => effect.type === "emitReorder")).toBe(true);
    expect(allowed.context.items.map((item) => item.value)).toEqual(["a", "c", "b"]);
  });

  test("keyboard reorder steps refuse to cross partitions", () => {
    const items: PinnedItem[] = [
      { value: "a", pinned: "start" },
      { value: "b" },
      { value: "c" },
    ];
    const refused = tabsTransition(context(items), { type: "REORDER_STEP", direction: -1, fromIndex: 1 });
    expect(refused.effects).toEqual([]);
  });

  test("applyReorder stays purely positional for hosts that pre-check", () => {
    const items: PinnedItem[] = [{ value: "a" }, { value: "b" }];
    expect(applyReorder(items, 0, 1).items.map((item) => item.value)).toEqual(["b", "a"]);
  });
});
