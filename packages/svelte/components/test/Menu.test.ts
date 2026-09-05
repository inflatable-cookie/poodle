import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Menu from "../src/Menu.svelte";
import type { MenuItem } from "../src/types";

const items: MenuItem[] = [
  { value: "rename", label: "Rename" },
  { value: "delete", label: "Delete" },
];

describe("Menu (svelte) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-menu__trigger") as HTMLElement;

  // The surface is portalled to the theme root, so it is not reachable from
  // the render container.
  const surfaceOf = () => document.querySelector(".poodle-menu-surface") as HTMLElement;

  it("dismisses the menu on outside mousedown by default", async () => {
    const { container } = render(Menu, { props: { items } });
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).toBeNull();
  });

  it("keeps the menu open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(Menu, {
      props: { items, dismissOnOutsideInteract: false },
    });
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).not.toBeNull();
  });
});

/** g14.007 retained regression — see the Select pair for the claim. */
describe("Menu (svelte) item identity", () => {
  it("addresses every item by its value", async () => {
    const { container } = render(Menu, { props: { items } });
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const values = [...document.querySelectorAll('[role="menuitem"]')].map((el) =>
      el.getAttribute("data-value"),
    );
    expect(values).toEqual(["rename", "delete"]);
  });

  it("keeps one enabled menu item in the sequential tab order", async () => {
    const { container } = render(Menu, { props: { items } });
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const menuItems = [...document.querySelectorAll('[role="menuitem"]')] as HTMLButtonElement[];
    expect(menuItems.map((item) => item.tabIndex)).toEqual([0, -1]);

    await fireEvent.keyDown(menuItems[0], { key: "ArrowDown" });
    expect(document.activeElement).toBe(menuItems[1]);
  });
});
