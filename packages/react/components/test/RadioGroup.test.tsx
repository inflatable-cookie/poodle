import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { RadioGroup } from "../src/RadioGroup";
import type { RadioGroupOption } from "../src/types";

const planOptions: RadioGroupOption[] = [
  { value: "free", label: "Free" },
  { value: "pro", label: "Pro" },
  { value: "enterprise", label: "Enterprise" },
];

describe("RadioGroup (react)", () => {
  it("reflects the controlled value and reports a change to another option", () => {
    const onValueChange = vi.fn();
    const { container, rerender } = render(
      <RadioGroup value="pro" options={planOptions} ariaLabel="Plan" onValueChange={onValueChange} />,
    );
    const inputs = () => [...container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];

    expect(inputs()[1].checked).toBe(true);
    expect(inputs()[0].checked).toBe(false);

    fireEvent.click(inputs()[0]);
    expect(onValueChange).toHaveBeenCalledWith("free");

    rerender(<RadioGroup value="free" options={planOptions} ariaLabel="Plan" onValueChange={onValueChange} />);
    expect(inputs()[0].checked).toBe(true);
    expect(inputs()[1].checked).toBe(false);
  });

  it("does not re-emit when reselecting the current value", () => {
    const onValueChange = vi.fn();
    const { container } = render(<RadioGroup value="pro" options={planOptions} onValueChange={onValueChange} />);
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];

    fireEvent.click(inputs[1]);
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("suppresses selection when the group or an option is disabled", () => {
    const onValueChange = vi.fn();
    const group = render(
      <RadioGroup disabled value="free" options={planOptions} onValueChange={onValueChange} />,
    );
    const groupInputs = [...group.container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];
    expect(groupInputs.every((el) => el.disabled)).toBe(true);
    fireEvent.click(groupInputs[1]);
    expect(onValueChange).not.toHaveBeenCalled();

    const withDisabledOption = [...planOptions, { value: "vip", label: "VIP", disabled: true }] satisfies RadioGroupOption[];
    const partial = render(
      <RadioGroup value="free" options={withDisabledOption} onValueChange={onValueChange} />,
    );
    const partialInputs = [...partial.container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];
    expect(partialInputs[3].disabled).toBe(true);
    fireEvent.click(partialInputs[3]);
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("seeds uncontrolled mode from defaultValue and owns state thereafter", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <RadioGroup defaultValue="pro" options={planOptions} onValueChange={onValueChange} />,
    );
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];

    expect(inputs[1].checked).toBe(true);
    fireEvent.click(inputs[2]);
    expect(onValueChange).toHaveBeenCalledWith("enterprise");
    expect(inputs[2].checked).toBe(true);
    expect(inputs[1].checked).toBe(false);
  });

  it("shares an auto-generated name across options and honours an explicit one", () => {
    const generated = render(<RadioGroup options={planOptions} />);
    const generatedInputs = [
      ...generated.container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control"),
    ];
    const generatedName = generatedInputs[0].getAttribute("name");
    expect(generatedName).toBeTruthy();
    expect(generatedInputs.every((el) => el.getAttribute("name") === generatedName)).toBe(true);

    const explicit = render(<RadioGroup options={planOptions} name="plan" />);
    const explicitInputs = [...explicit.container.querySelectorAll<HTMLInputElement>(".poodle-radio-group__control")];
    expect(explicitInputs.every((el) => el.getAttribute("name") === "plan")).toBe(true);
  });

  it("exposes radiogroup semantics with the label and orientation", () => {
    const { container } = render(
      <RadioGroup options={planOptions} ariaLabel="Plan" orientation="horizontal" />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-radio-group");
    expect(root?.getAttribute("role")).toBe("radiogroup");
    expect(root?.getAttribute("aria-label")).toBe("Plan");
    expect(root?.getAttribute("data-orientation")).toBe("horizontal");
  });
});
