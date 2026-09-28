import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Drawer from "../src/Drawer.svelte";

// Web Animations stub comes from the shared test/vitest.setup.ts.

/**
 * `dismissOnOutsideInteract` on a modal drawer. Drawer registers the dismiss
 * layer with `false` today (a modal that vanishes on an outside click loses
 * work), so the default must stay false; the prop makes the layer's outside
 * axis refusable *and* enableable. The backdrop button remains the drawer's
 * own dismissal path (`dismissOnBackdrop`), untouched here.
 */
describe("Drawer (svelte) dismissOnOutsideInteract", () => {
  it("keeps the drawer open on outside mousedown by default (false)", async () => {
    render(Drawer, { props: { defaultOpen: true, title: "Settings" } });
    expect(screen.getByRole("dialog")).toBeTruthy();

    await fireEvent.mouseDown(document.body);
    expect(screen.getByRole("dialog")).toBeTruthy();
  });

  it("dismisses the drawer on outside mousedown when true", async () => {
    render(Drawer, {
      props: { defaultOpen: true, title: "Settings", dismissOnOutsideInteract: true },
    });
    expect(screen.getByRole("dialog")).toBeTruthy();

    await fireEvent.mouseDown(document.body);
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
  });
});
