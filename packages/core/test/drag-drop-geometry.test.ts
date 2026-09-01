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
  test("a child dropped on its parent can append inside or un-nest", () => {
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
    ).toBe("inside");
    expect(
      treeResolveDropPosition({
        nodes: tree,
        from: "a.ts",
        to: "src",
        y: 135,
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
const indent = { indentPx: 16, gutterPx: 0 };

describe("treeResolveOutlineDrop", () => {
  test("after the last child, X walks out to after the parent", () => {
    const deep = treeResolveOutlineDrop({
      rows: outlineRows,
      from: "a.ts",
      to: "b.ts",
      y: 130,
      x: 32,
      rect: outlineRect,
      ...indent,
    });
    expect(deep).toEqual({ to: "b.ts", position: "after", depth: 1, indicator: "after" });
    expect(
      treeResolveOutlineDrop({
        rows: outlineRows,
        from: "a.ts",
        to: "a.ts",
        y: 120,
        x: 32,
        rect: outlineRect,
        ...indent,
      }),
    ).toBeNull();

    const lastChild = treeResolveOutlineDrop({
      rows: outlineRows,
      from: "b.ts",
      to: "b.ts",
      y: 120,
      x: 0,
      rect: outlineRect,
      ...indent,
    });
    expect(lastChild).toEqual({ to: "src", position: "after", depth: 0, indicator: "after" });
    expect(
      treeResolveOutlineDrop({
        rows: outlineRows,
        from: "b.ts",
        to: "b.ts",
        y: 120,
        x: 32,
        rect: outlineRect,
        ...indent,
      }),
    ).toEqual({ to: "b.ts", position: "after", depth: 1, indicator: "after" });

    const shallow = treeResolveOutlineDrop({
      rows: outlineRows,
      from: "a.ts",
      to: "b.ts",
      y: 130,
      x: 0,
      rect: outlineRect,
      ...indent,
    });
    expect(shallow).toEqual({ to: "src", position: "after", depth: 0, indicator: "after" });
  });

  test("the last nested row in the tree X-walks out to root on itself", () => {
    const rows = [
      { value: "docs", depth: 0, parent: null, branch: true },
      { value: "api", depth: 1, parent: "docs", branch: true },
      { value: "tree.md", depth: 2, parent: "api", branch: false },
    ];
    const rect = { top: 180, height: 40, left: 0, width: 400 };
    expect(
      treeResolveOutlineDrop({
        rows,
        from: "tree.md",
        to: "tree.md",
        y: 200,
        x: 0,
        rect,
        ...indent,
      }),
    ).toEqual({ to: "docs", position: "after", depth: 0, indicator: "after" });
    expect(
      treeResolveOutlineDrop({
        rows,
        from: "tree.md",
        to: "tree.md",
        y: 200,
        x: 32,
        rect,
        ...indent,
      }),
    ).toEqual({ to: "api", position: "after", depth: 1, indicator: "after" });
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
        y: 136,
        x: 64,
        rect: outlineRect,
        ...indent,
      }),
    ).toEqual({ to: "lib", position: "inside", depth: 2, indicator: "after" });
  });

  test("the gap under an open folder header is before the first child, not inside", () => {
    const rows = [
      { value: "docs", depth: 0, parent: null, branch: true },
      { value: "intro.md", depth: 1, parent: "docs", branch: false },
      { value: "guide.md", depth: 1, parent: "docs", branch: false },
      { value: "notes.txt", depth: 0, parent: null, branch: false },
    ];
    const docsRect = { top: 100, height: 40, left: 0, width: 400 };
    const introRect = { top: 140, height: 40, left: 0, width: 400 };
    expect(
      treeResolveOutlineDrop({
        rows,
        from: "notes.txt",
        to: "docs",
        y: 136,
        x: 150,
        rect: docsRect,
        ...indent,
      }),
    ).toEqual({ to: "intro.md", position: "before", depth: 1, indicator: "after" });
    expect(
      treeResolveOutlineDrop({
        rows,
        from: "notes.txt",
        to: "intro.md",
        y: 145,
        x: 150,
        rect: introRect,
        ...indent,
      }),
    ).toEqual({ to: "intro.md", position: "before", depth: 1, indicator: "before" });
  });

  test("dropping on the next sibling folder appends inside it", () => {
    const rows = [
      { value: "docs", depth: 0, parent: null, branch: true },
      { value: "guide.md", depth: 1, parent: "docs", branch: false },
      { value: "assets", depth: 0, parent: null, branch: true },
      { value: "logo.svg", depth: 1, parent: "assets", branch: false },
    ];
    const assetsRect = { top: 180, height: 40, left: 0, width: 400 };
    expect(
      treeResolveOutlineDrop({
        rows,
        from: "guide.md",
        to: "assets",
        y: 200,
        x: 150,
        rect: assetsRect,
        ...indent,
      }),
    ).toEqual({ to: "assets", position: "inside", depth: 1, indicator: "inside" });
  });

  test("the origin gap X-walks from the dragged row and the last child, independent of Y", () => {
    const rows = [
      { value: "docs", depth: 0, parent: null, branch: true },
      { value: "intro.md", depth: 1, parent: "docs", branch: false },
      { value: "guide.md", depth: 1, parent: "docs", branch: false },
      { value: "notes.txt", depth: 0, parent: null, branch: false },
    ];
    const notesRect = { top: 220, height: 40, left: 0, width: 400 };
    const guideRect = { top: 180, height: 40, left: 0, width: 400 };

    for (const [to, y, rect, indicator] of [
      ["notes.txt", 240, notesRect, "before"],
      ["guide.md", 185, guideRect, "after"],
      ["guide.md", 210, guideRect, "after"],
    ] as const) {
      expect(
        treeResolveOutlineDrop({
          rows,
          from: "notes.txt",
          to,
          y,
          x: 0,
          rect,
          ...indent,
        }),
      ).toEqual({ to: "docs", position: "after", depth: 0, indicator });
      expect(
        treeResolveOutlineDrop({
          rows,
          from: "notes.txt",
          to,
          y,
          x: 32,
          rect,
          ...indent,
        }),
      ).toEqual({ to: "guide.md", position: "after", depth: 1, indicator });
    }
  });

  test("the gap above an open folder stays before the folder at any X", () => {
    const above = [
      { value: "notes.txt", depth: 0, parent: null, branch: false },
      { value: "docs", depth: 0, parent: null, branch: true },
      { value: "intro.md", depth: 1, parent: "docs", branch: false },
      { value: "guide.md", depth: 1, parent: "docs", branch: false },
    ];
    const below = [
      { value: "docs", depth: 0, parent: null, branch: true },
      { value: "intro.md", depth: 1, parent: "docs", branch: false },
      { value: "guide.md", depth: 1, parent: "docs", branch: false },
      { value: "notes.txt", depth: 0, parent: null, branch: false },
    ];
    const notesRect = { top: 100, height: 40, left: 0, width: 400 };
    const docsRect = { top: 140, height: 40, left: 0, width: 400 };

    for (const [to, y, rect, indicator] of [
      ["notes.txt", 120, notesRect, "after"],
      ["docs", 145, docsRect, "before"],
    ] as const) {
      for (const x of [0, 32, 150]) {
        expect(
          treeResolveOutlineDrop({
            rows: above,
            from: "notes.txt",
            to,
            y,
            x,
            rect,
            ...indent,
          }),
        ).toEqual({ to: "docs", position: "before", depth: 0, indicator });
      }
    }

    expect(
      treeResolveOutlineDrop({
        rows: above,
        from: "notes.txt",
        to: "docs",
        y: 160,
        x: 150,
        rect: docsRect,
        ...indent,
      }),
    ).toEqual({ to: "docs", position: "inside", depth: 1, indicator: "inside" });

    expect(
      treeResolveOutlineDrop({
        rows: below,
        from: "notes.txt",
        to: "docs",
        y: 145,
        x: 150,
        rect: docsRect,
        ...indent,
      }),
    ).toEqual({ to: "docs", position: "before", depth: 0, indicator: "before" });
  });
});
