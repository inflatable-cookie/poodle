import { describe, expect, test } from "bun:test";

import { findNextEnabledIndex, firstEnabledIndex } from "../src/nav.ts";

const items = [{ disabled: false }, { disabled: true }, { disabled: false }, { disabled: false }];

describe("findNextEnabledIndex", () => {
  test("skips disabled items", () => {
    expect(findNextEnabledIndex(items, 0, 1)).toBe(2);
  });

  test("wraps in both directions", () => {
    expect(findNextEnabledIndex(items, 3, 1)).toBe(0);
    expect(findNextEnabledIndex(items, 0, -1)).toBe(3);
  });

  test("returns startIndex when nothing else is enabled", () => {
    expect(findNextEnabledIndex([{ disabled: false }, { disabled: true }], 0, 1)).toBe(0);
  });

  test("returns -1 for an empty list", () => {
    expect(findNextEnabledIndex([], 0, 1)).toBe(-1);
  });

  test("honors isDisabled alias", () => {
    expect(findNextEnabledIndex([{ isDisabled: false }, { isDisabled: true }, {}], 0, 1)).toBe(2);
  });
});

describe("firstEnabledIndex", () => {
  test("finds first enabled item", () => {
    expect(firstEnabledIndex([{ disabled: true }, { disabled: false }])).toBe(1);
  });

  test("returns -1 when all disabled", () => {
    expect(firstEnabledIndex([{ disabled: true }])).toBe(-1);
  });
});
