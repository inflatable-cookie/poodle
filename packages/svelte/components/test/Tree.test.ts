import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { tick } from "svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import Tree from "../src/Tree.svelte";
import type { TreeReorderAuthority, TreeReorderCandidate } from "@inflatable-cookie/poodle-core";

const nested = [
  {
    value: "src",
    label: "src",
    children: [
      { value: "a.ts", label: "a.ts" },
      { value: "lib", label: "lib", children: [{ value: "c.ts", label: "c.ts" }] },
    ],
  },
  { value: "docs", label: "docs", isBranch: true },
];

const files = [
  { value: "a.ts", label: "a.ts" },
  { value: "b.ts", label: "b.ts" },
  { value: "c.ts", label: "c.ts" },
];

function asRect(top: number, height = 40): DOMRect {
  return {
    x: 10,
    y: top,
    width: 200,
    height,
    top,
    left: 10,
    right: 210,
    bottom: top + height,
    toJSON: () => ({}),
  } as DOMRect;
}

function pointer(type: "pointerdown" | "pointermove" | "pointerup" | "pointercancel", init: PointerEventInit): PointerEvent {
  return new PointerEvent(type, {
    bubbles: true,
    cancelable: true,
    pointerId: 1,
    pointerType: "mouse",
    button: 0,
    buttons: type === "pointerdown" || type === "pointermove" ? 1 : 0,
    isPrimary: true,
    clientX: 120,
    clientY: 0,
    ...init,
  });
}

function layoutTree(container: HTMLElement): Map<string, HTMLElement> {
  const rows = new Map<string, HTMLElement>();
  for (const [index, item] of [...container.querySelectorAll<HTMLElement>('[role="treeitem"][data-value]')].entries()) {
    const row = item.querySelector<HTMLElement>(".poodle-tree__row") ?? item;
    const rect = asRect(10 + index * 40);
    item.getBoundingClientRect = () => rect;
    row.getBoundingClientRect = () => rect;
    row.setPointerCapture = vi.fn();
    row.releasePointerCapture = vi.fn();
    item.setPointerCapture = vi.fn();
    rows.set(item.dataset.value ?? "", row);
  }
  return rows;
}

function drag(from: HTMLElement, to: HTMLElement, y: number): void {
  from.dispatchEvent(pointer("pointerdown", { clientY: from.getBoundingClientRect().top + 4 }));
  document.dispatchEvent(pointer("pointermove", { clientY: y }));
  document.dispatchEvent(pointer("pointerup", { clientY: y }));
}

