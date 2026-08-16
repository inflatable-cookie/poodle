import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import CardRadioGroup from "../src/CardRadioGroup.svelte";

const items = [
  { value: "free", label: "Free" },
  { value: "pro", label: "Pro" },
  { value: "enterprise", label: "Enterprise", disabled: true },
];

describe("CardRadioGroup (svelte)", () => {
  it("renders a labelled radiogroup with radio options", () => {
    const { container } = render(CardRadioGroup, {
      props: { items, value: "pro", ariaLabel: "Select a plan", columns: 2 },
    });
    const root = container.querySelector(".poodle-card-radio-group") as HTMLElement;
    expect(root.getAttribute("role")).toBe("radiogroup");
    expect(root.getAttribute("aria-label")).toBe("Select a plan");
    expect(root.style.getPropertyValue("--columns")).toBe("2");

    const options = [...container.querySelectorAll('[role="radio"]')];
    expect(options.length).toBe(3);
    expect(options[1].getAttribute("aria-checked")).toBe("true");
    expect(options[2].getAttribute("aria-disabled")).toBe("true");
  });

  it("shows the indicator dot on the checked option only", () => {
    const { container } = render(CardRadioGroup, { props: { items, value: "pro" } });
    const indicators = [
      ...container.querySelectorAll(".poodle-card-radio-group__indicator"),
    ] as HTMLElement[];
    expect(indicators[0].dataset.checked).toBe("false");
    expect(indicators[1].dataset.checked).toBe("true");
    expect(indicators[1].querySelector(".poodle-card-radio-group__dot")).not.toBeNull();
    expect(indicators[0].querySelector(".poodle-card-radio-group__dot")).toBeNull();
  });

  it("reports the selected value on activation and never activates disabled items", async () => {
    const onValueChange = vi.fn();
    const { container } = render(CardRadioGroup, { props: { items, onValueChange } });
    const options = [...container.querySelectorAll('[role="radio"]')];

    await fireEvent.click(options[1]);
    expect(onValueChange).toHaveBeenCalledWith("pro");

    await fireEvent.click(options[2]);
    expect(onValueChange).toHaveBeenCalledTimes(1);
  });

  it("maintains a roving tabindex across enabled options", () => {
    const { container } = render(CardRadioGroup, { props: { items, value: "pro" } });
    const options = [...container.querySelectorAll('[role="radio"]')];
    expect(options[1].getAttribute("tabindex")).toBe("0");
    expect(options[0].getAttribute("tabindex")).toBe("-1");
    expect(options[2].getAttribute("tabindex")).toBe("-1");
  });

  it("moves selection with arrow keys, skipping disabled options", async () => {
    const onValueChange = vi.fn();
    const { container } = render(CardRadioGroup, {
      props: { items, value: "free", onValueChange },
    });
    const options = [...container.querySelectorAll('[role="radio"]')];
    await fireEvent.keyDown(options[0], { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenCalledWith("pro");
  });

  it("disables every option when the group is disabled", () => {
    const onValueChange = vi.fn();
    const { container } = render(CardRadioGroup, {
      props: { items, disabled: true, onValueChange },
    });
    const options = [...container.querySelectorAll('[role="radio"]')];
    expect(options.every((option) => option.getAttribute("aria-disabled") === "true")).toBe(true);
  });
});