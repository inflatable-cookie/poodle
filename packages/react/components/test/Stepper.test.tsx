import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Stepper } from "../src/Stepper";
import type { StepperStep } from "../src/types";

const steps: StepperStep[] = [
  { value: "plan", label: "Plan", status: "pending" },
  { value: "build", label: "Build", status: "running" },
  { value: "ship", label: "Ship", status: "pending" },
  { value: "review", label: "Review", status: "complete" },
];

describe("Stepper (react)", () => {
  it("falls back to the first step as current and exposes status in accessible names", () => {
    const { container } = render(<Stepper steps={steps} />);
    const current = container.querySelector('[aria-current="step"]') as HTMLElement;
    expect(current.textContent).toContain("Plan");

    const labels = [
      ...container.querySelectorAll<HTMLButtonElement>(".poodle-stepper__trigger"),
    ].map((el) => el.getAttribute("aria-label"));
    expect(labels[1]).toContain(", running");
    expect(labels[3]).toContain(", complete");
  });

  it("selects a step on click and reports the change", () => {
    const onValueChange = vi.fn();
    const { container } = render(<Stepper steps={steps} onValueChange={onValueChange} />);
    const triggers = [...container.querySelectorAll<HTMLButtonElement>(".poodle-stepper__trigger")];

    fireEvent.click(triggers[2]);

    expect(onValueChange).toHaveBeenCalledWith("ship");
    expect(triggers[2].getAttribute("aria-current")).toBe("step");
    expect(triggers[0].getAttribute("aria-current")).toBeNull();
  });

  it("skips disabled steps when selecting", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <Stepper
        steps={[
          ...steps.slice(0, 2),
          { value: "skip", label: "Skip", isDisabled: true, status: "pending" },
        ]}
        onValueChange={onValueChange}
      />,
    );
    const triggers = [...container.querySelectorAll<HTMLButtonElement>(".poodle-stepper__trigger")];

    fireEvent.click(triggers[2]);

    expect(triggers[2].disabled).toBe(true);
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("shows a rerun trigger for complete steps when onRerun is provided", () => {
    const onRerun = vi.fn();
    const { container } = render(<Stepper steps={steps} onRerun={onRerun} />);
    const rerun = container.querySelectorAll<HTMLButtonElement>(".poodle-stepper__rerun");
    expect(rerun.length).toBe(1);
    expect(rerun[0].getAttribute("aria-label")).toBe("Re-run step: Review");
  });

  it("keeps collapse vertical-only and exposes the collapse summary there", () => {
    const onCollapsedChange = vi.fn();
    const horizontal = render(<Stepper steps={steps} collapsible />);
    expect(horizontal.container.querySelector(".poodle-stepper__summary")).toBeNull();

    const vertical = render(
      <Stepper steps={steps} collapsible orientation="vertical" onCollapsedChange={onCollapsedChange} />,
    );
    const summary = vertical.container.querySelector(".poodle-stepper__summary") as HTMLButtonElement;
    expect(summary.getAttribute("aria-expanded")).toBe("true");
    expect(summary.getAttribute("aria-label")).toBe("Plan, 1 of 4 steps complete");

    fireEvent.click(summary);
    expect(onCollapsedChange).toHaveBeenCalledWith(true);
    expect(vertical.container.querySelector(".poodle-stepper__list")).toBeNull();
  });
});
