import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Drawer } from "../src/Drawer";

/**
 * `dismissOnOutsideInteract` on a modal drawer. Drawer registers the dismiss
 * layer with `false` today (a modal that vanishes on an outside click loses
 * work), so the default must stay false; the prop makes the layer's outside
 * axis refusable *and* enableable. The backdrop button remains the drawer's
 * own dismissal path (`dismissOnBackdrop`), untouched here.
 */
describe("Drawer (react) dismissOnOutsideInteract", () => {
  it("keeps the drawer open on outside mousedown by default (false)", async () => {
    render(<Drawer defaultOpen title="Settings" />);
    expect(screen.getByRole("dialog")).toBeTruthy();

    await fireEvent.mouseDown(document.body);
    expect(screen.getByRole("dialog")).toBeTruthy();
  });

  it("dismisses the drawer on outside mousedown when true", async () => {
    render(<Drawer defaultOpen title="Settings" dismissOnOutsideInteract />);
    expect(screen.getByRole("dialog")).toBeTruthy();

    await fireEvent.mouseDown(document.body);
    expect(screen.queryByRole("dialog")).toBeNull();
  });
});
