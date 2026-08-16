import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Box } from "../src/Box";

describe("Box (react)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-box") as HTMLElement;

  it("maps the padding scale to space tokens", () => {
    const root = rootOf(render(<Box padding="lg">x</Box>).container);
    expect(root.style.padding).toBe("var(--poodle-space-panel-x)");
    const none = rootOf(render(<Box>x</Box>).container);
    expect(["0", "0px"]).toContain(none.style.padding);
  });

  it("applies explicit width and height constraints inline", () => {
    const root = rootOf(render(<Box width="12rem" height="6rem">x</Box>).container);
    expect(root.style.width).toBe("12rem");
    expect(root.style.height).toBe("6rem");
  });

  it("projects the semantic role and label only when opted in", () => {
    const plain = rootOf(render(<Box>x</Box>).container);
    expect(plain.getAttribute("role")).toBeNull();

    const region = rootOf(render(<Box asRole="region" ariaLabel="Tools">x</Box>).container);
    expect(region.getAttribute("role")).toBe("region");
    expect(region.getAttribute("aria-label")).toBe("Tools");
  });
});
