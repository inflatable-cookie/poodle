import { act, fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { NumberInput } from "../src/NumberInput";

describe("NumberInput (react)", () => {
  it("emits committed numbers for valid drafts and null on clear", () => {
    const onValueChange = vi.fn();
    const onDraftValueChange = vi.fn();
    const { container } = render(
      <NumberInput value={5} onValueChange={onValueChange} onDraftValueChange={onDraftValueChange} />,
    );
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    fireEvent.change(control, { target: { value: "7" } });
    expect(onDraftValueChange).toHaveBeenLastCalledWith("7");
    expect(onValueChange).toHaveBeenLastCalledWith(7);

    fireEvent.change(control, { target: { value: "" } });
    expect(onValueChange).toHaveBeenLastCalledWith(null);
  });

  it("keeps invalid drafts visible without emitting or clamping", () => {
    const onValueChange = vi.fn();
    const onCommit = vi.fn();
    const { container } = render(
      <NumberInput value={5} min={0} max={10} step={2} onValueChange={onValueChange} onCommit={onCommit} />,
    );
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    fireEvent.change(control, { target: { value: "14" } });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(control.getAttribute("aria-invalid")).toBe("true");

    fireEvent.blur(control);
    expect(onValueChange).not.toHaveBeenCalled();
    expect(onCommit).not.toHaveBeenCalled();
    expect(control.value).toBe("5");
  });

  it("steps with arrow keys, commits, and honours readOnly", () => {
    const onValueChange = vi.fn();
    const onCommit = vi.fn();
    const { container } = render(
      <NumberInput defaultValue={5} min={0} max={10} step={1} onValueChange={onValueChange} onCommit={onCommit} />,
    );
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    fireEvent.keyDown(control, { key: "ArrowUp" });
    expect(onValueChange).toHaveBeenLastCalledWith(6);
    expect(onCommit).toHaveBeenLastCalledWith(6);

    fireEvent.keyDown(control, { key: "ArrowDown" });
    expect(onValueChange).toHaveBeenLastCalledWith(5);
    expect(onCommit).toHaveBeenLastCalledWith(5);

    const readOnly = render(<NumberInput defaultValue={5} readOnly onValueChange={onValueChange} onCommit={onCommit} />);
    const readOnlyControl = readOnly.container.querySelector<HTMLInputElement>(
      ".poodle-number-input__control",
    ) as HTMLInputElement;
    fireEvent.keyDown(readOnlyControl, { key: "ArrowUp" });
    expect(onCommit).toHaveBeenCalledTimes(2);
  });

  it("renders prefix and suffix and preserves incomplete drafts", () => {
    const onValueChange = vi.fn();
    const onDraftValueChange = vi.fn();
    const { container } = render(
      <NumberInput value={2026} prefix="FY" suffix="kg" onValueChange={onValueChange} onDraftValueChange={onDraftValueChange} />,
    );
    expect(container.querySelector(".poodle-number-input__prefix")?.textContent).toBe("FY");
    expect(container.querySelector(".poodle-number-input__suffix")?.textContent).toBe("kg");

    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    fireEvent.change(control, { target: { value: "-" } });
    expect(onDraftValueChange).toHaveBeenLastCalledWith("-");
    expect(onValueChange).not.toHaveBeenCalled();
    expect(control.getAttribute("role")).toBe("spinbutton");
  });

  it("commits on Enter and reverts unresolved drafts on Escape", () => {
    const onCommit = vi.fn();
    const onValueChange = vi.fn();
    const { container } = render(<NumberInput value={5} onCommit={onCommit} onValueChange={onValueChange} />);
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    fireEvent.keyDown(control, { key: "Enter" });
    expect(onCommit).toHaveBeenCalledWith(5);

    fireEvent.change(control, { target: { value: "1e2" } });
    fireEvent.keyDown(control, { key: "Escape" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(control.value).toBe("5");
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

  it("omits aria-valuenow for a controlled empty draft", () => {
    const { container } = render(<NumberInput value={5} draftValue="" ariaLabel="Amount" />);
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;
    expect(control.value).toBe("");
    expect(control.getAttribute("aria-valuenow")).toBeNull();
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
    const { container } = render(
      <NumberInput value={5} min={0} max={10} validate={validate} onValidationChange={onValidationChange} />,
    );
    const control = container.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    fireEvent.change(control, { target: { value: "3" } });
    await waitFor(() => expect(validate).toHaveBeenCalledTimes(1));
    fireEvent.change(control, { target: { value: "" } });
    await waitFor(() => {
      expect(onValidationChange).toHaveBeenCalledWith(
        expect.objectContaining({ status: "idle" }),
      );
    });

    release({ valid: false, message: "stale" });
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(onValidationChange).not.toHaveBeenCalledWith(
      expect.objectContaining({ status: "invalid", message: "stale" }),
    );
    const field = container.querySelector<HTMLElement>(".poodle-number-input__field");
    expect(field?.getAttribute("data-validation-state")).not.toBe("invalid");
  });

  it("ignores a stale async validation result after external replacement", async () => {
    const releases: Array<(result: { valid: boolean; message: string }) => void> = [];
    const validate = vi.fn(
      () =>
        new Promise<{ valid: boolean; message: string }>((resolve) => {
          releases.push(resolve);
        }),
    );
    const onValidationChange = vi.fn();
    const { rerender } = render(
      <NumberInput value={5} min={0} max={10} validate={validate} onValidationChange={onValidationChange} />,
    );
    const control = document.querySelector<HTMLInputElement>(".poodle-number-input__control") as HTMLInputElement;

    fireEvent.change(control, { target: { value: "3" } });
    await waitFor(() => expect(releases).toHaveLength(1));

    rerender(
      <NumberInput value={4} min={0} max={10} validate={validate} onValidationChange={onValidationChange} />,
    );
    await waitFor(() => expect(releases).toHaveLength(2));
    expect(validate).toHaveBeenLastCalledWith("4", undefined);

    await act(async () => {
      releases[0]!({ valid: false, message: "stale" });
      await Promise.resolve();
    });
    expect(onValidationChange).not.toHaveBeenCalledWith(
      expect.objectContaining({ status: "invalid", message: "stale" }),
    );
  });
});
