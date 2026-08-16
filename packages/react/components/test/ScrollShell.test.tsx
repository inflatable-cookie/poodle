import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { ScrollShell } from "../src/ScrollShell";

describe("ScrollShell (react)", () => {
  it("renders slotted content inside the viewport", () => {
    const { container } = render(<ScrollShell>Scrollable item</ScrollShell>);
    const content = container.querySelector(".poodle-scroll-shell__content")!;
    expect(content.textContent).toBe("Scrollable item");
  });

  it("maps direction to the owned overflow axis on the viewport", () => {
    const vertical = render(<ScrollShell direction="vertical" />);
    const vp = vertical.container.querySelector<HTMLElement>(".poodle-scroll-shell__viewport")!;
    expect(vp.getAttribute("style")).toContain("overflow-y: auto");
    expect(vp.getAttribute("style")).toContain("overflow-x: hidden");

    const horizontal = render(<ScrollShell direction="horizontal" />);
    const hp = horizontal.container.querySelector<HTMLElement>(".poodle-scroll-shell__viewport")!;
    expect(hp.getAttribute("style")).toContain("overflow-x: auto");
    expect(hp.getAttribute("style")).toContain("overflow-y: hidden");
    expect(horizontal.container.querySelector(".poodle-scroll-shell__content--h")).not.toBeNull();

    const both = render(<ScrollShell direction="both" />);
    expect(both.container.querySelector<HTMLElement>(".poodle-scroll-shell__viewport")!.getAttribute("style")).toContain(
      "overflow: auto",
    );
    expect(both.container.querySelector(".poodle-scroll-shell__content--h")).not.toBeNull();
  });

  it("joins the tab order with region semantics when focusable", () => {
    const { container } = render(<ScrollShell focusable />);
    const viewport = container.querySelector<HTMLElement>(".poodle-scroll-shell__viewport")!;
    expect(viewport.getAttribute("tabindex")).toBe("0");
    expect(viewport.getAttribute("role")).toBe("region");
    expect(viewport.getAttribute("aria-label")).toBe("Scrollable content");
  });

  it("applies explicit role and label overrides", () => {
    const { container } = render(<ScrollShell focusable asRole="group" label="Items" />);
    const viewport = container.querySelector<HTMLElement>(".poodle-scroll-shell__viewport")!;
    expect(viewport.getAttribute("role")).toBe("group");
    expect(viewport.getAttribute("aria-label")).toBe("Items");
  });

  it("forwards the native scroll event", () => {
    const onScroll = vi.fn();
    const { container } = render(<ScrollShell onScroll={onScroll} />);
    const viewport = container.querySelector<HTMLElement>(".poodle-scroll-shell__viewport")!;

    fireEvent.scroll(viewport);
    expect(onScroll).toHaveBeenCalledOnce();
    expect(onScroll.mock.calls[0][0].target).toBe(viewport);
  });
});
