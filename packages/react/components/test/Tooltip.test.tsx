import { act, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { Tooltip } from "../src/Tooltip";

afterEach(() => {
  vi.useRealTimers();
});

describe("Tooltip (react)", () => {
  // The bubble is portalled to the theme root, so it is not reachable from the
  // render container — same pattern as the HoverCard suite.
  const bubbleOf = () => document.querySelector(".poodle-tooltip__bubble") as HTMLElement | null;

  it("shows the bubble with tooltip role and content when controlled open", () => {
    render(
      <Tooltip open content="Save your changes">
        <button>Hover me</button>
      </Tooltip>,
    );
    const bubble = bubbleOf();
    expect(bubble).not.toBeNull();
    expect(bubble?.getAttribute("role")).toBe("tooltip");
    expect(bubble?.textContent).toBe("Save your changes");
  });

  it("hides the bubble when controlled closed", () => {
    render(
      <Tooltip open={false} content="Save your changes">
        <button>Hover me</button>
      </Tooltip>,
    );
    expect(bubbleOf()).toBeNull();
  });

  it("wires aria-describedby on the trigger only while open", () => {
    const { container, rerender } = render(
      <Tooltip open content="Save">
        <button>Hover me</button>
      </Tooltip>,
    );
    const trigger = container.querySelector(".poodle-tooltip button") as HTMLElement;
    const bubble = bubbleOf();
    expect(bubble?.id).toBeTruthy();
    expect(trigger.getAttribute("aria-describedby")).toBe(bubble?.id);

    rerender(
      <Tooltip open={false} content="Save">
        <button>Hover me</button>
      </Tooltip>,
    );
    expect(trigger.getAttribute("aria-describedby")).toBeNull();
  });

  it("reports open changes driven by focus on an uncontrolled tooltip", () => {
    vi.useFakeTimers();
    const onOpenChange = vi.fn();
    const { container } = render(
      <Tooltip content="Save" onOpenChange={onOpenChange}>
        <button>Hover me</button>
      </Tooltip>,
    );
    const trigger = container.querySelector(".poodle-tooltip button") as HTMLElement;

    fireEvent.focus(trigger);
    expect(onOpenChange).not.toHaveBeenCalled();

    act(() => {
      vi.advanceTimersByTime(300);
    });
    expect(onOpenChange).toHaveBeenCalledWith(true);
  });

  it("reports open changes driven by hover on an uncontrolled tooltip", () => {
    vi.useFakeTimers();
    const onOpenChange = vi.fn();
    const { container } = render(
      <Tooltip content="Save" onOpenChange={onOpenChange}>
        <button>Hover me</button>
      </Tooltip>,
    );
    const root = container.querySelector(".poodle-tooltip") as HTMLElement;

    // React synthesizes pointerenter from a bubbling pointerover, so the
    // non-bubbling pointerenter event never reaches the handler.
    fireEvent.pointerOver(root);
    expect(onOpenChange).not.toHaveBeenCalled();

    act(() => {
      vi.advanceTimersByTime(300);
    });
    expect(onOpenChange).toHaveBeenCalledWith(true);
  });

  it("closes on Escape and reports the change", () => {
    const onOpenChange = vi.fn();
    const { container } = render(
      <Tooltip open content="Save" onOpenChange={onOpenChange}>
        <button>Hover me</button>
      </Tooltip>,
    );
    const root = container.querySelector(".poodle-tooltip") as HTMLElement;

    fireEvent.keyDown(root, { key: "Escape" });
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });
});