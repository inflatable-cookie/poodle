import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import CodeInput from "../src/CodeInput.svelte";

describe("CodeInput grouping (svelte)", () => {
  it("marks every boundary in an explicit group pattern", () => {
    const { container } = render(CodeInput, {
      props: { length: 20, groups: [5, 5, 5, 5], numbersOnly: false },
    });
    const slots = [...container.querySelectorAll(".poodle-code-input__slot")];
    const ends = slots
      .map((slot, index) =>
        slot.classList.contains("poodle-code-input__slot--group-end") ? index : null,
      )
      .filter((index) => index !== null);

    expect(ends).toEqual([4, 9, 14]);
  });

  it("does not invent grouping when no pattern is supplied", () => {
    const { container } = render(CodeInput, { props: { length: 6 } });
    expect(container.querySelectorAll(".poodle-code-input__slot--group-end")).toHaveLength(0);
  });
});
