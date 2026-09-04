import { act, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { Tree } from "../src/Tree";
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

function drag(from: HTMLElement, y: number): void {
  act(() => {
    from.dispatchEvent(pointer("pointerdown", { clientY: from.getBoundingClientRect().top + 4 }));
    document.dispatchEvent(pointer("pointermove", { clientY: y }));
    document.dispatchEvent(pointer("pointerup", { clientY: y }));
  });
}

describe("Tree row metadata (react)", () => {
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

  it("renders an end label while muted rows remain selectable", () => {
    const onSelectionChange = vi.fn();
    const { getByRole, getByText } = render(
      <Tree
        nodes={[{ value: "empty", label: "Empty area", endLabel: "0", isMuted: true }]}
        onSelectionChange={onSelectionChange}
      />,
    );

    const row = getByRole("treeitem");
    expect(row.getAttribute("data-muted")).toBe("true");
    expect(row.hasAttribute("aria-disabled")).toBe(false);
    expect(getByText("0").classList.contains("poodle-tree__end-label")).toBe(true);

    fireEvent.click(row);
    expect(onSelectionChange).toHaveBeenCalledWith(["empty"]);
  });

  it("maps a child-on-parent drop to before, inside, or after", () => {
    const onReorder = vi.fn();
    const view = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(view.container);
    const source = rows.get("a.ts")!;
    const target = rows.get("src")!;
    const top = target.getBoundingClientRect().top;

    drag(source, top + 4);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "before");

    onReorder.mockClear();
    view.rerender(<Tree nodes={nested} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />);
    const rowsInside = layoutTree(view.container);
    drag(rowsInside.get("a.ts")!, top + 20);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "inside");

    onReorder.mockClear();
    view.rerender(<Tree nodes={nested} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />);
    const rowsAfter = layoutTree(view.container);
    drag(rowsAfter.get("a.ts")!, top + 36);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "after");
  });

  it("appends into the next sibling folder when dropped on the folder row", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const target = rows.get("lib")!;
    drag(rows.get("a.ts")!, target.getBoundingClientRect().top + 20);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "lib", "inside");
  });

  it("lands a sibling leaf at the hovered row even on the origin-facing half", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["src"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const target = rows.get("b.ts")!;
    drag(rows.get("a.ts")!, target.getBoundingClientRect().top + 4);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "b.ts", "after");
  });

  it("un-nests after an open parent when the pointer is left on the last child", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["src"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const y = rows.get("b.ts")!.getBoundingClientRect().top + 36;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
      document.dispatchEvent(pointer("pointermove", { clientX: 12, clientY: y }));
      document.dispatchEvent(pointer("pointerup", { clientX: 12, clientY: y }));
    });
    expect(onReorder).toHaveBeenCalledWith("a.ts", "src", "after");
  });

  it("un-nests the last nested row from itself without leaving it", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["src"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("b.ts")!;
    const y = source.getBoundingClientRect().top + 20;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 150 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 12 }));
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 12 }));
    });
    expect(onReorder).toHaveBeenCalledWith("b.ts", "src", "after");
  });

  it("indents into the folder above from the dragged row without leaving it", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["docs"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    });
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "guide.md", "after");
  });

  it("keeps the origin gap at root when the pointer stays left on the dragged row", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["docs"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 150 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 40 }));
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 40 }));
    });
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "docs", "after");
  });

  it("updates drop indent while the pointer stays on the dragged row", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["docs"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const item = container.querySelector<HTMLElement>('[role="treeitem"][data-value="notes.txt"]')!;
    const y = source.getBoundingClientRect().top + 20;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
    });
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("1");
    act(() => {
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 40 }));
    });
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("0");
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 40 }));
    });
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "docs", "after");
  });

  it("nests from the last child's origin-facing half at the same X", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["docs"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const lastChild = rows.get("guide.md")!;
    const startY = source.getBoundingClientRect().top + 20;
    const y = lastChild.getBoundingClientRect().top + 4;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: startY, clientX: 40 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    });
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "guide.md", "after");
  });

  it("does not indent from the row above an open folder", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["docs"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    });
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "docs", "before");
  });

  it("does not indent when dragged above an open folder from below", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["docs"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const target = rows.get("docs")!;
    const startY = source.getBoundingClientRect().top + 20;
    const y = target.getBoundingClientRect().top + 4;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: startY, clientX: 40 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
    });
    const item = container.querySelector<HTMLElement>('[role="treeitem"][data-value="docs"]')!;
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("0");
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    });
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "docs", "before");
  });

  it("inserts at the top of a folder when dropped on the first nested row", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["docs"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const firstChild = rows.get("intro.md")!;
    const folder = rows.get("docs")!;
    const startY = source.getBoundingClientRect().top + 20;

    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: startY, clientX: 40 }));
      document.dispatchEvent(
        pointer("pointermove", { clientY: firstChild.getBoundingClientRect().top + 4, clientX: 150 }),
      );
      document.dispatchEvent(
        pointer("pointerup", { clientY: firstChild.getBoundingClientRect().top + 4, clientX: 150 }),
      );
    });
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "intro.md", "before");

    onReorder.mockClear();
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: startY, clientX: 40 }));
      document.dispatchEvent(
        pointer("pointermove", { clientY: folder.getBoundingClientRect().top + 36, clientX: 150 }),
      );
      document.dispatchEvent(
        pointer("pointerup", { clientY: folder.getBoundingClientRect().top + 36, clientX: 150 }),
      );
    });
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "intro.md", "before");
  });

  it("rejects a drop into the source subtree and after the source is removed", () => {
    const onReorder = vi.fn();
    const view = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(view.container);
    drag(rows.get("src")!, rows.get("c.ts")!.getBoundingClientRect().top + 8);
    expect(onReorder).not.toHaveBeenCalled();

    const live = layoutTree(view.container);
    act(() => {
      live.get("a.ts")!.dispatchEvent(pointer("pointerdown", { clientY: 14 }));
      document.dispatchEvent(pointer("pointermove", { clientY: 54 }));
    });
    view.rerender(<Tree nodes={[nested[1]!]} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />);
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: 54 }));
    });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("keeps Space selection and Alt+Arrow sibling reorder off the HTML drag path", () => {
    const onReorder = vi.fn();
    const onSelectionChange = vi.fn();
    const { container } = render(
      <Tree
        nodes={nested}
        expandedValues={["src"]}
        reorderable
        onReorder={onReorder}
        onSelectionChange={onSelectionChange}
      />,
    );
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    fireEvent.keyDown(item, { key: " " });
    expect(onSelectionChange).toHaveBeenCalledWith(["a.ts"]);
    expect(onReorder).not.toHaveBeenCalled();

    fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(onReorder).toHaveBeenCalledWith("a.ts", "lib", "after");
    expect(document.activeElement).toBe(item);
  });

  it("keeps one roving treeitem tab stop when rows are reorderable", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src"]} reorderable onReorder={onReorder} />,
    );
    const items = [...container.querySelectorAll<HTMLElement>('[role="treeitem"][data-value]')];
    const rows = [...container.querySelectorAll<HTMLElement>(".poodle-tree__row")];
    expect(rows.length).toBeGreaterThan(0);
    expect(rows.every((row) => row.tabIndex < 0)).toBe(true);
    expect(items.filter((item) => item.tabIndex >= 0)).toHaveLength(1);

    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(onReorder).toHaveBeenCalledWith("a.ts", "lib", "after");
    expect(document.activeElement).toBe(item);
  });

  it("returns pointer-commit focus to the treeitem, not the row handle", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    drag(rows.get("a.ts")!, rows.get("lib")!.getBoundingClientRect().top + 36);
    expect(onReorder).toHaveBeenCalled();
    expect(document.activeElement).toBe(item);
    expect([...container.querySelectorAll<HTMLElement>(".poodle-tree__row")].every((row) => row.tabIndex < 0)).toBe(
      true,
    );
    expect(
      [...container.querySelectorAll<HTMLElement>('[role="treeitem"][data-value]')].filter((node) => node.tabIndex >= 0),
    ).toHaveLength(1);
  });

  it("returns pointer-cancel focus to the treeitem, not the row handle", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    act(() => {
      rows.get("a.ts")!.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
      document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
      document.dispatchEvent(pointer("pointercancel", { clientY: 90 }));
    });
    expect(onReorder).not.toHaveBeenCalled();
    expect(document.activeElement).toBe(item);
    expect([...container.querySelectorAll<HTMLElement>(".poodle-tree__row")].every((row) => row.tabIndex < 0)).toBe(
      true,
    );
    expect(
      [...container.querySelectorAll<HTMLElement>('[role="treeitem"][data-value]')].filter((node) => node.tabIndex >= 0),
    ).toHaveLength(1);
  });

  it("does not Alt+Arrow reorder onto a disabled sibling", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree
        nodes={[
          {
            value: "src",
            label: "src",
            children: [
              { value: "a.ts", label: "a.ts" },
              { value: "lib", label: "lib", isDisabled: true },
            ],
          },
        ]}
        expandedValues={["src"]}
        reorderable
        onReorder={onReorder}
      />,
    );
    layoutTree(container);
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not start a drag from a checkbox or rename field", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree
        nodes={nested}
        expandedValues={["src"]}
        reorderable
        showCheckboxes
        editingValue="a.ts"
        onReorder={onReorder}
      />,
    );
    layoutTree(container);
    const checkbox = container.querySelector("input[type='checkbox']") as HTMLInputElement;
    act(() => {
      checkbox.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
      document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
      document.dispatchEvent(pointer("pointerup", { clientY: 90 }));
    });
    expect(onReorder).not.toHaveBeenCalled();

    const rename = container.querySelector(".poodle-tree__rename") as HTMLInputElement;
    act(() => {
      rename.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
      document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
      document.dispatchEvent(pointer("pointerup", { clientY: 90 }));
    });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("selects a reorderable row on click without committing a drag", () => {
    const onReorder = vi.fn();
    const onSelectionChange = vi.fn();
    const { container } = render(
      <Tree
        nodes={nested}
        expandedValues={["src"]}
        reorderable
        onReorder={onReorder}
        onSelectionChange={onSelectionChange}
      />,
    );
    const rows = layoutTree(container);
    const row = rows.get("a.ts")!;
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    const provider = container.querySelector<HTMLElement>(".poodle-drag-drop-provider")!;
    const y = row.getBoundingClientRect().top + 4;

    act(() => {
      row.dispatchEvent(pointer("pointerdown", { clientY: y }));
    });
    expect(provider.style.getPropertyValue("user-select")).toBe("none");
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: y }));
    });
    expect(provider.style.getPropertyValue("user-select")).toBe("");

    fireEvent.click(item);
    expect(onSelectionChange).toHaveBeenCalledTimes(1);
    expect(onSelectionChange).toHaveBeenCalledWith(["a.ts"]);
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not suppress root selection when pressing a rename field", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree
        nodes={nested}
        expandedValues={["src"]}
        reorderable
        editingValue="a.ts"
        onReorder={onReorder}
      />,
    );
    layoutTree(container);
    const provider = container.querySelector<HTMLElement>(".poodle-drag-drop-provider")!;
    const rename = container.querySelector(".poodle-tree__rename") as HTMLInputElement;

    act(() => {
      rename.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
    });
    expect(provider.style.getPropertyValue("user-select")).toBe("");
    rename.setSelectionRange(0, 2);
    expect(rename.selectionStart).toBe(0);
    expect(rename.selectionEnd).toBe(2);
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: 54 }));
    });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not select a row from an activated drag's compatibility click", () => {
    const onReorder = vi.fn();
    const onSelectionChange = vi.fn();
    const { container } = render(
      <Tree nodes={files} reorderable onReorder={onReorder} onSelectionChange={onSelectionChange} />,
    );
    const rows = layoutTree(container);
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;

    drag(rows.get("a.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    act(() => {
      item.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    });
    expect(onReorder).toHaveBeenCalledTimes(1);
    expect(onSelectionChange).not.toHaveBeenCalled();
  });

  it("does not select after rejected, failed, or cancelled activated drags", () => {
    const onSelectionChange = vi.fn();
    const rejected = authority({
      onDrop: () => ({ status: "rejected", reason: "occupied" }),
    });
    const first = render(
      <Tree nodes={files} reorderable reorderAuthority={rejected} onSelectionChange={onSelectionChange} />,
    );
    const rows = layoutTree(first.container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    act(() => {
      first.container.querySelector<HTMLElement>('[data-value="a.ts"]')!.dispatchEvent(
        new MouseEvent("click", { bubbles: true, cancelable: true }),
      );
    });
    expect(onSelectionChange).not.toHaveBeenCalled();
    first.unmount();

    const failed = authority({
      onDrop: () => ({ status: "failed", reason: "io" }),
    });
    const again = render(
      <Tree nodes={files} reorderable reorderAuthority={failed} onSelectionChange={onSelectionChange} />,
    );
    const failedRows = layoutTree(again.container);
    drag(failedRows.get("a.ts")!, failedRows.get("c.ts")!.getBoundingClientRect().top + 4);
    act(() => {
      again.container.querySelector<HTMLElement>('[data-value="a.ts"]')!.dispatchEvent(
        new MouseEvent("click", { bubbles: true, cancelable: true }),
      );
    });
    expect(onSelectionChange).not.toHaveBeenCalled();
    again.unmount();

    const onReorder = vi.fn();
    const cancelled = render(
      <Tree nodes={files} reorderable onReorder={onReorder} onSelectionChange={onSelectionChange} />,
    );
    const cancelRows = layoutTree(cancelled.container);
    const source = cancelRows.get("a.ts")!;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
      document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
      document.dispatchEvent(pointer("pointerup", { clientY: 800 }));
      cancelled.container.querySelector<HTMLElement>('[data-value="a.ts"]')!.dispatchEvent(
        new MouseEvent("click", { bubbles: true, cancelable: true }),
      );
    });
    expect(onReorder).not.toHaveBeenCalled();
    expect(onSelectionChange).not.toHaveBeenCalled();
    cancelled.unmount();
  });

  it("still selects after a stale activated-drag click guard expires", async () => {
    const onReorder = vi.fn();
    const onSelectionChange = vi.fn();
    const { container } = render(
      <Tree nodes={files} reorderable onReorder={onReorder} onSelectionChange={onSelectionChange} />,
    );
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(onReorder).toHaveBeenCalledTimes(1);
    await new Promise((resolve) => setTimeout(resolve, 0));
    fireEvent.click(container.querySelector<HTMLElement>('[data-value="a.ts"]')!);
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
    const { container } = render(
      <Tree nodes={files} reorderable reorderAuthority={host} onSelectionChange={onSelectionChange} />,
    );
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    act(() => {
      container.querySelector<HTMLElement>('[data-value="a.ts"]')!.dispatchEvent(
        new MouseEvent("click", { bubbles: true, cancelable: true }),
      );
    });
    expect(onSelectionChange).not.toHaveBeenCalled();
    await act(async () => {
      finish?.({ status: "committed" });
    });
    expect(onSelectionChange).not.toHaveBeenCalled();
  });

  it("prefers a nested descendant when ancestor and descendant both contain the pointer", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const srcItem = container.querySelector<HTMLElement>('[data-value="src"]')!;
    const childItem = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    srcItem.getBoundingClientRect = () => asRect(10, 80);
    rows.get("src")!.getBoundingClientRect = () => asRect(10, 80);
    childItem.getBoundingClientRect = () => asRect(50, 40);
    rows.get("a.ts")!.getBoundingClientRect = () => asRect(50, 40);

    drag(rows.get("docs")!, 70);
    expect(onReorder).toHaveBeenCalledTimes(1);
    expect(onReorder).toHaveBeenCalledWith("docs", "a.ts", "after");
  });

  it("rejects a drop when the live target is disabled or removed before release", () => {
    const onReorder = vi.fn();
    const view = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(view.container);
    const source = rows.get("a.ts")!;
    const target = rows.get("docs")!;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
      document.dispatchEvent(pointer("pointermove", { clientY: target.getBoundingClientRect().top + 8 }));
    });
    view.rerender(
      <Tree
        nodes={[nested[0]!, { value: "docs", label: "docs", isBranch: true, isDisabled: true }]}
        expandedValues={["src", "lib"]}
        reorderable
        onReorder={onReorder}
      />,
    );
    layoutTree(view.container);
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: 178 }));
    });
    expect(onReorder).not.toHaveBeenCalled();

    const live = layoutTree(view.container);
    act(() => {
      live.get("a.ts")!.dispatchEvent(pointer("pointerdown", { clientY: 54 }));
      document.dispatchEvent(pointer("pointermove", { clientY: 178 }));
    });
    view.rerender(
      <Tree nodes={[nested[0]!]} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />,
    );
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: 178 }));
    });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("revalidates live Tree state at drop so a newly illegal nest does not commit", () => {
    const onReorder = vi.fn();
    const view = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(view.container);
    const source = rows.get("docs")!;
    const target = rows.get("src")!;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
      document.dispatchEvent(pointer("pointermove", { clientY: target.getBoundingClientRect().top + 20 }));
    });
    view.rerender(
      <Tree
        nodes={[{ value: "docs", label: "docs", isBranch: true, children: [nested[0]!] }]}
        expandedValues={["src", "lib", "docs"]}
        reorderable
        onReorder={onReorder}
      />,
    );
    layoutTree(view.container);
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: 30 }));
    });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not start a drag from the expansion twisty", () => {
    const onReorder = vi.fn();
    const onExpandedChange = vi.fn();
    const { container } = render(
      <Tree nodes={nested} reorderable onReorder={onReorder} onExpandedChange={onExpandedChange} />,
    );
    layoutTree(container);
    const twisty = container.querySelector<HTMLElement>('[data-value="src"] .poodle-tree__twisty')!;
    act(() => {
      twisty.dispatchEvent(pointer("pointerdown", { clientY: 14 }));
      document.dispatchEvent(pointer("pointermove", { clientY: 90 }));
      document.dispatchEvent(pointer("pointerup", { clientY: 90 }));
    });
    expect(onReorder).not.toHaveBeenCalled();

    fireEvent.click(twisty);
    expect(onExpandedChange).toHaveBeenCalledWith(["src"]);
  });

  it("pins a virtualized source so it is not unmounted mid-drag", () => {
    const nodes = Array.from({ length: 40 }, (_, index) => ({
      value: `n${index}`,
      label: `Node ${index}`,
    }));
    const onReorder = vi.fn();
    const { container } = render(
      <Tree nodes={nodes} reorderable virtualized virtualHeight={80} onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("n0")!;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: 14 }));
      document.dispatchEvent(pointer("pointermove", { clientY: 30 }));
    });
    const scroller = container.querySelector(".poodle-tree") as HTMLElement;
    Object.defineProperty(scroller, "scrollTop", { configurable: true, value: 600 });
    act(() => {
      scroller.dispatchEvent(new Event("scroll", { bubbles: true }));
    });
    expect(container.querySelector('[data-value="n0"]')).not.toBeNull();
    act(() => {
      document.dispatchEvent(pointer("pointercancel", { clientY: 30 }));
    });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("does not commit an expanded folder into its own first child", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("src")!;
    const rect = source.getBoundingClientRect();
    const y = rect.top + rect.height - 2;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: rect.top + 4 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y }));
      document.dispatchEvent(pointer("pointerup", { clientY: y }));
    });
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("announces the committed destination, not the hovered row, on an origin-gap drop", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["docs"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    });
    expect(onReorder).toHaveBeenCalledWith("notes.txt", "guide.md", "after");
    const live = container.querySelector(".poodle-drag-live-region")?.textContent ?? "";
    expect(live).toContain("guide.md");
    expect(live).not.toMatch(/on notes\.txt$/);
  });

  it("does not commit a remapped drop onto a disabled destination", () => {
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
    const { container } = render(
      <Tree nodes={nodes} expandedValues={["docs"]} reorderable onReorder={onReorder} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const y = source.getBoundingClientRect().top + 20;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 40 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 150 }));
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 150 }));
    });
    expect(onReorder).not.toHaveBeenCalled();
  });
});

