import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Breadcrumbs from "../src/Breadcrumbs.svelte";

const items = [
  { value: "home", label: "Home", href: "/" },
  { value: "guide", label: "Guide" },
  { value: "docs", label: "Docs", href: "/docs" },
];

describe("Breadcrumbs (svelte)", () => {
  it("marks the last item as the current page", () => {
    const { container } = render(Breadcrumbs, { props: { items } });
    const current = container.querySelector('[aria-current="page"]');
    expect(current?.textContent).toBe("Docs");
  });

  it("renders href items as links and non-current, linkless items as buttons", () => {
    const { container } = render(Breadcrumbs, { props: { items } });
    const links = container.querySelectorAll("a");
    expect(links.length).toBe(1);
    expect(links[0].getAttribute("href")).toBe("/");
    expect(container.querySelectorAll("button").length).toBe(1);
  });

  it("emits onNavigate for a non-current, linkless item", async () => {
    const onNavigate = vi.fn();
    const { container } = render(Breadcrumbs, { props: { items, onNavigate } });
    const button = container.querySelector("button") as HTMLButtonElement;

    await fireEvent.click(button);

    expect(onNavigate).toHaveBeenCalledWith("guide");
  });

  it("collapses overflow to a first item and an ellipsis", () => {
    const { container } = render(Breadcrumbs, { props: { items, maxVisibleItems: 2 } });
    const buttons = container.querySelectorAll("button");
    const ellipsis = [...container.querySelectorAll("li span")].find(
      (el) => el.getAttribute("aria-hidden") === "true" && el.textContent === "…",
    );
    expect(buttons.length).toBe(0);
    expect(ellipsis).not.toBeUndefined();
  });

  it("renders an item icon and its label inside one navigation target", () => {
    const { container } = render(Breadcrumbs, {
      props: { items: [{ value: "projects", label: "Projects", icon: "folder", href: "/projects" }, ...items] },
    });
    const link = container.querySelector('a[href="/projects"]') as HTMLAnchorElement;

    expect(link.querySelector("svg.poodle-icon")).not.toBeNull();
    expect(link.textContent?.trim()).toBe("Projects");
  });

  it("keeps the label as the accessible name of an icon-only item", () => {
    const { container } = render(Breadcrumbs, {
      props: {
        items: [
          { value: "home", label: "Home", icon: "home", iconOnly: true, href: "/" },
          ...items.slice(1),
        ],
      },
    });
    const link = container.querySelector('a[href="/"]') as HTMLAnchorElement;
    const label = link.querySelector(".poodle-breadcrumbs__label") as HTMLElement;

    expect(link.querySelector("svg.poodle-icon")).not.toBeNull();
    expect(label.textContent).toBe("Home");
    expect(label.classList.contains("poodle-breadcrumbs__label--hidden")).toBe(true);
    expect(label.getAttribute("aria-hidden")).toBeNull();
  });

  it("keeps item icons decorative", () => {
    const { container } = render(Breadcrumbs, {
      props: { items: [{ value: "home", label: "Home", icon: "home", iconOnly: true, href: "/" }, ...items.slice(1)] },
    });
    const icon = container.querySelector(".poodle-breadcrumbs__content svg") as SVGElement;

    expect(icon.getAttribute("aria-hidden")).toBe("true");
    expect(icon.getAttribute("role")).toBe("presentation");
  });

  it("renders an icon-only current item as a named current-page span", () => {
    const { container } = render(Breadcrumbs, {
      props: { items: [...items.slice(0, 2), { value: "docs", label: "Docs", icon: "package", iconOnly: true }] },
    });
    const current = container.querySelector('[aria-current="page"]') as HTMLElement;

    expect(current.querySelector("svg.poodle-icon")).not.toBeNull();
    expect(current.querySelector(".poodle-breadcrumbs__label")?.textContent).toBe("Docs");
  });

  it("does not give the synthetic ellipsis an icon", () => {
    const iconItems = items.map((item) => ({ ...item, icon: "folder" }));
    const { container } = render(Breadcrumbs, { props: { items: iconItems, maxVisibleItems: 2 } });
    const ellipsis = [...container.querySelectorAll("li span")].find(
      (el) => el.getAttribute("aria-hidden") === "true" && el.textContent === "\u2026",
    ) as HTMLElement;

    expect(ellipsis.querySelector("svg")).toBeNull();
  });

  it("sizes item icons from the resolved breadcrumbs size", () => {
    const { container } = render(Breadcrumbs, {
      props: { items: [{ value: "home", label: "Home", icon: "home", href: "/" }, ...items.slice(1)], size: "lg" },
    });
    const icon = container.querySelector(".poodle-breadcrumbs__content svg") as SVGElement;

    expect(icon.getAttribute("data-size")).toBe("lg");
  });
});
