import { describe, expect, test } from "bun:test";

import { listboxInitialFocus, listboxTransition, type ListboxContext } from "../src/listbox.ts";

const items = [
  { value: "alpha", label: "Alpha" },
  { value: "disabled", label: "Disabled", disabled: true },
  { value: "alpine", label: "Alpine" },
  { value: "beta", label: "Beta" },
];

function ctx(overrides: Partial<ListboxContext> = {}): ListboxContext {
  return {
    items,
    selectionMode: "single",
    orientation: "vertical",
    disabled: false,
    selectedValues: [],
    focusedValue: "alpha",
    anchorValue: null,
    typeahead: "",
    typeaheadAt: 0,
    ...overrides,
  };
}

describe("listboxInitialFocus", () => {
  test("prefers an enabled selected option, then the first enabled option", () => {
    expect(listboxInitialFocus(items, ["beta"])).toBe("beta");
    expect(listboxInitialFocus(items, ["disabled"])).toBe("alpha");
    expect(listboxInitialFocus(items, [], true)).toBeNull();
  });
});

describe("listboxTransition", () => {
  test("arrows, Home and End move focus and single selection while skipping disabled options", () => {
    const next = listboxTransition(ctx(), { type: "MOVE", direction: 1 });
    expect(next.context.focusedValue).toBe("alpine");
    expect(next.context.selectedValues).toEqual(["alpine"]);
    expect(next.effects).toContainEqual({ type: "focus", value: "alpine" });
    expect(listboxTransition(next.context, { type: "BOUNDARY", boundary: "last" }).context.focusedValue).toBe("beta");
    expect(listboxTransition(ctx({ focusedValue: "beta" }), { type: "MOVE", direction: 1 }).context.focusedValue).toBe("beta");
  });

  test("multiple mode separates focus from selection and toggles with Space", () => {
    const initial = ctx({ selectionMode: "multiple", selectedValues: ["alpha"] });
    const moved = listboxTransition(initial, { type: "MOVE", direction: 1 });
    expect(moved.context.focusedValue).toBe("alpine");
    expect(moved.context.selectedValues).toEqual(["alpha"]);
    expect(listboxTransition(moved.context, { type: "SPACE" }).context.selectedValues).toEqual(["alpha", "alpine"]);
    expect(listboxTransition(moved.context, { type: "SELECT_ALL" }).context.selectedValues).toEqual(["alpha", "alpine", "beta"]);
  });

  test("typeahead is case-insensitive, cycles repeated letters, and skips disabled options", () => {
    const first = listboxTransition(ctx(), { type: "TYPEAHEAD", character: "A", now: 100 });
    expect(first.context.focusedValue).toBe("alpine");
    const second = listboxTransition(first.context, { type: "TYPEAHEAD", character: "a", now: 180 });
    expect(second.context.focusedValue).toBe("alpha");
    const beta = listboxTransition(second.context, { type: "TYPEAHEAD", character: "b", now: 900 });
    expect(beta.context.focusedValue).toBe("beta");
    const disabledMatch = listboxTransition(ctx(), { type: "TYPEAHEAD", character: "d", now: 1_500 });
    expect(disabledMatch.context.focusedValue).toBe("alpha");
  });

  test("incremental typeahead keeps the current option when it matches the longer buffer", () => {
    const first = listboxTransition(ctx(), { type: "TYPEAHEAD", character: "a", now: 100 });
    expect(first.context.focusedValue).toBe("alpine");
    const next = listboxTransition(first.context, { type: "TYPEAHEAD", character: "l", now: 150 });
    expect(next.context.typeahead).toBe("al");
    expect(next.context.focusedValue).toBe("alpine");
  });

  test("Shift+arrow and Shift+click selection span enabled options from the range anchor", () => {
    const moved = listboxTransition(ctx({
      selectionMode: "multiple",
      focusedValue: "alpha",
      anchorValue: "alpha",
      selectedValues: ["beta"],
    }), { type: "MOVE", direction: 1, extendSelection: true });
    expect(moved.context.focusedValue).toBe("alpine");
    expect(moved.context.selectedValues).toEqual(["alpha", "alpine"]);
    const extended = listboxTransition(moved.context, { type: "SELECT", value: "beta", range: true });
    expect(extended.context.selectedValues).toEqual(["alpha", "alpine", "beta"]);
  });

  test("activation emits only for enabled options, and a disabled listbox is inert", () => {
    expect(listboxTransition(ctx(), { type: "ACTIVATE" }).effects).toEqual([{ type: "activate", value: "alpha" }]);
    expect(listboxTransition(ctx({ focusedValue: "disabled" }), { type: "ACTIVATE" }).effects).toEqual([]);
    expect(listboxTransition(ctx({ disabled: true }), { type: "MOVE", direction: 1 }).effects).toEqual([]);
  });
});
