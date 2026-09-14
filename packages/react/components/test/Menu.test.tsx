import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Menu } from "../src/Menu";
import type { MenuItem } from "../src/types";

const items: MenuItem[] = [
  { value: "rename", label: "Rename" },
  { value: "delete", label: "Delete", tone: "danger" },
  { value: "divider", label: "divider", kind: "separator" },
  { value: "show-grid", label: "Show grid", kind: "checkbox", checked: true, shortcutLabel: "⌘G" },
  { value: "small", label: "Small icons", kind: "radio" },
  { value: "export", label: "Export…", disabled: true },
];

/** Original g14.007 fixture; retained regressions below still assume it. */
const identityItems: MenuItem[] = [
  { value: "rename", label: "Rename" },
  { value: "delete", label: "Delete" },
];

/** g18.037 — every non-separator row names itself with the exact required label. */
describe("Menu (react) item accessible names", () => {
  it("exposes each non-separator item's explicit aria-label and leaves separators unnamed", async () => {
    const { container } = render(<Menu items={items} />);
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const surface = document.querySelector(".poodle-menu-surface") as HTMLElement;
    const rows = [...surface.querySelectorAll(":scope > button.poodle-menu-surface__item")];
    expect(rows.map((row) => row.getAttribute("aria-label"))).toEqual([
      "Rename",
      "Delete",
      "Show grid",
      "Small icons",
      "Export…",
    ]);

    const separator = surface.querySelector('[role="separator"]');
    expect(separator).not.toBeNull();
    expect(separator?.getAttribute("aria-label")).toBeNull();
  });

  it("keeps the disabled row's exact label alongside its disabled behavior", async () => {
    const { container } = render(<Menu items={items} />);
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const disabled = document.querySelector(
      '.poodle-menu-surface__item[data-value="export"]',
    ) as HTMLButtonElement;
    expect(disabled.getAttribute("aria-label")).toBe("Export…");
    expect(disabled.disabled).toBe(true);
  });
});

describe("Menu (react) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-menu__trigger") as HTMLElement;

  // The surface is portalled to the theme root, so it is not reachable from
  // the render container.
  const surfaceOf = () => document.querySelector(".poodle-menu-surface") as HTMLElement;

  it("dismisses the menu on outside mousedown by default", async () => {
    const { container } = render(<Menu items={items} />);
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).toBeNull();
  });

  it("keeps the menu open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(<Menu items={items} dismissOnOutsideInteract={false} />);
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).not.toBeNull();
  });
});

/** g14.007 retained regression — see the Select pair for the claim. */
describe("Menu (react) item identity", () => {
  it("addresses every item by its value", async () => {
    const { container } = render(<Menu items={identityItems} />);
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const values = [...document.querySelectorAll('[role="menuitem"]')].map((el) =>
      el.getAttribute("data-value"),
    );
    expect(values).toEqual(["rename", "delete"]);
  });

  it("keeps one enabled menu item in the sequential tab order", async () => {
    const { container } = render(<Menu items={identityItems} />);
    await fireEvent.click(container.querySelector(".poodle-menu__trigger") as HTMLElement);

    const menuItems = [...document.querySelectorAll('[role="menuitem"]')] as HTMLButtonElement[];
    expect(menuItems.map((item) => item.tabIndex)).toEqual([0, -1]);

    await fireEvent.keyDown(menuItems[0], { key: "ArrowDown" });
    expect(document.activeElement).toBe(menuItems[1]);
  });
});
