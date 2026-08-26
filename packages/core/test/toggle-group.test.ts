import { describe, expect, test } from "bun:test";

import {
  toggleGroupArrowTarget,
  toggleGroupEnabledValues,
  toggleGroupTabStopValue,
  type ToggleGroupContext,
} from "../src/toggle-group.ts";

function ctx(overrides: Partial<ToggleGroupContext> = {}): ToggleGroupContext {
  return {
    value: "list",
    options: [
      { value: "grid" },
      { value: "list" },
      { value: "board", disabled: true },
      { value: "kanban" },
    ],
    selectionMode: "single",
    allowDeactivation: false,
    disabled: false,
    ...overrides,
  };
}

describe("toggleGroupEnabledValues", () => {
  test("returns declared enabled order", () => {
    expect(toggleGroupEnabledValues(ctx())).toEqual(["grid", "list", "kanban"]);
  });

  test("is empty when the group is disabled", () => {
    expect(toggleGroupEnabledValues(ctx({ disabled: true }))).toEqual([]);
  });
});

describe("toggleGroupTabStopValue", () => {
  test("selected enabled option is the entry target", () => {
    expect(toggleGroupTabStopValue(ctx())).toBe("list");
  });

  test("unknown or disabled selection falls back to the first enabled option", () => {
    expect(toggleGroupTabStopValue(ctx({ value: "missing" }))).toBe("grid");
    expect(toggleGroupTabStopValue(ctx({ value: "board" }))).toBe("grid");
    expect(toggleGroupTabStopValue(ctx({ value: null }))).toBe("grid");
  });

  test("disabled group has no tab stop", () => {
    expect(toggleGroupTabStopValue(ctx({ disabled: true }))).toBeNull();
  });

  test("multiple mode is not roving", () => {
    expect(
      toggleGroupTabStopValue(ctx({ selectionMode: "multiple", value: ["list"] })),
    ).toBeNull();
  });
});

describe("toggleGroupArrowTarget", () => {
  test("wraps and skips disabled options", () => {
    expect(toggleGroupArrowTarget(ctx(), "list", 1)).toBe("kanban");
    expect(toggleGroupArrowTarget(ctx(), "kanban", 1)).toBe("grid");
    expect(toggleGroupArrowTarget(ctx(), "grid", -1)).toBe("kanban");
    expect(toggleGroupArrowTarget(ctx(), "list", -1)).toBe("grid");
  });

  test("one enabled option is inert", () => {
    const one = ctx({
      value: "grid",
      options: [{ value: "grid" }, { value: "list", disabled: true }],
    });
    expect(toggleGroupArrowTarget(one, "grid", 1)).toBeNull();
    expect(toggleGroupArrowTarget(one, "grid", -1)).toBeNull();
  });

  test("disabled group and unknown origin are inert", () => {
    expect(toggleGroupArrowTarget(ctx({ disabled: true }), "list", 1)).toBeNull();
    expect(toggleGroupArrowTarget(ctx(), "missing", 1)).toBeNull();
  });

  test("multiple mode does not intercept arrows", () => {
    expect(
      toggleGroupArrowTarget(ctx({ selectionMode: "multiple", value: ["list"] }), "list", 1),
    ).toBeNull();
  });
});
