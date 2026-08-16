import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Stack } from "../src/Stack";

describe("Stack (react)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-stack") as HTMLElement;

  it("applies direction and gap scale as inline styles", () => {
    const root = rootOf(render(<Stack direction="row" gap="lg">x</Stack>).container);
    expect(root.style.flexDirection).toBe("row");
    expect(root.style.gap).toBe("var(--poodle-space-panel-x)");
  });

  it("defaults column align to stretch and row align to center", () => {
    const column = rootOf(render(<Stack>x</Stack>).container);
    expect(column.style.alignItems).toBe("stretch");

    const row = rootOf(render(<Stack direction="row">x</Stack>).container);
    expect(row.style.alignItems).toBe("center");
  });

  it("maps justify and wrap props onto the flex container", () => {
    const root = rootOf(render(<Stack justify="between" wrap>x</Stack>).container);
    expect(root.style.justifyContent).toBe("space-between");
    expect(root.style.flexWrap).toBe("wrap");
  });

  it("projects the semantic role and merges a caller class", () => {
    const root = rootOf(
      render(
        <Stack asRole="region" ariaLabel="Toolbar" className="my-stack">
          x
        </Stack>,
      ).container,
    );
    expect(root.getAttribute("role")).toBe("region");
    expect(root.getAttribute("aria-label")).toBe("Toolbar");
    expect(root.className).toContain("my-stack");
  });
});
