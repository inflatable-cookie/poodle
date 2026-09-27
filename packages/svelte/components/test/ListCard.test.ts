import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import ListCard from "../src/ListCard.svelte";
import ListCardEyebrowHarness from "./ListCardEyebrowHarness.svelte";

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
});

describe("ListCard (svelte) eyebrow", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-list-card") as HTMLElement;
  const eyebrowOf = (container: HTMLElement) =>
    container.querySelector(".poodle-list-card__eyebrow") as HTMLElement | null;

  it("renders the eyebrow above the title", () => {
    const { container } = render(ListCard, {
      props: { title: "logo-primary.svg", eyebrow: "Brand kit" },
    });

    const eyebrow = eyebrowOf(container);
    expect(eyebrow).not.toBeNull();
    expect(eyebrow?.textContent).toBe("Brand kit");
    // The eyebrow lane sits directly above the header row in the body.
    expect(eyebrow?.nextElementSibling?.classList.contains("poodle-list-card__header")).toBe(
      true,
    );
  });

  it("renders nothing when unset", () => {
    const { container } = render(ListCard, { props: { title: "Card" } });

    expect(eyebrowOf(container)).toBeNull();
  });

  it("prefers the eyebrowContent snippet over the plain eyebrow", () => {
    const { container } = render(ListCardEyebrowHarness);

    const eyebrow = eyebrowOf(container);
    expect(eyebrow?.querySelector(".harness-eyebrow")?.textContent).toBe("Contracts · rich");
    expect(eyebrow?.textContent).not.toContain("Ignored fallback");
  });

  it("does not change the accessible name on either root", () => {
    const div = rootOf(
      render(ListCard, {
        props: { title: "logo-primary.svg", eyebrow: "Brand kit", interactive: true },
      }).container,
    );
    const anchor = rootOf(
      render(ListCard, {
        props: { title: "logo-primary.svg", eyebrow: "Brand kit", href: "#brand-kit" },
      }).container,
    );

    expect(div.getAttribute("aria-label")).toBe("logo-primary.svg");
    expect(anchor.getAttribute("aria-label")).toBe("logo-primary.svg");
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
