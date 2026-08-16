import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import StatusIndicator from "../src/StatusIndicator.svelte";
import StatusIndicatorSlotHarness from "./StatusIndicatorSlotHarness.svelte";

describe("StatusIndicator (svelte)", () => {
  it("projects the status tone and renders the dot hidden from assistive tech", () => {
    const { container } = render(StatusIndicator, { props: { status: "danger" } });
    const root = container.querySelector(".poodle-status-indicator") as HTMLElement;
    expect(root.dataset.status).toBe("danger");
    const dot = root.querySelector(".poodle-status-indicator__dot") as HTMLElement;
    expect(dot.getAttribute("aria-hidden")).toBe("true");
  });

  it("shows the label prop and supports slot content as the visible label", () => {
    const viaLabel = render(StatusIndicator, { props: { status: "success", label: "Build passing" } });
    expect(viaLabel.container.querySelector(".poodle-status-indicator__label")?.textContent).toBe(
      "Build passing",
    );

    const viaSlot = render(StatusIndicatorSlotHarness);
    expect(viaSlot.container.querySelector(".poodle-status-indicator__label")).toBeNull();
    expect(viaSlot.container.querySelector(".poodle-status-indicator")?.textContent).toContain(
      "Build passing",
    );
  });

  it("exposes the ariaLabel on the root and renders a dot-only state when no label exists", () => {
    const { container } = render(StatusIndicator, {
      props: { status: "warning", ariaLabel: "Away" },
    });
    const root = container.querySelector(".poodle-status-indicator") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Away");
    expect(root.querySelector(".poodle-status-indicator__label")).toBeNull();
    expect(root.textContent?.trim()).toBe("");
  });

  it("reflects size, density, and typography data attributes", () => {
    const { container } = render(StatusIndicator, {
      props: { status: "pending", size: "lg", density: "compact", typography: "inherit" },
    });
    const root = container.querySelector(".poodle-status-indicator") as HTMLElement;
    expect(root.dataset.size).toBe("lg");
    expect(root.dataset.density).toBe("compact");
    expect(root.dataset.typography).toBe("inherit");
  });
});