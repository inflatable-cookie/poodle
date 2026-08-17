import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import TooltipHarness from "./TooltipHarness.svelte";

afterEach(() => {
  vi.useRealTimers();
});

describe("Tooltip (svelte)", () => {
  // The bubble is portalled to the theme root, so it is not reachable from the
  // render container — same pattern as the HoverCard and Popover suites.
  const bubbleOf = () => document.querySelector(".poodle-tooltip__bubble") as HTMLElement | null;
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-tooltip button") as HTMLElement;

  it("shows the bubble with tooltip role and content when controlled open", () => {
    const { container } = render(TooltipHarness, { props: { open: true } });
    const bubble = bubbleOf();
    expect(bubble).not.toBeNull();
    expect(bubble?.getAttribute("role")).toBe("tooltip");
    expect(bubble?.textContent).toBe("Save your changes");
    expect(triggerOf(container)).not.toBeNull();
  });

  it("hides the bubble when controlled closed", () => {
    render(TooltipHarness, { props: { open: false } });
    expect(bubbleOf()).toBeNull();
  });

  it("wires aria-describedby on the trigger only while open", async () => {
    const { container, rerender } = render(TooltipHarness, { props: { open: true } });
    const trigger = triggerOf(container);
    const bubble = bubbleOf();
    expect(bubble?.id).toBeTruthy();
    expect(trigger.getAttribute("aria-describedby")).toBe(bubble?.id);

    await rerender({ open: false });
    expect(trigger.getAttribute("aria-describedby")).toBeNull();
  });

  it("reports open changes driven by hover on an uncontrolled tooltip", async () => {
    vi.useFakeTimers();
    const onOpenChange = vi.fn();
    const { container } = render(TooltipHarness, { props: { onOpenChange } });
    const root = container.querySelector(".poodle-tooltip") as HTMLElement;

    fireEvent.pointerEnter(root);
    expect(onOpenChange).not.toHaveBeenCalled();

    await vi.advanceTimersByTimeAsync(300);
    expect(onOpenChange).toHaveBeenCalledWith(true);
  });

  it("reports open changes driven by focus on an uncontrolled tooltip", async () => {
    vi.useFakeTimers();
    const onOpenChange = vi.fn();
    const { container } = render(TooltipHarness, { props: { onOpenChange } });

    await fireEvent.focusIn(triggerOf(container));
    expect(onOpenChange).not.toHaveBeenCalled();

    await vi.advanceTimersByTimeAsync(300);
    expect(onOpenChange).toHaveBeenCalledWith(true);
  });

  it("asks to reopen through hover after the host closed a controlled tooltip", async () => {
    vi.useFakeTimers();
    const onOpenChange = vi.fn();
    const { container, rerender } = render(TooltipHarness, {
      props: { open: true, onOpenChange },
    });
    const root = container.querySelector(".poodle-tooltip") as HTMLElement;

    // The host closed it, so the machine never saw LEAVE; without a SET_OPEN
    // sync it would still think it is open and swallow the next ENTER.
    await rerender({ open: false, onOpenChange });
    expect(bubbleOf()).toBeNull();
    onOpenChange.mockClear();

    await fireEvent.pointerEnter(root);
    await vi.advanceTimersByTimeAsync(300);
    expect(onOpenChange).toHaveBeenCalledWith(true);
  });

  it("closes on Escape and reports the change", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(TooltipHarness, { props: { open: true, onOpenChange } });
    const root = container.querySelector(".poodle-tooltip") as HTMLElement;

    await fireEvent.keyDown(root, { key: "Escape" });
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });
});