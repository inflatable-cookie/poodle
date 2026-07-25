import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Tree from "../src/Tree.svelte";

describe("Tree row metadata", () => {
  it("renders an end label while muted rows remain selectable", async () => {
    const onSelectionChange = vi.fn();
    const { getByRole, getByText } = render(Tree, {
      props: {
        nodes: [{ value: "empty", label: "Empty area", endLabel: "0", isMuted: true }],
        onSelectionChange,
      },
    });

    const row = getByRole("treeitem");
    expect(row.getAttribute("data-muted")).toBe("true");
    expect(row.hasAttribute("aria-disabled")).toBe(false);
    expect(getByText("0").classList.contains("poodle-tree__end-label")).toBe(true);

    await fireEvent.click(row);
    expect(onSelectionChange).toHaveBeenCalledWith(["empty"]);
  });
});
