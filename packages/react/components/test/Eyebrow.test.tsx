import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Eyebrow } from "../src/Eyebrow";

describe("Eyebrow (react)", () => {
  it("renders the requested semantic element", () => {
    const { container } = render(<Eyebrow as="h2">Report</Eyebrow>);
    expect(container.querySelector(".poodle-eyebrow")?.tagName).toBe("H2");
    const span = render(<Eyebrow>Plain</Eyebrow>);
    expect(span.container.querySelector(".poodle-eyebrow")?.tagName).toBe("SPAN");
  });

  it("projects size and spacing data attributes", () => {
    const { container } = render(<Eyebrow as="h3" size="md" spacing="bottom">Meta</Eyebrow>);
    const root = container.querySelector(".poodle-eyebrow") as HTMLElement;
    expect(root.dataset.size).toBe("md");
    expect(root.dataset.spacing).toBe("bottom");
  });

  it("passes an explicit accessible label", () => {
    const { container } = render(<Eyebrow ariaLabel="Abbreviated label">Abbr</Eyebrow>);
    expect(container.querySelector(".poodle-eyebrow")?.getAttribute("aria-label")).toBe(
      "Abbreviated label",
    );
  });
});
