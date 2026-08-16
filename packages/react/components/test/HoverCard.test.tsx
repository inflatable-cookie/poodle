import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { HoverCard } from "../src/HoverCard";

describe("HoverCard (react)", () => {
  it("renders the surface with dialog semantics when controlled open", () => {
    const { container } = render(
      <HoverCard open ariaLabel="User card" trigger={<span>Hover me</span>}>
        Surface content
      </HoverCard>,
    );
    // The anchored surface portals to the theme root, so it is not reachable
    // from the render container — same pattern as the Popover suites.
    const surface = document.querySelector(".poodle-hover-card__surface") as HTMLElement;
    expect(surface).not.toBeNull();
    expect(surface.getAttribute("role")).toBe("dialog");
    expect(surface.getAttribute("aria-label")).toBe("User card");
    expect(container.querySelector(".poodle-hover-card__trigger")).not.toBeNull();
  });

  it("hides the surface when controlled closed", () => {
    const { container } = render(
      <HoverCard open={false} trigger={<span>Hover me</span>}>
        Surface content
      </HoverCard>,
    );
    expect(container.querySelector(".poodle-hover-card__surface")).toBeNull();
  });

  it("exposes the trigger as an expanded button", () => {
    const { container } = render(
      <HoverCard open trigger={<span>Hover me</span>}>
        Surface content
      </HoverCard>,
    );
    const trigger = container.querySelector(".poodle-hover-card__trigger") as HTMLElement;
    expect(trigger.getAttribute("role")).toBe("button");
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(trigger.getAttribute("tabindex")).toBe("0");
  });

  it("closes on Escape and reports the change", () => {
    const onOpenChange = vi.fn();
    const { container } = render(
      <HoverCard open onOpenChange={onOpenChange} trigger={<span>Hover me</span>}>
        Surface content
      </HoverCard>,
    );
    const trigger = container.querySelector(".poodle-hover-card__trigger") as HTMLElement;

    fireEvent.keyDown(trigger, { key: "Escape" });

    expect(onOpenChange).toHaveBeenCalledWith(false);
  });
});
