import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import NumberInput from "../src/NumberInput.svelte";

describe("NumberInput (svelte)", () => {
  it("emits committed numbers for valid drafts and null on clear", async () => {
    const onValueChange = vi.fn();
    const onDraftValueChange = vi.fn();
    const { container } = render(NumberInput, {
      props: { value: 5, onValueChange, onDraftValueChange },
    });
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    await fireEvent.input(control, { target: { value: "7" } });
    expect(onDraftValueChange).toHaveBeenLastCalledWith("7");
    expect(onValueChange).toHaveBeenLastCalledWith(7);

    await fireEvent.input(control, { target: { value: "" } });
    expect(onValueChange).toHaveBeenLastCalledWith(null);
  });

  it("keeps invalid drafts visible without emitting or clamping", async () => {
    const onValueChange = vi.fn();
    const onCommit = vi.fn();
    const { container } = render(NumberInput, {
      props: { value: 5, min: 0, max: 10, step: 2, onValueChange, onCommit },
    });
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    await fireEvent.input(control, { target: { value: "14" } });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(control.getAttribute("aria-invalid")).toBe("true");

    await fireEvent.blur(control);
    expect(onValueChange).not.toHaveBeenCalled();
    expect(onCommit).not.toHaveBeenCalled();
    expect(control.value).toBe("5");
  });

  it("steps with arrow keys, commits, and honours readOnly", async () => {
    const onValueChange = vi.fn();
    const onCommit = vi.fn();
    const { container } = render(NumberInput, {
      props: { defaultValue: 5, min: 0, max: 10, step: 1, onValueChange, onCommit },
    });
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    await fireEvent.keyDown(control, { key: "ArrowUp" });
    expect(onValueChange).toHaveBeenLastCalledWith(6);
    expect(onCommit).toHaveBeenLastCalledWith(6);

    await fireEvent.keyDown(control, { key: "ArrowDown" });
    expect(onValueChange).toHaveBeenLastCalledWith(5);
    expect(onCommit).toHaveBeenLastCalledWith(5);

    const readOnly = render(NumberInput, {
      props: { defaultValue: 5, readOnly: true, onValueChange, onCommit },
    });
    const readOnlyControl = readOnly.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    await fireEvent.keyDown(readOnlyControl, { key: "ArrowUp" });
    expect(onCommit).toHaveBeenCalledTimes(2);
  });

  it("renders prefix and suffix and preserves incomplete drafts", async () => {
    const onValueChange = vi.fn();
    const onDraftValueChange = vi.fn();
    const { container } = render(NumberInput, {
      props: { value: 2026, prefix: "FY", suffix: "kg", onValueChange, onDraftValueChange },
    });
    expect(container.querySelector(".poodle-number-input__prefix")?.textContent).toBe("FY");
    expect(container.querySelector(".poodle-number-input__suffix")?.textContent).toBe("kg");

    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    await fireEvent.input(control, { target: { value: "-" } });
    expect(onDraftValueChange).toHaveBeenLastCalledWith("-");
    expect(onValueChange).not.toHaveBeenCalled();
    expect(control.getAttribute("role")).toBe("spinbutton");
  });

  it("commits on Enter and reverts unresolved drafts on Escape", async () => {
    const onCommit = vi.fn();
    const onValueChange = vi.fn();
    const { container } = render(NumberInput, { props: { value: 5, onCommit, onValueChange } });
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    await fireEvent.keyDown(control, { key: "Enter" });
    expect(onCommit).toHaveBeenCalledWith(5);

    await fireEvent.input(control, { target: { value: "1e2" } });
    await fireEvent.keyDown(control, { key: "Escape" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(control.value).toBe("5");
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

  it("ignores a stale async validation result after clear", async () => {
    let release!: (result: { valid: boolean; message: string }) => void;
    const validate = vi.fn(
      () =>
        new Promise<{ valid: boolean; message: string }>((resolve) => {
          release = resolve;
        }),
    );
    const onValidationChange = vi.fn();
    const { container } = render(NumberInput, {
      props: { value: 5, min: 0, max: 10, validate, onValidationChange },
    });
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    await fireEvent.input(control, { target: { value: "3" } });
    await waitFor(() => expect(validate).toHaveBeenCalledTimes(1));
    await fireEvent.input(control, { target: { value: "" } });
    await waitFor(() => {
      expect(onValidationChange).toHaveBeenCalledWith(expect.objectContaining({ status: "idle" }));
    });

    release({ valid: false, message: "stale" });
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(onValidationChange).not.toHaveBeenCalledWith(
      expect.objectContaining({ status: "invalid", message: "stale" }),
    );
    const field = container.querySelector<HTMLElement>(".poodle-number-input__field");
    expect(field?.getAttribute("data-validation-state")).not.toBe("invalid");
  });
});
