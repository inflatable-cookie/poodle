import { act, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { IconButton } from "../src/IconButton";

afterEach(() => {
  vi.useRealTimers();
});

describe("IconButton (react)", () => {
  it("always carries the required accessible name and hides the glyph from the tree", () => {
    const { container } = render(<IconButton icon="x" ariaLabel="Close" />);
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button");
    expect(button?.getAttribute("aria-label")).toBe("Close");
    expect(button?.getAttribute("type")).toBe("button");
    const glyph = container.querySelector<HTMLElement>(".poodle-icon-button__glyph");
    expect(glyph?.getAttribute("aria-hidden")).toBe("true");
  });

  it("reports toggle changes and only renders aria-pressed when pressed is configured", () => {
    const onPressedChange = vi.fn();
    const { container, rerender } = render(
      <IconButton icon="star" ariaLabel="Star" pressed={false} onPressedChange={onPressedChange} />,
    );
    const toggleButton = container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(toggleButton.getAttribute("aria-pressed")).toBe("false");
    expect(toggleButton.getAttribute("data-pressed")).toBe("false");

    fireEvent.click(toggleButton);
    expect(onPressedChange).toHaveBeenCalledWith(true);

    rerender(<IconButton icon="star" ariaLabel="Star" pressed={true} onPressedChange={onPressedChange} />);
    fireEvent.click(toggleButton);
    expect(onPressedChange).toHaveBeenCalledWith(false);

    const plain = render(<IconButton icon="x" ariaLabel="Close" />);
    const plainButton = plain.container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(plainButton.getAttribute("aria-pressed")).toBeNull();
  });

  it("swaps the glyph for a spinner while loading and gates activation", () => {
    const onClick = vi.fn();
    const { container } = render(
      <IconButton icon="refresh-cw" ariaLabel="Refresh" loading onClick={onClick} />,
    );
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(container.querySelector(".poodle-icon-button__glyph")).toBeNull();
    expect(container.querySelector(".poodle-icon-button__spinner")).not.toBeNull();
    expect(button.getAttribute("aria-busy")).toBe("true");
    expect(button.disabled).toBe(true);

    fireEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("suppresses activation while disabled", () => {
    const onClick = vi.fn();
    const { container } = render(<IconButton icon="circle-x" ariaLabel="Block" disabled onClick={onClick} />);
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(button.disabled).toBe(true);
    expect(button.getAttribute("data-variant")).toBe("primary");

    fireEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("opens the tooltip after the hover delay and dismisses it on Escape", () => {
    vi.useFakeTimers();
    const { container } = render(<IconButton icon="plus" ariaLabel="Add" tooltip="Add item" />);
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(document.querySelector(".poodle-icon-button__tooltip")).toBeNull();

    fireEvent.mouseEnter(button);
    act(() => {
      vi.advanceTimersByTime(300);
    });

    const tooltip = document.querySelector<HTMLElement>(".poodle-icon-button__tooltip");
    expect(tooltip).not.toBeNull();
    expect(tooltip?.getAttribute("role")).toBe("tooltip");
    expect(tooltip?.textContent).toBe("Add item");
    expect(button.getAttribute("aria-describedby")).toBe(tooltip?.getAttribute("id"));

    fireEvent.keyDown(button, { key: "Escape" });
    expect(document.querySelector(".poodle-icon-button__tooltip")).toBeNull();
  });

  it("falls back to ariaLabel as the tooltip text and keeps describedBy while closed", () => {
    vi.useFakeTimers();
    const { container } = render(
      <IconButton icon="plus" ariaLabel="Add" describedBy="external-description" />,
    );
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(button.getAttribute("aria-describedby")).toBe("external-description");

    fireEvent.mouseEnter(button);
    act(() => {
      vi.advanceTimersByTime(300);
    });

    const tooltip = document.querySelector<HTMLElement>(".poodle-icon-button__tooltip");
    expect(tooltip?.textContent).toBe("Add");
    expect(tooltip?.getAttribute("data-placement")).toBe("top");
    expect(button.getAttribute("aria-describedby")).not.toBe("external-description");
  });
});
