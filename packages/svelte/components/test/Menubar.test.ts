import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Menubar from "../src/Menubar.svelte";
import type { MenubarItem } from "../src/types";

const items: MenubarItem[] = [
  {
    value: "file",
    label: "File",
    items: [
      { value: "new", label: "New" },
      { value: "open", label: "Open" },
    ],
  },
];

describe("Menubar (svelte) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-menubar__trigger") as HTMLButtonElement;

  // The open menu is portalled to the theme root via the anchored action, so
  // it is not reachable from the render container.
  const overlayOf = () => document.querySelector(".poodle-menubar__overlay") as HTMLElement;

  it("dismisses the open menu on outside mousedown by default", async () => {
    const { container } = render(Menubar, { props: { items } });
    await fireEvent.click(triggerOf(container));
    expect(overlayOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(overlayOf()).toBeNull();
  });

  it("keeps the menu open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(Menubar, {
      props: { items, dismissOnOutsideInteract: false },
    });
    await fireEvent.click(triggerOf(container));
    expect(overlayOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(overlayOf()).not.toBeNull();
  });
});
