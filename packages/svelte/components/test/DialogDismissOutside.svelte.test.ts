import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Dialog from "../src/Dialog.svelte";

/**
 * `dismissOnOutsideInteract` on a modal. Dialog registers the dismiss layer
 * with `false` today (a modal that vanishes on an outside click loses work),
 * so the default must stay false; the prop makes the layer's outside axis
 * refusable *and* enableable. The backdrop button remains the modal's own
 * dismissal path (`dismissOnBackdrop`), untouched here.
 */
describe("Dialog (svelte) dismissOnOutsideInteract", () => {
  it("keeps the dialog open on outside mousedown by default (false)", async () => {
    render(Dialog, { props: { defaultOpen: true, title: "Settings" } });
    expect(screen.getByRole("dialog")).toBeTruthy();

    await fireEvent.mouseDown(document.body);
    expect(screen.getByRole("dialog")).toBeTruthy();
  });

  it("dismisses the dialog on outside mousedown when true", async () => {
    render(Dialog, {
      props: { defaultOpen: true, title: "Settings", dismissOnOutsideInteract: true },
    });
    expect(screen.getByRole("dialog")).toBeTruthy();

    await fireEvent.mouseDown(document.body);
    expect(screen.queryByRole("dialog")).toBeNull();
  });
});
