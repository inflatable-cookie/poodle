import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Tabs } from "../src/Tabs";

const items = [
  { value: "mix", label: "Mix" },
  { value: "master", label: "Master" },
  { value: "notes", label: "Notes" },
];

/**
 * g14.004 retained regression. Under manual activation an arrow key moves
 * focus without moving selection. React derived the roving tab stop from
 * selection on every render, so the focused tab kept `tabIndex=-1` — tabbing
 * back into the tablist landed on the selected tab and the operator's arrow
 * navigation was silently discarded.
 */
describe("Tabs (react) roving tab stop", () => {
  const tabStops = () =>
    screen.getAllByRole("tab").map((tab) => tab.getAttribute("tabindex"));

  it("moves the tab stop with manual arrow focus, not with selection", () => {
    render(<Tabs items={items} defaultValue="mix" activationMode="manual" />);

    expect(tabStops()).toEqual(["0", "-1", "-1"]);

    const first = screen.getAllByRole("tab")[0];
    first.focus();
    fireEvent.keyDown(first, { key: "ArrowRight" });

    // Selection has not moved…
    expect(screen.getAllByRole("tab").map((tab) => tab.getAttribute("aria-selected"))).toEqual([
      "true",
      "false",
      "false",
    ]);
    // …but the tab stop has followed focus.
    expect(tabStops()).toEqual(["-1", "0", "-1"]);
  });

  it("reseeds the tab stop from selection when selection changes", () => {
    const { rerender } = render(<Tabs items={items} value="mix" activationMode="manual" />);
    expect(tabStops()).toEqual(["0", "-1", "-1"]);

    rerender(<Tabs items={items} value="notes" activationMode="manual" />);
    expect(tabStops()).toEqual(["-1", "-1", "0"]);
  });
});
