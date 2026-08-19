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

  it("defaults to the tint appearance and emits no fill axis", () => {
    const { container } = render(<Pill>Neutral</Pill>);
    const root = container.querySelector(".poodle-pill") as HTMLElement;
    expect(root.dataset.appearance).toBe("tint");
    expect(root.hasAttribute("data-fill")).toBe(false);

    const explicitTint = render(<Pill appearance="tint">Neutral</Pill>).container.querySelector(
      ".poodle-pill",
    ) as HTMLElement;
    expect(explicitTint.dataset.appearance).toBe(root.dataset.appearance);
    expect(explicitTint.outerHTML).toBe(root.outerHTML);
  });

  it("projects all four appearances as one mutually exclusive axis", () => {
    for (const appearance of ["tint", "solid", "subtle", "badge"] as const) {
      const { container } = render(
        <Pill tone="warning" appearance={appearance} dot title="Warning">
          Warning
        </Pill>,
      );
      const root = container.querySelector(".poodle-pill") as HTMLElement;
      expect(root.dataset.appearance).toBe(appearance);
      expect(root.hasAttribute("data-fill")).toBe(false);
      expect(root.title).toBe("Warning");
      expect(container.querySelector(".poodle-pill__dot")).not.toBeNull();
    }
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