describe("Tree row metadata", () => {
  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.useRealTimers();
  });

  it("renders an end label while muted rows remain selectable", async () => {
    const onSelectionChange = vi.fn();
    const { getByRole, getByText } = render(Tree, {
      props: {
        nodes: [{ value: "empty", label: "Empty area", endLabel: "0", isMuted: true }],
        onSelectionChange,
      },
    });

    const row = getByRole("treeitem");
    expect(row.getAttribute("data-muted")).toBe("true");
    expect(row.hasAttribute("aria-disabled")).toBe(false);
    expect(getByText("0").classList.contains("poodle-tree__end-label")).toBe(true);

    await fireEvent.click(row);
    expect(onSelectionChange).toHaveBeenCalledWith(["empty"]);
  });

  it("maps a child-on-parent drop to before, inside, or after", async () => {
    const onReorder = vi.fn();
    const { container, rerender } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("src")!;
    const top = target.getBoundingClientRect().top;

    drag(source, target, top + 4);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "before");

    await rerender({ nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder });
    onReorder.mockClear();
    const rowsInside = layoutTree(container);
    drag(rowsInside.get("a.ts")!, rowsInside.get("src")!, top + 20);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "inside");

    await rerender({ nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder });
    onReorder.mockClear();
    const rowsAfter = layoutTree(container);
    drag(rowsAfter.get("a.ts")!, rowsAfter.get("src")!, top + 36);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "after");
  });

  it("appends into the next sibling folder when dropped on the folder row", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("lib")!;
    drag(source, target, target.getBoundingClientRect().top + 20);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "lib", "inside");
  });

  it("lands a sibling leaf at the hovered row even on the origin-facing half", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "src",
        label: "src",
        children: [
          { value: "a.ts", label: "a.ts" },
          { value: "b.ts", label: "b.ts" },
        ],
      },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["src"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("b.ts")!;
    drag(source, target, target.getBoundingClientRect().top + 4);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "b.ts", "after");
  });

  it("un-nests after an open parent when the pointer is left on the last child", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "src",
        label: "src",
        children: [
          { value: "a.ts", label: "a.ts" },
          { value: "b.ts", label: "b.ts" },
        ],
      },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["src"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("b.ts")!;
    const y = target.getBoundingClientRect().top + 36;
    source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 12, clientY: y }));
    document.dispatchEvent(pointer("pointerup", { clientX: 12, clientY: y }));
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "after");
  });

  it("un-nests the last nested row from itself without leaving it", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "src",
        label: "src",
        children: [
          { value: "a.ts", label: "a.ts" },
          { value: "b.ts", label: "b.ts" },
        ],
      },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["src"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("b.ts")!;
    const y = source.getBoundingClientRect().top + 20;
    source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 150 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 12 }));
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 12 }));
    expect(onReorder).toHaveBeenCalledWith("b.ts", "src", "after");
  });

  it("indents into the folder above from the dragged row without leaving it", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "docs",
        label: "docs",
        children: [
          { value: "intro.md", label: "intro.md" },
          { value: "guide.md", label: "guide.md" },
        ],
      },
      { value: "notes.txt", label: "notes.txt" },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["docs"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "guide.md", "after");
  });

  it("keeps the origin gap at root when the pointer stays left on the dragged row", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "docs",
        label: "docs",
        children: [
          { value: "intro.md", label: "intro.md" },
          { value: "guide.md", label: "guide.md" },
        ],
      },
      { value: "notes.txt", label: "notes.txt" },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["docs"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 150 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 40 }));
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 40 }));
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "docs", "after");
  });

  it("updates drop indent while the pointer stays on the dragged row", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "docs",
        label: "docs",
        children: [
          { value: "intro.md", label: "intro.md" },
          { value: "guide.md", label: "guide.md" },
        ],
      },
      { value: "notes.txt", label: "notes.txt" },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["docs"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const item = container.querySelector<HTMLElement>('[role="treeitem"][data-value="notes.txt"]')!;
    const y = source.getBoundingClientRect().top + 20;
    source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("1");
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 40 }));
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("0");
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 40 }));
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "docs", "after");
  });

  it("nests from the last child's origin-facing half at the same X", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "docs",
        label: "docs",
        children: [
          { value: "intro.md", label: "intro.md" },
          { value: "guide.md", label: "guide.md" },
        ],
      },
      { value: "notes.txt", label: "notes.txt" },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["docs"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const lastChild = rows.get("guide.md")!;
    const startY = source.getBoundingClientRect().top + 20;
    const y = lastChild.getBoundingClientRect().top + 4;
    source.dispatchEvent(pointer("pointerdown", { clientY: startY, clientX: 40 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "guide.md", "after");
  });

  it("does not indent from the row above an open folder", async () => {
    const onReorder = vi.fn();
    const nodes = [
      { value: "notes.txt", label: "notes.txt" },
      {
        value: "docs",
        label: "docs",
        children: [
          { value: "intro.md", label: "intro.md" },
          { value: "guide.md", label: "guide.md" },
        ],
      },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["docs"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "docs", "before");
  });

  it("does not indent when dragged above an open folder from below", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "docs",
        label: "docs",
        children: [
          { value: "intro.md", label: "intro.md" },
          { value: "guide.md", label: "guide.md" },
        ],
      },
      { value: "notes.txt", label: "notes.txt" },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["docs"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const target = rows.get("docs")!;
    const startY = source.getBoundingClientRect().top + 20;
    const y = target.getBoundingClientRect().top + 4;
    source.dispatchEvent(pointer("pointerdown", { clientY: startY, clientX: 40 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
    const item = container.querySelector<HTMLElement>('[role="treeitem"][data-value="docs"]')!;
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("0");
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "docs", "before");
  });

  it("inserts at the top of a folder when dropped on the first nested row", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "docs",
        label: "docs",
        children: [
          { value: "intro.md", label: "intro.md" },
          { value: "guide.md", label: "guide.md" },
        ],
      },
      { value: "notes.txt", label: "notes.txt" },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["docs"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const firstChild = rows.get("intro.md")!;
    const folder = rows.get("docs")!;
    const startY = source.getBoundingClientRect().top + 20;

    source.dispatchEvent(pointer("pointerdown", { clientY: startY, clientX: 40 }));
    document.dispatchEvent(pointer("pointermove", { clientY: firstChild.getBoundingClientRect().top + 4, clientX: 150 }));
    document.dispatchEvent(pointer("pointerup", { clientY: firstChild.getBoundingClientRect().top + 4, clientX: 150 }));
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "intro.md", "before");

    onReorder.mockClear();
    source.dispatchEvent(pointer("pointerdown", { clientY: startY, clientX: 40 }));
    document.dispatchEvent(pointer("pointermove", { clientY: folder.getBoundingClientRect().top + 36, clientX: 150 }));
    document.dispatchEvent(pointer("pointerup", { clientY: folder.getBoundingClientRect().top + 36, clientX: 150 }));
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "intro.md", "before");
  });

  it("rejects a drop into the source subtree and after the source is removed", async () => {
    const onReorder = vi.fn();
    const { container, rerender } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("src")!;
    const child = rows.get("c.ts")!;
    drag(source, child, child.getBoundingClientRect().top + 8);
    expect(onReorder).not.toHaveBeenCalled();

    const live = layoutTree(container);
    live.get("a.ts")!.dispatchEvent(pointer("pointerdown", { clientY: 14 }));
    document.dispatchEvent(pointer("pointermove", { clientY: 54 }));
    await rerender({
      nodes: [nested[1]!],
      expandedValues: ["src", "lib"],
      reorderable: true,
      onReorder,
    });
    document.dispatchEvent(pointer("pointerup", { clientY: 54 }));
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("keeps Space selection and Alt+Arrow sibling reorder off the HTML drag path", async () => {
    const onReorder = vi.fn();
    const onSelectionChange = vi.fn();
    const { container } = render(Tree, {
      props: {
        nodes: nested,
        expandedValues: ["src"],
        reorderable: true,
        onReorder,
        onSelectionChange,
      },
    });
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    await fireEvent.keyDown(item, { key: " " });
    expect(onSelectionChange).toHaveBeenCalledWith(["a.ts"]);
    expect(onReorder).not.toHaveBeenCalled();

    await fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(onReorder).toHaveBeenCalledWith("a.ts", "lib", "after");
    expect(document.activeElement).toBe(item);
  });

  it("keeps one roving treeitem tab stop when rows are reorderable", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src"], reorderable: true, onReorder },
    });
    const items = [...container.querySelectorAll<HTMLElement>('[role="treeitem"][data-value]')];
    const rows = [...container.querySelectorAll<HTMLElement>(".poodle-tree__row")];
    expect(rows.length).toBeGreaterThan(0);
    expect(rows.every((row) => row.tabIndex < 0)).toBe(true);
    expect(items.filter((item) => item.tabIndex >= 0)).toHaveLength(1);

    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    await fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(onReorder).toHaveBeenCalledWith("a.ts", "lib", "after");
    expect(document.activeElement).toBe(item);
  });

  it("returns pointer-commit focus to the treeitem, not the row handle", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    drag(rows.get("a.ts")!, rows.get("lib")!, rows.get("lib")!.getBoundingClientRect().top + 36);
    expect(onReorder).toHaveBeenCalled();
    expect(document.activeElement).toBe(item);
    expect([...container.querySelectorAll<HTMLElement>(".poodle-tree__row")].every((row) => row.tabIndex < 0)).toBe(
      true,
    );
    expect(
      [...container.querySelectorAll<HTMLElement>('[role="treeitem"][data-value]')].filter((node) => node.tabIndex >= 0),
    ).toHaveLength(1);
  });

  it("returns pointer-cancel focus to the treeitem, not the row handle", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    rows.get("a.ts")!.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
    document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
    document.dispatchEvent(pointer("pointercancel", { clientY: 90 }));
    expect(onReorder).not.toHaveBeenCalled();
    expect(document.activeElement).toBe(item);
    expect([...container.querySelectorAll<HTMLElement>(".poodle-tree__row")].every((row) => row.tabIndex < 0)).toBe(
      true,
    );
    expect(
      [...container.querySelectorAll<HTMLElement>('[role="treeitem"][data-value]')].filter((node) => node.tabIndex >= 0),
    ).toHaveLength(1);
  });

  it("does not Alt+Arrow reorder onto a disabled sibling", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: {
        nodes: [
          {
            value: "src",
            label: "src",
            children: [
              { value: "a.ts", label: "a.ts" },
              { value: "lib", label: "lib", isDisabled: true },
            ],
          },
        ],
        expandedValues: ["src"],
        reorderable: true,
        onReorder,
      },
    });
    layoutTree(container);
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    await fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not start a drag from a checkbox or rename field", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: {
        nodes: nested,
        expandedValues: ["src"],
        reorderable: true,
        showCheckboxes: true,
        editingValue: "a.ts",
        onReorder,
      },
    });
    layoutTree(container);
    const checkbox = container.querySelector("input[type='checkbox']") as HTMLInputElement;
    checkbox.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
    document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
    document.dispatchEvent(pointer("pointerup", { clientY: 90 }));
    expect(onReorder).not.toHaveBeenCalled();

    const rename = container.querySelector(".poodle-tree__rename") as HTMLInputElement;
    rename.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
    document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
    document.dispatchEvent(pointer("pointerup", { clientY: 90 }));
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("selects a reorderable row on click without committing a drag", async () => {
    const onReorder = vi.fn();
    const onSelectionChange = vi.fn();
    const { container } = render(Tree, {
      props: {
        nodes: nested,
        expandedValues: ["src"],
        reorderable: true,
        onReorder,
        onSelectionChange,
      },
    });
    const rows = layoutTree(container);
    const row = rows.get("a.ts")!;
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    const provider = container.querySelector<HTMLElement>(".poodle-drag-drop-provider")!;
    const y = row.getBoundingClientRect().top + 4;

    row.dispatchEvent(pointer("pointerdown", { clientY: y }));
    expect(provider.style.getPropertyValue("user-select")).toBe("none");
    document.dispatchEvent(pointer("pointerup", { clientY: y }));
    expect(provider.style.getPropertyValue("user-select")).toBe("");

    await fireEvent.click(item);
    expect(onSelectionChange).toHaveBeenCalledTimes(1);
    expect(onSelectionChange).toHaveBeenCalledWith(["a.ts"]);
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not suppress root selection when pressing a rename field", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: {
        nodes: nested,
        expandedValues: ["src"],
        reorderable: true,
        editingValue: "a.ts",
        onReorder,
      },
    });
    layoutTree(container);
    const provider = container.querySelector<HTMLElement>(".poodle-drag-drop-provider")!;
    const rename = container.querySelector(".poodle-tree__rename") as HTMLInputElement;

    rename.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
    expect(provider.style.getPropertyValue("user-select")).toBe("");
    rename.setSelectionRange(0, 2);
    expect(rename.selectionStart).toBe(0);
    expect(rename.selectionEnd).toBe(2);
    document.dispatchEvent(pointer("pointerup", { clientY: 54 }));
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not select a row from an activated drag's compatibility click", async () => {
    const onReorder = vi.fn();
    const onSelectionChange = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: files, reorderable: true, onReorder, onSelectionChange },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;

    drag(source, rows.get("c.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    item.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    expect(onReorder).toHaveBeenCalledTimes(1);
    expect(onSelectionChange).not.toHaveBeenCalled();
  });

  it("does not select after rejected, failed, or cancelled activated drags", async () => {
    const onSelectionChange = vi.fn();
    const rejected = authority({
      onDrop: () => ({ status: "rejected", reason: "occupied" }),
    });
    const { container, unmount } = render(Tree, {
      props: { nodes: files, reorderable: true, reorderAuthority: rejected, onSelectionChange },
    });
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    container.querySelector<HTMLElement>('[data-value="a.ts"]')!.dispatchEvent(
      new MouseEvent("click", { bubbles: true, cancelable: true }),
    );
    expect(onSelectionChange).not.toHaveBeenCalled();
    unmount();

    const failed = authority({
      onDrop: () => ({ status: "failed", reason: "io" }),
    });
    const again = render(Tree, {
      props: { nodes: files, reorderable: true, reorderAuthority: failed, onSelectionChange },
    });
    const failedRows = layoutTree(again.container);
    drag(failedRows.get("a.ts")!, failedRows.get("c.ts")!, failedRows.get("c.ts")!.getBoundingClientRect().top + 4);
    again.container.querySelector<HTMLElement>('[data-value="a.ts"]')!.dispatchEvent(
      new MouseEvent("click", { bubbles: true, cancelable: true }),
    );
    expect(onSelectionChange).not.toHaveBeenCalled();
    again.unmount();

    const onReorder = vi.fn();
    const cancelled = render(Tree, {
      props: { nodes: files, reorderable: true, onReorder, onSelectionChange },
    });
    const cancelRows = layoutTree(cancelled.container);
    const source = cancelRows.get("a.ts")!;
    source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
    document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
    document.dispatchEvent(pointer("pointerup", { clientY: 800 }));
    cancelled.container.querySelector<HTMLElement>('[data-value="a.ts"]')!.dispatchEvent(
      new MouseEvent("click", { bubbles: true, cancelable: true }),
    );
    expect(onReorder).not.toHaveBeenCalled();
    expect(onSelectionChange).not.toHaveBeenCalled();
    cancelled.unmount();
  });

  it("still selects after a stale activated-drag click guard expires", async () => {
    const onReorder = vi.fn();
    const onSelectionChange = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: files, reorderable: true, onReorder, onSelectionChange },
    });
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(onReorder).toHaveBeenCalledTimes(1);
    await new Promise((resolve) => setTimeout(resolve, 0));
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    await fireEvent.click(item);
    expect(onSelectionChange).toHaveBeenCalledTimes(1);
    expect(onSelectionChange).toHaveBeenCalledWith(["a.ts"]);
  });

  it("does not select from a compatibility click before an async drop settles", async () => {
    const onSelectionChange = vi.fn();
    let finish: ((result: { status: "committed" }) => void) | undefined;
    const host = authority({
      onDrop: () =>
        new Promise((resolve) => {
          finish = resolve;
        }),
    });
    const { container } = render(Tree, {
      props: { nodes: files, reorderable: true, reorderAuthority: host, onSelectionChange },
    });
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    container.querySelector<HTMLElement>('[data-value="a.ts"]')!.dispatchEvent(
      new MouseEvent("click", { bubbles: true, cancelable: true }),
    );
    expect(onSelectionChange).not.toHaveBeenCalled();
    finish?.({ status: "committed" });
    await tick();
    expect(onSelectionChange).not.toHaveBeenCalled();
  });

  it("prefers a nested descendant when ancestor and descendant both contain the pointer", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const srcItem = container.querySelector<HTMLElement>('[data-value="src"]')!;
    const childItem = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    srcItem.getBoundingClientRect = () => asRect(10, 80);
    rows.get("src")!.getBoundingClientRect = () => asRect(10, 80);
    childItem.getBoundingClientRect = () => asRect(50, 40);
    rows.get("a.ts")!.getBoundingClientRect = () => asRect(50, 40);

    drag(rows.get("docs")!, rows.get("a.ts")!, 70);
    expect(onReorder).toHaveBeenCalledTimes(1);
    expect(onReorder).toHaveBeenCalledWith("docs", "a.ts", "after");
  });

  it("rejects a drop when the live target is disabled or removed before release", async () => {
    const onReorder = vi.fn();
    const { container, rerender } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("docs")!;
    source.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
    document.dispatchEvent(pointer("pointermove", { clientY: target.getBoundingClientRect().top + 8 }));
    await rerender({
      nodes: [nested[0]!, { value: "docs", label: "docs", isBranch: true, isDisabled: true }],
      expandedValues: ["src", "lib"],
      reorderable: true,
      onReorder,
    });
    layoutTree(container);
    document.dispatchEvent(pointer("pointerup", { clientY: 178 }));
    expect(onReorder).not.toHaveBeenCalled();

    const live = layoutTree(container);
    live.get("a.ts")!.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
    document.dispatchEvent(pointer("pointermove", { clientY: 178 }));
    await rerender({
      nodes: [nested[0]!],
      expandedValues: ["src", "lib"],
      reorderable: true,
      onReorder,
    });
    document.dispatchEvent(pointer("pointerup", { clientY: 178 }));
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("revalidates live Tree state at drop so a newly illegal nest does not commit", async () => {
    const onReorder = vi.fn();
    const { container, rerender } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("docs")!;
    const target = rows.get("src")!;
    source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
    document.dispatchEvent(pointer("pointermove", { clientY: target.getBoundingClientRect().top + 20 }));
    await rerender({
      nodes: [{ value: "docs", label: "docs", isBranch: true, children: [nested[0]!] }],
      expandedValues: ["src", "lib", "docs"],
      reorderable: true,
      onReorder,
    });
    layoutTree(container);
    document.dispatchEvent(pointer("pointerup", { clientY: 30 }));
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not start a drag from the expansion twisty", async () => {
    const onReorder = vi.fn();
    const onExpandedChange = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: nested, reorderable: true, onReorder, onExpandedChange },
    });
    layoutTree(container);
    const twisty = container.querySelector<HTMLElement>('[data-value="src"] .poodle-tree__twisty')!;
    twisty.dispatchEvent(pointer("pointerdown", { clientY: 14 }));
    document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
    document.dispatchEvent(pointer("pointerup", { clientY: 90 }));
    expect(onReorder).not.toHaveBeenCalled();

    await fireEvent.click(twisty);
    expect(onExpandedChange).toHaveBeenCalledWith(["src"]);
  });

  it("pins a virtualized source so it is not unmounted mid-drag", async () => {
    const nodes = Array.from({ length: 40 }, (_, index) => ({
      value: `n${index}`,
      label: `Node ${index}`,
    }));
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: { nodes, reorderable: true, virtualized: true, virtualHeight: 80, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("n0")!;
    source.dispatchEvent(pointer("pointerdown", { clientY: 14 }));
    document.dispatchEvent(pointer("pointermove", { clientY: 30 }));
    const scroller = container.querySelector(".poodle-tree") as HTMLElement;
    Object.defineProperty(scroller, "scrollTop", { configurable: true, value: 600 });
    scroller.dispatchEvent(new Event("scroll", { bubbles: true }));
    expect(container.querySelector('[data-value="n0"]')).not.toBeNull();
    document.dispatchEvent(pointer("pointercancel", { clientY: 30 }));
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not commit an expanded folder into its own first child", () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("src")!;
    const rect = source.getBoundingClientRect();
    const y = rect.top + rect.height - 2;
    source.dispatchEvent(pointer("pointerdown", { clientY: rect.top + 4 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y }));
    document.dispatchEvent(pointer("pointerup", { clientY: y }));
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("announces the committed destination, not the hovered row, on an origin-gap drop", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "docs",
        label: "docs",
        children: [
          { value: "intro.md", label: "intro.md" },
          { value: "guide.md", label: "guide.md" },
        ],
      },
      { value: "notes.txt", label: "notes.txt" },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["docs"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    await tick();
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "guide.md", "after");
    const live = container.querySelector(".poodle-drag-live-region")?.textContent ?? "";
    expect(live).toContain("guide.md");
    expect(live).not.toMatch(/on notes\.txt$/);
  });

  it("does not commit a remapped drop onto a disabled destination", async () => {
    const onReorder = vi.fn();
    const nodes = [
      {
        value: "docs",
        label: "docs",
        children: [
          { value: "intro.md", label: "intro.md" },
          { value: "guide.md", label: "guide.md", isDisabled: true },
        ],
      },
      { value: "notes.txt", label: "notes.txt" },
    ];
    const { container } = render(Tree, {
      props: { nodes, expandedValues: ["docs"], reorderable: true, onReorder },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    expect(onReorder).not.toHaveBeenCalled();
  });
});

describe("Tree accessible names", () => {
  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.useRealTimers();
  });

  it("names each treeitem with its node label, not its rendered contents", () => {
    const { getByRole } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"] },
    });

    // An expanded row contains descendant labels (a.ts, lib, c.ts); a
    // content-derived name would concatenate them and fail this exact query.
    const src = getByRole("treeitem", { name: "src" });
    expect(src.getAttribute("aria-label")).toBe("src");
    expect(getByRole("treeitem", { name: "a.ts" })).toBeTruthy();
    expect(getByRole("treeitem", { name: "c.ts" })).toBeTruthy();
  });

  it("keeps the label as name while the row is being renamed", async () => {
    const { getByRole, queryByRole } = render(Tree, { props: { nodes: nested } });

    await fireEvent.keyDown(getByRole("treeitem", { name: "src" }), { key: "F2" });

    // The visible label child is gone; the rename input is separate.
    expect(queryByRole("textbox", { name: "Rename src" })).toBeTruthy();
    expect(getByRole("treeitem", { name: "src" })).toBeTruthy();
  });

  it("updates the accessible name when the label is renamed", async () => {
    const onRenameCommit = vi.fn();
    const renamed = [
      {
        value: "src",
        label: "source",
        children: [
          { value: "a.ts", label: "a.ts" },
          { value: "lib", label: "lib", children: [{ value: "c.ts", label: "c.ts" }] },
        ],
      },
      { value: "docs", label: "docs", isBranch: true },
    ];
    const { getByRole, queryByRole, rerender } = render(Tree, {
      props: { nodes: nested, onRenameCommit },
    });

    await fireEvent.keyDown(getByRole("treeitem", { name: "src" }), { key: "F2" });
    const input = getByRole("textbox", { name: "Rename src" }) as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "source" } });
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(onRenameCommit).toHaveBeenCalledWith("src", "source");

    await rerender({ nodes: renamed, onRenameCommit });
    expect(getByRole("treeitem", { name: "source" })).toBeTruthy();
    expect(queryByRole("treeitem", { name: "src" })).toBeNull();
  });

  it("names the loading placeholder with its visible text", () => {
    const { getByRole } = render(Tree, {
      props: {
        nodes: [{ value: "docs", label: "docs", isBranch: true }],
        expandedValues: ["docs"],
        loadingValues: ["docs"],
      },
    });

    expect(getByRole("treeitem", { name: "Loading…" })).toBeTruthy();
  });
});

