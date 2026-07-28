import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Checkbox from "../src/Checkbox.svelte";
import DockRegion from "../src/DockRegion.svelte";
import Switch from "../src/Switch.svelte";
import Tabs from "../src/Tabs.svelte";

// Interaction wiring: the @poodle/headless machines have their own suite; these
// assert the Svelte binding actually drives a click through to the documented
// callback (the machine -> DOM -> event round trip).
describe("svelte interaction", () => {
  it("Checkbox fires onCheckedChange(true) on click", async () => {
    const onCheckedChange = vi.fn();
    const { getByRole } = render(Checkbox, { props: { onCheckedChange } });
    await fireEvent.click(getByRole("checkbox"));
    expect(onCheckedChange).toHaveBeenCalledWith(true);
  });

  it("Switch fires onCheckedChange(true) on click", async () => {
    const onCheckedChange = vi.fn();
    const { getByRole } = render(Switch, { props: { onCheckedChange } });
    await fireEvent.click(getByRole("switch"));
    expect(onCheckedChange).toHaveBeenCalledWith(true);
  });

  it("Tabs reorders and preserves the dragged value through dragend", async () => {
    const onReorder = vi.fn();
    const onDragEnd = vi.fn();
    const onDragStart = vi.fn((_value: string, event: DragEvent) => {
      event.dataTransfer?.setData(
        "application/x-poodle-panel-drag",
        "panel-payload",
      );
    });
    const { getAllByRole } = render(Tabs, {
      props: {
        items: [
          { value: "surface-1", label: "Surface 1" },
          { value: "surface-2", label: "Surface 2" },
        ],
        reorderable: true,
        onReorder,
        onDragStart,
        onDragEnd,
      },
    });
    const [firstTab, secondTab] = getAllByRole("tab");
    const firstItem = firstTab.parentElement!;
    const secondItem = secondTab.parentElement!;
    const dataTransfer = new DataTransfer();

    await fireEvent.dragStart(firstItem, { dataTransfer });
    await fireEvent.dragOver(secondItem, { dataTransfer });
    await fireEvent.drop(secondItem, { dataTransfer });
    await fireEvent.dragEnd(firstItem, { dataTransfer });

    expect(dataTransfer.types).toContain("application/x-poodle-panel-drag");
    expect(onReorder).toHaveBeenCalledWith(["surface-2", "surface-1"]);
    expect(onDragEnd).toHaveBeenCalledWith("surface-1", expect.any(DragEvent));
  });

  it("DockRegion tab drags expose the panel transfer payload", async () => {
    const { getByRole } = render(DockRegion, {
      props: {
        edge: "left",
        collapsed: true,
        items: [{ value: "inspector", label: "Inspector" }],
        value: "inspector",
      },
    });
    const dataTransfer = new DataTransfer();

    await fireEvent.dragStart(getByRole("tab").parentElement!, {
      dataTransfer,
    });

    expect(
      JSON.parse(
        dataTransfer.getData("application/x-poodle-panel-drag"),
      ),
    ).toEqual({
      panelId: "inspector",
      sourceEdge: "left",
    });
  });
});
