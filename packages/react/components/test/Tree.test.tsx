import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Tree } from "../src/Tree";

// Mirrors packages/svelte/components/test/Tree.test.ts — row metadata renders
// an end label while muted rows remain selectable.
describe("Tree row metadata (react)", () => {
  it("renders an end label while muted rows remain selectable", () => {
    const onSelectionChange = vi.fn();
    const { getByRole, getByText } = render(
      <Tree
        nodes={[{ value: "empty", label: "Empty area", endLabel: "0", isMuted: true }]}
        onSelectionChange={onSelectionChange}
      />,
    );

    const row = getByRole("treeitem");
    expect(row.getAttribute("data-muted")).toBe("true");
    expect(row.hasAttribute("aria-disabled")).toBe(false);
    expect(getByText("0").classList.contains("poodle-tree__end-label")).toBe(true);

    fireEvent.click(row);
    expect(onSelectionChange).toHaveBeenCalledWith(["empty"]);
  });
});