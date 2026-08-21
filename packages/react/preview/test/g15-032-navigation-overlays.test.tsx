import { act, fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { PopoverSpecimen } from "../src/gallery/specimens/PopoverSpecimen";

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const buttonByText = (root: ParentNode, text: string) =>
  [...root.querySelectorAll("button")].find((b) => b.textContent?.includes(text)) as
    | HTMLButtonElement
    | undefined;

/**
 * g15.032: the specimen anchors each popover to a real Button, so it must
 * compose with `triggerIsInteractive` — otherwise the default wrapper adds a
 * second button role around it and focus restore lands on the inert wrapper.
 */
describe("g15.032 Popover specimen", () => {
  it("composes the Button trigger without a nested button role", () => {
    const { container } = render(<PopoverSpecimen />);

    for (const wrapper of container.querySelectorAll(".poodle-popover__trigger")) {
      expect(wrapper.getAttribute("role")).toBeNull();
      expect(wrapper.getAttribute("tabindex")).toBeNull();
      expect(wrapper.querySelector("button")).not.toBeNull();
    }
  });

  it("opens on click and returns focus to the inner button on Escape", async () => {
    const { container } = render(<PopoverSpecimen />);
    const open = buttonByText(container, "Open popover");
    expect(open).toBeTruthy();

    open!.focus();
    await fireEvent.click(open!);
    await act(() => wait(50));
    expect(document.querySelector(".poodle-popover__surface")).not.toBeNull();

    await fireEvent.keyDown(document.body, { key: "Escape" });
    await act(() => wait(50));
    expect(document.querySelector(".poodle-popover__surface")).toBeNull();
    expect(document.activeElement).toBe(open);

    // Repeatable: a second open/close cycle behaves the same.
    await fireEvent.click(open!);
    await act(() => wait(50));
    expect(document.querySelector(".poodle-popover__surface")).not.toBeNull();
    await fireEvent.keyDown(document.body, { key: "Escape" });
    await act(() => wait(50));
    expect(document.activeElement).toBe(open);
  });
});
