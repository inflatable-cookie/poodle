import { act, fireEvent, render } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { Tree } from "../src/Tree";

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
    clientX: 20,
    clientY: 0,
    ...init,
  });
}

function layoutTree(container: HTMLElement): Map<string, HTMLElement> {
  const rows = new Map<string, HTMLElement>();
  for (const [index, item] of [...container.querySelectorAll<HTMLElement>('[role="treeitem"][data-value]')].entries()) {
    const row = item.querySelector<HTMLElement>(".poodle-tree__row") ?? item;
    row.getBoundingClientRect = () => asRect(10 + index * 40);
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

  it("maps row geometry to before, inside, and after on a nested branch", () => {
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
});