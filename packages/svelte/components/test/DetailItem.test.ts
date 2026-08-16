import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import DetailItem from "../src/DetailItem.svelte";
import { asSnippet } from "./snippet";

describe("DetailItem (svelte)", () => {
  it("renders the label and falls back to emptyText when value is null", () => {
    const { container } = render(DetailItem, { props: { label: "Status" } });
    expect(container.querySelector(".poodle-detail-item__label")?.textContent).toContain("Status");
    expect(container.querySelector(".poodle-detail-item__value")?.textContent).toBe("—");
  });

  it("renders a plain value and honours the layout data attributes", () => {
    const { container } = render(DetailItem, {
      props: { label: "Name", value: "Ada", layout: "stacked", span: 2 },
    });
    expect(container.querySelector(".poodle-detail-item__value")?.textContent).toBe("Ada");
    const root = container.querySelector(".poodle-detail-item") as HTMLElement;
    expect(root.dataset.layout).toBe("stacked");
    expect(root.dataset.span).toBe("2");
  });

  it("adds the truncate class when truncateValue is set", () => {
    const { container } = render(DetailItem, {
      props: { label: "Path", value: "/a/long/path", truncateValue: true },
    });
    expect(container.querySelector(".poodle-detail-item__value")?.classList.contains("poodle-truncate")).toBe(true);
  });

  it("renders the action region when an action snippet is present", () => {
    const { container } = render(DetailItem, {
      props: { label: "Owner", value: "Ada", action: asSnippet(() => "Edit") },
    });
    expect(container.querySelector(".poodle-detail-item__action")).not.toBeNull();
  });
});
