import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ResizeHandle from "../src/ResizeHandle.svelte";

const nextFrame = () => new Promise((resolve) => requestAnimationFrame(() => resolve(null)));

describe("ResizeHandle (svelte)", () => {
  it("reports keyboard steps along the orientation axis", async () => {
    const onResizeStep = vi.fn();
    const horizontal = render(ResizeHandle, { props: { orientation: "horizontal", onResizeStep } });
    const hRoot = horizontal.container.querySelector<HTMLElement>(".poodle-resize-handle")!;

    await fireEvent.keyDown(hRoot, { key: "ArrowLeft" });
    expect(onResizeStep).toHaveBeenCalledWith(-8);
    await fireEvent.keyDown(hRoot, { key: "ArrowRight" });
    expect(onResizeStep).toHaveBeenCalledWith(8);
    await fireEvent.keyDown(hRoot, { key: "Home" });
    expect(onResizeStep).toHaveBeenCalledWith(-9999);
    await fireEvent.keyDown(hRoot, { key: "End" });
    expect(onResizeStep).toHaveBeenCalledWith(9999);

    const vertical = render(ResizeHandle, { props: { orientation: "vertical", onResizeStep } });
    const vRoot = vertical.container.querySelector<HTMLElement>(".poodle-resize-handle")!;
    await fireEvent.keyDown(vRoot, { key: "ArrowUp" });
    expect(onResizeStep).toHaveBeenCalledWith(-8);
    await fireEvent.keyDown(vRoot, { key: "ArrowDown" });
    expect(onResizeStep).toHaveBeenCalledWith(8);
  });

  it("ignores keys off the axis and unhandled keys", async () => {
    const onResizeStep = vi.fn();
    const { container } = render(ResizeHandle, { props: { onResizeStep } });
    const root = container.querySelector<HTMLElement>(".poodle-resize-handle")!;

    await fireEvent.keyDown(root, { key: "ArrowUp" });
    await fireEvent.keyDown(root, { key: "a" });
    expect(onResizeStep).not.toHaveBeenCalled();
  });

  it("removes a disabled handle from the tab order and swallows steps", async () => {
    const onResizeStep = vi.fn();
    const { container } = render(ResizeHandle, { props: { disabled: true, onResizeStep } });
    const root = container.querySelector<HTMLElement>(".poodle-resize-handle")!;
    expect(root.getAttribute("tabindex")).toBe("-1");
    expect(root.getAttribute("data-disabled")).toBe("true");

    await fireEvent.keyDown(root, { key: "ArrowLeft" });
    expect(onResizeStep).not.toHaveBeenCalled();
  });

  it("exposes separator semantics with value attributes", () => {
    const { container } = render(ResizeHandle, {
      props: {
        orientation: "vertical",
        ariaLabel: "Resize vertical",
        ariaValueNow: 42,
        ariaValueMin: 0,
        ariaValueMax: 100,
      },
    });
    const root = container.querySelector(".poodle-resize-handle")!;
    expect(root.getAttribute("role")).toBe("separator");
    expect(root.getAttribute("aria-orientation")).toBe("vertical");
    expect(root.getAttribute("aria-label")).toBe("Resize vertical");
    expect(root.getAttribute("aria-valuenow")).toBe("42");
    expect(root.getAttribute("aria-valuemin")).toBe("0");
    expect(root.getAttribute("aria-valuemax")).toBe("100");
    expect(root.getAttribute("tabindex")).toBe("0");

    const defaults = render(ResizeHandle);
    const defaultRoot = defaults.container.querySelector(".poodle-resize-handle")!;
    expect(defaultRoot.getAttribute("aria-label")).toBe("Resize");
    expect(defaultRoot.getAttribute("aria-orientation")).toBe("horizontal");
  });

  it("reports drag start, per-move deltas, and the final position", async () => {
    const onResizeStart = vi.fn();
    const onResizeMove = vi.fn();
    const onResizeEnd = vi.fn();
    const { container } = render(ResizeHandle, {
      props: { onResizeStart, onResizeMove, onResizeEnd },
    });
    const root = container.querySelector<HTMLElement>(".poodle-resize-handle")!;

    await fireEvent.mouseDown(root, { clientX: 100, clientY: 0 });
    expect(onResizeStart).toHaveBeenCalledWith(100);
    expect(root.getAttribute("data-dragging")).toBe("true");

    await fireEvent.mouseMove(window, { clientX: 120, clientY: 0 });
    await nextFrame();
    expect(onResizeMove).toHaveBeenCalledWith(20);

    await fireEvent.mouseUp(window, { clientX: 130, clientY: 0 });
    expect(onResizeEnd).toHaveBeenCalledWith(130);
    expect(root.getAttribute("data-dragging")).toBeNull();
  });

  it("does not drag while disabled", async () => {
    const onResizeStart = vi.fn();
    const onResizeEnd = vi.fn();
    const { container } = render(ResizeHandle, { props: { disabled: true, onResizeStart, onResizeEnd } });
    const root = container.querySelector<HTMLElement>(".poodle-resize-handle")!;

    await fireEvent.mouseDown(root, { clientX: 100, clientY: 0 });
    expect(onResizeStart).not.toHaveBeenCalled();
    expect(root.getAttribute("data-dragging")).toBeNull();
  });
});
