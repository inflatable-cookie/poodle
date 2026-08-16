import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Radio from "../src/Radio.svelte";

describe("Radio (svelte)", () => {
  it("checks on selection and reports the change", async () => {
    const onCheckedChange = vi.fn();
    const { container } = render(Radio, {
      props: { name: "shipping", value: "standard", label: "Standard shipping", onCheckedChange },
    });
    const control = container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;
    expect(control.checked).toBe(false);

    await fireEvent.click(control);
    expect(control.checked).toBe(true);
    expect(onCheckedChange).toHaveBeenCalledWith(true);
    expect(container.querySelector(".poodle-radio__label")?.textContent).toBe("Standard shipping");
  });

  it("reflects the controlled checked state and does not re-report on re-click", async () => {
    const onCheckedChange = vi.fn();
    const { container } = render(Radio, {
      props: { checked: true, name: "shipping", value: "standard", label: "Standard shipping", onCheckedChange },
    });
    const control = container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;
    expect(control.checked).toBe(true);

    await fireEvent.click(control);
    expect(onCheckedChange).not.toHaveBeenCalled();
    expect(control.checked).toBe(true);
  });

  it("reverts the change without reporting while readonly", async () => {
    const onCheckedChange = vi.fn();
    const { container } = render(Radio, {
      props: { readOnly: true, name: "shipping", value: "standard", label: "Standard shipping", onCheckedChange },
    });
    const control = container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;

    await fireEvent.click(control);
    expect(control.checked).toBe(false);
    expect(onCheckedChange).not.toHaveBeenCalled();
  });

  it("blocks selection while disabled", async () => {
    const onCheckedChange = vi.fn();
    const { container } = render(Radio, {
      props: { disabled: true, name: "shipping", value: "standard", label: "Standard shipping", onCheckedChange },
    });
    const control = container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;
    expect(control.disabled).toBe(true);

    await fireEvent.click(control);
    expect(control.checked).toBe(false);
    expect(onCheckedChange).not.toHaveBeenCalled();
  });

  it("passes through id, name, and value and names the control via aria-label when label-less", () => {
    const labelled = render(Radio, {
      props: { id: "ship-standard", name: "shipping", value: "standard", label: "Standard shipping" },
    });
    const labelledControl = labelled.container.querySelector<HTMLInputElement>(".poodle-radio__control") as HTMLInputElement;
    expect(labelledControl.getAttribute("id")).toBe("ship-standard");
    expect(labelledControl.getAttribute("name")).toBe("shipping");
    expect(labelledControl.getAttribute("value")).toBe("standard");
    expect(labelledControl.getAttribute("aria-label")).toBeNull();

    const unlabelled = render(Radio, {
      props: { name: "shipping", value: "standard", ariaLabel: "Standard shipping" },
    });
    expect(
      unlabelled.container.querySelector<HTMLInputElement>(".poodle-radio__control")?.getAttribute("aria-label"),
    ).toBe("Standard shipping");
  });

  it("applies the selected color as a local custom property on the root", () => {
    const { container } = render(Radio, {
      props: { checked: true, selectedColor: "#7c3aed", label: "Standard shipping" },
    });
    const root = container.querySelector<HTMLElement>(".poodle-radio");
    expect(root?.getAttribute("style")).toContain("--poodle-radio-selected-color: #7c3aed");
  });
});
