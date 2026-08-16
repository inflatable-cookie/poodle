import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { DetailSection } from "../src/DetailSection";

describe("DetailSection (react)", () => {
  it("omits the header entirely without a title, description, or actions", () => {
    const { container } = render(<DetailSection>row</DetailSection>);
    expect(container.querySelector(".poodle-detail-section__header")).toBeNull();
    expect(container.querySelector(".poodle-detail-section__body")).not.toBeNull();
  });

  it("renders an h3 title and description when provided", () => {
    const { container } = render(
      <DetailSection title="Project details" description="Core metadata.">
        row
      </DetailSection>,
    );
    const title = container.querySelector(".poodle-detail-section__title") as HTMLElement;
    expect(title.tagName).toBe("H3");
    expect(title.textContent).toBe("Project details");
    expect(container.querySelector(".poodle-detail-section__description")?.textContent).toContain(
      "Core metadata.",
    );
  });

  it("projects the separated, columns, and max-auto-columns data attributes", () => {
    const { container } = render(
      <DetailSection title="Details" columns={3} maxAutoColumns={5}>
        row
      </DetailSection>,
    );
    const root = container.querySelector(".poodle-detail-section") as HTMLElement;
    expect(root.dataset.separated).toBe("true");
    expect(root.dataset.columns).toBe("3");
    expect(root.dataset.maxAutoColumns).toBe("5");
  });

  it("renders the actions region when actions content is provided", () => {
    const { container } = render(
      <DetailSection title="Billing" actions={<button>Edit</button>}>
        row
      </DetailSection>,
    );
    expect(container.querySelector(".poodle-detail-section__actions")).not.toBeNull();
  });

  it("applies the accessible label to the section", () => {
    const { container } = render(
      <DetailSection ariaLabel="Account settings">row</DetailSection>,
    );
    const root = container.querySelector(".poodle-detail-section") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Account settings");
  });

  it("forwards the item min column width as a custom property", () => {
    const { container } = render(
      <DetailSection itemMinColumnWidth="16rem">row</DetailSection>,
    );
    const root = container.querySelector(".poodle-detail-section") as HTMLElement;
    expect(root.style.getPropertyValue("--poodle-detail-section-item-min")).toBe("16rem");
  });
});
