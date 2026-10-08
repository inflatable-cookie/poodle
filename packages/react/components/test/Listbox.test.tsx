import axe from "axe-core";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Listbox } from "../src/Listbox";

const items = [
  { value: "alpha", label: "Alpha" },
  { value: "disabled", label: "Disabled", disabled: true },
  { value: "alpine", label: "Alpine" },
  { value: "beta", label: "Beta" },
];

describe("Listbox (react)", () => {
  it("renders listbox and option roles with selected state and a single roving tab stop", () => {
    const { getByRole, getAllByRole } = render(<Listbox items={items} defaultValue="alpine" ariaLabel="Libraries" />);
    expect(getByRole("listbox", { name: "Libraries" })).toBeTruthy();
    const options = getAllByRole("option");
    expect(options.map((option) => option.getAttribute("aria-selected"))).toEqual(["false", "false", "true", "false"]);
    expect(options.map((option) => option.getAttribute("tabindex"))).toEqual(["-1", "-1", "0", "-1"]);
    expect(options[1]?.getAttribute("aria-disabled")).toBe("true");
  });

  it("moves focus and selection through enabled options and invokes activation separately", () => {
    const onValueChange = vi.fn();
    const onActivate = vi.fn();
    const { getAllByRole } = render(<Listbox items={items} defaultValue="alpha" ariaLabel="Libraries" onValueChange={onValueChange} onActivate={onActivate} />);
    const options = getAllByRole("option");
    fireEvent.keyDown(options[0]!, { key: "ArrowDown" });
    expect(options[2]?.getAttribute("aria-selected")).toBe("true");
    expect(onValueChange).toHaveBeenCalledWith("alpine");
    fireEvent.keyDown(options[2]!, { key: "Enter" });
    expect(onActivate).toHaveBeenCalledWith("alpine");
  });

  it("uses the configured arrow axis for horizontal lists", () => {
    const onValueChange = vi.fn();
    const { getAllByRole } = render(<Listbox items={items} orientation="horizontal" defaultValue="alpha" ariaLabel="Libraries" onValueChange={onValueChange} />);
    const options = getAllByRole("option");
    fireEvent.keyDown(options[0]!, { key: "ArrowDown" });
    expect(onValueChange).not.toHaveBeenCalled();
    fireEvent.keyDown(options[0]!, { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenCalledWith("alpine");
  });

  it("does multiple selection by Space, range, select-all, and typeahead", () => {
    const onValuesChange = vi.fn();
    const { getAllByRole } = render(<Listbox items={items} selectionMode="multiple" defaultValues={["alpha"]} ariaLabel="Libraries" onValuesChange={onValuesChange} />);
    const options = getAllByRole("option");
    fireEvent.keyDown(options[0]!, { key: "ArrowDown", shiftKey: true });
    expect(onValuesChange).toHaveBeenLastCalledWith(["alpha", "alpine"]);
    fireEvent.keyDown(options[2]!, { key: "b" });
    expect(options[3]?.getAttribute("tabindex")).toBe("0");
    fireEvent.keyDown(options[3]!, { key: " " });
    expect(onValuesChange).toHaveBeenLastCalledWith(["alpha", "alpine", "beta"]);
    fireEvent.keyDown(options[3]!, { key: " " });
    expect(onValuesChange).toHaveBeenLastCalledWith(["alpha", "alpine"]);
    fireEvent.keyDown(options[3]!, { key: "a", ctrlKey: true });
    expect(onValuesChange).toHaveBeenLastCalledWith(["alpha", "alpine", "beta"]);
  });

  it("passes role and aria-selected checks", async () => {
    const { container } = render(<Listbox items={items} ariaLabel="Libraries" />);
    const results = await axe.run(container, {
      runOnly: { type: "rule", values: ["aria-required-attr", "aria-required-children", "aria-required-parent", "aria-roles", "aria-valid-attr-value"] },
    });
    expect(results.violations).toEqual([]);
  });
});
