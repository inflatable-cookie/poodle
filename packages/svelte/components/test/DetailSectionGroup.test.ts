import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import DetailSectionGroup from "../src/DetailSectionGroup.svelte";
import { asSnippet } from "./snippet";

describe("DetailSectionGroup (svelte)", () => {
  it("projects layout and max-columns data attributes", () => {
    const { container } = render(DetailSectionGroup, {
      props: { layout: "grid", maxColumns: 3 },
    });
    const root = container.querySelector(".poodle-detail-section-group") as HTMLElement;
    expect(root.dataset.layout).toBe("grid");
    expect(root.dataset.maxColumns).toBe("3");
  });

  it("exposes min column widths as custom properties on the root", () => {
    const { container } = render(DetailSectionGroup, {
      props: { minColumnWidth: "16rem", itemMinColumnWidth: "10rem" },
    });
    const root = container.querySelector(".poodle-detail-section-group") as HTMLElement;
    expect(root.style.getPropertyValue("--poodle-detail-section-group-min")).toBe("16rem");
    expect(root.style.getPropertyValue("--poodle-detail-section-group-item-min")).toBe("10rem");
  });

  it("applies the accessible label to the grouping region", () => {
    const { container } = render(DetailSectionGroup, {
      props: { ariaLabel: "Record groups" },
    });
    const root = container.querySelector(".poodle-detail-section-group") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Record groups");
  });

  it("renders children into the grid", () => {
    const { container } = render(DetailSectionGroup, {
      props: { children: asSnippet(() => "<div>section</div>") },
    });
    expect(container.querySelector(".poodle-detail-section-group__grid")).not.toBeNull();
  });
});
