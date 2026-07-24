import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Checkbox from "../src/Checkbox.svelte";
import Switch from "../src/Switch.svelte";

// Interaction wiring: the @poodle/headless machines have their own suite; these
// assert the Svelte binding actually drives a click through to the documented
// callback (the machine -> DOM -> event round trip).
describe("svelte interaction", () => {
  it("Checkbox fires onCheckedChange(true) on click", async () => {
    const onCheckedChange = vi.fn();
    const { getByRole } = render(Checkbox, { props: { onCheckedChange } });
    await fireEvent.click(getByRole("checkbox"));
    expect(onCheckedChange).toHaveBeenCalledWith(true);
  });

  it("Switch fires onCheckedChange(true) on click", async () => {
    const onCheckedChange = vi.fn();
    const { getByRole } = render(Switch, { props: { onCheckedChange } });
    await fireEvent.click(getByRole("switch"));
    expect(onCheckedChange).toHaveBeenCalledWith(true);
  });
});
