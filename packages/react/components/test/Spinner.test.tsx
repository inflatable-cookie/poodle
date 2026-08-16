import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Spinner } from "../src/Spinner";

describe("Spinner (react)", () => {
  it("renders the ring anatomy by default and dots/grid on request", () => {
    const ring = render(<Spinner />);
    expect(ring.container.querySelector(".poodle-spinner__ring")).not.toBeNull();

    const dots = render(<Spinner variant="dots" />);
    expect(dots.container.querySelectorAll(".poodle-spinner__dot").length).toBe(3);

    const grid = render(<Spinner variant="grid" />);
    expect(grid.container.querySelectorAll(".poodle-spinner__cell").length).toBe(6);
  });

  it("stays hidden from assistive tech without a label and becomes a live status with one", () => {
    const silent = render(<Spinner />);
    const silentRoot = silent.container.querySelector(".poodle-spinner") as HTMLElement;
    expect(silentRoot.getAttribute("aria-hidden")).toBe("true");
    expect(silentRoot.getAttribute("role")).toBeNull();

    const announced = render(<Spinner ariaLabel="Loading results" />);
    const announcedRoot = announced.container.querySelector(".poodle-spinner") as HTMLElement;
    expect(announcedRoot.getAttribute("role")).toBe("status");
    expect(announcedRoot.getAttribute("aria-live")).toBe("polite");
    expect(announcedRoot.getAttribute("aria-label")).toBe("Loading results");
    expect(announcedRoot.getAttribute("aria-hidden")).toBeNull();
  });

  it("projects variant, tone, and size data attributes", () => {
    const { container } = render(<Spinner variant="dots" tone="accent" size="lg" />);
    const root = container.querySelector(".poodle-spinner") as HTMLElement;
    expect(root.dataset.variant).toBe("dots");
    expect(root.dataset.tone).toBe("accent");
    expect(root.dataset.size).toBe("lg");
  });
});
