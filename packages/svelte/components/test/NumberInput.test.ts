import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import NumberInput from "../src/NumberInput.svelte";

describe("NumberInput (svelte)", () => {
  it("commits parsed numbers as the user types and null on empty input", async () => {
    const onValueChange = vi.fn();
    const { container } = render(NumberInput, { props: { value: 5, onValueChange } });
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    await fireEvent.input(control, { target: { value: "7" } });
    expect(onValueChange).toHaveBeenLastCalledWith(7);

    await fireEvent.input(control, { target: { value: "" } });
    expect(onValueChange).toHaveBeenLastCalledWith(null);
  });

  it("clamps to max and snaps to the step on blur", async () => {
    const onValueChange = vi.fn();
    const over = render(NumberInput, {
      props: { value: 5, min: 0, max: 10, step: 2, onValueChange },
    });
    const overControl = over.container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    await fireEvent.input(overControl, { target: { value: "14" } });
    await fireEvent.blur(overControl);
    expect(onValueChange).toHaveBeenLastCalledWith(10);

    const snap = render(NumberInput, {
      props: { value: 5, min: 0, max: 10, step: 2, onValueChange },
    });
    const snapControl = snap.container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    await fireEvent.input(snapControl, { target: { value: "3" } });
    await fireEvent.blur(snapControl);
    expect(onValueChange).toHaveBeenLastCalledWith(4);
  });

  it("steps with arrow keys, reports increment/decrement, and honours readOnly", async () => {
    const onValueChange = vi.fn();
    const onIncrement = vi.fn();
    const onDecrement = vi.fn();
    const { container } = render(NumberInput, {
      props: { defaultValue: 5, min: 0, max: 10, step: 1, onValueChange, onIncrement, onDecrement },
    });
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    await fireEvent.keyDown(control, { key: "ArrowUp" });
    expect(onValueChange).toHaveBeenLastCalledWith(6);
    expect(onIncrement).toHaveBeenCalledWith(6);

    await fireEvent.keyDown(control, { key: "ArrowDown" });
    expect(onValueChange).toHaveBeenLastCalledWith(5);
    expect(onDecrement).toHaveBeenCalledWith(5);

    const readOnly = render(NumberInput, {
      props: { defaultValue: 5, readOnly: true, onValueChange, onIncrement },
    });
    const readOnlyControl = readOnly.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    await fireEvent.keyDown(readOnlyControl, { key: "ArrowUp" });
    expect(onIncrement).toHaveBeenCalledTimes(1);
  });

  it("round-trips string values and renders prefix and suffix", async () => {
    const onValueChange = vi.fn();
    const { container } = render(NumberInput, {
      props: { value: "2026", prefix: "FY", suffix: "kg", onValueChange },
    });
    expect(container.querySelector(".poodle-number-input__prefix")?.textContent).toBe("FY");
    expect(container.querySelector(".poodle-number-input__suffix")?.textContent).toBe("kg");

    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    await fireEvent.input(control, { target: { value: "7" } });
    expect(onValueChange).toHaveBeenCalledWith("7");
  });

  it("submits the current value on Enter", async () => {
    const onSubmit = vi.fn();
    const { container } = render(NumberInput, { props: { value: 5, onSubmit } });
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    await fireEvent.keyDown(control, { key: "Enter" });
    expect(onSubmit).toHaveBeenCalledWith(5);
  });

  it("maps validation state to aria attributes and reports async validation results", async () => {
    const invalid = render(NumberInput, { props: { value: 5, validationState: "invalid" } });
    const invalidControl = invalid.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    expect(invalidControl.getAttribute("aria-invalid")).toBe("true");

    const pending = render(NumberInput, { props: { value: 5, validationState: "pending" } });
    const pendingControl = pending.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    expect(pendingControl.getAttribute("aria-busy")).toBe("true");

    const validate = vi.fn().mockResolvedValue({ valid: false, message: "Out of range" });
    const onValidationChange = vi.fn();
    const asyncCase = render(NumberInput, {
      props: { value: 5, min: 0, max: 10, validate, onValidationChange },
    });
    const asyncControl = asyncCase.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    await fireEvent.input(asyncControl, { target: { value: "3" } });

    await waitFor(() => {
      expect(onValidationChange).toHaveBeenCalledWith(
        expect.objectContaining({ status: "invalid", valid: false, message: "Out of range" }),
      );
    });
    const field = asyncCase.container.querySelector<HTMLElement>(".poodle-number-input__field");
    expect(field?.getAttribute("data-validation-state")).toBe("invalid");
  });
});
