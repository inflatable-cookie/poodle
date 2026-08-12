import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { ListCard } from "../src/ListCard";

// Mirrors packages/svelte/components/test/ListCard.test.ts: the <a> and <div>
// roots must resolve data-size from the same sizeRole.
describe("ListCard (react)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-list-card") as HTMLElement;

  it("emits the same data-size from the div and anchor roots", () => {
    const div = rootOf(render(<ListCard title="Card" />).container);
    const anchor = rootOf(render(<ListCard title="Card" href="#" />).container);

    expect(anchor.tagName).toBe("A");
    expect(div.tagName).toBe("DIV");
    expect(anchor.dataset.size).toBe(div.dataset.size);
  });

  it("honours an explicit size on both roots", () => {
    const div = rootOf(render(<ListCard title="Card" size="lg" />).container);
    const anchor = rootOf(
      render(<ListCard title="Card" href="#" size="lg" />).container,
    );

    expect(div.dataset.size).toBe("lg");
    expect(anchor.dataset.size).toBe("lg");
  });
});

describe("ListCard (react) dismissOnOutsideInteract", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-list-card") as HTMLElement;

  // The context menu is portalled to the theme root via the anchored surface,
  // so it is not reachable from the render container.
  const menuOf = () => document.querySelector(".poodle-list-card__context-menu") as HTMLElement;

  const contextMenuItems = [
    { value: "rename", label: "Rename" },
    { value: "delete", label: "Delete" },
  ];

  it("dismisses the context menu on outside mousedown by default", async () => {
    const { container } = render(
      <ListCard title="Card" contextMenuItems={contextMenuItems} />,
    );
    await fireEvent.contextMenu(rootOf(container));
    expect(menuOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(menuOf()).toBeNull();
  });

  it("keeps the context menu open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(
      <ListCard title="Card" contextMenuItems={contextMenuItems} dismissOnOutsideInteract={false} />,
    );
    await fireEvent.contextMenu(rootOf(container));
    expect(menuOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(menuOf()).not.toBeNull();
  });
});
