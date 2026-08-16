import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import RadioGroup from "../src/RadioGroup.svelte";
import type { RadioGroupOption } from "../src/types";

const planOptions: RadioGroupOption[] = [
  { value: "free", label: "Free" },
  { value: "pro", label: "Pro" },
  { value: "enterprise", label: "Enterprise" },
];

describe("RadioGroup (svelte)", () => {
  it("reflects the controlled value and reports a change to another option", async () => {
    const onValueChange = vi.fn();
    const { container } = render(RadioGroup, {
      props: { value: "pro", options: planOptions, ariaLabel: "Plan", onValueChange },
    });
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];

    expect(inputs[1].checked).toBe(true);
    expect(inputs[0].checked).toBe(false);

    await fireEvent.click(inputs[0]);
    expect(onValueChange).toHaveBeenCalledWith("free");
    expect(inputs[0].checked).toBe(true);
    expect(inputs[1].checked).toBe(false);
  });

  it("does not re-emit when reselecting the current value", async () => {
    const onValueChange = vi.fn();
    const { container } = render(RadioGroup, {
      props: { value: "pro", options: planOptions, onValueChange },
    });
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];

    await fireEvent.click(inputs[1]);
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("suppresses selection when the group or an option is disabled", async () => {
    const onValueChange = vi.fn();
    const group = render(RadioGroup, {
      props: { disabled: true, value: "free", options: planOptions, onValueChange },
    });
    const groupInputs = [...group.container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];
    expect(groupInputs.every((el) => el.disabled)).toBe(true);
    await fireEvent.click(groupInputs[1]);
    expect(onValueChange).not.toHaveBeenCalled();

    const withDisabledOption = [
      ...planOptions,
      { value: "vip", label: "VIP", disabled: true },
    ] satisfies RadioGroupOption[];
    const partial = render(RadioGroup, {
      props: { value: "free", options: withDisabledOption, onValueChange },
    });
    const partialInputs = [...partial.container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];
    expect(partialInputs[3].disabled).toBe(true);
    await fireEvent.click(partialInputs[3]);
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("seeds uncontrolled mode from defaultValue and owns state thereafter", async () => {
    const onValueChange = vi.fn();
    const { container } = render(RadioGroup, {
      props: { defaultValue: "pro", options: planOptions, onValueChange },
    });
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];

    expect(inputs[1].checked).toBe(true);
    await fireEvent.click(inputs[2]);
    expect(onValueChange).toHaveBeenCalledWith("enterprise");
    expect(inputs[2].checked).toBe(true);
    expect(inputs[1].checked).toBe(false);
  });

  it("shares an auto-generated name across options and honours an explicit one", () => {
    const generated = render(RadioGroup, { props: { options: planOptions } });
    const generatedInputs = [
      ...generated.container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control"),
    ];
    const generatedName = generatedInputs[0].getAttribute("name");
    expect(generatedName).toMatch(/^poodle-radio-group-/);
    expect(generatedInputs.every((el) => el.getAttribute("name") === generatedName)).toBe(true);

    const explicit = render(RadioGroup, { props: { options: planOptions, name: "plan" } });
    const explicitInputs = [...explicit.container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];
    expect(explicitInputs.every((el) => el.getAttribute("name") === "plan")).toBe(true);
  });

  it("exposes radiogroup semantics with the label and orientation", () => {
    const { container } = render(RadioGroup, {
      props: { options: planOptions, ariaLabel: "Plan", orientation: "horizontal" },
    });
    const root = container.querySelector<HTMLElement>(".poodle-radio-group");
    expect(root?.getAttribute("role")).toBe("radiogroup");
    expect(root?.getAttribute("aria-label")).toBe("Plan");
    expect(root?.getAttribute("data-orientation")).toBe("horizontal");
  });
});
