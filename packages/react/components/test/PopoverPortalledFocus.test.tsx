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
 * never reaches the Select trigger/input key handler: without listbox
 * handling the dropdown would stay dangling open while focus leaves in
 * document order. The listbox therefore handles Tab itself, per the Select
 * contract (Tab closes the dropdown without changing value): it closes,
 * returns focus to the combobox trigger, and lets the Tab pass through
 * natively so it exits the control exactly like a Tab from the trigger.
 * No preventDefault anywhere on this path — the popover contract forbids
 * trapping Tab — and the exit blur consumes the skip-blur flag just like
 * the trigger path, so no later freeform commit is suppressed.
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
  it("closes the listbox and returns focus to the trigger on Tab from an option", () => {
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
    expect(prevented).toBe(true);
    expect(document.querySelector('[role="listbox"]')).toBeNull();
    expect(document.activeElement).toBe(trigger);
  });

  it("closes the listbox and returns focus to the trigger on shift+Tab from an option", () => {
    const { container } = renderHarness();
    const trigger = container.querySelector(".poodle-select__trigger") as HTMLElement;
    fireEvent.click(trigger);

    const listbox = document.querySelector('[role="listbox"]') as HTMLElement;
    const option = listbox.querySelector('[role="option"]') as HTMLElement;
    option.focus();

    const prevented = fireEvent.keyDown(option, { key: "Tab", shiftKey: true });
    expect(prevented).toBe(true);
    expect(document.querySelector('[role="listbox"]')).toBeNull();
    expect(document.activeElement).toBe(trigger);
  });

  it("does not trap: Tab from the trigger flows natively", () => {
    const { container } = renderHarness();
    const trigger = container.querySelector(".poodle-select__trigger") as HTMLElement;
    fireEvent.click(trigger);
    trigger.focus();

    // The trigger's own Tab handling closes without preventDefault, so the
    // key keeps its native pass-through: the non-modal popover never traps.
    const prevented = fireEvent.keyDown(trigger, { key: "Tab" });
    expect(prevented).toBe(true);
  });
});
