import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { MetaBar } from "../src/MetaBar";
import { MetaItem } from "../src/MetaItem";
import { Pill } from "../src/Pill";

describe("MetaBar (react)", () => {
  it("projects separator and label attributes", () => {
    const { container } = render(<MetaBar ariaLabel="Article meta">x</MetaBar>);
    const root = container.querySelector(".poodle-meta-bar") as HTMLElement;
    expect(root.dataset.separators).toBe("true");
    expect(root.getAttribute("aria-label")).toBe("Article meta");
  });

  it("inherits the pill context size and typography onto composed pills", () => {
    const { container } = render(
      <MetaBar>
        <Pill tone="info">Beta</Pill>
      </MetaBar>,
    );
    const pill = container.querySelector(".poodle-pill") as HTMLElement;
    expect(pill).not.toBeNull();
    expect(pill.dataset.size).toBe("md");
    expect(pill.dataset.typography).toBe("inherit");
  });
});

describe("MetaItem (react)", () => {
  it("renders label and value with the separator and typography data", () => {
    const { container } = render(
      <MetaItem label="Updated" typography="inherit" separator={false}>
        2 days ago
      </MetaItem>,
    );
    const root = container.querySelector(".poodle-meta-item") as HTMLElement;
    expect(container.querySelector(".poodle-meta-item__label")?.textContent).toBe("Updated");
    expect(container.querySelector(".poodle-meta-item__value")?.textContent).toContain("2 days ago");
    expect(root.dataset.typography).toBe("inherit");
    expect(root.dataset.separator).toBe("false");
  });

  it("omits the label element when no label is given", () => {
    const { container } = render(<MetaItem>value</MetaItem>);
    expect(container.querySelector(".poodle-meta-item__label")).toBeNull();
  });
});
