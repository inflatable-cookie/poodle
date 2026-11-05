import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Menu from "../src/Menu.svelte";
import type { MenuItem } from "../src/types";

const items: MenuItem[] = [
  { value: "rename", label: "Rename" },
  { value: "delete", label: "Delete", tone: "danger" },
  { value: "divider", label: "divider", kind: "separator" },
  { value: "show-grid", label: "Show grid", kind: "checkbox", checked: true, shortcutLabel: "⌘G" },
  { value: "small", label: "Small icons", kind: "radio" },
  { value: "export", label: "Export…", disabled: true },
  {
    value: "open-in",
    label: "Open In",
    children: [{ value: "finder", label: "Finder" }],
  },
];

/** Original g14.007 fixture; retained regressions below still assume it. */
const identityItems: MenuItem[] = [
  { value: "rename", label: "Rename" },
  { value: "delete", label: "Delete" },
];

/** g18.037 — every non-separator row names itself with the exact required label. */
describe("Menu (svelte) item accessible names", () => {
  it("exposes each non-separator item's explicit aria-label and leaves separators unnamed", async () => {
    const { container } = render(Menu, { props: { items } });
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const surface = document.querySelector(".poodle-menu-surface") as HTMLElement;
    const rows = [...surface.querySelectorAll(":scope > div .poodle-menu-surface__item")];
    expect(rows.map((row) => row.getAttribute("aria-label"))).toEqual([
      "Rename",
      "Delete",
      "Show grid",
      "Small icons",
      "Export…",
      "Open In",
    ]);

    const separator = surface.querySelector('[role="separator"]');
    expect(separator).not.toBeNull();
    expect(separator?.getAttribute("aria-label")).toBeNull();
  });

  it("names the submenu parent row exactly while keeping its popup semantics", async () => {
    const { container } = render(Menu, { props: { items } });
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const parent = document.querySelector(
      '.poodle-menu-surface__item[data-kind="submenu"]',
    ) as HTMLElement;
    expect(parent.getAttribute("aria-label")).toBe("Open In");
    expect(parent.getAttribute("aria-haspopup")).toBe("menu");
    expect(parent.getAttribute("aria-expanded")).toBe("false");
  });
});

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
    const { container } = render(Menu, { props: { items: identityItems } });
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const values = [...document.querySelectorAll('[role="menuitem"]')].map((el) =>
      el.getAttribute("data-value"),
    );
    expect(values).toEqual(["rename", "delete"]);
  });

  it("keeps one enabled menu item in the sequential tab order", async () => {
    const { container } = render(Menu, { props: { items: identityItems } });
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const menuItems = [...document.querySelectorAll('[role="menuitem"]')] as HTMLButtonElement[];
    expect(menuItems.map((item) => item.tabIndex)).toEqual([0, -1]);

    await fireEvent.keyDown(menuItems[0], { key: "ArrowDown" });
    expect(document.activeElement).toBe(menuItems[1]);
  });
});

describe("Menu cascading keyboard ownership", () => {
  it("skips disabled children, returns to the parent and commits only leaves", async () => {
    const actions: string[] = [];
    const { container } = render(Menu, { props: { items: [{ value: "parent", label: "Parent", children: [
      { value: "disabled", label: "Disabled", disabled: true },
      { value: "first", label: "First" },
      { value: "last", label: "Last", kind: "radio", checked: true },
    ] }], onAction: (value: string) => actions.push(value) } });
    await fireEvent.click(container.querySelector('.poodle-menu__trigger')!);
    const parent = document.querySelector('[data-value="parent"]') as HTMLElement;
    await fireEvent.keyDown(parent, { key: "ArrowRight" });
    await new Promise((resolve) => setTimeout(resolve, 0));
    const first = document.querySelector('[data-value="first"]') as HTMLElement;
    const last = document.querySelector('[data-value="last"]') as HTMLElement;
    expect(document.activeElement).toBe(first);
    await fireEvent.keyDown(first, { key: "End" });
    expect(document.activeElement).toBe(last);
    await fireEvent.keyDown(last, { key: "ArrowLeft" });
    expect(document.activeElement).toBe(parent);
    expect(document.querySelector('[data-value="last"]')).toBeNull();
    expect(actions).toEqual([]);
    await fireEvent.keyDown(parent, { key: "ArrowRight" });
    await new Promise((resolve) => setTimeout(resolve, 0));
    await fireEvent.mouseDown(document.querySelector('[data-value="last"]')!);
    expect(document.querySelector('[data-value="last"]')).not.toBeNull();
    await fireEvent.click(document.querySelector('[data-value="last"]')!);
    expect(actions).toEqual(["last"]);
    expect(document.querySelector('[role="menu"]')).toBeNull();
  });
});
