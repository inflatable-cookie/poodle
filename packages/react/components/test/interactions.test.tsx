import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Checkbox, NavigationMenu, Switch, Tabs } from "../src";

// Interaction wiring: the @inflatable-cookie/poodle-core machines have their own suite; these
// assert the React binding actually drives a click through to the documented
// callback (the machine -> DOM -> event round trip).
describe("react interaction", () => {
  it("Checkbox fires onCheckedChange(true) on click", () => {
    const onCheckedChange = vi.fn();
    const { getByRole } = render(<Checkbox onCheckedChange={onCheckedChange} />);
    fireEvent.click(getByRole("checkbox"));
    expect(onCheckedChange).toHaveBeenCalledWith(true);
  });

  it("Switch fires onCheckedChange(true) on click", () => {
    const onCheckedChange = vi.fn();
    const { getByRole } = render(<Switch onCheckedChange={onCheckedChange} />);
    fireEvent.click(getByRole("switch"));
    expect(onCheckedChange).toHaveBeenCalledWith(true);
  });

  it("NavigationMenu defaults to tint fill with no edge", () => {
    const { container } = render(<NavigationMenu items={[{ value: "a", label: "A" }]} />);
    const root = container.querySelector(".poodle-navigation-menu")!;
    expect(root.getAttribute("data-active-fill")).toBe("tint");
    expect(root.getAttribute("data-active-edge")).toBe("none");
  });

  it("NavigationMenu emits activeEdge and solid fill data attributes", () => {
    const { container } = render(
      <NavigationMenu items={[{ value: "a", label: "A" }]} activeEdge="underline" activeFill="solid" />,
    );
    const root = container.querySelector(".poodle-navigation-menu")!;
    expect(root.getAttribute("data-active-edge")).toBe("underline");
    expect(root.getAttribute("data-active-fill")).toBe("solid");
  });

  it("NavigationMenu activeFill=none emits the no-fill attribute", () => {
    const { container } = render(
      <NavigationMenu items={[{ value: "a", label: "A" }]} value="a" activeEdge="underline" activeFill="none" />,
    );
    const root = container.querySelector(".poodle-navigation-menu")!;
    expect(root.getAttribute("data-active-fill")).toBe("none");
    expect(root.getAttribute("data-active-edge")).toBe("underline");
    const open = root.querySelector('.poodle-navigation-menu__trigger[data-open="true"]')!;
    expect(open).not.toBeNull();
  });

  it("Tabs defaults to card variant with tint fill, no edge and no border", () => {
    const { container } = render(<Tabs items={[{ value: "a", label: "A" }]} />);
    const root = container.querySelector(".poodle-tabs")!;
    expect(root.getAttribute("data-variant")).toBe("card");
    expect(root.getAttribute("data-active-fill")).toBe("tint");
    expect(root.getAttribute("data-active-edge")).toBe("none");
    expect(root.getAttribute("data-bordered")).toBe("false");
  });

  it("Tabs block + underline renders the underline edge", () => {
    const { container } = render(<Tabs items={[{ value: "a", label: "A" }]} variant="block" activeEdge="underline" />);
    const root = container.querySelector(".poodle-tabs")!;
    // The edge axis is a single enum member: exactly one value is emitted.
    expect(root.getAttribute("data-variant")).toBe("block");
    expect(root.getAttribute("data-active-edge")).toBe("underline");
  });

  it("Tabs activeFill=none suppresses the selected fill while the underline renders", () => {
    const { container } = render(
      <Tabs items={[{ value: "a", label: "A" }]} variant="block" activeEdge="underline" activeFill="none" />,
    );
    const root = container.querySelector(".poodle-tabs")!;
    expect(root.getAttribute("data-active-fill")).toBe("none");
    // The CSS suppression keys off root `[data-active-fill="none"]` paired
    // with the item's selected state; the underline still renders from
    // `data-active-edge="underline"`.
    expect(root.getAttribute("data-active-edge")).toBe("underline");
    const selected = root.querySelector('.poodle-tabs__item[data-selected="true"]')!;
    expect(selected).not.toBeNull();
    expect(selected.getAttribute("data-selected")).toBe("true");
  });
});
