import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import Tree from "../src/Tree.svelte";

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

  it("maps a child-on-parent drop to before or after, not inside", async () => {
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
    // Child-on-parent: inside would no-op, so the lower half un-nests after.
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "after");

    await rerender({ nodes: nested, expandedValues: ["src", "lib"], reorderable: true, onReorder });
    onReorder.mockClear();
    const rowsAfter = layoutTree(container);
    drag(rowsAfter.get("a.ts")!, rowsAfter.get("src")!, top + 36);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "after");
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
});
