import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Eyebrow from "../src/Eyebrow.svelte";
import { asSnippet } from "./snippet";

describe("Eyebrow (svelte)", () => {
  it("renders the requested semantic element", () => {
    const { container } = render(Eyebrow, { props: { as: "h2", children: asSnippet(() => "Report") } });
    expect(container.querySelector(".poodle-eyebrow")?.tagName).toBe("H2");
    const span = render(Eyebrow, { props: { children: asSnippet(() => "Plain") } });
    expect(span.container.querySelector(".poodle-eyebrow")?.tagName).toBe("SPAN");
  });

  it("projects size and spacing data attributes", () => {
    const { container } = render(Eyebrow, {
      props: { as: "h3", size: "md", spacing: "bottom", children: asSnippet(() => "Meta") },
    });
    const root = container.querySelector(".poodle-eyebrow") as HTMLElement;
    expect(root.dataset.size).toBe("md");
    expect(root.dataset.spacing).toBe("bottom");
  });

  it("passes an explicit accessible label", () => {
    const { container } = render(Eyebrow, {
      props: { children: asSnippet(() => "Abbr"), ariaLabel: "Abbreviated label" },
    });
    expect(container.querySelector(".poodle-eyebrow")?.getAttribute("aria-label")).toBe(
      "Abbreviated label",
    );
  });
});
