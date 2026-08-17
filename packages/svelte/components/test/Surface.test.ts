import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Surface from "../src/Surface.svelte";
import { asSnippet } from "./snippet";

describe("Surface (svelte)", () => {
  it("projects tone, border, and elevation data attributes", () => {
    const { container } = render(Surface, {
      props: { tone: "elevated", border: "default", elevated: true },
    });
    const root = container.querySelector(".poodle-surface") as HTMLElement;
    expect(root.dataset.tone).toBe("elevated");
    expect(root.dataset.border).toBe("default");
    expect(root.dataset.elevated).toBe("true");
  });

  it("applies the panel defaults when nothing is specified", () => {
    const { container } = render(Surface, { props: { children: asSnippet(() => "Content") } });
    const root = container.querySelector(".poodle-surface") as HTMLElement;
    expect(root.dataset.tone).toBe("panel");
    expect(root.dataset.border).toBe("subtle");
    expect(root.dataset.elevated).toBe("false");
  });

  it("stays accessibility-neutral without asRole and opts into region semantics with a label", () => {
    const decorative = render(Surface, { props: { children: asSnippet(() => "Content") } });
    const neutralRoot = decorative.container.querySelector(".poodle-surface") as HTMLElement;
    expect(neutralRoot.getAttribute("role")).toBeNull();
    expect(neutralRoot.getAttribute("aria-label")).toBeNull();

    const region = render(Surface, {
      props: { asRole: "region", label: "Inspector", children: asSnippet(() => "Content") },
    });
    const regionRoot = region.container.querySelector(".poodle-surface") as HTMLElement;
    expect(regionRoot.getAttribute("role")).toBe("region");
    expect(regionRoot.getAttribute("aria-label")).toBe("Inspector");
  });

  it("maps padding to the surface-local scale", () => {
    const md = render(Surface, { props: { padding: "md" } });
    expect((md.container.querySelector(".poodle-surface") as HTMLElement).style.padding).toBe(
      "1rem",
    );

    const none = render(Surface, { props: { padding: "none" } });
    expect((none.container.querySelector(".poodle-surface") as HTMLElement).style.padding).toBe(
      "0px",
    );
  });
});