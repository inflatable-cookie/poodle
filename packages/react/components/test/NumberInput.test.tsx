import { fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { NumberInput } from "../src/NumberInput";

describe("NumberInput (react)", () => {
  it("commits parsed numbers as the user types and null on empty input", () => {
    const onValueChange = vi.fn();
    const { container } = render(<NumberInput value={5} onValueChange={onValueChange} />);
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    fireEvent.change(control, { target: { value: "7" } });
    expect(onValueChange).toHaveBeenLastCalledWith(7);

    fireEvent.change(control, { target: { value: "" } });
    expect(onValueChange).toHaveBeenLastCalledWith(null);
  });

  it("clamps to max and snaps to the step on blur", () => {
    const onValueChange = vi.fn();
    const over = render(<NumberInput value={5} min={0} max={10} step={2} onValueChange={onValueChange} />);
    const overControl = over.container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    fireEvent.change(overControl, { target: { value: "14" } });
    fireEvent.blur(overControl);
    expect(onValueChange).toHaveBeenLastCalledWith(10);

    const snap = render(<NumberInput value={5} min={0} max={10} step={2} onValueChange={onValueChange} />);
    const snapControl = snap.container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    fireEvent.change(snapControl, { target: { value: "3" } });
    fireEvent.blur(snapControl);
    expect(onValueChange).toHaveBeenLastCalledWith(4);
  });

  it("steps with arrow keys, reports increment/decrement, and honours readOnly", () => {
    const onValueChange = vi.fn();
    const onIncrement = vi.fn();
    const onDecrement = vi.fn();
    const { container } = render(
      <NumberInput defaultValue={5} min={0} max={10} step={1} onValueChange={onValueChange} onIncrement={onIncrement} onDecrement={onDecrement} />,
    );
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    fireEvent.keyDown(control, { key: "ArrowUp" });
    expect(onValueChange).toHaveBeenLastCalledWith(6);
    expect(onIncrement).toHaveBeenCalledWith(6);

    fireEvent.keyDown(control, { key: "ArrowDown" });
    expect(onValueChange).toHaveBeenLastCalledWith(5);
    expect(onDecrement).toHaveBeenCalledWith(5);

    const readOnly = render(<NumberInput defaultValue={5} readOnly onValueChange={onValueChange} onIncrement={onIncrement} />);
    const readOnlyControl = readOnly.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    fireEvent.keyDown(readOnlyControl, { key: "ArrowUp" });
    expect(onIncrement).toHaveBeenCalledTimes(1);
  });

  it("round-trips string values and renders prefix and suffix", () => {
    const onValueChange = vi.fn();
    const { container } = render(<NumberInput value="2026" prefix="FY" suffix="kg" onValueChange={onValueChange} />);
    expect(container.querySelector(".poodle-number-input__prefix")?.textContent).toBe("FY");
    expect(container.querySelector(".poodle-number-input__suffix")?.textContent).toBe("kg");

    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    fireEvent.change(control, { target: { value: "7" } });
    expect(onValueChange).toHaveBeenCalledWith("7");
  });

  it("submits the current value on Enter", () => {
    const onSubmit = vi.fn();
    const { container } = render(<NumberInput value={5} onSubmit={onSubmit} />);
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    fireEvent.keyDown(control, { key: "Enter" });
    expect(onSubmit).toHaveBeenCalledWith(5);
  });

  it("maps validation state to aria attributes and reports async validation results", async () => {
    const invalid = render(<NumberInput value={5} validationState="invalid" />);
    const invalidControl = invalid.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    expect(invalidControl.getAttribute("aria-invalid")).toBe("true");

    const pending = render(<NumberInput value={5} validationState="pending" />);
    const pendingControl = pending.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    expect(pendingControl.getAttribute("aria-busy")).toBe("true");

    const validate = vi.fn().mockResolvedValue({ valid: false, message: "Out of range" });
    const onValidationChange = vi.fn();
    const asyncCase = render(
      <NumberInput value={5} min={0} max={10} validate={validate} onValidationChange={onValidationChange} />,
    );
    const asyncControl = asyncCase.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    fireEvent.change(asyncControl, { target: { value: "3" } });

    await waitFor(() => {
      expect(onValidationChange).toHaveBeenCalledWith(
        expect.objectContaining({ status: "invalid", valid: false, message: "Out of range" }),
      );
    });
    const field = asyncCase.container.querySelector<HTMLElement>(".poodle-number-input__field");
    expect(field?.getAttribute("data-validation-state")).toBe("invalid");
  });
});