describe("Tree accessible names (react)", () => {
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
    const { getByRole } = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} />,
    );

    // An expanded row contains descendant labels (a.ts, lib, c.ts); a
    // content-derived name would concatenate them and fail this exact query.
    const src = getByRole("treeitem", { name: "src" });
    expect(src.getAttribute("aria-label")).toBe("src");
    expect(getByRole("treeitem", { name: "a.ts" })).toBeTruthy();
    expect(getByRole("treeitem", { name: "c.ts" })).toBeTruthy();
  });

  it("keeps the label as name while the row is being renamed", () => {
    const { getByRole, queryByRole } = render(<Tree nodes={nested} />);

    fireEvent.keyDown(getByRole("treeitem", { name: "src" }), { key: "F2" });

    // The visible label child is gone; the rename input is separate.
    expect(queryByRole("textbox", { name: "Rename src" })).toBeTruthy();
    expect(getByRole("treeitem", { name: "src" })).toBeTruthy();
  });

  it("updates the accessible name when the label is renamed", () => {
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
    const view = render(<Tree nodes={nested} onRenameCommit={onRenameCommit} />);

    fireEvent.keyDown(view.getByRole("treeitem", { name: "src" }), { key: "F2" });
    const input = view.getByRole("textbox", { name: "Rename src" }) as HTMLInputElement;
    fireEvent.change(input, { target: { value: "source" } });
    fireEvent.keyDown(input, { key: "Enter" });
    expect(onRenameCommit).toHaveBeenCalledWith("src", "source");

    view.rerender(<Tree nodes={renamed} onRenameCommit={onRenameCommit} />);
    expect(view.getByRole("treeitem", { name: "source" })).toBeTruthy();
    expect(view.queryByRole("treeitem", { name: "src" })).toBeNull();
  });

  it("names the loading placeholder with its visible text", () => {
    const { getByRole } = render(
      <Tree
        nodes={[{ value: "docs", label: "docs", isBranch: true }]}
        expandedValues={["docs"]}
        loadingValues={["docs"]}
      />,
    );

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

describe("Tree reorderAuthority (react)", () => {
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

  it("latches the projected moving set for the session and projects again on the next", () => {
    const projectMovingValues = vi.fn((source: string, selected: readonly string[]) =>
      selected.includes(source) ? [...selected] : [source],
    );
    const host = authority({ projectMovingValues });
    const { container, rerender } = render(
      <Tree
        nodes={files}
        selectedValues={["a.ts", "b.ts"]}
        reorderable
        reorderAuthority={host}
      />,
    );
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const y = rows.get("c.ts")!.getBoundingClientRect().top + 4;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y }));
    });
    expect(projectMovingValues).toHaveBeenCalledTimes(1);

    rerender(
      <Tree
        nodes={files}
        selectedValues={["c.ts"]}
        reorderable
        reorderAuthority={host}
      />,
    );
    layoutTree(container);
    act(() => {
      document.dispatchEvent(pointer("pointermove", { clientY: y }));
      document.dispatchEvent(pointer("pointerup", { clientY: y }));
    });
    expect(host.drops).toHaveLength(1);
    expect(host.drops[0]?.subject.movingValues).toEqual(["a.ts", "b.ts"]);
    expect(projectMovingValues).toHaveBeenCalledTimes(1);

    const next = layoutTree(container);
    drag(next.get("a.ts")!, next.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(host.drops[1]?.subject.movingValues).toEqual(["a.ts"]);
    drag(next.get("c.ts")!, next.get("a.ts")!.getBoundingClientRect().top + 4);
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
    const { container } = render(
      <Tree nodes={files} reorderable reorderAuthority={host} />,
    );
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
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
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable reorderAuthority={host} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("docs")!;
    const y = target.getBoundingClientRect().top + 4;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y }));
    });
    const item = container.querySelector<HTMLElement>('[data-value="docs"]')!;
    expect(item.getAttribute("data-poodle-drop-target")).not.toBe("accepted");
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("");
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: y }));
    });
    expect(host.drops).toHaveLength(0);
  });

  it("rewrites destination depth, announcement, and commit together", () => {
    const host = authority({
      canDrop: (candidate) => ({
        accepted: true,
        intent: {
          ...candidate.intent,
          destination: { targetId: "guide.md", position: "after" },
        },
      }),
    });
    const { container } = render(
      <Tree nodes={outline} expandedValues={["docs"]} reorderable reorderAuthority={host} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("notes.txt")!;
    const item = container.querySelector<HTMLElement>('[role="treeitem"][data-value="notes.txt"]')!;
    const y = source.getBoundingClientRect().top + 20;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 150 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 40 }));
    });
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("1");
    expect(item.getAttribute("data-poodle-drop-target")).toBe("accepted");
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 40 }));
    });
    expect(host.drops).toHaveLength(1);
    expect(host.drops[0]?.intent.destination).toEqual({ targetId: "guide.md", position: "after" });
    const live = container.querySelector(".poodle-drag-live-region")?.textContent ?? "";
    expect(live).toContain("guide.md");
    expect(live).not.toMatch(/on notes\.txt$/);
  });

  it("paints rewritten dest depth when that dest is collapsed", () => {
    const host = authority({
      canDrop: (candidate) => ({
        accepted: true,
        intent: {
          ...candidate.intent,
          destination: { targetId: "guide.md", position: "after" },
        },
      }),
    });
    const { container } = render(
      <Tree nodes={outline} expandedValues={[]} reorderable reorderAuthority={host} />,
    );
    const rows = layoutTree(container);
    expect(container.querySelector('[data-value="guide.md"]')).toBeNull();
    const source = rows.get("notes.txt")!;
    const item = container.querySelector<HTMLElement>('[role="treeitem"][data-value="notes.txt"]')!;
    const y = source.getBoundingClientRect().top + 20;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: y, clientX: 150 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y, clientX: 40 }));
    });
    expect(item.getAttribute("data-poodle-drop-target")).toBe("accepted");
    expect(item.style.getPropertyValue("--poodle-tree-drop-depth")).toBe("1");
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: y, clientX: 40 }));
    });
    expect(host.drops).toHaveLength(1);
    expect(host.drops[0]?.intent.destination).toEqual({ targetId: "guide.md", position: "after" });
  });

  it("revalidates live authority at release", () => {
    const host = authority();
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable reorderAuthority={host} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("docs")!;
    const y = target.getBoundingClientRect().top + 4;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y }));
    });
    expect(container.querySelector('[data-value="docs"]')?.getAttribute("data-poodle-drop-target")).toBe("accepted");
    host.canDrop = () => ({ accepted: false, reason: "stale" });
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: y }));
    });
    expect(host.drops).toHaveLength(0);
  });

  it("removing authority mid-session refuses rather than falling through to onReorder", () => {
    const onReorder = vi.fn();
    const host = authority();
    const { container, rerender } = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable reorderAuthority={host} />,
    );
    const rows = layoutTree(container);
    const source = rows.get("a.ts")!;
    const target = rows.get("docs")!;
    const y = target.getBoundingClientRect().top + 4;
    act(() => {
      source.dispatchEvent(pointer("pointerdown", { clientY: source.getBoundingClientRect().top + 4 }));
      document.dispatchEvent(pointer("pointermove", { clientY: y }));
    });
    rerender(
      <Tree
        nodes={nested}
        expandedValues={["src", "lib"]}
        reorderable
        reorderAuthority={null}
        onReorder={onReorder}
      />,
    );
    layoutTree(container);
    act(() => {
      document.dispatchEvent(pointer("pointerup", { clientY: y }));
    });
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
    const { container } = render(
      <Tree nodes={files} reorderable reorderAuthority={host} />,
    );
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(onDrop).toHaveBeenCalledTimes(1);
    expect(container.querySelector('[data-value="a.ts"]')?.getAttribute("data-poodle-drag-source")).toBe(
      "dropping",
    );
    await act(async () => {
      finish?.(outcome);
      await pending;
    });
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
    const { container, rerender } = render(
      <Tree nodes={files} reorderable reorderAuthority={host} />,
    );
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(container.querySelector('[data-value="a.ts"]')?.getAttribute("data-poodle-drag-source")).toBe(
      "dropping",
    );

    await act(async () => {
      rerender(
        <Tree
          nodes={files.filter((node) => node.value !== "a.ts")}
          reorderable
          reorderAuthority={host}
        />,
      );
    });
    expect(container.querySelector('[data-poodle-drag-source="dropping"]')).toBeNull();

    await act(async () => {
      rerender(<Tree nodes={files} reorderable reorderAuthority={host} />);
    });
    const next = layoutTree(container);
    drag(next.get("a.ts")!, next.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(container.querySelector('[data-value="a.ts"]')?.getAttribute("data-poodle-drag-source")).toBe(
      "dropping",
    );
    await act(async () => {
      finish?.({ status: "rejected", reason: "late" });
      await pending;
    });
    expect(container.querySelector('[data-value="a.ts"]')?.getAttribute("data-poodle-drag-source")).toBe(
      "dropping",
    );
  });

  it("routes Alt+↑/↓ through the same authority path", () => {
    const host = authority();
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src"]} reorderable reorderAuthority={host} />,
    );
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(host.drops).toHaveLength(1);
    expect(host.drops[0]?.subject.sourceValue).toBe("a.ts");
    expect(host.drops[0]?.intent.destination ?? {
      targetId: host.drops[0]?.intent.targetId,
      position: host.drops[0]?.intent.position,
    }).toEqual({ targetId: "lib", position: "after" });
  });

  it("refuses an invalid projection instead of reducing it to the source", () => {
    const host = authority({ projectMovingValues: () => [] });
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src", "lib"]} reorderable reorderAuthority={host} />,
    );
    const rows = layoutTree(container);
    drag(rows.get("a.ts")!, rows.get("docs")!.getBoundingClientRect().top + 4);
    expect(host.drops).toHaveLength(0);
    expect(container.querySelector("[data-poodle-drop-target='accepted']")).toBeNull();
  });

  it("keeps onReorder when no authority is installed", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tree nodes={nested} expandedValues={["src"]} reorderable onReorder={onReorder} />,
    );
    const item = container.querySelector<HTMLElement>('[data-value="a.ts"]')!;
    item.focus();
    fireEvent.keyDown(item, { key: "ArrowDown", altKey: true });
    expect(onReorder).toHaveBeenCalledWith("a.ts", "lib", "after");
    onReorder.mockClear();
    const fileTree = render(<Tree nodes={files} reorderable onReorder={onReorder} />);
    const fileRows = layoutTree(fileTree.container);
    drag(fileRows.get("a.ts")!, fileRows.get("c.ts")!.getBoundingClientRect().top + 4);
    expect(onReorder).toHaveBeenCalledWith("a.ts", "c.ts", "after");
    fileTree.unmount();
  });
});
