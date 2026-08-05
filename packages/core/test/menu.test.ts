import { describe, expect, test } from "bun:test";

import { menuTransition } from "../src/menu.ts";

import { menuItemHasSubmenu, menuListCanActivate, menuListNavigate, menuNavigableItems } from "../src/menu.ts";

describe("menu list machinery", () => {
  const items = [
    { value: "a" },
    { value: "b", disabled: true },
    { value: "c" },
    { value: "d" },
  ];

  test("next/prev wrap and skip disabled", () => {
    expect(menuListNavigate(items, 0, "next")).toBe(2);
    expect(menuListNavigate(items, 3, "next")).toBe(0);
    expect(menuListNavigate(items, 2, "prev")).toBe(0);
    expect(menuListNavigate(items, 0, "prev")).toBe(3);
  });

  test("first/last land on enabled boundaries", () => {
    expect(menuListNavigate(items, 2, "first")).toBe(0);
    expect(menuListNavigate(items, 0, "last")).toBe(3);
    expect(menuListNavigate([{ value: "x", disabled: true }, { value: "y" }], 1, "first")).toBe(1);
  });

  test("all-disabled and empty lists stay put", () => {
    expect(menuListNavigate([{ value: "x", disabled: true }], 0, "next")).toBe(0);
    expect(menuListNavigate([], 0, "next")).toBe(0);
  });

  test("navigable items exclude separators; activation guards", () => {
    const mixed = [{ value: "a" }, { value: "s", kind: "separator" }, { value: "b" }];
    expect(menuNavigableItems(mixed).map((item) => item.value)).toEqual(["a", "b"]);
    expect(menuListCanActivate({ kind: "separator" })).toBe(false);
    expect(menuListCanActivate({ disabled: true })).toBe(false);
    expect(menuListCanActivate({ kind: "checkbox" })).toBe(true);
  });
});

describe("menuTransition", () => {
  test("TOGGLE opens with focus-first-item intent", () => {
    const result = menuTransition("closed", {}, { type: "TOGGLE" });

    expect(result.state).toBe("open");
    expect(result.effects).toEqual([
      { type: "emitOpenChange", open: true },
      { type: "focusFirstItem" },
    ]);
  });

  test("close paths emit open-change only (no trigger-focus restore)", () => {
    for (const event of [{ type: "TOGGLE" }, { type: "CLOSE" }, { type: "ESCAPE" }, { type: "OUTSIDE_INTERACT" }] as const) {
      const result = menuTransition("open", {}, event);

      expect(result.state).toBe("closed");
      expect(result.effects).toEqual([{ type: "emitOpenChange", open: false }]);
    }
  });

  test("ACTION emits the action before closing", () => {
    const result = menuTransition("open", {}, { type: "ACTION", value: "rename" });

    expect(result.state).toBe("closed");
    expect(result.effects).toEqual([
      { type: "emitAction", value: "rename" },
      { type: "emitOpenChange", open: false },
    ]);
  });

  test("wrong-state events and disabled context are inert", () => {
    expect(menuTransition("closed", {}, { type: "ESCAPE" }).effects).toEqual([]);
    expect(menuTransition("closed", {}, { type: "ACTION", value: "x" }).effects).toEqual([]);
    expect(menuTransition("open", {}, { type: "OPEN" }).effects).toEqual([]);
    expect(menuTransition("closed", { disabled: true }, { type: "TOGGLE" }).effects).toEqual([]);
  });
});

describe("menuItemHasSubmenu", () => {
  test("true only for non-separator items with at least one child", () => {
    expect(menuItemHasSubmenu({ children: [{ value: "x" }] })).toBe(true);
    expect(menuItemHasSubmenu({ kind: "action", children: [{ value: "x" }] })).toBe(true);
    expect(menuItemHasSubmenu({})).toBe(false);
    expect(menuItemHasSubmenu({ children: [] })).toBe(false);
    expect(menuItemHasSubmenu({ kind: "separator", children: [{ value: "x" }] })).toBe(false);
  });

  test("disabled parents still report a submenu (activation gate is separate)", () => {
    expect(menuItemHasSubmenu({ disabled: true, children: [{ value: "x" }] })).toBe(true);
  });
});
