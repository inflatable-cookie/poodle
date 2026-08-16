import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Meter from "../src/Meter.svelte";

describe("Meter (svelte)", () => {
  it("sizes the fill by percentage and feeds the native meter", () => {
    const { container } = render(Meter, { props: { value: 50, ariaLabel: "Storage usage" } });
    const root = container.querySelector<HTMLElement>(".poodle-meter");
    const fill = container.querySelector<HTMLElement>(".poodle-meter__fill");
    expect(fill?.getAttribute("style")).toContain("width: 50%");
    expect(root?.getAttribute("aria-label")).toBe("Storage usage");

    const native = container.querySelector<HTMLMeterElement>(".poodle-meter__native");
    expect(native?.getAttribute("value")).toBe("50");
    expect(native?.getAttribute("min")).toBe("0");
    expect(native?.getAttribute("max")).toBe("100");
  });

  it("clamps the value into the range", () => {
    const over = render(Meter, { props: { value: 150, max: 100 } });
    expect(over.container.querySelector<HTMLElement>(".poodle-meter__fill")?.getAttribute("style")).toContain(
      "width: 100%",
    );
    expect(over.container.querySelector<HTMLMeterElement>(".poodle-meter__native")?.getAttribute("value")).toBe("100");

    const under = render(Meter, { props: { value: -5 } });
    expect(under.container.querySelector<HTMLElement>(".poodle-meter__fill")?.getAttribute("style")).toContain(
      "width: 0%",
    );
  });

  it("maps a custom range to percentage", () => {
    const { container } = render(Meter, { props: { value: 350, min: 0, max: 500 } });
    expect(container.querySelector<HTMLElement>(".poodle-meter__fill")?.getAttribute("style")).toContain("width: 70%");
  });

  it("flags high and low levels with high winning", () => {
    const above = render(Meter, { props: { value: 82, low: 25, high: 75 } });
    expect(above.container.querySelector<HTMLElement>(".poodle-meter")?.getAttribute("data-level")).toBe("high");

    const below = render(Meter, { props: { value: 10, low: 25, high: 75 } });
    expect(below.container.querySelector<HTMLElement>(".poodle-meter")?.getAttribute("data-level")).toBe("low");

    const normal = render(Meter, { props: { value: 50, low: 25, high: 75 } });
    expect(normal.container.querySelector<HTMLElement>(".poodle-meter")?.getAttribute("data-level")).toBe("normal");

    const both = render(Meter, { props: { value: 80, low: 100, high: 75 } });
    expect(both.container.querySelector<HTMLElement>(".poodle-meter")?.getAttribute("data-level")).toBe("high");
  });

  it("shows the computed readout or an explicit valueText", () => {
    const computed = render(Meter, { props: { value: 50, showValue: true } });
    expect(computed.container.querySelector(".poodle-meter__value")?.textContent).toBe("50%");

    const explicit = render(Meter, { props: { value: 350, min: 0, max: 500, showValue: true, valueText: "150 / 500 used" } });
    expect(explicit.container.querySelector(".poodle-meter__value")?.textContent).toBe("150 / 500 used");

    const hidden = render(Meter, { props: { value: 50 } });
    expect(hidden.container.querySelector(".poodle-meter__value")).toBeNull();
  });

  it("drives the ring shape through data attributes and the arc variable instead of width", () => {
    const { container } = render(Meter, { props: { value: 38, shape: "ring" } });
    const root = container.querySelector<HTMLElement>(".poodle-meter");
    expect(root?.getAttribute("data-shape")).toBe("ring");
    expect(root?.getAttribute("data-tone")).toBe("success");
    expect(root?.getAttribute("style")).toContain("--poodle-meter-percentage: 38");
    expect(container.querySelector<HTMLElement>(".poodle-meter__fill")?.getAttribute("style")).toBeNull();
  });
});
