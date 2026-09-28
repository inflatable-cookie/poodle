import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Harness from "./PopoverPortalledFocusHarness.svelte";

/**
 * The Select listbox portals to the theme root, so a Tab from an option
 * never reaches the Select trigger/input key handler and never bubbles
 * through the popover surface: natively it would leave in document order
 * while the dropdown stays dangling open. The Select closes the dropdown
 * on the key, and the Popover routes portal-originated Tabs home — to the
 * control the portal opened from — for any portalled child, not just this
 * pair. Containment, not a trap: the destination is always in-surface,
 * where keys flow natively, so the follow-up Tab exits freely and nothing
 * can cycle.
 */
describe("Select listbox Tab in a Popover (svelte)", () => {
  it("routes Tab from a portalled option home and closes the listbox", async () => {
    const { container } = render(Harness);
    const trigger = container.querySelector(".poodle-select__trigger") as HTMLButtonElement;
    await fireEvent.click(trigger);

    const listbox = document.querySelector('[role="listbox"]') as HTMLElement;
    expect(listbox).not.toBeNull();
    // Portalled out of the popover root: native Tab would leave the popover.
    const popover = container.querySelector(".poodle-popover") as HTMLElement;
    expect(popover.contains(listbox)).toBe(false);

    const option = listbox.querySelector('[role="option"]') as HTMLElement;
    (option as HTMLElement).focus();

    const prevented = await fireEvent.keyDown(option, { key: "Tab" });
    expect(prevented).toBe(false);
    expect(document.querySelector('[role="listbox"]')).toBeNull();
    expect(document.activeElement).toBe(trigger);
  });

  it("routes shift+Tab from a portalled option home and closes the listbox", async () => {
    const { container } = render(Harness);
    const trigger = container.querySelector(".poodle-select__trigger") as HTMLButtonElement;
    await fireEvent.click(trigger);

    const listbox = document.querySelector('[role="listbox"]') as HTMLElement;
    const option = listbox.querySelector('[role="option"]') as HTMLElement;
    option.focus();

    const prevented = await fireEvent.keyDown(option, { key: "Tab", shiftKey: true });
    expect(prevented).toBe(false);
    expect(document.querySelector('[role="listbox"]')).toBeNull();
    expect(document.activeElement).toBe(trigger);
  });

  it("leaves in-surface Tab alone so the next key exits freely", async () => {
    const { container } = render(Harness);
    const trigger = container.querySelector(".poodle-select__trigger") as HTMLButtonElement;
    await fireEvent.click(trigger);

    // The redirect lands in-surface, where keys flow natively: Tab from the
    // trigger passes through, proving nothing cycles.
    trigger.focus();
    const prevented = await fireEvent.keyDown(trigger, { key: "Tab" });
    expect(prevented).toBe(true);
  });
});
