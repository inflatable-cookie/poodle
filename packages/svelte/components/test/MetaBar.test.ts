import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import MetaBar from "../src/MetaBar.svelte";
import MetaItem from "../src/MetaItem.svelte";
import { asSnippet } from "./snippet";
import MetaBarPillHarness from "./MetaBarPillHarness.svelte";

describe("MetaBar (svelte)", () => {
  it("projects separator and label attributes", () => {
    const { container } = render(MetaBar, { props: { ariaLabel: "Article meta" } });
    const root = container.querySelector(".poodle-meta-bar") as HTMLElement;
    expect(root.dataset.separators).toBe("true");
    expect(root.getAttribute("aria-label")).toBe("Article meta");
  });

  it("inherits the pill context size and typography onto composed pills", () => {
    const { container } = render(MetaBarPillHarness, {});
    const pill = container.querySelector(".poodle-pill") as HTMLElement;
    expect(pill).not.toBeNull();
    expect(pill.dataset.size).toBe("md");
    expect(pill.dataset.typography).toBe("inherit");
  });
});

describe("MetaItem (svelte)", () => {
  it("renders label and value with the separator and typography data", () => {
    const { container } = render(MetaItem, {
      props: {
        label: "Updated",
        typography: "inherit",
        separator: false,
        children: asSnippet(() => "2 days ago"),
      },
    });
    const root = container.querySelector(".poodle-meta-item") as HTMLElement;
    expect(container.querySelector(".poodle-meta-item__label")?.textContent).toBe("Updated");
    expect(root.dataset.typography).toBe("inherit");
    expect(root.dataset.separator).toBe("false");
  });

  it("omits the label element when no label is given", () => {
    const { container } = render(MetaItem, { props: { children: asSnippet(() => "value") } });
    expect(container.querySelector(".poodle-meta-item__label")).toBeNull();
  });
});
