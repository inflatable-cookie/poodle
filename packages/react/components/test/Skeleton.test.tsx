import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Skeleton } from "../src/Skeleton";

describe("Skeleton (react)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-skeleton") as HTMLElement;

  it("resolves circle shape to square rem dimensions and hides it from assistive tech", () => {
    const root = rootOf(render(<Skeleton shape="circle" />).container);
    expect(root.dataset.shape).toBe("circle");
    expect(root.style.getPropertyValue("--poodle-skeleton-width")).toBe("2.5rem");
    expect(root.style.getPropertyValue("--poodle-skeleton-height")).toBe("2.5rem");
    expect(root.getAttribute("aria-hidden")).toBe("true");
  });

  it("applies explicit width and height overrides", () => {
    const root = rootOf(render(<Skeleton width="12rem" height="3rem" />).container);
    expect(root.style.getPropertyValue("--poodle-skeleton-width")).toBe("12rem");
    expect(root.style.getPropertyValue("--poodle-skeleton-height")).toBe("3rem");
  });

  it("renders the table-row preset anatomy when preset is set", () => {
    const { container } = render(<Skeleton preset="table-row" animated={false} />);
    const preset = container.querySelector(".poodle-skeleton-preset--table-row") as HTMLElement;
    expect(preset).not.toBeNull();
    expect(preset.dataset.animated).toBe("false");
    expect(preset.querySelectorAll(".poodle-skeleton--cell").length).toBe(4);
  });

  it("honours the detail-section preset line count", () => {
    const { container } = render(<Skeleton preset="detail-section" lines={5} />);
    const items = container.querySelectorAll(".poodle-skeleton-preset__detail-item");
    expect(items.length).toBe(5);
  });
});
