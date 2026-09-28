import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Popover } from "../src/Popover";
import { Select } from "../src/Select";

const options = [
  { value: "alpha", label: "Alpha" },
  { value: "beta", label: "Beta" },
];

/**
 * The Select listbox portals to the theme root, so Tab from an open
 * listbox never bubbles through the popover surface and no content-level
 * trap can see it. The Popover's document-level trap must hold those keys
 * inside the popover's own scope — trigger, surface, and portalled
 * descendants — for any portalled child, not just this pair.
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

function popoverScope(container: HTMLElement): HTMLElement {
  const scope = container.querySelector(".poodle-popover") as HTMLElement;
  if (!scope) throw new Error("popover root did not mount");
  return scope;
}

/**
 * The root, the portalled surface, and the portalled listbox together are
 * the popover's scope. The surface and the listbox both live at the theme
 * root, so neither is a descendant of the popover root — and after Tab the
 * Select closes, detaching the listbox while keeping its subtree addressable.
 */
function inScope(container: HTMLElement, listbox: HTMLElement, node: Node | null): boolean {
  if (node === null) return false;
  const surface = document.querySelector(".poodle-popover__surface");
  return (
    popoverScope(container).contains(node) ||
    (surface !== null && surface.contains(node)) ||
    listbox.contains(node)
  );
}

describe("Popover portalled focus trap (react)", () => {
  it("keeps forward Tab from a portalled listbox inside the popover", () => {
    const { container } = renderHarness();
    fireEvent.click(container.querySelector(".poodle-select__trigger") as HTMLElement);

    const listbox = document.querySelector('[role="listbox"]') as HTMLElement;
    expect(listbox).not.toBeNull();
    // Portalled out of the popover root: the trap cannot rely on ancestry.
    expect(popoverScope(container).contains(listbox)).toBe(false);

    const option = listbox.querySelector('[role="option"]') as HTMLElement;
    option.focus();
    expect(document.activeElement).toBe(option);

    const prevented = fireEvent.keyDown(option, { key: "Tab" });
    expect(prevented).toBe(false);
    expect(document.activeElement).not.toBe(option);
    expect(inScope(container, listbox, document.activeElement)).toBe(true);
  });

  it("keeps shift+Tab from a portalled listbox inside the popover", () => {
    const { container } = renderHarness();
    fireEvent.click(container.querySelector(".poodle-select__trigger") as HTMLElement);

    const listbox = document.querySelector('[role="listbox"]') as HTMLElement;
    const option = listbox.querySelector('[role="option"]') as HTMLElement;
    option.focus();

    const prevented = fireEvent.keyDown(option, { key: "Tab", shiftKey: true });
    expect(prevented).toBe(false);
    expect(document.activeElement).not.toBe(option);
    expect(inScope(container, listbox, document.activeElement)).toBe(true);
  });

  it("leaves in-surface Tab to the content's own traps", () => {
    const { container } = renderHarness();
    const action = container.querySelector('[data-testid="surface-action"]') as HTMLElement;
    action.focus();

    // Not prevented: the popover only intercepts Tabs from its portalled
    // descendants, never from inside its own root or surface.
    const prevented = fireEvent.keyDown(action, { key: "Tab" });
    expect(prevented).toBe(true);
  });
});
