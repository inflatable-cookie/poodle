import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Rating from "../src/Rating.svelte";

describe("Rating (svelte)", () => {
  it("renders whole stars as radios with the current value checked", () => {
    const { container } = render(Rating, { props: { value: 3, max: 5, step: 1 } });
    const items = [...container.querySelectorAll(".poodle-rating__item")];
    expect(items.length).toBe(5);
    expect(items[2].getAttribute("aria-checked")).toBe("true");
    expect(items[3].getAttribute("aria-checked")).toBe("false");
  });

  it("selects a star on click and reports the value", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Rating, { props: { max: 5, step: 1, onValueChange } });
    const items = [...container.querySelectorAll(".poodle-rating__item")];

    await fireEvent.click(items[3]);

    expect(onValueChange).toHaveBeenCalledWith(4);
    expect(items[3].getAttribute("aria-checked")).toBe("true");
  });

  it("renders a fractional slider with hidden pointer targets", () => {
    const { container } = render(Rating, { props: { value: 2.5, max: 5, step: 0.5 } });
    const root = container.querySelector(".poodle-rating") as HTMLElement;
    expect(root.dataset.mode).toBe("fractional");
    expect(root.getAttribute("role")).toBe("slider");
    expect(root.getAttribute("aria-valuenow")).toBe("2.5");
    expect(container.querySelectorAll(".poodle-rating__item[aria-hidden]").length).toBe(5);
  });

  it("clears the rating when clicking the selected star with allowClear", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Rating, {
      props: { value: 3, max: 5, step: 1, allowClear: true, onValueChange },
    });
    const items = [...container.querySelectorAll(".poodle-rating__item")];

    await fireEvent.click(items[2]);

    expect(onValueChange).toHaveBeenCalledWith(null);
  });

  it("does not respond while disabled", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Rating, {
      props: { max: 5, step: 1, disabled: true, onValueChange },
    });
    const items = [...container.querySelectorAll(".poodle-rating__item")];
    expect(items.every((el) => (el as HTMLButtonElement).disabled)).toBe(true);

    await fireEvent.click(items[2]);
    expect(onValueChange).not.toHaveBeenCalled();
  });
});
