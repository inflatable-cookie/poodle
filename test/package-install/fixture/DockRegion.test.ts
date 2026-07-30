import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import { DockRegion } from "@poodle/svelte";
import type {
  DockExternalDragSource,
  PanelTabItem,
} from "@poodle/svelte";

const items: PanelTabItem[] = [
  { value: "explorer", label: "Explorer" },
  { value: "inspector", label: "Inspector" },
];

describe("packed @poodle/svelte", () => {
  it("mounts the public drag seam and keeps local reorder", async () => {
    const onReorder = vi.fn();
    const end = vi.fn();
    const externalDragSource: DockExternalDragSource = {
      prepare: () => ({
        start: ({ dataTransfer }) => {
          dataTransfer.setData(
            "application/x-consumer-panel",
            "prepared-panel",
          );
        },
        end,
      }),
    };
    const { getAllByRole, getByRole } = render(DockRegion, {
      props: {
        items,
        value: "explorer",
        ariaLabel: "Consumer panels",
        onReorder,
        externalDragSource,
      },
    });
    const [firstTab, secondTab] = getAllByRole("tab");
    const dataTransfer = new DataTransfer();

    expect(getByRole("region", { name: "Consumer panels" })).toBeTruthy();
    await fireEvent.pointerDown(firstTab, { button: 0 });
    await fireEvent.dragStart(firstTab, { dataTransfer });
    await fireEvent.dragOver(secondTab, { dataTransfer });
    await fireEvent.drop(secondTab, { dataTransfer });
    await fireEvent.dragEnd(firstTab, { dataTransfer });

    expect(dataTransfer.getData("application/x-consumer-panel")).toBe(
      "prepared-panel",
    );
    expect(onReorder).toHaveBeenCalledWith(["inspector", "explorer"]);
    expect(end).toHaveBeenCalledOnce();
  });
});
