import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import PopoverPackageHarness from "./PopoverPackageHarness.svelte";

describe("packed @inflatable-cookie/poodle-svelte Popover", () => {
  it("composes the state-aware interactive trigger with Button controls from the packed root", async () => {
    const view = render(PopoverPackageHarness);

    const wrapper = view.container.querySelector(
      ".poodle-popover__trigger",
    ) as HTMLElement;
    expect(wrapper.getAttribute("role")).toBeNull();
    expect(wrapper.getAttribute("tabindex")).toBeNull();

    const trigger = wrapper.querySelector("button.poodle-button") as HTMLButtonElement;
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(trigger.getAttribute("aria-controls")).toBeNull();

    await fireEvent.click(trigger);

    const surface = document.body.querySelector(
      '.poodle-popover__surface[data-part="surface"]',
    ) as HTMLElement;
    expect(surface).not.toBeNull();
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(trigger.getAttribute("aria-controls")).toBe(surface.id);

    view.unmount();
  });
});
