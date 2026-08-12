import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import SplitButton from "../src/SplitButton.svelte";
import type { MenuItem } from "../src/types";

const items: MenuItem[] = [
  { value: "save-as", label: "Save as" },
  { value: "export", label: "Export" },
];

describe("SplitButton (svelte) dismissOnOutsideInteract", () => {
  const toggleOf = (container: HTMLElement) =>
    container.querySelector(".poodle-split-button__toggle") as HTMLButtonElement;

  // The menu is portalled to the theme root via the anchored action, so it is
  // not reachable from the render container.
  const menuOf = () => document.querySelector(".poodle-split-button__menu") as HTMLElement;

  it("dismisses the menu on outside mousedown by default", async () => {
    const { container } = render(SplitButton, { props: { items, children: () => "Save" } });
    await fireEvent.click(toggleOf(container));
    expect(menuOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(menuOf()).toBeNull();
  });

  it("keeps the menu open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(SplitButton, {
      props: { items, dismissOnOutsideInteract: false, children: () => "Save" },
    });
    await fireEvent.click(toggleOf(container));
    expect(menuOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(menuOf()).not.toBeNull();
  });
});
