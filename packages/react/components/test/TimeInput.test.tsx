import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { TimeInput } from "../src/TimeInput";

function stubValidity(input: HTMLInputElement, badInput: boolean): void {
  Object.defineProperty(input, "validity", {
    configurable: true,
    get() {
      return { badInput };
    },
  });
}

describe("TimeInput (react)", () => {
  it("renders a native time input with forwarded constraints", () => {
    const { container } = render(<TimeInput min="08:00" max="18:00" step={300} ariaLabel="Office hours" />);
    const input = container.querySelector("input.poodle-time-input") as HTMLInputElement;
    expect(input.type).toBe("time");
    expect(input.min).toBe("08:00");
    expect(input.max).toBe("18:00");
    expect(input.getAttribute("step")).toBe("300");
    expect(input.getAttribute("aria-label")).toBe("Office hours");
  });

  it("seeds the uncontrolled value from defaultValue and reports changes", () => {
    const onValueChange = vi.fn();
    const { container } = render(<TimeInput defaultValue="14:30" onValueChange={onValueChange} />);
    const input = container.querySelector("input") as HTMLInputElement;
    expect(input.value).toBe("14:30");

    fireEvent.change(input, { target: { value: "09:15" } });
    expect(onValueChange).toHaveBeenCalledWith("09:15");
  });

  it("reports a cleared value as null", () => {
    const onValueChange = vi.fn();
    const { container } = render(<TimeInput defaultValue="14:30" onValueChange={onValueChange} />);
    const input = container.querySelector("input") as HTMLInputElement;

    fireEvent.change(input, { target: { value: "" } });
    expect(onValueChange).toHaveBeenCalledWith(null);
  });

  it("keeps a native incomplete draft local and reverts it without emitting", () => {
    const onValueChange = vi.fn();
    const { container } = render(<TimeInput defaultValue="14:30" onValueChange={onValueChange} />);
    const input = container.querySelector("input") as HTMLInputElement;
    stubValidity(input, true);

    fireEvent.change(input, { target: { value: "" } });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.getAttribute("aria-invalid")).toBe("true");

    fireEvent.blur(input);
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.value).toBe("14:30");
    expect(input.hasAttribute("aria-invalid")).toBe(false);

    stubValidity(input, true);
    fireEvent.change(input, { target: { value: "" } });
    fireEvent.keyDown(input, { key: "Escape" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.value).toBe("14:30");
  });

  it("stays controlled when value is supplied", () => {
    const onValueChange = vi.fn();
    const { container } = render(<TimeInput value="10:00" onValueChange={onValueChange} />);
    const input = container.querySelector("input") as HTMLInputElement;
    expect(input.value).toBe("10:00");

    fireEvent.change(input, { target: { value: "11:00" } });
    expect(onValueChange).toHaveBeenCalledWith("11:00");
  });

  it("disables the input and projects size and density", () => {
    const { container } = render(<TimeInput disabled size="lg" density="compact" />);
    const input = container.querySelector("input") as HTMLInputElement;
    expect(input.disabled).toBe(true);
    expect(input.dataset.size).toBe("lg");
    expect(input.dataset.density).toBe("compact");
  });

  it("does not emit off-step or out-of-range native values and marks them invalid", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <TimeInput defaultValue="09:00" step={300} min="08:00" max="18:00" onValueChange={onValueChange} />,
    );
    const input = container.querySelector("input") as HTMLInputElement;

    fireEvent.change(input, { target: { value: "09:07" } });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.getAttribute("aria-invalid")).toBe("true");
    expect(input.value).toBe("09:07");
  });

  it("reverts an invalid draft on blur and Escape without emitting", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <TimeInput defaultValue="09:00" step={300} min="08:00" max="18:00" onValueChange={onValueChange} />,
    );
    const input = container.querySelector("input") as HTMLInputElement;

    fireEvent.change(input, { target: { value: "09:07" } });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.getAttribute("aria-invalid")).toBe("true");

    fireEvent.blur(input);
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.value).toBe("09:00");
    expect(input.hasAttribute("aria-invalid")).toBe(false);

    fireEvent.change(input, { target: { value: "09:07" } });
    fireEvent.keyDown(input, { key: "Escape" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.value).toBe("09:00");
  });

  it("discards an invalid draft when the controlled value is replaced", () => {
    const onValueChange = vi.fn();
    const { container, rerender } = render(<TimeInput value="09:00" step={300} onValueChange={onValueChange} />);
    const input = container.querySelector("input") as HTMLInputElement;

    fireEvent.change(input, { target: { value: "09:07" } });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.getAttribute("aria-invalid")).toBe("true");

    rerender(<TimeInput value="11:00" step={300} onValueChange={onValueChange} />);
    expect(input.value).toBe("11:00");
    expect(input.hasAttribute("aria-invalid")).toBe(false);
    expect(onValueChange).not.toHaveBeenCalled();
  });
});