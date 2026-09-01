import { describe, expect, test } from "bun:test";

import { resolveNestedDropPosition, treeResolveDropPosition } from "../src/dom/drag-drop-geometry";

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

const tree = [
  {
    value: "src",
    children: [{ value: "a.ts" }, { value: "b.ts" }, { value: "lib", isBranch: true }],
  },
  { value: "docs", isBranch: true },
];

describe("treeResolveDropPosition", () => {
  test("a child dropped on its parent un-nests instead of offering inside", () => {
    expect(
      treeResolveDropPosition({
        nodes: tree,
        from: "a.ts",
        to: "src",
        y: 105,
        rect,
        targetIsBranch: true,
      }),
    ).toBe("before");
    expect(
      treeResolveDropPosition({
        nodes: tree,
        from: "a.ts",
        to: "src",
        y: 120,
        rect,
        targetIsBranch: true,
      }),
    ).toBe("after");
  });

  test("same-parent leaves land at the hovered row", () => {
    expect(
      treeResolveDropPosition({
        nodes: tree,
        from: "a.ts",
        to: "b.ts",
        y: 105,
        rect,
        targetIsBranch: false,
      }),
    ).toBe("after");
    expect(
      treeResolveDropPosition({
        nodes: tree,
        from: "b.ts",
        to: "a.ts",
        y: 135,
        rect,
        targetIsBranch: false,
      }),
    ).toBe("before");
  });

  test("a branch that is not the source parent keeps an inside band", () => {
    expect(
      treeResolveDropPosition({
        nodes: tree,
        from: "a.ts",
        to: "docs",
        y: 120,
        rect,
        targetIsBranch: true,
      }),
    ).toBe("inside");
  });
});
