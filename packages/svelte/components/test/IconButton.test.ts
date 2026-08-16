import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import IconButton from "../src/IconButton.svelte";

afterEach(() => {
  vi.useRealTimers();
});

describe("IconButton (svelte)", () => {
  it("always carries the required accessible name and hides the glyph from the tree", () => {
    const { container } = render(IconButton, { props: { icon: "x", ariaLabel: "Close" } });
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button");
    expect(button?.getAttribute("aria-label")).toBe("Close");
    expect(button?.getAttribute("type")).toBe("button");
    const glyph = container.querySelector<HTMLElement>(".poodle-icon-button__glyph");
    expect(glyph?.getAttribute("aria-hidden")).toBe("true");
  });

  it("reports toggle changes and only renders aria-pressed when pressed is configured", async () => {
    const onPressedChange = vi.fn();
    const toggle = render(IconButton, {
      props: { icon: "star", ariaLabel: "Star", pressed: false, onPressedChange },
    });
    const toggleButton = toggle.container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(toggleButton.getAttribute("aria-pressed")).toBe("false");
    expect(toggleButton.getAttribute("data-pressed")).toBe("false");

    await fireEvent.click(toggleButton);
    expect(onPressedChange).toHaveBeenCalledWith(true);
    expect(toggleButton.getAttribute("aria-pressed")).toBe("true");

    await fireEvent.click(toggleButton);
    expect(onPressedChange).toHaveBeenCalledWith(false);

    const plain = render(IconButton, { props: { icon: "x", ariaLabel: "Close" } });
    const plainButton = plain.container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(plainButton.getAttribute("aria-pressed")).toBeNull();
  });

  it("swaps the glyph for a spinner while loading and gates activation", async () => {
    const onClick = vi.fn();
    const { container } = render(IconButton, {
      props: { icon: "refresh-cw", ariaLabel: "Refresh", loading: true, onClick },
    });
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(container.querySelector(".poodle-icon-button__glyph")).toBeNull();
    expect(container.querySelector(".poodle-icon-button__spinner")).not.toBeNull();
    expect(button.getAttribute("aria-busy")).toBe("true");
    expect(button.disabled).toBe(true);

    await fireEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("suppresses activation while disabled", async () => {
    const onClick = vi.fn();
    const { container } = render(IconButton, {
      props: { icon: "circle-x", ariaLabel: "Block", disabled: true, onClick },
    });
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    expect(button.disabled).toBe(true);
    expect(button.getAttribute("data-variant")).toBe("primary");

    await fireEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("opens the tooltip after the hover delay and dismisses it on Escape", async () => {
    vi.useFakeTimers();
    const { container } = render(IconButton, {
      props: { icon: "plus", ariaLabel: "Add", tooltip: "Add item" },
    });
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    const wrap = container.querySelector<HTMLElement>(".poodle-icon-button-wrap") as HTMLElement;
    expect(document.querySelector(".poodle-icon-button__tooltip")).toBeNull();

    await fireEvent.mouseEnter(wrap);
    await vi.advanceTimersByTimeAsync(300);

    const tooltip = document.querySelector<HTMLElement>(".poodle-icon-button__tooltip");
    expect(tooltip).not.toBeNull();
    expect(tooltip?.getAttribute("role")).toBe("tooltip");
    expect(tooltip?.textContent).toBe("Add item");
    expect(button.getAttribute("aria-describedby")).toBe(tooltip?.getAttribute("id"));

    await fireEvent.keyDown(button, { key: "Escape" });
    expect(document.querySelector(".poodle-icon-button__tooltip")).toBeNull();
  });

  it("falls back to ariaLabel as the tooltip text and keeps describedBy while closed", async () => {
    vi.useFakeTimers();
    const { container } = render(IconButton, {
      props: { icon: "plus", ariaLabel: "Add", describedBy: "external-description" },
    });
    const button = container.querySelector<HTMLButtonElement>(".poodle-icon-button") as HTMLButtonElement;
    const wrap = container.querySelector<HTMLElement>(".poodle-icon-button-wrap") as HTMLElement;
    expect(button.getAttribute("aria-describedby")).toBe("external-description");

    await fireEvent.mouseEnter(wrap);
    await vi.advanceTimersByTimeAsync(300);

    const tooltip = document.querySelector<HTMLElement>(".poodle-icon-button__tooltip");
    expect(tooltip?.textContent).toBe("Add");
    expect(tooltip?.getAttribute("data-placement")).toBe("top");
    expect(button.getAttribute("aria-describedby")).not.toBe("external-description");
  });
});
