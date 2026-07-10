import { describe, expect, test } from "bun:test";

import {
  resolveTabsValue,
  tabsKeydownEvent,
  tabsTooltipTransition,
  tabsTabParts,
  tabsTransition,
  type TabsContext,
  type TabsItem,
} from "../src/tabs";

const items: TabsItem[] = [
  { value: "a" },
  { value: "b", disabled: true },
  { value: "c", closable: true },
  { value: "d" },
];

function ctx(overrides: Partial<TabsContext> = {}): TabsContext {
  return {
    items,
    value: "a",
    focusIndex: 0,
    activationMode: "automatic",
    reorderable: false,
    ...overrides,
  };
}

describe("tabsTransition SELECT", () => {
  test("selects and syncs focus, emits value change", () => {
    const result = tabsTransition(ctx(), { type: "SELECT", value: "d" });

    expect(result.context.value).toBe("d");
    expect(result.context.focusIndex).toBe(3);
    expect(result.effects).toEqual([{ type: "emitValueChange", value: "d" }]);
  });

  test("disabled and unknown values are inert", () => {
    expect(tabsTransition(ctx(), { type: "SELECT", value: "b" }).effects).toEqual([]);
    expect(tabsTransition(ctx(), { type: "SELECT", value: "zz" }).effects).toEqual([]);
  });
});

describe("tabsTransition FOCUS_MOVE", () => {
  test("next skips disabled and wraps; automatic mode commits selection", () => {
    const result = tabsTransition(ctx(), { type: "FOCUS_MOVE", direction: "next" });

    expect(result.context.focusIndex).toBe(2); // skips disabled "b"
    expect(result.context.value).toBe("c");
    expect(result.effects).toEqual([
      { type: "focusTab", index: 2 },
      { type: "emitValueChange", value: "c" },
    ]);
  });

  test("prev wraps from first to last", () => {
    const result = tabsTransition(ctx(), { type: "FOCUS_MOVE", direction: "prev" });
    expect(result.context.focusIndex).toBe(3);
  });

  test("manual mode moves focus without committing selection", () => {
    const result = tabsTransition(ctx({ activationMode: "manual" }), {
      type: "FOCUS_MOVE",
      direction: "next",
    });

    expect(result.context.focusIndex).toBe(2);
    expect(result.context.value).toBe("a");
    expect(result.effects).toEqual([{ type: "focusTab", index: 2 }]);
  });

  test("first and last land on enabled boundaries", () => {
    expect(
      tabsTransition(ctx({ focusIndex: 3 }), { type: "FOCUS_MOVE", direction: "first" }).context.focusIndex,
    ).toBe(0);
    expect(tabsTransition(ctx(), { type: "FOCUS_MOVE", direction: "last" }).context.focusIndex).toBe(3);
  });
});

describe("tabsTransition ACTIVATE / CLOSE", () => {
  test("ACTIVATE commits only in manual mode", () => {
    const manual = tabsTransition(ctx({ activationMode: "manual", focusIndex: 3 }), { type: "ACTIVATE" });
    expect(manual.context.value).toBe("d");

    const automatic = tabsTransition(ctx({ focusIndex: 3 }), { type: "ACTIVATE" });
    expect(automatic.context.value).toBe("a");
    expect(automatic.effects).toEqual([]);
  });

  test("CLOSE emits request for closable items only; parent owns removal", () => {
    const closable = tabsTransition(ctx(), { type: "CLOSE", value: "c" });
    expect(closable.effects).toEqual([{ type: "emitClose", value: "c" }]);
    expect(closable.context.items).toHaveLength(4);

    expect(tabsTransition(ctx(), { type: "CLOSE", value: "a" }).effects).toEqual([]);
  });
});

