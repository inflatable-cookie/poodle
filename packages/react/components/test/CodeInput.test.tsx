import { fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { CodeInput } from "../src/CodeInput";

describe("CodeInput grouping (react)", () => {
  it("marks every boundary in an explicit group pattern", () => {
    const { container } = render(
      <CodeInput length={20} groups={[5, 5, 5, 5]} separator="-" numbersOnly={false} />,
    );
    const slots = [...container.querySelectorAll(".poodle-code-input__slot")];
    const ends = slots
      .map((slot, index) =>
        slot.classList.contains("poodle-code-input__slot--group-end") ? index : null,
      )
      .filter((index) => index !== null);

    expect(ends).toEqual([4, 9, 14]);
    expect(
      [...container.querySelectorAll(".poodle-code-input__separator")].map(
        (separator) => separator.textContent,
      ),
    ).toEqual(["-", "-", "-"]);
  });

  it("does not invent grouping when no pattern is supplied", () => {
    const { container } = render(<CodeInput length={6} />);
    expect(container.querySelectorAll(".poodle-code-input__slot--group-end")).toHaveLength(0);
    expect(container.querySelectorAll(".poodle-code-input__separator")).toHaveLength(0);
  });

  it("shows the completion validator result and clears it when incomplete", async () => {
    const validate = vi.fn((value: string) => ({ valid: value === "123456" }));
    const { container } = render(<CodeInput length={6} validate={validate} />);
    const input = container.querySelector<HTMLInputElement>(".poodle-code-input__control")!;

    fireEvent.change(input, { target: { value: "123456" } });
    await waitFor(() => expect(container.querySelector('[aria-label="Code check passed"]')).not.toBeNull());

    fireEvent.change(input, { target: { value: "654321" } });
    await waitFor(() => expect(container.querySelector('[aria-label="Code check failed"]')).not.toBeNull());

    fireEvent.change(input, { target: { value: "65432" } });
    expect(container.querySelector(".poodle-code-input__validation-indicator")).toBeNull();
    expect(validate).toHaveBeenCalledTimes(2);
  });

  it("ignores a completion response after the value changes", async () => {
    let resolveValidation!: (result: { valid: boolean }) => void;
    const validate = vi.fn(
      () => new Promise<{ valid: boolean }>((resolve) => (resolveValidation = resolve)),
    );
    const { container } = render(<CodeInput length={6} validate={validate} />);
    const input = container.querySelector<HTMLInputElement>(".poodle-code-input__control")!;

    fireEvent.change(input, { target: { value: "123456" } });
    fireEvent.change(input, { target: { value: "12345" } });
    resolveValidation({ valid: true });

    await Promise.resolve();
    expect(container.querySelector(".poodle-code-input__validation-indicator")).toBeNull();
  });
});
