import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Drawer from "../src/Drawer.svelte";

// jsdom lacks the Web Animations API, which Svelte 5 transitions call through
// `element.animate`. Drawer slides/fades out on close, so a close would throw
// before unmounting. The fake fires `onfinish` on the next microtask — after
// Svelte has attached it — which drives the outro to completion.
if (!("animate" in Element.prototype)) {
  (Element.prototype as unknown as { animate: () => unknown }).animate = () => {
    const animation = {
      onfinish: null as (() => void) | null,
      cancel: () => {},
      playState: "finished",
      currentTime: 0,
      effect: null,
      finished: Promise.resolve(),
    };
    queueMicrotask(() => animation.onfinish?.());
    return animation;
  };
}

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
