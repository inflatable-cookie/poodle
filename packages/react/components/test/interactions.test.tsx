import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Checkbox, Switch } from "../src";

// Interaction wiring: the @inflatable-cookie/poodle-headless machines have their own suite; these
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
});
