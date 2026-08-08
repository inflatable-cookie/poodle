import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import { resolveLayerZIndex } from "@inflatable-cookie/poodle-headless";

import Harness from "./PopoverInDialogHarness.svelte";

/**
 * A Popover opened from inside a Dialog has to render above it.
 *
 * Portalling to the theme root fixes clipping but moves the surface out of the
 * dialog's subtree, so it stops being a descendant and has only its own token
 * to argue with: `overlay.z.menu` is 400 against `overlay.z.dialog` 800. The
 * popover lands behind the modal that opened it — visible, unclickable, and
 * useless.
 *
 * The z values live in the stylesheet, which jsdom does not load, so the
 * component-level test can only prove the surface is layered *at all*. The
 * arithmetic is pinned separately against explicit values.
 */
describe("resolveLayerZIndex", () => {
  it("lifts a surface above the highest stacking ancestor", () => {
    const outer = document.createElement("div");
    outer.style.zIndex = "800"; // a dialog
    const trigger = document.createElement("button");
    outer.appendChild(trigger);
    document.body.appendChild(outer);

    // Its own token (menu, 400) loses to the dialog, so it takes 801.
    expect(resolveLayerZIndex(trigger, 400)).toBe(801);
    outer.remove();
  });

  it("leaves a surface on its own token when nothing outranks it", () => {
    const plain = document.createElement("div");
    const trigger = document.createElement("button");
    plain.appendChild(trigger);
    document.body.appendChild(plain);

    // Page-level menu: no stacking ancestor, so the token stands and a menu
    // does not start floating over unrelated modals.
    expect(resolveLayerZIndex(trigger, 400)).toBe(400);
    plain.remove();
  });

  it("composes to any depth", () => {
    const drawer = document.createElement("div");
    drawer.style.zIndex = "800";
    const dialog = document.createElement("div");
    dialog.style.zIndex = "801";
    const trigger = document.createElement("button");
    dialog.appendChild(trigger);
    drawer.appendChild(dialog);
    document.body.appendChild(drawer);

    expect(resolveLayerZIndex(trigger, 400)).toBe(802);
    drawer.remove();
  });
});

describe("Popover opened from a Dialog", () => {
  it("is given an explicit z-index when it portals", async () => {
    render(Harness);
    await fireEvent.click(screen.getAllByRole("button", { name: /open popover/i })[0]);

    const surface = document.querySelector<HTMLElement>('[data-poodle-anchored="true"]');
    expect(surface).toBeTruthy();
    // Not asserting a number: the stylesheet that supplies the real tokens is
    // absent here, so only the fact that layering happened is observable.
    expect(surface!.style.zIndex).not.toBe("");
  });
});
