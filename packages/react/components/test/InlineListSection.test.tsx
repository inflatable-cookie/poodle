import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { InlineListSection } from "../src/InlineListSection";

const versions = [
  { id: "v1", label: "abc123" },
  { id: "v2", label: "def456" },
];

describe("InlineListSection (react)", () => {
  it("renders the titled section with items via the item renderer", () => {
    const { container } = render(
      <InlineListSection
        title="Versions"
        items={versions}
        item={(entry) => <span>{entry.label}</span>}
      />,
    );
    const section = container.querySelector(".poodle-inline-list-section") as HTMLElement;
    expect(section.getAttribute("aria-label")).toBe("Versions");
    const heading = section.querySelector("h4") as HTMLElement;
    expect(heading.textContent).toBe("Versions");
    expect(section.querySelectorAll(".poodle-inline-list-section__item").length).toBe(2);
    expect(section.textContent).toContain("abc123");
  });

  it("wraps the section in a card when framed and drops it when not", () => {
    const framed = render(
      <InlineListSection title="Versions" items={versions} item={() => <span>x</span>} />,
    );
    expect(framed.container.querySelector(".poodle-inline-list-section")).not.toBeNull();

    const bare = render(
      <InlineListSection title="Versions" items={versions} framed={false} item={() => <span>x</span>} />,
    );
    expect(bare.container.querySelector(".poodle-inline-list-section")).not.toBeNull();
    expect(bare.container.querySelectorAll(".poodle-inline-list-section").length).toBe(1);
  });

  it("shows the count badge when provided and omits it when null", () => {
    const withCount = render(
      <InlineListSection title="Versions" items={versions} count={2} item={() => <span>x</span>} />,
    );
    expect(withCount.container.querySelector(".poodle-inline-list-section__count")?.textContent).toBe(
      "2",
    );

    const withoutCount = render(
      <InlineListSection title="Versions" items={versions} item={() => <span>x</span>} />,
    );
    expect(withoutCount.container.querySelector(".poodle-inline-list-section__count")).toBeNull();
  });

  it("shows the empty message when there are no items", () => {
    const { container } = render(
      <InlineListSection
        title="Versions"
        items={[]}
        emptyMessage="No versions yet"
        item={() => <span>x</span>}
      />,
    );
    expect(container.querySelector(".poodle-inline-list-section__empty")?.textContent).toContain(
      "No versions yet",
    );
    expect(container.querySelector(".poodle-inline-list-section__items")).toBeNull();
  });

  it("renders the header actions region when provided", () => {
    const { container } = render(
      <InlineListSection
        title="Versions"
        items={versions}
        actions={<button>Upload</button>}
        item={() => <span>x</span>}
      />,
    );
    expect(container.querySelector(".poodle-inline-list-section__header-actions")).not.toBeNull();
  });
});