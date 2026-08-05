import { describe, expect, test } from "bun:test";

import {
  filterSelectGroups,
  filterSelectOptions,
  flattenSelectOptions,
  isSelectOptionDisabled,
  selectOpenHighlightIndex,
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
