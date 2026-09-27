import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ListCard from "../src/ListCard.svelte";

// ListCard has a dual root: <a> when href is set (and not selectable), <div>
// otherwise. Both roots must resolve data-size from the same sizeRole — the
// anchor branch previously emitted the "chrome" role, so identical props
// rendered at different sizes depending on href.
describe("ListCard (svelte)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-list-card") as HTMLElement;

  it("emits the same data-size from the div and anchor roots", () => {
    const div = rootOf(render(ListCard, { props: { title: "Card" } }).container);
    const anchor = rootOf(
      render(ListCard, { props: { title: "Card", href: "#" } }).container,
    );

    expect(anchor.tagName).toBe("A");
    expect(div.tagName).toBe("DIV");
    expect(anchor.dataset.size).toBe(div.dataset.size);
  });

  it("honours an explicit size on both roots", () => {
    const div = rootOf(
      render(ListCard, { props: { title: "Card", size: "lg" } }).container,
    );
    const anchor = rootOf(
      render(ListCard, { props: { title: "Card", href: "#", size: "lg" } })
        .container,
    );

    expect(div.dataset.size).toBe("lg");
    expect(anchor.dataset.size).toBe("lg");
  });

  it("keeps button semantics when itemRole is unset", () => {
    const root = rootOf(
      render(ListCard, { props: { title: "Card", interactive: true } }).container,
    );
    expect(root.getAttribute("role")).toBe("button");
    expect(root.tabIndex).toBe(0);
    expect(root.getAttribute("aria-selected")).toBeNull();
  });

  it("renders option semantics and aria-selected without a nested button role", () => {
    const selected = rootOf(
      render(ListCard, {
        props: { title: "Chosen", itemRole: "option", selected: true, interactive: true },
      }).container,
    );
    expect(selected.tagName).toBe("DIV");
    expect(selected.getAttribute("role")).toBe("option");
    expect(selected.getAttribute("aria-selected")).toBe("true");
    expect(selected.getAttribute("aria-pressed")).toBeNull();
    expect(selected.tabIndex).toBe(-1);
    expect(selected.querySelector('[role="button"]')).toBeNull();

    const idle = rootOf(
      render(ListCard, {
        props: { title: "Idle", itemRole: "option", interactive: true },
      }).container,
    );
    expect(idle.getAttribute("aria-selected")).toBe("false");
  });

  it("renders listitem semantics without button or selected ARIA", () => {
    const root = rootOf(
      render(ListCard, {
        props: { title: "Row", itemRole: "listitem", selected: true, interactive: true },
      }).container,
    );
    expect(root.getAttribute("role")).toBe("listitem");
    expect(root.getAttribute("aria-selected")).toBeNull();
    expect(root.getAttribute("aria-pressed")).toBeNull();
    expect(root.tabIndex).toBe(-1);
  });

  it("does not render a link root when itemRole is set", () => {
    const root = rootOf(
      render(ListCard, {
        props: { title: "Linked option", href: "#", itemRole: "option", interactive: true },
      }).container,
    );
    expect(root.tagName).toBe("DIV");
    expect(root.getAttribute("role")).toBe("option");
    expect(root.getAttribute("href")).toBeNull();
  });

  it("does not activate on Enter or Space in option mode", async () => {
    const onClick = vi.fn();
    const onSelectedChange = vi.fn();
    const root = rootOf(
      render(ListCard, {
        props: {
          title: "Option",
          itemRole: "option",
          interactive: true,
          selectable: true,
          onClick,
          onSelectedChange,
        },
      }).container,
    );

    await fireEvent.keyDown(root, { key: "Enter" });
    await fireEvent.keyDown(root, { key: " " });
    expect(onClick).not.toHaveBeenCalled();
    expect(onSelectedChange).not.toHaveBeenCalled();
  });

  it("still toggles from the pointer in option mode", async () => {
    const onClick = vi.fn();
    const onSelectedChange = vi.fn();
    const root = rootOf(
      render(ListCard, {
        props: {
          title: "Option",
          itemRole: "option",
          interactive: true,
          selectable: true,
          onClick,
          onSelectedChange,
        },
      }).container,
    );

    await fireEvent.click(root);
    expect(onSelectedChange).toHaveBeenCalledWith(true);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("still activates from Enter when itemRole is unset", async () => {
    const onClick = vi.fn();
    const root = rootOf(
      render(ListCard, {
        props: { title: "Card", interactive: true, onClick },
      }).container,
    );

    await fireEvent.keyDown(root, { key: "Enter" });
    expect(onClick).toHaveBeenCalled();
  });
});

describe("ListCard (svelte) dismissOnOutsideInteract", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-list-card") as HTMLElement;

  // The context menu is portalled to the theme root via the anchored action,
  // so it is not reachable from the render container.
  const menuOf = () => document.querySelector(".poodle-list-card__context-menu") as HTMLElement;

  const contextMenuItems = [
    { value: "rename", label: "Rename" },
    { value: "delete", label: "Delete" },
  ];

  it("dismisses the context menu on outside mousedown by default", async () => {
    const { container } = render(ListCard, {
      props: { title: "Card", contextMenuItems },
    });
    await fireEvent.contextMenu(rootOf(container));
    expect(menuOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(menuOf()).toBeNull();
  });

  it("keeps the context menu open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(ListCard, {
      props: { title: "Card", contextMenuItems, dismissOnOutsideInteract: false },
    });
    await fireEvent.contextMenu(rootOf(container));
    expect(menuOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(menuOf()).not.toBeNull();
  });
});
