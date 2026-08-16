import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Toolbar from "../src/Toolbar.svelte";
import ToolbarRovingHarness from "./ToolbarRovingHarness.svelte";
import { asSnippet } from "./snippet";

describe("Toolbar (svelte)", () => {
  it("renders a labelled toolbar with the group focus entry point", () => {
    const { container } = render(Toolbar, {
      props: { ariaLabel: "Formatting", children: asSnippet(() => "<button>Bold</button>") },
    });
    const root = container.querySelector(".poodle-toolbar") as HTMLElement;
    expect(root.getAttribute("role")).toBe("toolbar");
    expect(root.getAttribute("tabindex")).toBe("0");
    expect(root.getAttribute("aria-label")).toBe("Formatting");
    expect(root.dataset.orientation).toBe("horizontal");
  });

  it("projects orientation, size, and density", () => {
    const { container } = render(Toolbar, {
      props: {
        orientation: "vertical",
        size: "sm",
        density: "compact",
        children: asSnippet(() => "<button>Bold</button>"),
      },
    });
    const root = container.querySelector(".poodle-toolbar") as HTMLElement;
    expect(root.dataset.orientation).toBe("vertical");
    expect(root.dataset.size).toBe("sm");
    expect(root.dataset.density).toBe("compact");
  });

  it("moves roving focus between items with horizontal arrow keys and wraps", () => {
    const { container } = render(ToolbarRovingHarness);
    const root = container.querySelector(".poodle-toolbar") as HTMLElement;
    const buttons = Array.from(container.querySelectorAll(".poodle-toolbar button")) as HTMLButtonElement[];
    expect(buttons.length).toBe(3);

    buttons[0].focus();
    expect(document.activeElement).toBe(buttons[0]);

    fireEvent.keyDown(root, { key: "ArrowRight" });
    expect(document.activeElement).toBe(buttons[1]);

    fireEvent.keyDown(root, { key: "ArrowLeft" });
    expect(document.activeElement).toBe(buttons[0]);

    // Wraps forward past the last item and back past the first.
    fireEvent.keyDown(root, { key: "ArrowLeft" });
    expect(document.activeElement).toBe(buttons[2]);
    fireEvent.keyDown(root, { key: "ArrowRight" });
    expect(document.activeElement).toBe(buttons[0]);
  });

  it("navigates vertically only when orientation is vertical", () => {
    const { container } = render(ToolbarRovingHarness, { props: { orientation: "vertical" } });
    const root = container.querySelector(".poodle-toolbar") as HTMLElement;
    const buttons = Array.from(container.querySelectorAll(".poodle-toolbar button")) as HTMLButtonElement[];

    buttons[0].focus();
    fireEvent.keyDown(root, { key: "ArrowDown" });
    expect(document.activeElement).toBe(buttons[1]);

    // A horizontal arrow is inert in vertical mode.
    fireEvent.keyDown(root, { key: "ArrowRight" });
    expect(document.activeElement).toBe(buttons[1]);
  });
});