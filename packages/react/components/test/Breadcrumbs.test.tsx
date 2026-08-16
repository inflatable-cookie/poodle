import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Breadcrumbs } from "../src/Breadcrumbs";

const items = [
  { value: "home", label: "Home", href: "/" },
  { value: "guide", label: "Guide" },
  { value: "docs", label: "Docs", href: "/docs" },
];

describe("Breadcrumbs (react)", () => {
  it("marks the last item as the current page", () => {
    const { container } = render(<Breadcrumbs items={items} />);
    const current = container.querySelector('[aria-current="page"]');
    expect(current?.textContent).toBe("Docs");
  });

  it("renders href items as links and non-current, linkless items as buttons", () => {
    const { container } = render(<Breadcrumbs items={items} />);
    const links = container.querySelectorAll("a");
    expect(links.length).toBe(1);
    expect(links[0].getAttribute("href")).toBe("/");
    expect(container.querySelectorAll("button").length).toBe(1);
  });

  it("emits onNavigate for a non-current, linkless item", () => {
    const onNavigate = vi.fn();
    const { container } = render(<Breadcrumbs items={items} onNavigate={onNavigate} />);
    const button = container.querySelector("button") as HTMLButtonElement;

    fireEvent.click(button);

    expect(onNavigate).toHaveBeenCalledWith("guide");
  });

  it("collapses overflow to a first item and an ellipsis", () => {
    const { container } = render(<Breadcrumbs items={items} maxVisibleItems={2} />);
    const buttons = container.querySelectorAll("button");
    const ellipsis = [...container.querySelectorAll("li span")].find(
      (el) => el.getAttribute("aria-hidden") === "true" && el.textContent === "…",
    );
    expect(buttons.length).toBe(0);
    expect(ellipsis).not.toBeUndefined();
  });
});
