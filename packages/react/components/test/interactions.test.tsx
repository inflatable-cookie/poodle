import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Checkbox, NavigationMenu, Switch } from "../src";

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

  it("NavigationMenu defaults to tint fill with no active outline", () => {
    const { container } = render(<NavigationMenu items={[{ value: "a", label: "A" }]} />);
    const root = container.querySelector(".poodle-navigation-menu")!;
    expect(root.getAttribute("data-active-fill")).toBe("tint");
    expect(root.hasAttribute("data-active-outline")).toBe(false);
  });

  it("NavigationMenu emits activeOutline and solid fill data attributes", () => {
    const { container } = render(
      <NavigationMenu items={[{ value: "a", label: "A" }]} activeOutline activeFill="solid" />,
    );
    const root = container.querySelector(".poodle-navigation-menu")!;
    expect(root.getAttribute("data-active-outline")).toBe("true");
    expect(root.getAttribute("data-active-fill")).toBe("solid");
  });
});
