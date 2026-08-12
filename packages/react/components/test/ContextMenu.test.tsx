import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { ContextMenu } from "../src/ContextMenu";
import type { MenuItem } from "../src/types";

const items: MenuItem[] = [
  { value: "rename", label: "Rename" },
  { value: "delete", label: "Delete" },
];

describe("ContextMenu (react) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-context-menu") as HTMLElement;

  // The surface is portalled to the theme root, so it is not reachable from
  // the render container.
  const surfaceOf = () => document.querySelector(".poodle-menu-surface") as HTMLElement;

  it("dismisses the menu on outside mousedown by default", async () => {
    const { container } = render(<ContextMenu items={items} />);
    await fireEvent.contextMenu(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).toBeNull();
  });

  it("keeps the menu open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(
      <ContextMenu items={items} dismissOnOutsideInteract={false} />,
    );
    await fireEvent.contextMenu(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).not.toBeNull();
  });
});
