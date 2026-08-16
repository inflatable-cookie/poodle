import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import MetricTile from "../src/MetricTile.svelte";

describe("MetricTile (svelte)", () => {
  it("computes the accessible name from label and value", () => {
    const { container } = render(MetricTile, { props: { label: "Coverage", value: "94%" } });
    const root = container.querySelector(".poodle-state-tile") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Coverage: 94%");
    expect(container.querySelector(".poodle-state-tile__label")?.textContent).toBe("Coverage");
    expect(container.querySelector(".poodle-state-tile__value")?.textContent).toBe("94%");
  });

  it("honours an explicit ariaLabel override", () => {
    const { container } = render(MetricTile, {
      props: { label: "Coverage", value: "94%", ariaLabel: "Coverage percent" },
    });
    const root = container.querySelector(".poodle-state-tile") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Coverage percent");
  });

  it("projects the trend direction with its label and hides the arrow from assistive tech", () => {
    const { container } = render(MetricTile, {
      props: { label: "Active users", value: "2,847", trend: "up", trendLabel: "+12.3%" },
    });
    const trend = container.querySelector(".poodle-state-tile__trend") as HTMLElement;
    expect(trend.dataset.trend).toBe("up");
    expect(trend.textContent).toContain("+12.3%");
    const arrow = container.querySelector(".poodle-state-tile__trend-arrow") as HTMLElement;
    expect(arrow.getAttribute("aria-hidden")).toBe("true");
    expect(arrow.querySelector(".poodle-icon")).not.toBeNull();
  });

  it("renders no trend row without a trend", () => {
    const { container } = render(MetricTile, { props: { label: "CPU", value: "62%" } });
    expect(container.querySelector(".poodle-state-tile__trend")).toBeNull();
  });

  it("renders a decorative sparkline from 2+ data points", () => {
    const { container } = render(MetricTile, {
      props: { label: "Requests", value: "1,204", sparklineData: [800, 920, 850, 1100] },
    });
    const svg = container.querySelector(".poodle-state-tile__sparkline") as SVGElement;
    expect(svg).not.toBeNull();
    expect(svg.getAttribute("aria-hidden")).toBe("true");
    const path = svg.querySelector("path") as SVGPathElement;
    expect(path.getAttribute("d")).toMatch(/^M/);
  });

  it("omits the sparkline with fewer than two data points", () => {
    const { container } = render(MetricTile, {
      props: { label: "Requests", value: "10", sparklineData: [10] },
    });
    expect(container.querySelector(".poodle-state-tile__sparkline")).toBeNull();
  });
});
