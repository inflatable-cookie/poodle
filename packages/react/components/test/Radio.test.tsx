import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Radio } from "../src/Radio";

describe("Radio (react)", () => {
  it("checks on selection and reports the change", () => {
    const onCheckedChange = vi.fn();
    const { container } = render(
      <Radio name="shipping" value="standard" label="Standard shipping" onCheckedChange={onCheckedChange} />,
    );
    const control = container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;
    expect(control.checked).toBe(false);

    fireEvent.click(control);
    expect(control.checked).toBe(true);
    expect(onCheckedChange).toHaveBeenCalledWith(true);
    expect(container.querySelector(".poodle-radio__label")?.textContent).toBe("Standard shipping");
  });

  it("reflects the controlled checked state and does not re-report on re-click", () => {
    const onCheckedChange = vi.fn();
    const { container } = render(
      <Radio checked name="shipping" value="standard" label="Standard shipping" onCheckedChange={onCheckedChange} />,
    );
    const control = container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;
    expect(control.checked).toBe(true);

    fireEvent.click(control);
    expect(onCheckedChange).not.toHaveBeenCalled();
    expect(control.checked).toBe(true);
  });

  it("reverts the change without reporting while readonly", () => {
    const onCheckedChange = vi.fn();
    const { container } = render(
      <Radio readOnly name="shipping" value="standard" label="Standard shipping" onCheckedChange={onCheckedChange} />,
    );
    const control = container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;

    fireEvent.click(control);
    expect(control.checked).toBe(false);
    expect(onCheckedChange).not.toHaveBeenCalled();
  });

  it("blocks selection while disabled", () => {
    const onCheckedChange = vi.fn();
    const { container } = render(
      <Radio disabled name="shipping" value="standard" label="Standard shipping" onCheckedChange={onCheckedChange} />,
    );
    const control = container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;
    expect(control.disabled).toBe(true);

    fireEvent.click(control);
    expect(control.checked).toBe(false);
    expect(onCheckedChange).not.toHaveBeenCalled();
  });

  it("passes through id, name, and value and names the control via aria-label when label-less", () => {
    const labelled = render(
      <Radio id="ship-standard" name="shipping" value="standard" label="Standard shipping" />,
    );
    const labelledControl = labelled.container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;
    expect(labelledControl.getAttribute("id")).toBe("ship-standard");
    expect(labelledControl.getAttribute("name")).toBe("shipping");
    expect(labelledControl.getAttribute("value")).toBe("standard");
    expect(labelledControl.getAttribute("aria-label")).toBeNull();

    const unlabelled = render(<Radio name="shipping" value="standard" ariaLabel="Standard shipping" />);
    expect(
      unlabelled.container.querySelector<HTMLInputElement>(".poodle-radio__control")?.getAttribute("aria-label"),
    ).toBe("Standard shipping");
  });

  it("applies the selected color as a local custom property on the root", () => {
    const { container } = render(
      <Radio checked selectedColor="#7c3aed" label="Standard shipping" />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-radio");
    expect(root?.getAttribute("style")).toContain("--poodle-radio-selected-color: #7c3aed");
  });
});
