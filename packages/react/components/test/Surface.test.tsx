import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Surface } from "../src/Surface";

describe("Surface (react)", () => {
  it("projects tone, border, and elevation data attributes", () => {
    const { container } = render(
      <Surface tone="elevated" border="default" elevated>
        Content
      </Surface>,
    );
    const root = container.querySelector(".poodle-surface") as HTMLElement;
    expect(root.dataset.tone).toBe("elevated");
    expect(root.dataset.border).toBe("default");
    expect(root.dataset.elevated).toBe("true");
    expect(root.textContent).toBe("Content");
  });

  it("applies the panel defaults when nothing is specified", () => {
    const { container } = render(<Surface>Content</Surface>);
    const root = container.querySelector(".poodle-surface") as HTMLElement;
    expect(root.dataset.tone).toBe("panel");
    expect(root.dataset.border).toBe("subtle");
    expect(root.dataset.elevated).toBe("false");
  });

  it("stays accessibility-neutral without asRole and opts into region semantics with a label", () => {
    const decorative = render(<Surface>Content</Surface>);
    const neutralRoot = decorative.container.querySelector(".poodle-surface") as HTMLElement;
    expect(neutralRoot.getAttribute("role")).toBeNull();
    expect(neutralRoot.getAttribute("aria-label")).toBeNull();

    const region = render(
      <Surface asRole="region" label="Inspector">
        Content
      </Surface>,
    );
    const regionRoot = region.container.querySelector(".poodle-surface") as HTMLElement;
    expect(regionRoot.getAttribute("role")).toBe("region");
    expect(regionRoot.getAttribute("aria-label")).toBe("Inspector");
  });

  it("maps padding to the surface-local scale", () => {
    const md = render(<Surface padding="md">Content</Surface>);
    expect((md.container.querySelector(".poodle-surface") as HTMLElement).style.padding).toBe(
      "1rem",
    );

    const none = render(<Surface padding="none">Content</Surface>);
    expect((none.container.querySelector(".poodle-surface") as HTMLElement).style.padding).toBe("0px");
  });
});