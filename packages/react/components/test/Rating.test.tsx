import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Rating } from "../src/Rating";

describe("Rating (react)", () => {
  it("renders whole stars as radios with the current value checked", () => {
    const { container } = render(<Rating value={3} max={5} step={1} />);
    const items = [...container.querySelectorAll(".poodle-rating__item")];
    expect(items.length).toBe(5);
    expect(items[2].getAttribute("aria-checked")).toBe("true");
    expect(items[3].getAttribute("aria-checked")).toBe("false");
  });

  it("selects a star on click and reports the value", () => {
    const onValueChange = vi.fn();
    const { container } = render(<Rating max={5} step={1} onValueChange={onValueChange} />);
    const items = [...container.querySelectorAll(".poodle-rating__item")];

    fireEvent.click(items[3]);

    expect(onValueChange).toHaveBeenCalledWith(4);
    expect(items[3].getAttribute("aria-checked")).toBe("true");
  });

  it("renders a fractional slider with hidden pointer targets", () => {
    const { container } = render(<Rating value={2.5} max={5} step={0.5} />);
    const root = container.querySelector(".poodle-rating") as HTMLElement;
    expect(root.dataset.mode).toBe("fractional");
    expect(root.getAttribute("role")).toBe("slider");
    expect(root.getAttribute("aria-valuenow")).toBe("2.5");
    expect(container.querySelectorAll(".poodle-rating__item[aria-hidden]").length).toBe(5);
  });

  it("clears the rating when clicking the selected star with allowClear", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <Rating value={3} max={5} step={1} allowClear onValueChange={onValueChange} />,
    );
    const items = [...container.querySelectorAll(".poodle-rating__item")];

    fireEvent.click(items[2]);

    expect(onValueChange).toHaveBeenCalledWith(null);
  });

  it("does not respond while disabled", () => {
    const onValueChange = vi.fn();
    const { container } = render(<Rating max={5} step={1} disabled onValueChange={onValueChange} />);
    const items = [...container.querySelectorAll(".poodle-rating__item")];
    expect(items.every((el) => (el as HTMLButtonElement).disabled)).toBe(true);

    fireEvent.click(items[2]);
    expect(onValueChange).not.toHaveBeenCalled();
  });
});
