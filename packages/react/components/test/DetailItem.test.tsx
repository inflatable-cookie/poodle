import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { DetailItem } from "../src/DetailItem";

describe("DetailItem (react)", () => {
  it("renders the label and falls back to emptyText when value is null", () => {
    const { container } = render(<DetailItem label="Status" />);
    expect(container.querySelector(".poodle-detail-item__label")?.textContent).toContain("Status");
    expect(container.querySelector(".poodle-detail-item__value")?.textContent).toBe("—");
  });

  it("renders a plain value and honours the layout data attributes", () => {
    const { container } = render(<DetailItem label="Name" value="Ada" layout="stacked" span={2} />);
    expect(container.querySelector(".poodle-detail-item__value")?.textContent).toBe("Ada");
    const root = container.querySelector(".poodle-detail-item") as HTMLElement;
    expect(root.dataset.layout).toBe("stacked");
    expect(root.dataset.span).toBe("2");
  });

  it("adds the truncate class when truncateValue is set", () => {
    const { container } = render(<DetailItem label="Path" value="/a/long/path" truncateValue />);
    expect(
      container.querySelector(".poodle-detail-item__value")?.classList.contains("poodle-truncate"),
    ).toBe(true);
  });

  it("renders the action region when an action node is present", () => {
    const { container } = render(<DetailItem label="Owner" value="Ada" action={<span>Edit</span>} />);
    expect(container.querySelector(".poodle-detail-item__action")?.textContent).toBe("Edit");
  });
});
