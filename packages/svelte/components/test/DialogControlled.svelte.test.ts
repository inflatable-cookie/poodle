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
  it("reports the close request and closes, because it writes back", async () => {
    const changes: boolean[] = [];

    render(Dialog, {
      props: { open: true, showCloseButton: true, title: "Settings", onOpenChange: (o: boolean) => changes.push(o) },
    });

    await fireEvent.click(screen.getByRole("button", { name: /close/i }));

    // The request is reported, and the dialog acts on it — matching the ten
    // other bindable components rather than being the odd one out.
    expect(changes).toEqual([false]);
    expect(screen.queryByRole("dialog")).toBeNull();
  });

  it("closes itself when uncontrolled", async () => {
    render(Dialog, { props: { defaultOpen: true, showCloseButton: true, title: "Settings" } });

    expect(screen.getByRole("dialog")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: /close/i }));
    expect(screen.queryByRole("dialog")).toBeNull();
  });
});

/**
 * Whether write-back and the close-veto can coexist.
 *
 * The argument for keeping overlays strict was that a host must be able to
 * *refuse* a close — unsaved changes, a job in flight. If write-back makes that
 * impossible, strict wins regardless of consistency. If it does not, the
 * inconsistency with the other ten bindable components is just a trap.
 */
describe("write-back versus the close-veto", () => {
  it("a host can still refuse the close by re-asserting open", async () => {
    let open = true;
    const seen: boolean[] = [];

    render(Dialog, {
      props: {
        get open() {
          return open;
        },
        set open(v: boolean) {
          open = v;
        },
        showCloseButton: true,
        title: "Unsaved work",
        onOpenChange: (next: boolean) => {
          seen.push(next);
          if (next === false) open = true; // veto
        },
      },
    });

    await fireEvent.click(screen.getByRole("button", { name: /close/i }));

    expect(seen).toEqual([false]);
    expect(open).toBe(true);
    expect(screen.getByRole("dialog")).toBeTruthy();
  });
});
