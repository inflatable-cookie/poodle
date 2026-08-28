import { describe, expect, test } from "bun:test";

import {
  filterSelectGroups,
  filterSelectOptions,
  flattenSelectOptions,
  isSelectOptionDisabled,
  selectCommittedQuery,
  selectFreeformEnabled,
  selectOpenHighlightIndex,
  selectOpenHighlightValue,
  selectTransition,
  type SelectContext,
} from "../src/select.ts";

const flat = [
  { value: "a", label: "Apple" },
  { value: "b", label: "Banana", disabled: true },
  { value: "c", label: "Cherry" },
  { value: "d", label: "Date", isDisabled: true },
];

const grouped = [
  { label: "Fruit", options: [flat[0]!, flat[1]!] },
  { label: "Stone", options: [flat[2]!] },
];

describe("option helpers", () => {
  test("flatten handles grouped, flat, and empty", () => {
    expect(flattenSelectOptions(grouped).map((option) => option.value)).toEqual(["a", "b", "c"]);
    expect(flattenSelectOptions(flat).map((option) => option.value)).toEqual(["a", "b", "c", "d"]);
    expect(flattenSelectOptions([])).toEqual([]);
  });

  test("disabled honors both flags", () => {
    expect(isSelectOptionDisabled(flat[1]!)).toBe(true);
    expect(isSelectOptionDisabled(flat[3]!)).toBe(true);
    expect(isSelectOptionDisabled(flat[0]!)).toBe(false);
  });

  test("filter excludes disabled always; query matches label case-insensitively", () => {
    expect(filterSelectOptions(flat, "").map((option) => option.value)).toEqual(["a", "c"]);
    expect(filterSelectOptions(flat, "HERR").map((option) => option.value)).toEqual(["c"]);
    expect(filterSelectOptions(flat, "zzz")).toEqual([]);
  });

  test("group filter keeps matching options and drops empty groups", () => {
    const filtered = filterSelectGroups(grouped, "app");
    expect(filtered).toHaveLength(1);
    expect(filtered[0]?.options.map((option) => option.value)).toEqual(["a"]);
  });
});

describe("selectOpenHighlightIndex", () => {
  const filtered = [flat[0]!, flat[2]!];

  test("selected option wins; missing or null selection falls back to 0", () => {
    expect(selectOpenHighlightIndex(filtered, "c")).toBe(1);
    expect(selectOpenHighlightIndex(filtered, "zz")).toBe(0);
    expect(selectOpenHighlightIndex(filtered, null)).toBe(0);
  });
});

function ctx(overrides: Partial<SelectContext> = {}): SelectContext {
  return {
    value: "",
    open: false,
    query: "",
    highlightedValue: null,
    options: [
      { value: "a", label: "Apple", disabled: false },
      { value: "b", label: "Banana", disabled: true },
      { value: "c", label: "Cherry", disabled: false },
    ],
    clearValue: "",
    searchable: false,
    freeform: false,
    disabled: false,
    ...overrides,
  };
}

describe("select helpers", () => {
  test("freeform is effective only with searchable", () => {
    expect(selectFreeformEnabled(ctx({ freeform: true }))).toBe(false);
    expect(selectFreeformEnabled(ctx({ searchable: true, freeform: true }))).toBe(true);
  });

  test("open highlight uses the selected enabled option, else the first enabled, else null", () => {
    expect(selectOpenHighlightValue(ctx({ value: "c" }))).toBe("c");
    expect(selectOpenHighlightValue(ctx())).toBe("a");
    expect(
      selectOpenHighlightValue(
        ctx({
          options: [{ value: "b", label: "Banana", disabled: true }],
        }),
      ),
    ).toBeNull();
  });

  test("committed query is the selected label, else empty", () => {
    expect(selectCommittedQuery(ctx({ value: "c" }))).toBe("Cherry");
    expect(selectCommittedQuery(ctx())).toBe("");
  });
});

describe("selectTransition", () => {
  test("query edits do not report value changes", () => {
    const result = selectTransition(ctx({ searchable: true, freeform: true }), {
      type: "QUERY",
      query: "kiw",
    });

    expect(result.context.value).toBe("");
    expect(result.effects.map((effect) => effect.type)).toEqual(["openChanged", "queryChanged"]);
  });

  test("Home and End while closed are inert", () => {
    expect(selectTransition(ctx(), { type: "HIGHLIGHT_FIRST" }).effects).toEqual([]);
    expect(selectTransition(ctx(), { type: "HIGHLIGHT_LAST" }).effects).toEqual([]);
  });
});
