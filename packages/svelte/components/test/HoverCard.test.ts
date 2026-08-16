import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import HoverCard from "../src/HoverCard.svelte";
import { asSnippet } from "./snippet";

describe("HoverCard (svelte)", () => {
  const renderHoverCard = (props: Record<string, unknown> = {}) =>
    render(HoverCard, {
      props: {
        trigger: asSnippet(() => "Hover me"),
        children: asSnippet(() => "Surface content"),
        ...props,
      },
    });

  it("renders the surface with dialog semantics when controlled open", () => {
    const { container } = renderHoverCard({ open: true, ariaLabel: "User card" });
    // The anchored surface portals to the theme root, so it is not reachable
    // from the render container — same pattern as the Popover suites.
    const surface = document.querySelector(".poodle-hover-card__surface") as HTMLElement;
    expect(surface).not.toBeNull();
    expect(surface.getAttribute("role")).toBe("dialog");
    expect(surface.getAttribute("aria-label")).toBe("User card");
    expect(container.querySelector(".poodle-hover-card__trigger")).not.toBeNull();
  });

  it("hides the surface when controlled closed", () => {
    const { container } = renderHoverCard({ open: false });
    expect(container.querySelector(".poodle-hover-card__surface")).toBeNull();
  });

  it("exposes the trigger as an expanded button", () => {
    const { container } = renderHoverCard({ open: true });
    const trigger = container.querySelector(".poodle-hover-card__trigger") as HTMLElement;
    expect(trigger.getAttribute("role")).toBe("button");
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(trigger.getAttribute("tabindex")).toBe("0");
  });

  it("closes on Escape and reports the change", async () => {
    const onOpenChange = vi.fn();
    const { container } = renderHoverCard({ open: true, onOpenChange });
    const trigger = container.querySelector(".poodle-hover-card__trigger") as HTMLElement;

    await fireEvent.keyDown(trigger, { key: "Escape" });

    expect(onOpenChange).toHaveBeenCalledWith(false);
  });
});
