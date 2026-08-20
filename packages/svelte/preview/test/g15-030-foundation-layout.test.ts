import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import ResizeHandleSpecimen from "../src/specimens/ResizeHandleSpecimen.svelte";

describe("g15.030 ResizeHandle specimen", () => {
  it("wires drag deltas into pane size", () => {
    const { container } = render(ResizeHandleSpecimen);
    const row = container.querySelector(".poodle-specimen__row");
    const leftPane = row?.querySelector(".poodle-specimen__pane") as HTMLElement | null;
    const handle = row?.querySelector(
      '.poodle-resize-handle[data-orientation="horizontal"]:not([data-disabled])',
    ) as HTMLElement | null;

    expect(leftPane).not.toBeNull();
    expect(handle).not.toBeNull();
    expect(leftPane?.style.flex).toBe("0 0 120px");

    fireEvent.keyDown(handle!, { key: "ArrowRight" });
    expect(leftPane?.style.flex).toBe("0 0 128px");
  });
});
