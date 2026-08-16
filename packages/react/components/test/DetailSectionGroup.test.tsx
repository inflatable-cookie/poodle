import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { DetailSectionGroup } from "../src/DetailSectionGroup";

describe("DetailSectionGroup (react)", () => {
  it("projects layout and max-columns data attributes", () => {
    const { container } = render(
      <DetailSectionGroup layout="grid" maxColumns={3}>
        <div>section</div>
      </DetailSectionGroup>,
    );
    const root = container.querySelector(".poodle-detail-section-group") as HTMLElement;
    expect(root.dataset.layout).toBe("grid");
    expect(root.dataset.maxColumns).toBe("3");
  });

  it("exposes min column widths as custom properties on the root", () => {
    const { container } = render(
      <DetailSectionGroup minColumnWidth="16rem" itemMinColumnWidth="10rem">
        <div>section</div>
      </DetailSectionGroup>,
    );
    const root = container.querySelector(".poodle-detail-section-group") as HTMLElement;
    expect(root.style.getPropertyValue("--poodle-detail-section-group-min")).toBe("16rem");
    expect(root.style.getPropertyValue("--poodle-detail-section-group-item-min")).toBe("10rem");
  });

  it("applies the accessible label to the grouping region", () => {
    const { container } = render(
      <DetailSectionGroup ariaLabel="Record groups">
        <div>section</div>
      </DetailSectionGroup>,
    );
    const root = container.querySelector(".poodle-detail-section-group") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Record groups");
  });

  it("renders children into the grid", () => {
    const { container } = render(
      <DetailSectionGroup>
        <div>section</div>
      </DetailSectionGroup>,
    );
    expect(container.querySelector(".poodle-detail-section-group__grid")).not.toBeNull();
  });
});