function authority(overrides: Partial<TreeReorderAuthority> = {}): TreeReorderAuthority & {
  drops: TreeReorderCandidate[];
  hovers: TreeReorderCandidate[];
} {
  const drops: TreeReorderCandidate[] = [];
  const hovers: TreeReorderCandidate[] = [];
  return {
    projectMovingValues: (source, selected) =>
      selected.includes(source) && selected.length > 0 ? [...selected] : [source],
    canDrop: (candidate) => {
      hovers.push(candidate);
      return { accepted: true, intent: candidate.intent };
    },
    onDrop: (candidate) => {
      drops.push(candidate);
      return { status: "committed" };
    },
    ...overrides,
    drops,
    hovers,
  };
}

describe("Tree reorderAuthority", () => {
  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.useRealTimers();
  });

  const outline = [
    {
      value: "docs",
      label: "docs",
      children: [
        { value: "intro.md", label: "intro.md" },
        { value: "guide.md", label: "guide.md" },
      ],
    },
    { value: "notes.txt", label: "notes.txt" },
  ];

  it("latches the projected moving set for the session and projects again on the next", async () => {
    const projectMovingValues = vi.fn((source: string, selected: readonly string[]) =>
      selected.includes(source) ? [...selected] : [source],
    );
    const host = authority({ projectMovingValues });
    const { container, rerender } = render(Tree, {
      props: {
        nodes: files,
        selectedValues: ["a.ts", "b.ts"],
        reorderable: true,
        reorderAuthority: host,
      },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("c.ts")!;
    const y = target.getBoundingClientRect().top + 4;
    source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y }));
    expect(projectMovingValues).toHaveBeenCalledTimes(1);

    await rerender({
      nodes: files,
      selectedValues: ["c.ts"],
      reorderable: true,
      reorderAuthority: host,
    });
    layoutTree(container);
    document.dispatchEvent(pointer("pointermove", { clientY: y }));
    document.dispatchEvent(pointer("pointerup", { clientY: y }));
    expect(host.drops).toHaveLength(1);
    expect(host.drops[0]?.subject.movingValues).toEqual(["a.ts", "b.ts"]);
    expect(projectMovingValues).toHaveBeenCalledTimes(1);

    const next = layoutTree(container);
    drag(next.get("a.ts")!, next.get("c.ts")!, next.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(host.drops[1]?.subject.movingValues).toEqual(["a.ts"]);

    const later = layoutTree(container);
    drag(later.get("c.ts")!, later.get("a.ts")!, later.get("a.ts")!.getBoundingClientRect().top + 4);
    expect(host.drops.at(-1)?.subject.movingValues).toEqual(["c.ts"]);
  });

  it("a hostile canDrop cannot replace the latched subject or committed candidate", () => {
    let seen: TreeReorderCandidate | undefined;
    const host = authority({
      canDrop: (candidate) => {
        const hostile = candidate.subject as unknown as {
          sourceValue: string;
          movingValues: string[];
        };
        hostile.sourceValue = "b.ts";
        hostile.movingValues = ["b.ts"];
        seen = candidate;
        return { accepted: true, intent: candidate.intent };
      },
    });
    const { container } = render(Tree, {
      props: { nodes: files, reorderable: true, reorderAuthority: host },
    });
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(seen?.subject).toEqual({ sourceValue: "b.ts", movingValues: ["b.ts"] });
    expect(host.drops).toHaveLength(1);
    expect(host.drops[0]?.subject).toEqual({ sourceValue: "a.ts", movingValues: ["a.ts"] });
    expect(host.drops[0]?.subject).not.toBe(seen?.subject);
    expect(Object.isFrozen(host.drops[0]?.subject)).toBe(true);
    expect(Object.isFrozen(host.drops[0]?.subject.movingValues)).toBe(true);
  });

  it("refuses a generic-safe target the host withholds, before accepted paint", () => {
    const host = authority({
      canDrop: () => ({ accepted: false, reason: "occupied" }),
    });
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, reorderAuthority: host },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("docs")!;
    const y = target.getBoundingClientRect().top + 4;
    source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y }));
    const item = container.querySelector<HTMLElement>('[data-value="docs"]')!;
    expect(item.getAttribute("data-poodle-drop-target")).not.toBe("accepted");
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("");
    document.dispatchEvent(pointer("pointerup", { clientY: y }));
    expect(host.drops).toHaveLength(0);
  });

  it("rewrites destination depth, announcement, and commit together", async () => {
    const host = authority({
      canDrop: (candidate) => ({
        accepted: true,
        intent: {
          ...candidate.intent,
          destination: { targetId: "guide.md", position: "after" },
        },
      }),
    });
    const { container } = render(Tree, {
      props: { nodes: outline, expandedValues: ["docs"], reorderable: true, reorderAuthority: host },
    });
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const item = container.querySelector<HTMLElement>('[role="treeitem"][data-value="notes.txt"]')!;
    const y = source.getBoundingClientRect().top + 20;
    source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 150 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 40 }));
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("1");
    expect(item.getAttribute("data-poodle-drop-target")).toBe("accepted");
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 40 }));
    await tick();
    expect(host.drops).toHaveLength(1);
    expect(host.drops[0]?.intent.destination).toEqual({ targetId: "guide.md", position: "after" });
    const live = container.querySelector(".poodle-drag-live-region")?.textContent ?? "";
    expect(live).toContain("guide.md");
    expect(live).not.toMatch(/on notes\.txt$/);
  });

  it("paints rewritten dest depth when that dest is collapsed", async () => {
    const host = authority({
      canDrop: (candidate) => ({
        accepted: true,
        intent: {
          ...candidate.intent,
          destination: { targetId: "guide.md", position: "after" },
        },
      }),
    });
    const { container } = render(Tree, {
      props: { nodes: outline, expandedValues: [], reorderable: true, reorderAuthority: host },
    });
    const rows = layoutTree(container);
    expect(container.querySelector('[data-value="guide.md"]')).toBeNull();
    const source = rows.get("notes.txt")!;
    const item = container.querySelector<HTMLElement>('[role="treeitem"][data-value="notes.txt"]')!;
    const y = source.getBoundingClientRect().top + 20;
    source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 150 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 40 }));
    expect(item.getAttribute("data-poodle-drop-target")).toBe("accepted");
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("1");
    document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 40 }));
    await tick();
    expect(host.drops).toHaveLength(1);
    expect(host.drops[0]?.intent.destination).toEqual({ targetId: "guide.md", position: "after" });
  });

  it("revalidates live authority at release", () => {
    const host = authority();
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, reorderAuthority: host },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("docs")!;
    const y = target.getBoundingClientRect().top + 4;
    source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y }));
    expect(container.querySelector('[data-value="docs"]')?.getAttribute("data-poodle-drop-target")).toBe("accepted");
    host.canDrop = () => ({ accepted: false, reason: "stale" });
    document.dispatchEvent(pointer("pointerup", { clientY: y }));
    expect(host.drops).toHaveLength(0);
  });

  it("removing authority mid-session refuses rather than falling through to onReorder", async () => {
    const onReorder = vi.fn();
    const host = authority();
    const { container, rerender } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, reorderAuthority: host },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("docs")!;
    const y = target.getBoundingClientRect().top + 4;
    source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y }));
    await rerender({
      nodes: nested,
      expandedValues: ["src", "lib"],
      reorderable: true,
      reorderAuthority: null,
      onReorder,
    });
    layoutTree(container);
    document.dispatchEvent(pointer("pointerup", { clientY: y }));
    expect(host.drops).toHaveLength(0);
    expect(onReorder).not.toHaveBeenCalled();
  });

  it.each([
    { outcome: { status: "rejected" as const, reason: "late" }, announcement: "Drop rejected: late" },
    { outcome: { status: "failed" as const, reason: "late" }, announcement: "Drop failed: late" },
  ])("a pending authority Promise settles its own session once ($announcement)", async ({ outcome, announcement }) => {
    let finish: ((value: typeof outcome) => void) | undefined;
    const pending = new Promise<typeof outcome>((resolve) => {
      finish = resolve;
    });
    const onDrop = vi.fn(() => pending);
    const host = authority({ onDrop });
    const { container } = render(Tree, {
      props: { nodes: files, reorderable: true, reorderAuthority: host },
    });
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(onDrop).toHaveBeenCalledTimes(1);
    expect(container.querySelector('[data-value="a.ts"]')?.getAttribute("data-poodle-drag-source")).toBe(
      "dropping",
    );
    finish?.(outcome);
    await pending;
    await tick();
    expect(onDrop).toHaveBeenCalledTimes(1);
    expect(container.querySelector('[data-value="a.ts"]')?.getAttribute("data-poodle-drag-source")).not.toBe(
      "dropping",
    );
    expect(container.querySelector('[data-poodle-drag-source="dropping"]')).toBeNull();
    await waitFor(() => {
      expect(container.querySelector(".poodle-drag-live-region")?.textContent).toBe(announcement);
    });
  });

  it("a stale authority Promise cannot settle a later session on the same controller", async () => {
    let finish: ((value: { status: "rejected"; reason: string }) => void) | undefined;
    const pending = new Promise<{ status: "rejected"; reason: string }>((resolve) => {
      finish = resolve;
    });
    const laterPending = new Promise<{ status: "committed" }>(() => {});
    let drops = 0;
    const host = authority({
      onDrop: () => {
        drops += 1;
        return drops === 1 ? pending : laterPending;
      },
    });
    const { container, rerender } = render(Tree, {
      props: { nodes: files, reorderable: true, reorderAuthority: host },
    });
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(container.querySelector('[data-value="a.ts"]')?.getAttribute("data-poodle-drag-source")).toBe(
      "dropping",
    );

    await rerender({
      nodes: files.filter((node) => node.value !== "a.ts"),
      reorderable: true,
      reorderAuthority: host,
    });
    await tick();
    expect(container.querySelector('[data-poodle-drag-source="dropping"]')).toBeNull();

    await rerender({
      nodes: files,
      reorderable: true,
      reorderAuthority: host,
    });
    const next = layoutTree(container);
    drag(next.get("a.ts")!, next.get("c.ts")!, next.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(container.querySelector('[data-value="a.ts"]')?.getAttribute("data-poodle-drag-source")).toBe(
      "dropping",
    );
    finish?.({ status: "rejected", reason: "late" });
    await pending;
    await tick();
    expect(container.querySelector('[data-value="a.ts"]')?.getAttribute("data-poodle-drag-source")).toBe(
      "dropping",
    );
  });

  it("routes Alt+↑/↓ through the same authority path", async () => {
    const host = authority();
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src"], reorderable: true, reorderAuthority: host },
    });
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    await fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(host.drops).toHaveLength(1);
    expect(host.drops[0]?.subject.sourceValue).toBe("a.ts");
    expect(host.drops[0]?.intent.destination ?? {
      targetId: host.drops[0]?.intent.targetId,
      position: host.drops[0]?.intent.position,
    }).toEqual({ targetId: "lib", position: "after" });
  });

  it("refuses an invalid projection instead of reducing it to the source", () => {
    const host = authority({ projectMovingValues: () => [] });
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src", "lib"], reorderable: true, reorderAuthority: host },
    });
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("docs")!;
    drag(source, target, target.getBoundingClientRect().top + 4);
    expect(host.drops).toHaveLength(0);
    expect(container.querySelector("[data-poodle-drop-target='accepted']")).toBeNull();
  });

  it("keeps onReorder when no authority is installed", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tree, {
      props: { nodes: nested, expandedValues: ["src"], reorderable: true, onReorder },
    });
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    await fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(onReorder).toHaveBeenCalledWith("a.ts", "lib", "after");
    onReorder.mockClear();
    const { container: fileTree } = render(Tree, {
      props: { nodes: files, reorderable: true, onReorder },
    });
    const fileRows = layoutTree(fileTree);
    drag(fileRows.get("a.ts")!, fileRows.get("c.ts")!, fileRows.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "c.ts", "after");
  });
});
