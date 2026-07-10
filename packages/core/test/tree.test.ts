import { describe, expect, test } from "bun:test";

import {
  flattenVisibleTreeRows,
  treeCheckState,
  treeKeydownIntent,
  treeRangeSelection,
  treeSiblingReorderTarget,
  treeToggleCheck,
  treeVirtualWindow,
  type TreeNodeLike,
} from "../src/tree";

const nodes: TreeNodeLike[] = [
  {
    value: "src",
    children: [
      { value: "src/a.ts" },
      { value: "src/b.ts", isDisabled: true },
      { value: "src/lib", children: [{ value: "src/lib/c.ts" }] },
    ],
  },
  { value: "docs", isBranch: true },
  { value: "README.md" },
];

describe("flattenVisibleTreeRows", () => {
  test("collapsed tree shows only roots", () => {
    expect(flattenVisibleTreeRows(nodes, []).map((row) => row.node.value)).toEqual(["src", "docs", "README.md"]);
  });

  test("expansion reveals children with depth and parent", () => {
    const rows = flattenVisibleTreeRows(nodes, ["src", "src/lib"]);

    expect(rows.map((row) => row.node.value)).toEqual([
      "src", "src/a.ts", "src/b.ts", "src/lib", "src/lib/c.ts", "docs", "README.md",
    ]);
    expect(rows[4]).toMatchObject({ depth: 2, parent: "src/lib" });
  });
});

describe("tri-state cascade", () => {
  test("check state aggregates leaves", () => {
    const src = nodes[0]!;

    expect(treeCheckState(src, [])).toBe("unchecked");
    expect(treeCheckState(src, ["src/a.ts"])).toBe("mixed");
    expect(treeCheckState(src, ["src/a.ts", "src/b.ts", "src/lib/c.ts"])).toBe("checked");
  });

  test("toggle checks the subtree, then clears it", () => {
    const src = nodes[0]!;
    const checked = treeToggleCheck(src, []);

    expect(checked.sort()).toEqual(["src/a.ts", "src/b.ts", "src/lib/c.ts"]);
    expect(treeToggleCheck(src, checked)).toEqual([]);
  });
});

describe("treeRangeSelection", () => {
  const rows = flattenVisibleTreeRows(nodes, ["src"]);

  test("range spans the visible order and skips disabled rows", () => {
    expect(treeRangeSelection(rows, "src", "src/lib")).toEqual(["src", "src/a.ts", "src/lib"]);
  });

  test("reversed anchors normalize; unknown values return null", () => {
    expect(treeRangeSelection(rows, "src/lib", "src")).toEqual(["src", "src/a.ts", "src/lib"]);
    expect(treeRangeSelection(rows, "zz", "src")).toBeNull();
  });
});

describe("treeKeydownIntent", () => {
  const expanded = ["src"];
  const rows = flattenVisibleTreeRows(nodes, expanded);
  const opts = { reorderable: false, expandedValues: expanded };

  test("arrow navigation over visible order with shift extension", () => {
    expect(treeKeydownIntent(rows, "src", "ArrowDown", { altKey: false, shiftKey: false }, opts)).toEqual({
      type: "focus", value: "src/a.ts", extendSelection: false,
    });
    expect(treeKeydownIntent(rows, "src/a.ts", "ArrowDown", { altKey: false, shiftKey: true }, opts)).toEqual({
      type: "focus", value: "src/b.ts", extendSelection: false, // disabled row: no extension
    });
  });

  test("right expands collapsed branch, descends into expanded one", () => {
    expect(treeKeydownIntent(rows, "src/lib", "ArrowRight", { altKey: false, shiftKey: false }, opts)).toEqual({
      type: "expand", value: "src/lib",
    });
    expect(treeKeydownIntent(rows, "src", "ArrowRight", { altKey: false, shiftKey: false }, opts)).toEqual({
      type: "focus", value: "src/a.ts", extendSelection: false,
    });
  });

  test("left collapses or ascends; leaf ascends to parent", () => {
    expect(treeKeydownIntent(rows, "src", "ArrowLeft", { altKey: false, shiftKey: false }, opts)).toEqual({
      type: "collapse", value: "src",
    });
    expect(treeKeydownIntent(rows, "src/a.ts", "ArrowLeft", { altKey: false, shiftKey: false }, opts)).toEqual({
      type: "focusParent", parent: "src",
    });
  });

  test("alt+arrow reorders when reorderable; disabled guards Enter/Space", () => {
    expect(
      treeKeydownIntent(rows, "src", "ArrowDown", { altKey: true, shiftKey: false }, { ...opts, reorderable: true }),
    ).toEqual({ type: "moveSibling", direction: 1 });
    expect(treeKeydownIntent(rows, "src/b.ts", "Enter", { altKey: false, shiftKey: false }, opts)).toBeNull();
    expect(treeKeydownIntent(rows, "src/a.ts", " ", { altKey: false, shiftKey: false }, opts)).toEqual({
      type: "toggleSelection",
    });
  });
});

describe("treeSiblingReorderTarget / treeVirtualWindow", () => {
  test("reorder targets clamp at the ends", () => {
    const sibs = [{ value: "a" }, { value: "b" }, { value: "c" }];

    expect(treeSiblingReorderTarget(sibs, "a", 1)).toEqual({ target: "b", position: "after" });
    expect(treeSiblingReorderTarget(sibs, "c", -1)).toEqual({ target: "b", position: "before" });
    expect(treeSiblingReorderTarget(sibs, "a", -1)).toBeNull();
  });

  test("virtual window computes overscanned slice", () => {
    const window = treeVirtualWindow(1000, 28, 280, 300, 6);

    expect(window.startIndex).toBe(4); // 280/28 = 10, minus overscan
    expect(window.endIndex).toBe(27); // ceil(580/28)=21, plus overscan
    expect(window.offsetY).toBe(4 * 28);
    expect(window.totalHeight).toBe(28000);
  });
});
