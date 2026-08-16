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
});
