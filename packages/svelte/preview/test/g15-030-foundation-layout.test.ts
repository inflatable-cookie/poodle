import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import ResizeHandleSpecimen from "../src/specimens/ResizeHandleSpecimen.svelte";

describe("g15.030 ResizeHandle specimen", () => {
  it("wires keyboard steps into pane size and aria range", () => {
    const { container } = render(ResizeHandleSpecimen);
    const row = container.querySelector(".poodle-specimen__row");
    const leftPane = row?.querySelector(".poodle-specimen__pane") as HTMLElement | null;
    const handle = row?.querySelector(
      '.poodle-resize-handle[data-orientation="horizontal"]:not([data-disabled])',
    ) as HTMLElement | null;

    expect(leftPane).not.toBeNull();
    expect(handle).not.toBeNull();
    expect(leftPane?.style.flex).toBe("0 0 120px");
    expect(handle?.getAttribute("aria-valuenow")).toBe("120");
    expect(handle?.getAttribute("aria-valuemin")).toBe("48");
    expect(handle?.getAttribute("aria-valuemax")).toBe("280");

    fireEvent.keyDown(handle!, { key: "ArrowRight" });
    expect(leftPane?.style.flex).toBe("0 0 128px");
    expect(handle?.getAttribute("aria-valuenow")).toBe("128");
  });

  it("passes vertical specimen bounds through aria range", () => {
    const { container } = render(ResizeHandleSpecimen);
    const column = container.querySelector(".poodle-specimen__col");
    const topPane = column?.querySelector(".poodle-specimen__pane") as HTMLElement | null;
    const handle = column?.querySelector(
      '.poodle-resize-handle[data-orientation="vertical"]:not([data-disabled])',
    ) as HTMLElement | null;

    expect(topPane).not.toBeNull();
    expect(handle).not.toBeNull();
    expect(topPane?.style.flex).toBe("0 0 80px");
    expect(handle?.getAttribute("aria-valuenow")).toBe("80");
    expect(handle?.getAttribute("aria-valuemin")).toBe("40");
    expect(handle?.getAttribute("aria-valuemax")).toBe("120");

    fireEvent.keyDown(handle!, { key: "ArrowDown" });
    expect(topPane?.style.flex).toBe("0 0 88px");
    expect(handle?.getAttribute("aria-valuenow")).toBe("88");
  });
});
