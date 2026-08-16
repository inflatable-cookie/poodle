import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Spacer } from "../src/Spacer";

describe("Spacer (react)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-spacer") as HTMLElement;

  it("applies the grow factor to the flex shorthand", () => {
    const root = rootOf(render(<Spacer grow={3} />).container);
    expect(root.style.flex).toBe("3 1 0%");
  });

  it("applies minSize as a two-axis floor and stays hidden from assistive tech", () => {
    const root = rootOf(render(<Spacer minSize="1rem" />).container);
    expect(root.style.minWidth).toBe("1rem");
    expect(root.style.minHeight).toBe("1rem");
    expect(root.getAttribute("aria-hidden")).toBe("true");
  });
});
