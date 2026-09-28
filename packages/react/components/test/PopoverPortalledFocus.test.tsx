import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Popover } from "../src/Popover";
import { Select } from "../src/Select";

const options = [
  { value: "alpha", label: "Alpha" },
  { value: "beta", label: "Beta" },
];

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
function renderHarness() {
  return render(
    <div data-poodle-theme-root>
      <Popover defaultOpen trigger="Open">
        <button type="button" data-testid="surface-action">
          Surface action
        </button>
        <Select options={options} native={false} ariaLabel="Pick" />
      </Popover>
    </div>,
  );
}

describe("Select listbox Tab in a Popover (react)", () => {
  it("routes Tab from a portalled option home and closes the listbox", () => {
    const { container } = renderHarness();
    const trigger = container.querySelector(".poodle-select__trigger") as HTMLElement;
    fireEvent.click(trigger);

    const listbox = document.querySelector('[role="listbox"]') as HTMLElement;
    expect(listbox).not.toBeNull();
    // Portalled out of the popover root: native Tab would leave the popover.
    const popover = container.querySelector(".poodle-popover") as HTMLElement;
    expect(popover.contains(listbox)).toBe(false);

    const option = listbox.querySelector('[role="option"]') as HTMLElement;
    option.focus();

    const prevented = fireEvent.keyDown(option, { key: "Tab" });
    expect(prevented).toBe(false);
    expect(document.querySelector('[role="listbox"]')).toBeNull();
    expect(document.activeElement).toBe(trigger);
  });

  it("routes shift+Tab from a portalled option home and closes the listbox", () => {
    const { container } = renderHarness();
    const trigger = container.querySelector(".poodle-select__trigger") as HTMLElement;
    fireEvent.click(trigger);

    const listbox = document.querySelector('[role="listbox"]') as HTMLElement;
    const option = listbox.querySelector('[role="option"]') as HTMLElement;
    option.focus();

    const prevented = fireEvent.keyDown(option, { key: "Tab", shiftKey: true });
    expect(prevented).toBe(false);
    expect(document.querySelector('[role="listbox"]')).toBeNull();
    expect(document.activeElement).toBe(trigger);
  });

  it("leaves in-surface Tab alone so the next key exits freely", () => {
    const { container } = renderHarness();
    const trigger = container.querySelector(".poodle-select__trigger") as HTMLElement;
    fireEvent.click(trigger);

    // The redirect lands in-surface, where keys flow natively: Tab from the
    // trigger passes through, proving nothing cycles.
    trigger.focus();
    const prevented = fireEvent.keyDown(trigger, { key: "Tab" });
    expect(prevented).toBe(true);
  });
});
