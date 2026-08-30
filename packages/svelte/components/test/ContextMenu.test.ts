import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import ContextMenu from "../src/ContextMenu.svelte";
import type { MenuItem } from "../src/types";

const items: MenuItem[] = [
  { value: "rename", label: "Rename" },
  { value: "delete", label: "Delete" },
];

describe("ContextMenu (svelte) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-context-menu") as HTMLElement;

  // The surface is portalled to the theme root, so it is not reachable from
  // the render container.
  const surfaceOf = () => document.querySelector(".poodle-menu-surface") as HTMLElement;

  it("dismisses the menu on outside mousedown by default", async () => {
    const { container } = render(ContextMenu, { props: { items } });
    await fireEvent.contextMenu(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).toBeNull();
  });

  it("keeps the menu open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(ContextMenu, {
      props: { items, dismissOnOutsideInteract: false },
    });
    await fireEvent.contextMenu(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).not.toBeNull();
  });
});

describe("ContextMenu (svelte) triggerless overlay", () => {
  const surfaceOf = () => document.querySelector(".poodle-menu-surface") as HTMLElement;

  it("does not render a tab-stop host when trigger is false", () => {
    const { container } = render(ContextMenu, {
      props: {
        items,
        trigger: false,
        open: false,
        anchorPoint: { x: 12, y: 8 },
      },
    });
    expect(container.querySelector(".poodle-context-menu")).toBeNull();
    expect(container.querySelector("[role='button']")).toBeNull();
    expect(surfaceOf()).toBeNull();
  });

  it("opens a controlled overlay without an invocation button", () => {
    render(ContextMenu, {
      props: {
        items,
        trigger: false,
        open: true,
        anchorPoint: { x: 12, y: 8 },
        ariaLabel: "Row actions",
      },
    });
    expect(document.querySelector(".poodle-context-menu")).toBeNull();
    expect(surfaceOf()).not.toBeNull();
    expect(surfaceOf().getAttribute("role")).toBe("menu");
    expect(surfaceOf().getAttribute("aria-label")).toBe("Row actions");
  });
});
