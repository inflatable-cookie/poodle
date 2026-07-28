import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";

import Dialog from "../src/Dialog.svelte";

/**
 * What a *controlled* Dialog does when the user asks it to close.
 *
 * `dialog.md` §Behavior Machine states it plainly: "controlled mode never
 * writes `open`; the parent owns it and reacts to `emitOpenChange`". These
 * tests pin that down, because it is easy to read as a bug — the close button
 * appears to do nothing — and because ten other Poodle components with a
 * `$bindable` prop behave the opposite way.
 */
describe("Dialog controlled open", () => {
  it("reports the close request but does not close itself", async () => {
    const changes: boolean[] = [];

    render(Dialog, {
      props: { open: true, showCloseButton: true, title: "Settings", onOpenChange: (o: boolean) => changes.push(o) },
    });

    await fireEvent.click(screen.getByRole("button", { name: /close/i }));

    // The request is reported…
    expect(changes).toEqual([false]);
    // …and the dialog stays up, because the host owns `open` and has not
    // changed it. This is what makes a close-veto possible.
    expect(screen.getByRole("dialog")).toBeTruthy();
  });

  it("closes itself when uncontrolled", async () => {
    render(Dialog, { props: { defaultOpen: true, showCloseButton: true, title: "Settings" } });

    expect(screen.getByRole("dialog")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: /close/i }));
    expect(screen.queryByRole("dialog")).toBeNull();
  });
});
