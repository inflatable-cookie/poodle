import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Harness from "./PopoverPortalledFocusHarness.svelte";

/**
 * The Select listbox portals to the theme root, so a native Tab from an
 * option would move focus in document order — outside the popover — and
 * leave the listbox dangling open, because the Select trigger/input key
 * handler never sees the key. The listbox therefore handles Tab itself:
 * the dropdown closes (contract: Tab closes without changing value) and
 * focus returns to the combobox trigger inside the popover. A single stop,
 * not a trap — the popover contract forbids trapping, so Tabs from the
 * trigger onward flow natively.
 */
describe("Select listbox Tab in a Popover (svelte)", () => {
  it("closes the listbox and returns focus to the trigger on Tab from an option", async () => {
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

  it("closes the listbox and returns focus to the trigger on shift+Tab from an option", async () => {
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

  it("does not trap: Tab from the trigger flows natively", async () => {
    const { container } = render(Harness);
    const trigger = container.querySelector(".poodle-select__trigger") as HTMLButtonElement;
    await fireEvent.click(trigger);
    trigger.focus();

    // The trigger's own Tab handling closes without preventDefault, so the
    // key keeps its native pass-through: the non-modal popover never traps.
    const prevented = await fireEvent.keyDown(trigger, { key: "Tab" });
    expect(prevented).toBe(true);
  });
});
