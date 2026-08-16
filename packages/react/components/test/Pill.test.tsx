import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Pill } from "../src/Pill";

describe("Pill (react)", () => {
  it("projects tone, appearance, size, and density data attributes", () => {
    const { container } = render(<Pill tone="info" appearance="subtle" size="lg">Beta</Pill>);
    const root = container.querySelector(".poodle-pill") as HTMLElement;
    expect(root.dataset.tone).toBe("info");
    expect(root.dataset.appearance).toBe("subtle");
    expect(root.dataset.size).toBe("lg");
  });

  it("carries the accent token and marks it custom", () => {
    const { container } = render(<Pill accent="#ff9900">Beta</Pill>);
    const root = container.querySelector(".poodle-pill") as HTMLElement;
    expect(root.dataset.accent).toBe("custom");
    expect(root.style.getPropertyValue("--poodle-pill-accent")).toBe("#ff9900");
  });

  it("projects muted, adaptive-width, and dot anatomy", () => {
    const { container } = render(
      <Pill muted adaptiveWidth dot>
        Beta
      </Pill>,
    );
    const root = container.querySelector(".poodle-pill") as HTMLElement;
    expect(root.dataset.muted).toBe("true");
    expect(root.dataset.adaptiveWidth).toBe("true");
    expect(container.querySelector(".poodle-pill__dot")?.getAttribute("aria-hidden")).toBe(
      "true",
    );
  });
});
