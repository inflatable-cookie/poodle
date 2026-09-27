import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

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

  it("keeps button semantics when itemRole is unset", () => {
    const root = rootOf(render(<ListCard title="Card" interactive />).container);
    expect(root.getAttribute("role")).toBe("button");
    expect(root.tabIndex).toBe(0);
    expect(root.getAttribute("aria-selected")).toBeNull();
  });

  it("renders option semantics and aria-selected without a nested button role", () => {
    const selected = rootOf(
      render(<ListCard title="Chosen" itemRole="option" selected interactive />).container,
    );
    expect(selected.tagName).toBe("DIV");
    expect(selected.getAttribute("role")).toBe("option");
    expect(selected.getAttribute("aria-selected")).toBe("true");
    expect(selected.getAttribute("aria-pressed")).toBeNull();
    expect(selected.tabIndex).toBe(-1);
    expect(selected.querySelector('[role="button"]')).toBeNull();

    const idle = rootOf(
      render(<ListCard title="Idle" itemRole="option" interactive />).container,
    );
    expect(idle.getAttribute("aria-selected")).toBe("false");
  });

  it("renders listitem semantics without button or selected ARIA", () => {
    const root = rootOf(
      render(<ListCard title="Row" itemRole="listitem" selected interactive />).container,
    );
    expect(root.getAttribute("role")).toBe("listitem");
    expect(root.getAttribute("aria-selected")).toBeNull();
    expect(root.getAttribute("aria-pressed")).toBeNull();
    expect(root.tabIndex).toBe(-1);
  });

  it("does not render a link root when itemRole is set", () => {
    const root = rootOf(
      render(<ListCard title="Linked option" href="#" itemRole="option" interactive />).container,
    );
    expect(root.tagName).toBe("DIV");
    expect(root.getAttribute("role")).toBe("option");
    expect(root.getAttribute("href")).toBeNull();
  });

  it("does not activate on Enter or Space in option mode", () => {
    const onClick = vi.fn();
    const onSelectedChange = vi.fn();
    const root = rootOf(
      render(
        <ListCard
          title="Option"
          itemRole="option"
          interactive
          selectable
          onClick={onClick}
          onSelectedChange={onSelectedChange}
        />,
      ).container,
    );

    fireEvent.keyDown(root, { key: "Enter" });
    fireEvent.keyDown(root, { key: " " });
    expect(onClick).not.toHaveBeenCalled();
    expect(onSelectedChange).not.toHaveBeenCalled();
  });

  it("still toggles from the pointer in option mode", () => {
    const onClick = vi.fn();
    const onSelectedChange = vi.fn();
    const root = rootOf(
      render(
        <ListCard
          title="Option"
          itemRole="option"
          interactive
          selectable
          onClick={onClick}
          onSelectedChange={onSelectedChange}
        />,
      ).container,
    );

    fireEvent.click(root);
    expect(onSelectedChange).toHaveBeenCalledWith(true);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("still activates from Enter when itemRole is unset", () => {
    const onClick = vi.fn();
    const root = rootOf(
      render(<ListCard title="Card" interactive onClick={onClick} />).container,
    );

    fireEvent.keyDown(root, { key: "Enter" });
    expect(onClick).toHaveBeenCalled();
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