describe("tabsTransition REORDER", () => {
  test("REORDER_STEP moves the focused tab and emits new order", () => {
    const result = tabsTransition(ctx({ reorderable: true }), { type: "REORDER_STEP", direction: 1 });

    expect(result.context.items.map((item) => item.value)).toEqual(["b", "a", "c", "d"]);
    expect(result.context.focusIndex).toBe(1);
    expect(result.effects).toEqual([
      { type: "focusTab", index: 1 },
      { type: "emitReorder", order: ["b", "a", "c", "d"] },
    ]);
  });

  test("keyboard events prefer the originating tab index over focusIndex", () => {
    // focusIndex is stale (0) but the keydown came from tab 3
    const moved = tabsTransition(ctx({ reorderable: true, focusIndex: 0 }), {
      type: "REORDER_STEP",
      direction: -1,
      fromIndex: 3,
    });
    expect(moved.context.items.map((item) => item.value)).toEqual(["a", "b", "d", "c"]);

    const focus = tabsTransition(ctx({ focusIndex: 0, activationMode: "manual" }), {
      type: "FOCUS_MOVE",
      direction: "next",
      fromIndex: 2,
    });
    expect(focus.context.focusIndex).toBe(3);

    const activated = tabsTransition(ctx({ focusIndex: 0, activationMode: "manual" }), {
      type: "ACTIVATE",
      index: 3,
    });
    expect(activated.context.value).toBe("d");
  });

  test("reorder is guarded by reorderable and bounds", () => {
    expect(tabsTransition(ctx(), { type: "REORDER_STEP", direction: 1 }).effects).toEqual([]);
    expect(
      tabsTransition(ctx({ reorderable: true }), { type: "REORDER_STEP", direction: -1 }).effects,
    ).toEqual([]);
    expect(
      tabsTransition(ctx({ reorderable: true }), { type: "REORDER", fromIndex: 0, toIndex: 9 }).effects,
    ).toEqual([]);
  });
});

describe("tabsKeydownEvent", () => {
  const base = { reorderable: false, activationMode: "automatic" as const };

  test("orientation maps arrow keys", () => {
    expect(tabsKeydownEvent("ArrowRight", false, "horizontal", base)).toEqual({
      type: "FOCUS_MOVE",
      direction: "next",
    });
    expect(tabsKeydownEvent("ArrowRight", false, "vertical", base)).toBeNull();
    expect(tabsKeydownEvent("ArrowDown", false, "vertical", base)).toEqual({
      type: "FOCUS_MOVE",
      direction: "next",
    });
  });

  test("alt+arrow reorders when reorderable", () => {
    expect(tabsKeydownEvent("ArrowRight", true, "horizontal", { ...base, reorderable: true })).toEqual({
      type: "REORDER_STEP",
      direction: 1,
    });
    expect(tabsKeydownEvent("ArrowRight", true, "horizontal", base)).toEqual({
      type: "FOCUS_MOVE",
      direction: "next",
    });
  });

  test("Enter/Space activate only in manual mode", () => {
    expect(tabsKeydownEvent("Enter", false, "horizontal", base)).toBeNull();
    expect(tabsKeydownEvent(" ", false, "horizontal", { ...base, activationMode: "manual" })).toEqual({
      type: "ACTIVATE",
    });
  });
});

describe("resolveTabsValue / parts", () => {
  test("falls back to first enabled item", () => {
    expect(resolveTabsValue(items, null)).toBe("a");
    expect(resolveTabsValue([{ value: "x", disabled: true }, { value: "y" }], null)).toBe("y");
    expect(resolveTabsValue(items, "zz")).toBe("a");
  });

  test("roving tabindex: only focusIndex is tabbable", () => {
    const props = { instanceId: "t1", orientation: "horizontal" as const, hasPanel: true };
    const context = ctx({ focusIndex: 2 });

    expect(tabsTabParts(context, props, 2)["tabindex"]).toBe(0);
    expect(tabsTabParts(context, props, 0)["tabindex"]).toBe(-1);
    expect(tabsTabParts(context, props, 0)["aria-selected"]).toBe("true");
    expect(tabsTabParts(context, props, 2)["aria-controls"]).toBe("t1-panel-c");
  });
});

describe("tabsTooltipTransition", () => {
  test("enter -> timer -> visible; leave cancels", () => {
    const pending = tabsTooltipTransition({ name: "hidden" }, { type: "POINTER_ENTER", index: 1 });
    expect(pending.state).toEqual({ name: "pending", index: 1 });
    expect(pending.effects).toEqual([{ type: "clearTimer" }, { type: "startTimer" }]);

    const visible = tabsTooltipTransition(pending.state, { type: "TIMER_FIRE" });
    expect(visible.state).toEqual({ name: "visible", index: 1 });

    const hidden = tabsTooltipTransition(visible.state, { type: "POINTER_LEAVE" });
    expect(hidden.state).toEqual({ name: "hidden" });
    expect(hidden.effects).toEqual([{ type: "clearTimer" }]);
  });

  test("stale timer fire in hidden state is inert", () => {
    expect(tabsTooltipTransition({ name: "hidden" }, { type: "TIMER_FIRE" }).state).toEqual({
      name: "hidden",
    });
  });
});
