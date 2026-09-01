import { describe, expect, test } from "bun:test";

import {
  resolveNestedDropPosition,
  treeResolveDropPosition,
  treeResolveOutlineDrop,
} from "../src/dom/drag-drop-geometry";

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

const outlineRows = [
  { value: "src", depth: 0, parent: null, branch: true },
  { value: "a.ts", depth: 1, parent: "src", branch: false },
  { value: "b.ts", depth: 1, parent: "src", branch: false },
];
const outlineRect = { top: 100, height: 40, left: 0, width: 200 };

describe("treeResolveOutlineDrop", () => {
  test("after the last child, X walks out to after the parent", () => {
    const deep = treeResolveOutlineDrop({
      rows: outlineRows,
      from: "a.ts",
      to: "b.ts",
      y: 130,
      x: 150,
      rect: outlineRect,
    });
    expect(deep).toEqual({ to: "b.ts", position: "after", depth: 1 });

    const shallow = treeResolveOutlineDrop({
      rows: outlineRows,
      from: "a.ts",
      to: "b.ts",
      y: 130,
      x: 40,
      rect: outlineRect,
    });
    expect(shallow).toEqual({ to: "src", position: "after", depth: 0 });
  });

  test("a collapsed last folder nests when X is on the right of the row", () => {
    const rows = [
      { value: "src", depth: 0, parent: null, branch: true },
      { value: "lib", depth: 1, parent: "src", branch: true },
    ];
    expect(
      treeResolveOutlineDrop({
        rows,
        from: "docs",
        to: "lib",
        y: 130,
        x: 170,
        rect: outlineRect,
      }),
    ).toEqual({ to: "lib", position: "inside", depth: 2 });
  });
});
