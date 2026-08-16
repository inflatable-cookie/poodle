import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { ResizeHandle } from "../src/ResizeHandle";

describe("ResizeHandle (react)", () => {
  it("reports keyboard steps along the orientation axis", () => {
    const onResizeStep = vi.fn();
    const horizontal = render(<ResizeHandle orientation="horizontal" onResizeStep={onResizeStep} />);
    const hRoot = horizontal.container.querySelector<HTMLElement>(".poodle-resize-handle")!;

    fireEvent.keyDown(hRoot, { key: "ArrowLeft" });
    expect(onResizeStep).toHaveBeenCalledWith(-8);
    fireEvent.keyDown(hRoot, { key: "ArrowRight" });
    expect(onResizeStep).toHaveBeenCalledWith(8);
    fireEvent.keyDown(hRoot, { key: "Home" });
    expect(onResizeStep).toHaveBeenCalledWith(-9999);
    fireEvent.keyDown(hRoot, { key: "End" });
    expect(onResizeStep).toHaveBeenCalledWith(9999);

    const vertical = render(<ResizeHandle orientation="vertical" onResizeStep={onResizeStep} />);
    const vRoot = vertical.container.querySelector<HTMLElement>(".poodle-resize-handle")!;
    fireEvent.keyDown(vRoot, { key: "ArrowUp" });
    expect(onResizeStep).toHaveBeenCalledWith(-8);
    fireEvent.keyDown(vRoot, { key: "ArrowDown" });
    expect(onResizeStep).toHaveBeenCalledWith(8);
  });

  it("ignores keys off the axis and unhandled keys", () => {
    const onResizeStep = vi.fn();
    const { container } = render(<ResizeHandle onResizeStep={onResizeStep} />);
    const root = container.querySelector<HTMLElement>(".poodle-resize-handle")!;

    fireEvent.keyDown(root, { key: "ArrowUp" });
    fireEvent.keyDown(root, { key: "a" });
    expect(onResizeStep).not.toHaveBeenCalled();
  });

  it("removes a disabled handle from the tab order and swallows steps", () => {
    const onResizeStep = vi.fn();
    const { container } = render(<ResizeHandle disabled onResizeStep={onResizeStep} />);
    const root = container.querySelector<HTMLElement>(".poodle-resize-handle")!;
    expect(root.getAttribute("tabindex")).toBe("-1");
    expect(root.getAttribute("data-disabled")).toBe("true");

    fireEvent.keyDown(root, { key: "ArrowLeft" });
    expect(onResizeStep).not.toHaveBeenCalled();
  });

  it("exposes separator semantics with value attributes", () => {
    const { container } = render(
      <ResizeHandle
        orientation="vertical"
        ariaLabel="Resize vertical"
        ariaValueNow={42}
        ariaValueMin={0}
        ariaValueMax={100}
      />,
    );
    const root = container.querySelector(".poodle-resize-handle")!;
    expect(root.getAttribute("role")).toBe("separator");
    expect(root.getAttribute("aria-orientation")).toBe("vertical");
    expect(root.getAttribute("aria-label")).toBe("Resize vertical");
    expect(root.getAttribute("aria-valuenow")).toBe("42");
    expect(root.getAttribute("aria-valuemin")).toBe("0");
    expect(root.getAttribute("aria-valuemax")).toBe("100");
    expect(root.getAttribute("tabindex")).toBe("0");

    const defaults = render(<ResizeHandle />);
    const defaultRoot = defaults.container.querySelector(".poodle-resize-handle")!;
    expect(defaultRoot.getAttribute("aria-label")).toBe("Resize");
    expect(defaultRoot.getAttribute("aria-orientation")).toBe("horizontal");
  });

  it("reports drag start, per-move deltas, and the final position", () => {
    const onResizeStart = vi.fn();
    const onResizeMove = vi.fn();
    const onResizeEnd = vi.fn();
    const { container } = render(
      <ResizeHandle onResizeStart={onResizeStart} onResizeMove={onResizeMove} onResizeEnd={onResizeEnd} />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-resize-handle")!;

    fireEvent.mouseDown(root, { clientX: 100, clientY: 0 });
    expect(onResizeStart).toHaveBeenCalledWith(100);
    expect(root.getAttribute("data-dragging")).toBe("true");

    fireEvent.mouseMove(window, { clientX: 120, clientY: 0 });
    expect(onResizeMove).toHaveBeenCalledWith(20);

    fireEvent.mouseUp(window, { clientX: 130, clientY: 0 });
    expect(onResizeEnd).toHaveBeenCalledWith(130);
    expect(root.getAttribute("data-dragging")).toBeNull();
  });

  it("does not drag while disabled", () => {
    const onResizeStart = vi.fn();
    const onResizeEnd = vi.fn();
    const { container } = render(
      <ResizeHandle disabled onResizeStart={onResizeStart} onResizeEnd={onResizeEnd} />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-resize-handle")!;

    fireEvent.mouseDown(root, { clientX: 100, clientY: 0 });
    expect(onResizeStart).not.toHaveBeenCalled();
    expect(root.getAttribute("data-dragging")).toBeNull();
  });
});
