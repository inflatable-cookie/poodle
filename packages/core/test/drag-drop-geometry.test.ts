import { describe, expect, test } from "bun:test";

import { resolveNestedDropPosition } from "../src/dom/drag-drop-geometry";

const rect = { top: 100, height: 40 };

describe("resolveNestedDropPosition", () => {
  test("an item is before above the midline and after below it", () => {
    expect(resolveNestedDropPosition({ y: 110, rect, kind: "item" })).toBe("before");
    expect(resolveNestedDropPosition({ y: 130, rect, kind: "item" })).toBe("after");
  });

  test("a container keeps a middle inside band", () => {
    expect(resolveNestedDropPosition({ y: 105, rect, kind: "container" })).toBe("before");
    expect(resolveNestedDropPosition({ y: 120, rect, kind: "container" })).toBe("inside");
    expect(resolveNestedDropPosition({ y: 135, rect, kind: "container" })).toBe("after");
  });

  test("a zero-height or out-of-rect point yields null", () => {
    expect(resolveNestedDropPosition({ y: 120, rect: { top: 100, height: 0 }, kind: "container" })).toBeNull();
    expect(resolveNestedDropPosition({ y: 90, rect, kind: "item" })).toBeNull();
    expect(resolveNestedDropPosition({ y: 150, rect, kind: "item" })).toBeNull();
  });
});
