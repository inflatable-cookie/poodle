import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Progress from "../src/Progress.svelte";

describe("Progress (svelte)", () => {
  it("projects value bounds and a computed value text on the progressbar", () => {
    const { container } = render(Progress, { props: { value: 50, max: 100, ariaLabel: "Upload" } });
    const root = container.querySelector(".poodle-progress") as HTMLElement;
    expect(root.getAttribute("role")).toBe("progressbar");
    expect(root.getAttribute("aria-valuemin")).toBe("0");
    expect(root.getAttribute("aria-valuemax")).toBe("100");
    expect(root.getAttribute("aria-valuenow")).toBe("50");
    expect(root.getAttribute("aria-valuetext")).toBe("50%");
    expect(root.getAttribute("aria-label")).toBe("Upload");
  });

  it("clamps out-of-range values and guards a non-positive max", () => {
    const { container } = render(Progress, { props: { value: 150 } });
    const root = container.querySelector(".poodle-progress") as HTMLElement;
    expect(root.getAttribute("aria-valuenow")).toBe("100");

    const badMax = render(Progress, { props: { value: 10, max: 0 } });
    expect(badMax.container.querySelector(".poodle-progress")?.getAttribute("aria-valuemax")).toBe(
      "100",
    );
  });

  it("drops progressbar value attributes in indeterminate mode", () => {
    const { container } = render(Progress, { props: { indeterminate: true, valueText: "Working" } });
    const root = container.querySelector(".poodle-progress") as HTMLElement;
    expect(root.dataset.indeterminate).toBe("true");
    expect(root.getAttribute("aria-valuenow")).toBeNull();
    expect(root.getAttribute("aria-valuemin")).toBeNull();
    expect(root.getAttribute("aria-valuetext")).toBe("Working");
  });
});
