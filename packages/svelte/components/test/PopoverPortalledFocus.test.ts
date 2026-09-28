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

describe("nested Popover portal Tab (svelte)", () => {
  it("routes Tab from a nested portalled surface to its trigger", async () => {
    const { container } = render(Harness);
    const triggers = container.querySelectorAll(".poodle-popover__trigger");
    const nestedTrigger = triggers[1] as HTMLElement;
    const action = container.querySelector('[data-testid="nested-action"]') as HTMLElement;
    expect(nestedTrigger).not.toBeUndefined();

    // The nested surface is portalled out of the outer root, but it is a
    // registered descendant of the outer scope, so the outer routing keeps
    // the key instead of yielding it to document order. Awaited first: the
    // mount-time initial-focus microtasks must flush before the key, or
    // they fire after the redirect and steal focus back.
    await fireEvent.focus(action);
    const prevented = await fireEvent.keyDown(action, { key: "Tab" });
    expect(prevented).toBe(false);
    expect(document.activeElement).toBe(nestedTrigger);
  });

  it("routes shift+Tab from a nested portalled surface to its trigger", async () => {
    const { container } = render(Harness);
    const nestedTrigger = container.querySelectorAll(".poodle-popover__trigger")[1] as HTMLElement;
    const action = container.querySelector('[data-testid="nested-action"]') as HTMLElement;

    await fireEvent.focus(action);
    const prevented = await fireEvent.keyDown(action, { key: "Tab", shiftKey: true });
    expect(prevented).toBe(false);
    expect(document.activeElement).toBe(nestedTrigger);
  });

  it("still passes Tab from the nested trigger through natively", async () => {
    const { container } = render(Harness);
    const nestedTrigger = container.querySelectorAll(".poodle-popover__trigger")[1] as HTMLElement;

    // In-surface keys are never routed: the redirect cannot loop back on
    // itself, so the follow-up key exits freely.
    await fireEvent.focus(nestedTrigger);
    const prevented = await fireEvent.keyDown(nestedTrigger, { key: "Tab" });
    expect(prevented).toBe(true);
  });
});
