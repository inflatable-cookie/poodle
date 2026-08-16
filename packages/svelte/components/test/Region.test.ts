import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Region from "../src/Region.svelte";

describe("Region (svelte)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-region") as HTMLElement;

  it("renders as a presentation placeholder with the label", () => {
    const { container } = render(Region, { props: { label: "Canvas area" } });
    const root = rootOf(container);
    expect(root.getAttribute("role")).toBe("presentation");
    expect(container.querySelector(".poodle-region__label")?.textContent).toBe("Canvas area");
  });

  it("omits the label element when no label is given", () => {
    const { container } = render(Region, {});
    expect(container.querySelector(".poodle-region__label")).toBeNull();
  });

  it("applies min-height and custom color as region tokens", () => {
    const { container } = render(Region, { props: { minHeight: "8rem", color: "#ff00aa" } });
    const root = rootOf(container);
    expect(root.style.minHeight).toBe("8rem");
    expect(root.style.getPropertyValue("--region-color")).toBe("#ff00aa");
  });
});
