import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { CollapseToggle } from "../src/CollapseToggle";

describe("CollapseToggle (react)", () => {
  it("projects collapsed state through aria-expanded and the default label", () => {
    const { container } = render(<CollapseToggle />);
    const button = container.querySelector("button") as HTMLButtonElement;
    expect(button.getAttribute("aria-expanded")).toBe("true");
    expect(button.getAttribute("aria-label")).toBe("Collapse");

    const collapsed = render(<CollapseToggle collapsed />);
    const collapsedButton = collapsed.container.querySelector("button") as HTMLButtonElement;
    expect(collapsedButton.getAttribute("aria-expanded")).toBe("false");
    expect(collapsedButton.getAttribute("aria-label")).toBe("Expand");
  });

  it("emits the next collapsed state on click (prop-driven, not self-toggling)", () => {
    const onToggle = vi.fn();
    const { container } = render(<CollapseToggle onToggle={onToggle} />);
    const button = container.querySelector("button") as HTMLButtonElement;

    fireEvent.click(button);
    fireEvent.click(button);

    expect(onToggle).toHaveBeenNthCalledWith(1, true);
    expect(onToggle).toHaveBeenNthCalledWith(2, true);
  });

  it("does not emit when disabled", () => {
    const onToggle = vi.fn();
    const { container } = render(<CollapseToggle disabled onToggle={onToggle} />);
    const button = container.querySelector("button") as HTMLButtonElement;

    fireEvent.click(button);

    expect(button.disabled).toBe(true);
    expect(onToggle).not.toHaveBeenCalled();
  });

  it("flips the chevron glyph with the collapsed state", () => {
    const { container } = render(<CollapseToggle direction="left" />);
    const expanded = container.querySelector(".poodle-collapse-toggle .poodle-icon")?.innerHTML;

    const collapsed = render(<CollapseToggle collapsed direction="left" />);
    const collapsedGlyph = collapsed.container.querySelector(".poodle-collapse-toggle .poodle-icon")?.innerHTML;

    expect(expanded).not.toBe(collapsedGlyph);
  });
});
