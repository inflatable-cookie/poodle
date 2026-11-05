import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

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
    const rows = [...surface.querySelectorAll(":scope > .poodle-menu-surface__item-anchor > button.poodle-menu-surface__item")];
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


describe("Menu (react) cascading children", () => {
  const children: MenuItem[] = [
    { value: "disabled", label: "Disabled", disabled: true },
    { value: "first", label: "First", kind: "radio", checked: true },
    { value: "separator", label: "Separator", kind: "separator" },
    { value: "last", label: "Last", kind: "radio" },
  ];
  const nestedItems: MenuItem[] = [
    { value: "parent", label: "Parent", children },
    { value: "sibling", label: "Sibling" },
  ];
  const row = (value: string) => document.querySelector(`[data-value="${value}"]`) as HTMLButtonElement;

  it("opens with ArrowRight, skips disabled rows, navigates boundaries and returns with ArrowLeft", async () => {
    const { container } = render(<Menu items={nestedItems} />);
    await fireEvent.click(container.querySelector(".poodle-menu__trigger")!);
    expect(row("parent").getAttribute("aria-haspopup")).toBe("menu");
    await fireEvent.keyDown(row("parent"), { key: "ArrowRight" });
    expect(document.activeElement).toBe(row("first"));
    expect(row("first").getAttribute("aria-checked")).toBe("true");
    await fireEvent.keyDown(row("first"), { key: "End" });
    expect(document.activeElement).toBe(row("last"));
    await fireEvent.keyDown(row("last"), { key: "ArrowDown" });
    expect(document.activeElement).toBe(row("first"));
    await fireEvent.keyDown(row("first"), { key: "ArrowLeft" });
    expect(document.activeElement).toBe(row("parent"));
    expect(row("first")).toBeNull();
  });

  it("keeps pointer interactions inside the cascade and commits only leaf actions", async () => {
    const onAction = vi.fn();
    const { container } = render(<Menu items={nestedItems} onAction={onAction} />);
    await fireEvent.click(container.querySelector(".poodle-menu__trigger")!);
    await fireEvent.pointerOver(row("parent"));
    await fireEvent.mouseDown(row("first"));
    expect(row("parent")).not.toBeNull();
    expect(onAction).not.toHaveBeenCalled();
    await fireEvent.click(row("last"));
    expect(onAction).toHaveBeenCalledExactlyOnceWith("last");
    expect(document.querySelector('[role="menu"]')).toBeNull();
  });

  it("closes the flyout when a sibling is hovered and dismisses on Escape", async () => {
    const { container } = render(<Menu items={nestedItems} />);
    await fireEvent.click(container.querySelector(".poodle-menu__trigger")!);
    await fireEvent.pointerOver(row("parent"));
    expect(row("first")).not.toBeNull();
    await fireEvent.pointerOver(row("sibling"));
    expect(row("first")).toBeNull();
    await fireEvent.keyDown(row("parent"), { key: "ArrowRight" });
    await fireEvent.keyDown(row("first"), { key: "Escape" });
    expect(document.querySelector('[role="menu"]')).toBeNull();
  });
});
